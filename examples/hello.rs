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
use cortex_m::peripheral::{Peripherals, DWT};

use core::cell::RefCell;
use core::i8::MIN;
use core::sync::atomic::{compiler_fence, Ordering};

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
        let mut offset = DWT::cycle_count();
        for (index, dest) in dest.iter_mut().enumerate() {
            let mut seed = (DWT::cycle_count() as u64 + offset as u64 + self.0*index as u64);
            *dest = SmallRng::seed_from_u64(seed).gen_range(0..=255);
            if index%2 == 0{
                offset += self.0 as u32 * index as u32;
            }else{
                offset -= index as u32;
                if offset < 0 {
                    offset = seed as u32;
                }
            }
            offset = offset + seed as u32;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for CustomRng {
}

struct Instant {
    clock_ticks: u64,
    milliseconds: u64
}

struct TickCounter {
    #[allow(dead_code)]
    syst: SYST,
    period: u64,
    clock_ticks: u64,
}

impl TickCounter{
    pub fn new(mut syst: SYST, clocks: &Clocks) -> TickCounter {
        syst.set_clock_source(SystClkSource::Core);
        let fraction: u32 = 100_000; // 8_000_000 / 100_000 => each tick is a nanosecond, after the tick the counter is reloaded
        //let fraction: u32 = 1_000_000; // 8_000_000 / 100_000 => each tick is a nanosecond
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

/*fn sleep_ms(ms: u64) {
    let deadline = millis() + ms;
    while millis() < deadline {
        cortex_m::asm::wfi();
    }
}*/

pub fn rand_with_range(a: i16, b: i16, r: &mut[i16]){
    // let mut rng = rand::thread_rng();
    for i in 0..r.len() {
        let mut small_rng = SmallRng::seed_from_u64(77324547 + (i as u64 * 12));
        r[i] = small_rng.gen_range(a..(b+1));
    }
}

macro_rules! op_cyccnt_diff {
    ( $( $x:expr )* ) => {
        {
            compiler_fence(Ordering::Acquire);
            let before = DWT::cycle_count();
            $(
                let (c, s) = $x.unwrap();
            )*
            let after = DWT::cycle_count();
            compiler_fence(Ordering::Release);
            let diff =
                if after >= before {
                    after - before
                } else {
                    after + (u32::MAX - before)
                };
            (c, s, diff)
        }
    };
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
        .hclk(24.MHz())
        .pclk1(12.MHz())
        .pclk2(24.MHz())
        .freeze(&mut flash.acr);

    //hprintln!("{:?}", rcc.cfgr.calc_pll());

    // let core_periphs = cortex_m::Peripherals::take().unwrap();
    //
    // cortex_m::interrupt::free(|cs| {
    //     TICK_COUNTER
    //     .borrow(cs)
    //     .borrow_mut()
    //     .replace(TickCounter::new(core_periphs.SYST, &clocks))
    // });

/*    for i in 1..=10 {
        let cycles = 8_000_000 * i as u32;
        hprintln!("Delay of {} secs", i as u32);
        hprintln!("START");
        let inst0 = instant();
        while instant().milliseconds < inst0.milliseconds + 1_000_000 {
            cortex_m::asm::wfi();
        }
        let inst1 = instant();
        hprintln!("END");
        let m_ticks = inst1.clock_ticks-inst0.clock_ticks;
        let m_millis = inst1.milliseconds-inst0.milliseconds;
        hprintln!("ticks {} \t millis: {}\n\n", m_ticks, m_millis);
    }*/
    let mut peripherals = Peripherals::take().unwrap();
    peripherals.DWT.enable_cycle_counter();

    let mut c: [[u8; 768]; 30] = [[0u8;768]; 30];
    let mut s: [[u8; 32]; 30] = [[0u8;32]; 30];
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    let mut sum = 0;
    for i in 0..30 {
        let mut rng = CustomRng(i as u64);
        let bob_keys = keypair(&mut rng);
        let (ciphertext, shared_key, diff) = op_cyccnt_diff!(encapsulate(&bob_keys.public, &mut rng));
        c[i] = ciphertext;
        s[i] = shared_key;
        sum += diff;
        if min > diff {
            min = diff;
        }
        if max < diff {
            max = diff;
        }
    }
    hprintln!("  C: {} \t S: {}\t MIN: {}\tMAX: {}\tAVG: {}\n", c[1][2], s[2][4], min, max, sum/30);
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