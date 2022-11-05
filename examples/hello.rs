//! Prints "Hello, world!" on the host console using semihosting

#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

use core::cell::UnsafeCell;
use panic_halt as _;

use cortex_m_rt::exception;

use cortex_m_rt::{entry, ExceptionFrame};
use cortex_m_semihosting::{dbg, hio, hprintln};
use pqc_kyber::*;
use rand_core::{RngCore, CryptoRng, Error,impls};
use core::mem::size_of_val;
use core::sync::atomic::{AtomicPtr, Ordering};

use self::alloc::vec;
use core::alloc::Layout;
use core::result::Result;
use core::result::Result::Ok;
use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;

extern crate alloc;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 24000; // in bytes

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

fn client_init(alice: &mut Uake, public_keys: PublicKey, rng: &mut CustomRng) -> UakeSendInit {
    hprintln!("{:?}", size_of_val(&alice)); //4
    let client_init = alice.client_init(&public_keys, rng);
    hprintln!("{:?}", size_of_val(&client_init));
    client_init
}

fn uake(alice: &mut Uake,bob_keys: Keypair, rng: &mut CustomRng) -> UakeSendInit {
    hprintln!("CIAONE");
    hprintln!("{:?}", size_of_val(&bob_keys)); //3584
    // let mut xs = vec![Uake::new(), Uake::new()];
    client_init(alice, bob_keys.public, rng)
}


#[entry]
unsafe fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut rng = CustomRng(2 as u64);

    // println!("{}", get_value());
    // set_value(&mut keypair(&mut rng));
    // hprintln!("{:?}", (*get_value()).public).unwrap();
    // hprintln!("{:?}", size_of_val(&bob_keys)).unwrap();

    // hprintln!("{:?}", BOB_KEYS.public).unwrap();
    // dbg!(size_of_val(&BOB_KEYS));

    // dbg!(BOB_KEYS.public);

    // let mut alice = Uake::new();
    // let mut bob = Uake::new();
    // Growable array allocated on the heap

    let bob_keys = keypair(&mut rng);
    let mut clients = vec![Uake::new(),Uake::new()];

    let client_init = uake(&mut clients[0], bob_keys, &mut rng);

    let server_response = clients[1].server_receive(
        client_init, &bob_keys.secret, &mut rng
    );
    clients[0].client_confirm(server_response.unwrap()).expect("TODO: panic message");
    //
    // assert_eq!(alice.shared_secret, bob.shared_secret);

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
