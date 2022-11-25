#![no_main]
#![no_std]

use core::convert::TryInto;
use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};
use pqc_kyber::{decapsulate, encapsulate, keypair, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES};
use pqc_kyber::Uake;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::mem::size_of_val;

use core::cell::RefCell;

use cortex_m::{
    interrupt::Mutex,
    peripheral::{syst::SystClkSource, SYST},
};
use cortex_m::peripheral::syst;
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

struct TickCounter {
    #[allow(dead_code)]
    /// keep syst as a resource
    syst: SYST,
    millisconds: u64,
}

impl TickCounter {
    pub fn new(mut syst: SYST, clocks: &Clocks) -> TickCounter {
        syst.set_clock_source(SystClkSource::Core);
        hprintln!("Clock {:?}", clocks.sysclk().0);

        SYST::get_current()
        // per ms counter
        syst.set_reload(clocks.sysclk().0 / 1_000 - 1); // To make the timer wrap every N ticks set the reload value to N - 1
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();

        TickCounter {
            syst,
            millisconds: 0,
        }
    }

    pub fn tick(&mut self) {
        self.millisconds += 1;
    }

    pub fn get(&self) -> u64 {
        self.millisconds
    }
}

static TICK_COUNTER: Mutex<RefCell<Option<TickCounter>>> = Mutex::new(RefCell::new(None));

fn millis() -> u64 {
    cortex_m::interrupt::free(|cs| {
        if let Some(counter) = &*TICK_COUNTER.borrow(cs).borrow_mut() {
            counter.get()
        } else {
            0
        }
    })
}

fn sleepms(ms: u64) {
    let deadline = millis() + ms;
    while millis() < deadline {
        cortex_m::asm::wfi();
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

    let mut rng = CustomRng(2 as u64);
    cortex_m::interrupt::free(|cs| {
        TICK_COUNTER
        .borrow(cs)
        .borrow_mut()
        .replace(TickCounter::new(core_periphs.SYST, &clocks))
    });
    // let bob_keys = keypair(&mut rng);
    // hprintln!("2-Millis; {:?}", millis());
    // let mut bob = ALICE.clone();
    // hprintln!("3-Millis; {:?}", millis());
    // let client_init = ALICE.client_init(&bob_keys.public, &mut rng);
    // hprintln!("4-Millis; {:?}", millis());
    // let server_response = bob.server_receive(client_init, &bob_keys.secret, &mut rng).unwrap();
    // hprintln!("5-Millis; {:?}", millis());
    // ALICE.client_confirm(server_response);
    // hprintln!("6-Millis; {:?}", millis());
    // hprintln!("Alice {:?}", ALICE.shared_secret);
    // hprintln!("Bob {:?}", bob.shared_secret);
    let inst1 = millis();
    let keys_bob = keypair(&mut rng);
    let inst2 = millis();
    let (ciphertext, shared_secret_alice) = encapsulate(&keys_bob.public, &mut rng).unwrap();
    let inst3 = millis();
    let shared_secret_bob = decapsulate(&ciphertext, &keys_bob.secret).unwrap();
    let inst4 = millis();
    hprintln!("Alice {:?}", inst2-inst1);
    hprintln!("Alice {:?}", inst3-inst2);
    hprintln!("Alice {:?}", inst4-inst3);
    hprintln!("Alice {:?}", shared_secret_alice);
    hprintln!("Bob {:?}", shared_secret_bob);
    loop {}
}

#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        if let Some(counter) = &mut *TICK_COUNTER.borrow(cs).borrow_mut() {
            counter.tick();
        }
    })
}