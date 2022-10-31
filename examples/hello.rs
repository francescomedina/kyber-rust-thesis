#![no_main]
#![no_std]

use core::mem::size_of_val;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use pqc_kyber::{keypair, Keypair, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, PublicKey, SecretKey, UAKE_INIT_BYTES, UakeSendInit};
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

pub struct Bob {
    s: SecretKey,
    c: UakeSendInit
}

fn create_bob(alice: &mut Uake, rng: &mut CustomRng) -> Bob {
    let bob_keys = keypair(rng);
    Bob {
        s: bob_keys.secret,
        c: alice.client_init(&bob_keys.public, rng)
    }
}

#[entry]
unsafe fn main() -> ! {
    let mut rng = CustomRng(2 as u64);
    let mut alice = Uake::new();
    let bob_info = create_bob(&mut alice, &mut rng);
    let mut bob = Uake::new();
    let server_response = bob.server_receive(
        bob_info.c, &bob_info.s, &mut rng
    );
    alice.client_confirm(server_response.unwrap()).expect("A");
    // assert_eq!(alice.shared_secret, bob.shared_secret, "a: {}, b {}", alice.shared_secret, bob.shared_secret);
    hprintln!("{:?}", alice.shared_secret);
    hprintln!("FINE");
    hprintln!("{:?}", bob.shared_secret);
    loop {}
}
