#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::{entry, exception};

use stm32f4xx_hal;

use pqc_kyber::{Poly, ntt};

#[entry]
unsafe fn main() -> ! {
    let mut v = Poly::new();
    let mut r = &mut v.coeffs;
    ntt(r);
    loop {}
}


#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    
}