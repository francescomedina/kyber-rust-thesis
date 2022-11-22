#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::{entry, ExceptionFrame,exception};
use cortex_m_semihosting::{hio, hprintln};
use pqc_kyber::{keypair, Keypair, KYBER_SECRETKEYBYTES, KYBER_SSBYTES, KyberError, SharedSecret, UAKE_INIT_BYTES, UAKE_RESPONSE_BYTES, UakeSendInit, UakeSendResponse};
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

/*#[link_section = ".ccrambss"]
static mut ALICE: Uake = Uake {
    shared_secret: [0u8; KYBER_SSBYTES],
    send_a: [0u8; UAKE_INIT_BYTES],
    send_b: [0u8; UAKE_RESPONSE_BYTES],
    temp_key: [0u8; KYBER_SSBYTES],
    eska: [0u8; KYBER_SECRETKEYBYTES],
};
//
#[link_section = ".ccrambss"]
static mut BOB: Uake = Uake {
    shared_secret: [0u8; KYBER_SSBYTES],
    send_a: [0u8; UAKE_INIT_BYTES],
    send_b: [0u8; UAKE_RESPONSE_BYTES],
    temp_key: [0u8; KYBER_SSBYTES],
    eska: [0u8; KYBER_SECRETKEYBYTES],
};*/

fn store_keys(flash: &Flash, rng: &mut CustomRng) -> [usize; 3] {
    let offset1 = 0;
    let mut u_ake: Uake = Uake::new();
    flash.write(offset1, &u_ake).unwrap();
    let offset2 = offset1 + size_of_val(&u_ake) as usize;
    u_ake = Uake::new();
    flash.write(offset2, &u_ake).unwrap();
    let offset3 = offset2 + size_of_val(&u_ake) as usize;
    let bob_keys = keypair(rng);
    flash.write(offset3, &bob_keys).unwrap();
    // u_ake.zeroize();
    return [offset1, offset2, offset3];
}

fn get_client_init(flash: &Flash, off1: usize, off_keypair: usize, rng: &mut CustomRng) -> UakeSendInit {
    let mut alice: Uake = flash.read(off1);
    let bob_keys: Keypair = flash.read(off_keypair);
    let u_ake_send_init = alice.client_init(&bob_keys.public, rng);
    // alice.zeroize();
    return u_ake_send_init;
}

fn get_server_response(flash: &Flash, off2: usize, off_keypair: usize, rng: &mut CustomRng, client_init: UakeSendInit) -> Result<UakeSendResponse, KyberError> {
    let mut bob: Uake = flash.read(off2);
    let bob_keys: Keypair = flash.read(off_keypair);
    let result = bob.server_receive(client_init, &bob_keys.secret, rng);
    // bob.zeroize();
    return result;
}

fn client_confirm(flash: &Flash, off1: usize, server_response: UakeSendResponse) {
    // let mut alice: Uake = flash.read(off1);
    // let output = 
    (flash.read(off1) as Uake).client_confirm(server_response).expect("TODO: panic message");
    // // alice.zeroize();
    // return output;
}

fn get_shared_secret(flash: &Flash, off: usize) -> SharedSecret {
    let mut client: Uake = flash.read(off);
    let shared_secret = client.shared_secret;
    // client.zeroize();
    return shared_secret;
}

#[entry]
unsafe fn main() -> ! {
    if let (Some(p), Some(_cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let flash = Flash::new(p.FLASH, 112);
        let mut rng = CustomRng(2 as u64);
        let [off1, off2, off_keypair] = store_keys(&flash, &mut rng);
        let client_init = get_client_init(&flash, off1, off_keypair, &mut rng);
        let server_response = get_server_response(&flash, off2, off_keypair, &mut rng, client_init);
        hprintln!("3- Alice client_init: {:?} bytes", size_of_val(&client_init));
        hprintln!("4- Bob server_response: {:?} bytes", size_of_val(&server_response));
        client_confirm(&flash, off1, server_response.unwrap());
        let alice_share_secret = get_shared_secret(&flash, off1);
        let bob_share_secret = get_shared_secret(&flash, off2);
        hprintln!("5- Alice shared secret: {:?}", alice_share_secret);
        hprintln!("6- Bob shared secret: {:?}", bob_share_secret);
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

