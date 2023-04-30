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
        let fraction: u32 = 100_000; // 24_000_000 / 100_000 => each tick is a nanosecond, after the tick the counter is reloaded
        //let fraction: u32 = 1_000_000; // 8_000_000 / 100_000 => each tick is a nanosecond
        hprintln!("Launch an interrupt every {:?} ticks", clocks.sysclk().0 / fraction - 1);

        // configures the system timer to trigger a SysTick exception every (24_000_000 / x) second
        // syst.set_reload(clocks.sysclk().0 / 1_000 - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.set_reload(clocks.sysclk().0 / fraction - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        hprintln!("Current {:?} ticks", SYST::get_current());
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        hprintln!("Current {:?} ticks", SYST::get_current());
        hprintln!("Reload after {:?} ticks", SYST::get_reload());
        hprintln!("10 milliseconds are {:?} ticks", SYST::get_ticks_per_10ms());
        hprintln!("Current {:?} ticks", SYST::get_current());
        hprintln!("Is precise ? {:?}", SYST::is_precise());

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
        self.clock_ticks / self.period
            // + ((self.syst.cvr.read() as u64) / self.period)
    }

    pub fn get_clock_ticks(&self) -> u64 {
        self.clock_ticks
            // + (self.syst.cvr.read() as u64)
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

#[entry]
unsafe fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(24.MHz())
        .freeze(&mut flash.acr);

    let core_periphs = cortex_m::Peripherals::take().unwrap();

    cortex_m::interrupt::free(|cs| {
        TICK_COUNTER
        .borrow(cs)
        .borrow_mut()
        .replace(TickCounter::new(core_periphs.SYST, &clocks))
    });

    let mut rng = CustomRng(2 as u64);
    for _ in 0..10 {
        let inst0 = instant();
        keypair(&mut rng);
        let inst1 = instant();
        let keypair_ticks = inst1.clock_ticks-inst0.clock_ticks;
        let keypair_millis = (inst1.milliseconds)-inst0.milliseconds;
        hprintln!("keypair_ticks; {:?}", keypair_ticks);
        hprintln!("keypair_millis-Millis; {:?}", keypair_millis);
    }
    // for i in 1..=3 {
    //     let cycles = 1_000_000 * i as u64;
    //     hprintln!("Delay of {}0 secs", i as u64);
    //     hprintln!("START");
    //     let inst0 = instant();
    //     while instant().milliseconds < inst0.milliseconds + cycles {
    //         cortex_m::asm::wfi();
    //     }
    //     let inst1 = instant();
    //     hprintln!("END");
    //     let m_ticks = inst1.clock_ticks-inst0.clock_ticks;
    //     let m_millis = inst1.milliseconds-inst0.milliseconds;
    //     hprintln!("ticks {} \t millis: {}\n\n", m_ticks, m_millis);
    // }

    // let mut bob = ALICE.clone();
    // let inst2 = instant();
    // let client_init = ALICE.client_init(&bob_keys.public, &mut rng);
    // let inst3 = instant();
    // let server_response = bob.server_receive(client_init, &bob_keys.secret, &mut rng).unwrap();
    // let inst4 = instant();
    // ALICE.client_confirm(server_response);
    // let inst5 = instant();
    // let keypair_ticks = inst1.clock_ticks-inst0.clock_ticks;
    // // let client_init_ticks = inst3.clock_ticks-inst2.clock_ticks;
    // // let server_receive_ticks = inst4.clock_ticks-inst3.clock_ticks;
    // // let client_confirm_ticks = inst5.clock_ticks-inst4.clock_ticks;
    // let keypair_millis = inst1.milliseconds-inst0.milliseconds;
    // // let client_init_millis = inst3.milliseconds-inst2.milliseconds;
    // // let server_receive_millis = inst4.milliseconds-inst3.milliseconds;
    // // let client_confirm_millis = inst5.milliseconds-inst4.milliseconds;
    // hprintln!("keypair_ticks; {:?}", keypair_ticks);
    // // hprintln!("client_init_ticks; {:?}", client_init_ticks);
    // // hprintln!("server_receive_ticks; {:?}", server_receive_ticks);
    // // hprintln!("client_confirm_ticks; {:?}", client_confirm_ticks);
    //
    // hprintln!("keypair_millis-Millis; {:?}", keypair_millis);
    // hprintln!("client_init_millis-Millis; {:?}", client_init_millis);
    // hprintln!("server_receive_millis-Millis; {:?}", server_receive_millis);
    // hprintln!("client_confirm_millis-Millis; {:?}", client_confirm_millis);
    // hprintln!("Alice {:?}", ALICE.shared_secret);
    // hprintln!("Bob {:?}", bob.shared_secret);

    loop {}

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

/*fn sleepms(ms: u64) {
    let deadline = millis() + ms;
    while millis() < deadline {
        cortex_m::asm::wfi();
    }
}

#[link_section= ".ccrambss"]
static mut KEYPAIR: Keypair = Keypair {
    public: [0u8; KYBER_PUBLICKEYBYTES],
    secret: [0u8; KYBER_SECRETKEYBYTES]
};*/