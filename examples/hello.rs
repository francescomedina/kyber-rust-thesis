//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use core::cell::UnsafeCell;
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{dbg, hprintln};
use pqc_kyber::*;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::mem::size_of_val;
use core::sync::atomic::{AtomicPtr, Ordering};

#[derive(Clone, Debug, Eq, PartialEq)]
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
//
// enum DivisionResult {
//     U(Uake),
//     DividedByZero,
// }
//
// static BOB: AtomicPtr<Option<Uake>> = AtomicPtr::new(None);
// static ALICE: AtomicPtr<Option<Uake>> = AtomicPtr::new(None);

// pub fn set_bob_value(val: *mut Uake) {
//     BOB.store(val, Ordering::Relaxed)
// }
//
// pub fn get_bob_value() -> *mut Uake {
//     BOB.load(Ordering::Relaxed)
// }
//
// pub fn set_alice_value(val: *mut Uake) {
//     BOB.store(val, Ordering::Relaxed)
// }
//
// pub fn get_alice_value() -> *mut Uake {
//     BOB.load(Ordering::Relaxed)
// }

#[entry]
unsafe fn main() -> ! {
    let mut rng = CustomRng(2 as u64);

    let bob_keys = keypair(&mut rng);

    // println!("{}", get_value());
    // set_value(&mut keypair(&mut rng));
    // hprintln!("{:?}", (*get_value()).public).unwrap();

    // hprintln!("{:?}", BOB_KEYS.public).unwrap();
    // dbg!(size_of_val(&BOB_KEYS));

    // dbg!(BOB_KEYS.public);

    // let mut alice = Uake::new();
    // let mut bob = Uake::new();
    // hprintln!("{:?}", size_of_val(&alice)).unwrap();
    //
    // let client_init = (*get_alice_value()).client_init(&bob_keys.public, &mut rng);
    // let server_response = (*get_bob_value()).server_receive(
    //     client_init, &bob_keys.secret, &mut rng
    // );
    // alice.client_confirm(server_response.unwrap()).expect("TODO: panic message");
    //
    // assert_eq!(alice.shared_secret, bob.shared_secret);

    loop {}
}
