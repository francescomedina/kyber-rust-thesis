#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::{entry, ExceptionFrame,exception};
use cortex_m_semihosting::{hio, hprintln};
use pqc_kyber::{keypair, Keypair, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES};
use pqc_kyber::Uake;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::fmt::Write;
use core::mem::size_of_val;

use kyber_rust::Flash;
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

#[link_section = ".ccrambss"]
static mut ALICE: Uake = Uake {
    shared_secret: [0u8; KYBER_SSBYTES],
    send_a: [0u8; UAKE_INIT_BYTES],
    send_b: [0u8; UAKE_RESPONSE_BYTES],
    temp_key: [0u8; KYBER_SSBYTES],
    eska: [0u8; KYBER_SECRETKEYBYTES],
};

#[link_section = ".ccrambss"]
static mut BOB_KEYS: Keypair = Keypair {
    public: [0u8; KYBER_PUBLICKEYBYTES],
    secret: [0u8; KYBER_SECRETKEYBYTES]
};

#[entry]
unsafe fn main() -> ! {
    let mut rng = CustomRng(2 as u64);
    BOB_KEYS = keypair(&mut rng);
    if let Some(p) = pac::Peripherals::take() {
        let flash = Flash::new(p.FLASH, 112);
        let off1 = 0;
        let off2 = 4032;
        let client_init = ALICE.client_init(&BOB_KEYS.public, &mut rng);
        flash.erase(off1,4032).unwrap();
        flash.write(off1, &ALICE).unwrap();
        flash.erase(off2,4032).unwrap();
        flash.write(off2, &ALICE).unwrap();
        let server_response = flash.read::<Uake>(off1).server_receive(client_init, &BOB_KEYS.secret, &mut rng).unwrap();
        // ALICE = flash.read::<Uake>(off1);
        ALICE.client_confirm(server_response).expect("TODO: panic message");
        // hprintln!("5- Alice shared secret: {:?}", ALICE.shared_secret);
        // ALICE = flash.read::<Uake>(off2);
        // hprintln!("6- Bob shared secret: {:?}",  ALICE.shared_secret);
    }
    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }
    loop {}
}

