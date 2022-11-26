#![no_main]
#![no_std]

use core::convert::TryInto;
use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use pqc_kyber::{decapsulate, encapsulate, keypair, Keypair, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES};
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

// #[link_section = ".ccrambss"]
// static mut ALICE: Uake = Uake {
//     shared_secret: [0u8; KYBER_SSBYTES],
//     send_a: [0u8; UAKE_INIT_BYTES],
//     send_b: [0u8; UAKE_RESPONSE_BYTES],
//     temp_key: [0u8; KYBER_SSBYTES],
//     eska: [0u8; KYBER_SECRETKEYBYTES],
// };

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
        hprintln!("Clock {:?}", clocks.sysclk().0);
        hprintln!("Launch an interrupt every {:?} ticks", clocks.sysclk().0 / 1_000 - 1);

        // per ms counter
        // syst.set_reload(clocks.sysclk().0 / 1_000 - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.set_reload(clocks.sysclk().0 / 1_000 - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        TickCounter {
            syst,
            period: clocks.sysclk().0 as u64 / 1_000,
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

// fn sleepms(ms: u64) {
//     let deadline = millis() + ms;
//     while millis() < deadline {
//         cortex_m::asm::wfi();
//     }
// }

#[link_section= ".ccrambss"]
static mut KEYPAIR: Keypair = Keypair {
    public: [0u8; KYBER_PUBLICKEYBYTES],
    secret: [0u8; KYBER_SECRETKEYBYTES]
};

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

    let mut rng = CustomRng(2 as u64);
    cortex_m::interrupt::free(|cs| {
        TICK_COUNTER
        .borrow(cs)
        .borrow_mut()
        .replace(TickCounter::new(core_periphs.SYST, &clocks))
    });
/*    let bob_keys = keypair(&mut rng);
    hprintln!("2-Millis; {:?}", millis());
    let mut bob = ALICE.clone();
    hprintln!("3-Millis; {:?}", millis());
    let client_init = ALICE.client_init(&bob_keys.public, &mut rng);
    hprintln!("4-Millis; {:?}", millis());
    let server_response = bob.server_receive(client_init, &bob_keys.secret, &mut rng).unwrap();
    hprintln!("5-Millis; {:?}", millis());
    ALICE.client_confirm(server_response);
    hprintln!("6-Millis; {:?}", millis());
    hprintln!("Alice {:?}", ALICE.shared_secret);
    hprintln!("Bob {:?}", bob.shared_secret);*/
    let mut k_max_clock = 0;
    let mut k_min_clock = u64::MAX;
    let mut e_max_clock = 0;
    let mut e_min_clock = u64::MAX;
    let mut d_max_clock = 0;
    let mut d_min_clock = u64::MAX;
    let mut k_sum = 0;
    let mut e_sum = 0;
    let mut d_sum = 0;

    let mut k_ticks;
    let mut e_ticks;
    let mut d_ticks;
    let mut k_millis = 0;
    let mut e_millis = 0;
    let mut d_millis = 0;
    for _ in 0..100 {
        let inst1 = instant();
        KEYPAIR = keypair(&mut rng);
        let inst2 = instant();
        let (ciphertext, shared_secret_alice) = encapsulate(&KEYPAIR.public, &mut rng).unwrap();
        let inst3 = instant();
        let shared_secret_bob = decapsulate(&ciphertext, &KEYPAIR.secret).unwrap();
        let inst4 = instant();

        k_ticks = inst2.clock_ticks-inst1.clock_ticks;
        e_ticks = inst3.clock_ticks-inst2.clock_ticks;
        d_ticks = inst4.clock_ticks-inst3.clock_ticks;
        k_sum += k_ticks;
        e_sum += e_ticks;
        d_sum += d_ticks;
        if k_max_clock < k_ticks {
            k_max_clock = k_ticks;
        }
        if e_max_clock < e_ticks {
            e_max_clock = e_ticks;
        }
        if d_max_clock < d_ticks {
            d_max_clock = d_ticks;
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
        k_millis = inst2.milliseconds-inst1.milliseconds;
        e_millis = inst3.milliseconds-inst2.milliseconds;
        d_millis = inst4.milliseconds-inst3.milliseconds;
    }
    hprintln!("Keypair: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", k_millis, k_min_clock, k_max_clock, k_sum/100);
    hprintln!("Encapsulate: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", e_millis, e_min_clock, e_max_clock, e_sum/100);
    hprintln!("Decapsulate: MILLIS({:?}) MIN({:?}) MAX({:?}) AVG({:?})", d_millis, d_min_clock, d_max_clock, d_sum/100);
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