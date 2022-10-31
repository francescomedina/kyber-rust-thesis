#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use pqc_kyber::{keypair};
use pqc_kyber::Uake;
use rand_core::{RngCore, CryptoRng, Error,impls};

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

// fn custom_client_init(bob_keys: Keypair, rng: &mut CustomRng){
//     // let mut bob = Uake::new();
//     // hprintln!("{:?}", size_of_val(&alice)).unwrap();
//     //
//     let mut alice = Uake::new();
//     let client_init = alice.client_init(&bob_keys.public, rng);
//     hprintln!("{:?}", size_of_val(&client_init));
// }

#[entry]
unsafe fn main() -> ! {
    let mut rng = CustomRng(2 as u64);
    let bob_keys = keypair(&mut rng);
    // custom_client_init(bob_keys, &mut rng);
    let mut alice = Uake::new();
    let mut bob = Uake::new();
    let client_init = alice.client_init(&bob_keys.public, &mut rng);
    let server_response = bob.server_receive(
        client_init, &bob_keys.secret, &mut rng
    );
    alice.client_confirm(server_response.unwrap()).expect("A");
    assert_eq!(alice.shared_secret, bob.shared_secret);
    loop {}
}
