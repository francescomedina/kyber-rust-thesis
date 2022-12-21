#![no_main]
#![no_std]

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use core::convert::TryInto;
use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use pqc_kyber::{decapsulate, encapsulate, keypair, Keypair, KYBER_K, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES};
use pqc_kyber::Uake;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::mem::size_of_val;
use cortex_m::peripheral::syst::RegisterBlock;

use core::cell::RefCell;
use core::i8::MIN;

use cortex_m::{
    interrupt::Mutex,
    peripheral::{syst::SystClkSource, SYST},
};
use cortex_m::peripheral::syst;
use cortex_m::register::primask::read;
use stm32f3xx_hal::{
    pac,
    prelude::*,
    rcc::Clocks,
};

const QINV: i32 = 62209;
const KYBER_Q: usize = 3329;
const KYBER_N: usize = 256;
const KYBER_ETA1: usize =  3;

mod kyber_lib;

use kyber_lib::{sha3_512,shake256};

#[derive(Clone)]
pub struct Polyvec {
    pub vec: [Poly; KYBER_K]
}

impl Copy for Polyvec {}

impl Polyvec {
    pub fn new() -> Self {
        Polyvec {
            vec: [Poly::new(); KYBER_K]
        }
    }

    // #[cfg(debug_assertions)]
    // pub fn checksum(&self) -> i16 {
    //   let mut out = 0i16;
    //   for i in 0..KYBER_K {
    //     for j in 0..KYBER_N {
    //       out ^= &self.vec[i].coeffs[j]
    //     }
    //   }
    //   out
    // }
}

fn load24_littleendian(x: &[u8]) -> u32
{
    let mut r = x[0] as u32;
    r |= (x[1] as u32) << 8;
    r |= (x[2] as u32) << 16;
    r
}

#[derive(Clone)]
pub struct Poly {
    pub coeffs: [i16; KYBER_N]
}

impl Copy for Poly {}

impl Default for Poly {
    fn default() -> Self {
        Poly {
            coeffs: [0i16; KYBER_N]
        }
    }
}

// new() is nicer
impl Poly {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn cbd3(r: &mut Poly, buf: &[u8])
{
    let (mut d, mut t, mut a, mut b);
    for i in 0..(KYBER_N/4) {
        t = load24_littleendian(&buf[3*i..]);
        d  = t & 0x00249249;
        d += (t>>1) & 0x00249249;
        d += (t>>2) & 0x00249249;
        for j in 0..4 {
            a = ((d >>  (6*j))  & 0x7) as i16;
            b = ((d >> (6*j+3)) & 0x7) as i16;
            r.coeffs[4*i+j] = a - b;
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomRng(u64);

impl RngCore for CustomRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for CustomRng {
}

#[link_section = ".ccrambss"]
static mut ALICE: Uake = Uake {
    shared_secret: [0u8; KYBER_SSBYTES],
    send_a: [0u8; UAKE_INIT_BYTES],
    send_b: [0u8; UAKE_RESPONSE_BYTES],
    temp_key: [0u8; KYBER_SSBYTES],
    eska: [0u8; KYBER_SECRETKEYBYTES],
};

struct Instant {
    clock_ticks: u64,
    milliseconds: u64
}

struct TickCounter {
    #[allow(dead_code)]
    /// keep syst as a resource
    syst: SYST,
    period: u64,
    clock_ticks: u64,
}

impl TickCounter{
    pub fn new(mut syst: SYST, clocks: &Clocks) -> TickCounter {
        syst.set_clock_source(SystClkSource::Core);
        let fraction: u32 = 100_000;
        hprintln!("Clock {:?}", clocks.sysclk().0);
        hprintln!("Launch an interrupt every {:?} ticks", clocks.sysclk().0 / fraction - 1);

        // per ms counter
        // syst.set_reload(clocks.sysclk().0 / 1_000 - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.set_reload(clocks.sysclk().0 / fraction - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        TickCounter {
            syst,
            period: clocks.sysclk().0 as u64 / fraction as u64,
            clock_ticks: 0
        }
    }

    pub fn clock_tick(&mut self) {
        self.clock_ticks += self.period;
    }

    pub fn get_milliseconds(&self) -> u64 {
        self.clock_ticks / self.period + ((self.syst.cvr.read() as u64) / self.period)
    }

    pub fn get_clock_ticks(&self) -> u64 {
        self.clock_ticks + (self.syst.cvr.read() as u64)
    }

    pub fn reset(&mut self) {
        self.clock_ticks = 0;
    }
}

static TICK_COUNTER: Mutex<RefCell<Option<TickCounter>>> = Mutex::new(RefCell::new(None));

fn instant() -> Instant {
    cortex_m::interrupt::free(|cs| {
        if let Some(counter) = &*TICK_COUNTER.borrow(cs).borrow_mut() {
            Instant {
                clock_ticks: counter.get_clock_ticks(),
                milliseconds: counter.get_milliseconds()
            }
        } else {
            Instant {
                clock_ticks: 0,
                milliseconds: 0
            }
        }
    })
}

fn reset() {
    cortex_m::interrupt::free(|cs| {
        if let Some(counter) = &mut*TICK_COUNTER.borrow(cs).borrow_mut() {
            counter.reset();
        }
    })
}

// fn sleepms(ms: u64) {
//     let deadline = millis() + ms;
//     while millis() < deadline {
//         cortex_m::asm::wfi();
//     }
// }

// #[link_section= ".ccrambss"]
// static mut KEYPAIR: Keypair = Keypair {
//     public: [0u8; KYBER_PUBLICKEYBYTES],
//     secret: [0u8; KYBER_SECRETKEYBYTES]
// };

pub fn poly_cbd_eta1(r: &mut Poly, buf: &[u8])
{
    cbd3(r, buf)
    //cbd2(r, buf)
}

fn shake256_prf(output: &mut[u8], outlen: usize, key: &[u8], nonce: u8)
{
    let mut extkey = [0u8; KYBER_SYMBYTES+1];
    extkey[..KYBER_SYMBYTES].copy_from_slice(key);
    extkey[KYBER_SYMBYTES] = nonce;
    shake256(output, outlen, &extkey, KYBER_SYMBYTES + 1);
}

fn prf(out: &mut[u8], outbytes: usize, key: &[u8], nonce: u8)
{
    shake256_prf(out, outbytes, &key, nonce);
}

pub fn poly_getnoise_eta1(r: &mut Poly, seed: &[u8], nonce: u8)
{
    const LENGTH: usize = KYBER_ETA1*KYBER_N/4;
    let mut buf = [0u8; LENGTH];
    prf(&mut buf, LENGTH, seed, nonce);
    poly_cbd_eta1(r, &buf);
}


const KYBER_SYMBYTES: usize = 32;

pub const ZETAS: [i16; 128] = [
    -1044,  -758,  -359, -1517,  1493,  1422,   287,   202,
    -171,   622,  1577,   182,   962, -1202, -1474,  1468,
    573, -1325,   264,   383,  -829,  1458, -1602,  -130,
    -681,  1017,   732,   608, -1542,   411,  -205, -1571,
    1223,   652,  -552,  1015, -1293,  1491,  -282, -1544,
    516,    -8,  -320,  -666, -1618, -1162,   126,  1469,
    -853,   -90,  -271,   830,   107, -1421,  -247,  -951,
    -398,   961, -1508,  -725,   448, -1065,   677, -1275,
    -1103,   430,   555,   843, -1251,   871,  1550,   105,
    422,   587,   177,  -235,  -291,  -460,  1574,  1653,
    -246,   778,  1159,  -147,  -777,  1483,  -602,  1119,
    -1590,   644,  -872,   349,   418,   329,  -156,   -75,
    817,  1097,   603,   610,  1322, -1285, -1465,   384,
    -1215,  -136,  1218, -1335,  -874,   220, -1187, -1659,
    -1185, -1530, -1278,   794, -1510,  -854,  -870,   478,
    -108,  -308,   996,   991,   958, -1460,  1522,  1628
];

pub fn montgomery_reduce(a: i32) -> i16 {
    let ua = a.wrapping_mul(QINV) as i16;
    let u = ua as i32;
    let mut t = u * KYBER_Q as i32;
    t = a - t;
    t >>= 16;
    t as i16
}

pub fn barrett_reduce(a: i16) -> i16
{
    let v = ((1u32 << 26)/KYBER_Q as u32 + 1) as i32;
    let mut t = v * a as i32 + (1 << 25);
    t >>= 26;
    t *= KYBER_Q as i32;
    a - t as i16
}

pub fn fqmul(a: i16, b: i16) -> i16 {
    montgomery_reduce(a as i32 * b as i32)
}

pub fn ntt(r: &mut[i16])
{
    let mut j;
    let mut k = 1usize;
    let mut len = 128;
    let (mut t, mut zeta);

    while len >= 2 {
        let mut start = 0;
        while start < 256 {
            zeta = ZETAS[k];
            k += 1;
            j = start;
            while j < (start + len) {
                t = fqmul(zeta, r[j + len]);
                r[j + len] = r[j] - t;
                r[j] += t;
                j += 1;
            }
            start = j + len;
        }
        len >>= 1;
    }
}

pub fn invntt(r: &mut[i16])
{
    let mut j;
    let mut k = 127usize;
    let mut len = 2;
    let (mut t, mut zeta);
    const F: i16 = 1441; // mont^2/128
    while len <= 128 {
        let mut start = 0;
        while start < 256 {
            zeta = ZETAS[k];
            k -= 1;
            j = start;
            while j < (start + len) {
                t = r[j];
                r[j] = barrett_reduce(t + r[j + len]);
                r[j + len] = r[j + len] -t;
                r[j + len] = fqmul(zeta, r[j + len]);
                j += 1
            }
            start = j + len;
        }
        len <<= 1;
    }
    for j in 0..256 {
        r[j] = fqmul(r[j], F);
    }
}

fn hash_g(out: &mut[u8], input: &[u8], inlen: usize)
{
    sha3_512(out, input, inlen);
}

pub fn randombytes<R>(x: &mut [u8], len: usize, rng: &mut R)
    where R: RngCore + CryptoRng,
{
    rng.fill_bytes(&mut x[..len]);
}

pub fn rand_with_range(a: i16, b: i16, r: &mut[i16]){
    // let mut rng = rand::thread_rng();
    for i in 0..r.len() {
        let mut small_rng = SmallRng::seed_from_u64(47324547 + (i as u64 * 12));
        r[i] = small_rng.gen_range(a..(b+1));
    }
}

#[entry]
unsafe fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(8.MHz())
        .freeze(&mut flash.acr);

    let core_periphs = cortex_m::Peripherals::take().unwrap();

    cortex_m::interrupt::free(|cs| {
        TICK_COUNTER
        .borrow(cs)
        .borrow_mut()
        .replace(TickCounter::new(core_periphs.SYST, &clocks))
    });

    let (mut e, mut pkpv, mut skpv) = (Polyvec::new(), Polyvec::new(), Polyvec::new());
    // let mut ntt_max_clock = 0;
    // let mut ntt_min_clock = u64::MAX;
    // let mut ntt_sum = 0;
    // let mut ntt_ticks: _;
    // let mut ntt_millis = 0;

    let mut m_max_clock = 0;
    let mut m_min_clock = u64::MAX;
    let mut m_sum = 0;
    let mut m_ticks = 0;
    let mut m_millis = 0;
    let max = 1;
    for j in 0..max {
        rand_with_range(-(KYBER_Q as i16 -1)/2,(KYBER_Q as i16 -1)/2,&mut skpv.vec[0].coeffs);
        // rand_with_range(-3,3,&mut skpv.vec[0].coeffs);
        // ntt(&mut skpv.vec[0].coeffs);
        for i in 0..KYBER_N {
            let mut z = i;
            if i > 127 {
                z = 255 - i;
            }
            let input = ZETAS[z] as i32 * skpv.vec[0].coeffs[i] as i32;
            let inst0 = instant();
            montgomery_reduce(input);
            let inst1 = instant();
            m_ticks = inst1.clock_ticks-inst0.clock_ticks;
            m_millis = inst1.milliseconds-inst0.milliseconds;
            m_sum += m_ticks;
            if m_max_clock < m_ticks {
                // hprintln!("{} {} {} {}", j, skpv.vec[0].coeffs[i], ZETAS[z], m_ticks);
                m_max_clock = m_ticks;
            }
            if m_min_clock > m_ticks {
                // hprintln!("{} {} {} {}", j, skpv.vec[0].coeffs[i], ZETAS[z], m_ticks);
                m_min_clock = m_ticks;
            }
        }
        hprintln!("NTT: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", m_millis, m_min_clock, m_max_clock, m_sum/256);
        m_sum = 0;
    }

/*    let mut rng = CustomRng(2 as u64);
    let inst0 = instant();
    let bob_keys = keypair(&mut rng);
    let inst1 = instant();
    // let mut bob = ALICE.clone();
    // let inst2 = instant();
    // let client_init = ALICE.client_init(&bob_keys.public, &mut rng);
    // let inst3 = instant();
    // let server_response = bob.server_receive(client_init, &bob_keys.secret, &mut rng).unwrap();
    // let inst4 = instant();
    // ALICE.client_confirm(server_response);
    // let inst5 = instant();
    let keypair_ticks = inst1.clock_ticks-inst0.clock_ticks;
    // let client_init_ticks = inst3.clock_ticks-inst2.clock_ticks;
    // let server_receive_ticks = inst4.clock_ticks-inst3.clock_ticks;
    // let client_confirm_ticks = inst5.clock_ticks-inst4.clock_ticks;
    let keypair_millis = inst1.milliseconds-inst0.milliseconds;
    // let client_init_millis = inst3.milliseconds-inst2.milliseconds;
    // let server_receive_millis = inst4.milliseconds-inst3.milliseconds;
    // let client_confirm_millis = inst5.milliseconds-inst4.milliseconds;
    hprintln!("keypair_ticks; {:?}", keypair_ticks);
    // hprintln!("client_init_ticks; {:?}", client_init_ticks);
    // hprintln!("server_receive_ticks; {:?}", server_receive_ticks);
    // hprintln!("client_confirm_ticks; {:?}", client_confirm_ticks);

    hprintln!("keypair_millis-Millis; {:?}", keypair_millis);
    // hprintln!("client_init_millis-Millis; {:?}", client_init_millis);
    // hprintln!("server_receive_millis-Millis; {:?}", server_receive_millis);
    // hprintln!("client_confirm_millis-Millis; {:?}", client_confirm_millis);
    // hprintln!("Alice {:?}", ALICE.shared_secret);
    // hprintln!("Bob {:?}", bob.shared_secret);*/
  /*  let mut k_max_clock = 0;
    let mut k_min_clock = u64::MAX;
    let mut e_max_clock = 0;
    let mut e_min_clock = u64::MAX;
    let mut d_max_clock = 0;
    let mut d_min_clock = u64::MAX;
    let mut ntt_max_clock = 0;
    let mut ntt_min_clock = u64::MAX;
    let mut k_sum = 0;
    let mut e_sum = 0;
    let mut d_sum = 0;
    let mut ntt_sum = 0;

    let mut k_ticks;
    let mut e_ticks;
    let mut d_ticks;
    let mut ntt_ticks;
    let mut k_millis = 0;
    let mut e_millis = 0;
    let mut d_millis = 0;
    let mut ntt_millis = 0;
    for i in 0..1000 {
        let mut rng = CustomRng(i+k_min_clock as u64);
        let inst1 = instant();
        KEYPAIR = keypair(&mut rng);
        let inst2 = instant();
        let (ciphertext, shared_secret_alice) = encapsulate(&KEYPAIR.public, &mut rng).unwrap();
        let inst3 = instant();
        let shared_secret_bob = decapsulate(&ciphertext, &KEYPAIR.secret).unwrap();
        let inst4 = instant();

        let mut buf = [0u8; 2*KYBER_SYMBYTES];
        let mut randbuf = [0u8; 2*KYBER_SYMBYTES];
        randombytes(&mut randbuf, KYBER_SYMBYTES, &mut rng);
        hash_g(&mut buf, &randbuf, KYBER_SYMBYTES);
        let (publicseed, noiseseed) = buf.split_at(KYBER_SYMBYTES);
        let mut skpv = Polyvec::new();
        poly_getnoise_eta1(&mut skpv.vec[0], noiseseed, 0);
        let inst5 = instant();
        ntt(&mut skpv.vec[0].coeffs);
        let inst6 = instant();

        k_ticks = inst2.clock_ticks-inst1.clock_ticks;
        e_ticks = inst3.clock_ticks-inst2.clock_ticks;
        d_ticks = inst4.clock_ticks-inst3.clock_ticks;
        ntt_ticks = inst6.clock_ticks-inst5.clock_ticks;
        k_sum += k_ticks;
        e_sum += e_ticks;
        d_sum += d_ticks;
        ntt_sum += ntt_ticks;
        if k_max_clock < k_ticks {
            k_max_clock = k_ticks;
        }
        if e_max_clock < e_ticks {
            e_max_clock = e_ticks;
        }
        if d_max_clock < d_ticks {
            d_max_clock = d_ticks;
        }
        if ntt_max_clock < ntt_ticks {
            ntt_max_clock = ntt_ticks;
        }
        if k_min_clock > k_ticks {
            k_min_clock = k_ticks;
        }
        if e_min_clock > e_ticks {
            e_min_clock = e_ticks;
        }
        if d_min_clock > d_ticks {
            d_min_clock = d_ticks;
        }
        if ntt_min_clock > ntt_ticks {
            ntt_min_clock = ntt_ticks;
        }
        k_millis = inst2.milliseconds-inst1.milliseconds;
        e_millis = inst3.milliseconds-inst2.milliseconds;
        d_millis = inst4.milliseconds-inst3.milliseconds;
        ntt_millis = inst6.milliseconds-inst5.milliseconds;
    }
    hprintln!("Keypair: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", k_millis, k_min_clock, k_max_clock, k_sum/1000);
    hprintln!("Encapsulate: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", e_millis, e_min_clock, e_max_clock, e_sum/1000);
    hprintln!("Decapsulate: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", d_millis, d_min_clock, d_max_clock, d_sum/1000);
    hprintln!("NTT: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", ntt_millis, ntt_min_clock, ntt_max_clock, ntt_sum/1000);*/
    // hprintln!("Alice {:?}", shared_secret_alice);
    // hprintln!("Bob {:?}", shared_secret_bob);
    loop {}
}

#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        if let Some(counter) = &mut *TICK_COUNTER.borrow(cs).borrow_mut() {
            counter.clock_tick();
        }
    })
}