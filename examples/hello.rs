#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::{entry, ExceptionFrame,exception};
use cortex_m_semihosting::{hio, hprintln};
use pqc_kyber::{keypair, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES};
use pqc_kyber::Uake;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::fmt::Write;
use core::mem::size_of_val;


use cortex_m::peripheral::Peripherals;
use kyber_rust::Flash;
use hal::rcc::RccExt;
use stm32f3xx_hal as hal;
use stm32f3xx_hal::flash::FlashExt;
use stm32f3xx_hal::pac;

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
//
// #[link_section = ".ccrambss"]
// static mut BOB: Uake = Uake {
//     shared_secret: [0u8; KYBER_SSBYTES],
//     send_a: [0u8; UAKE_INIT_BYTES],
//     send_b: [0u8; UAKE_RESPONSE_BYTES],
//     temp_key: [0u8; KYBER_SSBYTES],
//     eska: [0u8; KYBER_SECRETKEYBYTES],
// };

#[entry]
unsafe fn main() -> ! {
    if let (Some(p), Some(_cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let flash = Flash::new(p.FLASH, 60);
        /*let a = pac::Peripherals::take().unwrap();
        let mut f = a.FLASH.constrain();
        //setup CPU clock to 168MHz
        let mut rcc = a.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .sysclk(stm32f3xx_hal::time::rate::Megahertz(72))
            .freeze(&mut f.acr);
*/
        let mut value: u32;
        let mut asd: u32;
        let offset = 0;

        value = 123;
        flash.write(offset, &value).unwrap();
        asd = flash.read(offset);
        flash.erase().unwrap();
    }


    // let mut rng = CustomRng(2 as u64);
    // let bob_keys = keypair(&mut rng);
    //
    // let client_init = ALICE.client_init(&bob_keys.public, &mut rng);
    // let server_response = BOB.server_receive(
    //     client_init, &bob_keys.secret, &mut rng
    // );
    // hprintln!("1- rng: {:?} bytes", size_of_val(&rng));
    // hprintln!("2- bob_keys (Keypair): {:?} bytes", size_of_val(&bob_keys));
    // hprintln!("3- Alice client_init: {:?} bytes", size_of_val(&client_init));
    // hprintln!("4- Bob server_response: {:?} bytes", size_of_val(&server_response));
    // ALICE.client_confirm(server_response.unwrap());
    // hprintln!("5- Alice Uake: {:?} bytes", size_of_val(&ALICE));
    // hprintln!("6- Bob Uake: {:?} bytes", size_of_val(&BOB));
    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }
    loop {}
}

