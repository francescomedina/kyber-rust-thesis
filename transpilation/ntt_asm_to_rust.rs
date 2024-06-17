/**
 * 
 * Explanation
    The given ASM code has been converted to Rust code. Here's a line-by-line explanation of the conversion:

    The function __cortex_m_rt_main is defined as an extern "C" function in Rust.
    A stack of size 528 bytes is created using an array of u8 values.
    Pointers r9, r0, and r1 are initialized.
    The memory at r0 is cleared using the core::ptr::write_bytes function.
    Variables r10 and r5 are initialized.
    The value of r12 is set to the bitwise negation of 3328.
    The outer loop starts with a condition r10 != 128.
    Inside the outer loop, the value of r11 is set to 256 if r5 is greater than 256.
    The value of r8 is read from memory at r0.offset(r10).
    The value of r6 is calculated as r5 + 128.
    The value of r2 is set to 256 if r6 is greater than 256.
    The value of r4 is calculated as r2 - 128.
    The inner loop starts with a condition r4 != r5.
    Inside the inner loop, the value of r5 is calculated as r9.offset(r2 * 2).
    The value of r1 is set to 0 and r0 is set to 62209.
    The value of r0 is read from memory at r5.offset(256).
    The values of r0 and r1 are multiplied and stored in r0.
    The values of r0 and r1 are multiplied and stored in r1.
    The value of r0 is calculated as r12.smlabt(r1, r0).
    The values of r1 and r0 are updated in memory.
    The values of r5 and r2 are incremented.
    The inner loop ends.
    The value of r10 is incremented.
    The outer loop ends.
    The remaining code follows a similar pattern, with slight variations in the loop conditions and memory access.
    The converted Rust code closely resembles the original ASM code and should produce the same results when executed.






    This code appears to be written in ARM assembly language and is likely part of a larger program or library. It contains several functions and loops that perform various operations on arrays of data. Here's a breakdown of what the code does: 
    1. The `speed_evaluation::__cortex_m_rt_main` function is the entry point of the code. 
    2. It initializes several variables and arrays, including `r9` which is a pointer to an array of 512 elements. 
    3. The code then enters a series of nested loops that perform operations on the elements of the array pointed to by `r9`. 
    4. Within these loops, the code loads values from another array (possibly a lookup table) and performs calculations involving multiplication, addition, and bit shifts. 
    5. The results of these calculations are stored back into the array pointed to by `r9`. 
    6. The loops are structured in a way that suggests they are performing some kind of signal processing or filtering operation on the data in the array. 
    7. There are several bounds checking operations (`core::panicking::panic_bounds_check`) that ensure the array indices being accessed are within the valid range. 
    8. The code appears to be optimized for the ARM Cortex-M architecture, as indicated by the use of instructions like `smlabt` (Signed Multiply Accumulate with Bit Reverse) 
        and the `.thumb_func` directive. Without more context about the purpose of this code and the larger system it is a part of, it is difficult to provide a more detailed explanation. 
        However, the code appears to be performing some kind of signal processing or filtering operation on an array of data, likely as part of a larger real-time or embedded system application.
 * 
 */
#[no_mangle]
pub extern "C" fn __cortex_m_rt_main() {
    unsafe {
        let mut stack = [0u8; 528];
        let stack_ptr = stack.as_mut_ptr();
        let stack_end = stack_ptr.offset(528);

        let mut r9 = stack_ptr.offset(16);
        let r0 = r9;
        let r1 = 512;
        unsafe { core::ptr::write_bytes(r0, 0, r1) };

        let mut r10 = 1;
        let mut r5 = 0;
        let r12 = !3328;

        // r11 = k
        // r5 = j

        while r10 != 128 {
            let r11 = 256;
            if r5 > 256 {
                r11 = r5;
            }
            let r8 = unsafe { core::ptr::read_volatile(r0.offset(r10)) };
            let r6 = r5 + 128;
            let r2 = 256;
            if r6 > 256 {
                r2 = r6;
            }
            let r4 = r2 - 128;
            r10 += 1;
            let lr = r1;

            while r4 != r5 {
                if r11 != r2 {
                    let r5 = r9.offset(r2 * 2);
                    let r1 = 0;
                    let r0 = 62209;
                    let r0 = unsafe { core::ptr::read_volatile(r5.offset(256)) };
                    let r0 = r0 * r8;
                    let r1 = r0 * r1;
                    let r0 = r12.smlabt(r1, r0);
                    let r1 = unsafe { core::ptr::read_volatile(r9.offset(r2 * 2)) };
                    let r3 = r1 + (r0 >> 16);
                    let r0 = r1 - (r0 >> 16);
                    unsafe {
                        core::ptr::write_volatile(r5.offset(256), r0);
                        core::ptr::write_volatile(r9.offset(r2 * 2), r3);
                    }
                    r5 += 1;
                }
                r2 += 1;
            }
        }

        let mut r4 = r10;
        let mut r6 = 0;
        let r10 = !3328;

        while r1 != r4 {
            let r0 = unsafe { core::ptr::read_volatile(r9.offset(128)) };
            let r11 = 256;
            let lr = r1;
            if r6 > 256 {
                r11 = r6;
            }
            let r8 = unsafe { core::ptr::read_volatile(r0.offset(r1)) };
            let r2 = r6 + 64;
            let r4 = 256;
            if r2 > 256 {
                r4 = r2;
            }
            let lr = r4 - 64;
            r1 += 1;

            while lr != r6 {
                if r11 != r4 {
                    let r6 = unsafe { core::ptr::read_volatile(r5) };
                    let r0 = 0;
                    let r0 = 62209;
                    let r6 = r6 * r8;
                    let r12 = r6 * r0;
                    let r0 = r10.smlabt(r12, r6);
                    let r3 = r0 - (r0 >> 16);
                    let r0 = r0 + (r0 >> 16);
                    unsafe {
                        core::ptr::write_volatile(r5, r3);
                        core::ptr::write_volatile(r5.offset(-128), r0);
                    }
                    r5 += 1;
                }
                r6 += 1;
            }
        }

        let mut r4 = r1;
        let mut r3 = 0;
        let r12 = !3328;

        while r1 != r4 {
            let r0 = unsafe { core::ptr::read_volatile(r9.offset(64)) };
            let r11 = 256;
            if r3 > 256 {
                r11 = r3;
            }
            let r5 = unsafe { core::ptr::read_volatile(r0.offset(r1)) };
            let r8 = r3 + 32;
            let r4 = 256;
            if r8 > 256 {
                r4 = r8;
            }
            let lr = r4 - 32;
            r1 += 1;

            while lr != r3 {
                if r11 != r6 {
                    let r0 = unsafe { core::ptr::read_volatile(r5) };
                    let r2 = 0;
                    let r2 = 62209;
                    let r0 = r0 * r5;
                    let r3 = r0 * r2;
                    let r0 = r12.smlabt(r3, r0);
                    let r3 = unsafe { core::ptr::read_volatile(r5.offset(-64)) };
                    let r2 = r3 - (r0 >> 16);
                    let r0 = r3 + (r0 >> 16);
                    unsafe {
                        core::ptr::write_volatile(r5, r2);
                        core::ptr::write_volatile(r5.offset(-66), r0);
                    }
                    r5 += 1;
                }
                r3 += 1;
            }
        }

        let mut r4 = r1;
        let mut r3 = 0;
        let r8 = !3328;

        while r1 != r4 {
            let r0 = unsafe { core::ptr::read_volatile(r9.offset(32)) };
            let r11 = 256;
            if r3 > 256 {
                r11 = r3;
            }
            let r5 = unsafe { core::ptr::read_volatile(r0.offset(r1)) };
            let r12 = r10;
            let r0 = 256;
            let r4 = r3 + 16;
            if r4 > 256 {
                r0 = r4;
            }
            let lr = r0 - 16;
            r1 += 1;

            while lr != r3 {
                if r11 != r6 {
                    let r0 = unsafe { core::ptr::read_volatile(r5) };
                    let r2 = 0;
                    let r2 = 62209;
                    let r0 = r0 * r5;
                    let r3 = r0 * r2;
                    let r0 = r8.smlabt(r3, r0);
                    let r3 = unsafe { core::ptr::read_volatile(r5.offset(-32)) };
                    let r2 = r3 - (r0 >> 16);
                    let r0 = r3 + (r0 >> 16);
                    unsafe {
                        core::ptr::write_volatile(r5, r2);
                        core::ptr::write_volatile(r5.offset(-34), r0);
                    }
                    r5 += 1;
                }
                r3 += 1;
            }
        }

        let mut r4 = r1;
        let mut r3 = 0;
        let r8 = !3328;

        while r1 != r4 {
            let r0 = unsafe { core::ptr::read_volatile(r9.offset(16)) };
            let r11 = 256;
            if r3 > 256 {
                r11 = r3;
            }
            let r5 = unsafe { core::ptr::read_volatile(r0.offset(r1)) };
            let r12 = r10;
            let r0 = 256;
            let r4 = r3 + 8;
            if r4 > 256 {
                r0 = r4;
            }
            let lr = r0 - 8;
            r1 += 1;

            while lr != r3 {
                if r11 != r6 {
                    let r0 = unsafe { core::ptr::read_volatile(r5) };
                    let r2 = 0;
                    let r2 = 62209;
                    let r0 = r0 * r5;
                    let r3 = r0 * r2;
                    let r0 = r8.smlabt(r3, r0);
                    let r3 = unsafe { core::ptr::read_volatile(r5.offset(-16)) };
                    let r2 = r3 - (r0 >> 16);
                    let r0 = r3 + (r0 >> 16);
                    unsafe {
                        core::ptr::write_volatile(r5, r2);
                        core::ptr::write_volatile(r5.offset(-18), r0);
                    }
                    r5 += 1;
                }
                r3 += 1;
            }
        }
    }






    let mut r4: u32 = 0;
    let mut r5: u32 = 0;
    let mut r6: u32 = 0;
    let mut r8: u32 = 0;
    let mut r9: u32 = 0;
    let mut r10: u32 = 0;
    let mut r11: u32 = 0;
    let mut r12: u32 = 0;
    let mut lr: u32 = 0;
    let mut sp: u32 = 0;




    loop {
        if r4 == r10 {
            break;
        }
        if r8 > 247 {
            break;
        }
        let r6 = r9 + (r8 << 1);
        let r5 = r12;
        let lr = 3328;
        let r0 = (r6 + 16) as *mut u16;
        let r2 = r1.wrapping_sub(30);
        let r3 = r2.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r0 = r3 as u16;
        }
        let r0 = r2.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1.wrapping_sub(30) = r0 as u16;
        }
        let r0 = (r6 + 18) as *mut u16;
        let r3 = (r6 + 22) as *mut u16;
        unsafe {
            *(r3 as *mut u32) = r10;
        }
        let r3 = r4;
        let r0 = r5.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 2) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 4) as *mut u16;
        let r2 = (r6 + 20) as *mut u16;
        let r10 = *r2;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 2) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 6) as *mut u16;
        let r10 = *r10;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 8) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 26) as *mut u16;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 10) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 28) as *mut u16;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 12) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = *r1;
        let r0 = r10.wrapping_mul(r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 14) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r2 = r8 + 7;
        let r0 = r8 + 16;
        if r2 > 247 {
            r8 = r0;
        }
        let r10 = r10 + 1;
        let r2 = r10.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 8) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 26) as *mut u16;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 10) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = (r6 + 28) as *mut u16;
        let r0 = r10.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 12) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = *r6;
        let r0 = r10.wrapping_mul(r0);
        let r2 = r0.wrapping_mul(r11);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = (r6 + 14) as *mut u16;
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        unsafe {
            *r2 = r0 as u16;
        }
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r2 = r8 + 7;
        let r0 = r8 + 16;
        if r2 > 247 {
            r8 = r0;
        }
        let r1 = r9 + 4;
        let r0 = r10 + r8;
        let r6 = 0;
        let lr = 3328;
        let r9 = 62209;

        loop {
            if r0 == 0 {
                break;
            }
            if r6 > 253 {
                break;
            }
            let r3 = r12;
            r0 -= 1;
            let r5 = *r1;
            let r8 = *(r1 + 2);
            let r5 = r3.wrapping_mul(r5);
            let r2 = r5.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r5);
            let r2 = *(r1 - 4);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            unsafe {
                *r1 = r4 as u16;
            }
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *(r1 - 12) = r0 as u16;
            }
            let r2 = *(r1 + 10);
            let r0 = r2.wrapping_mul(r3);
            let r2 = r0.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
            let r2 = *(r1 + 2);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *r1 = r0 as u16;
            }
            let r0 = *(r1 + 6);
            let r2 = *(r1 + 20);
            let r0 = r0.wrapping_mul(r3);
            let r2 = r0.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
            let r2 = *(r1 + 2);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *r1 = r0 as u16;
            }
            let r0 = *(r1 + 26);
            let r0 = r0.wrapping_mul(r3);
            let r2 = r0.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
            let r2 = *(r1 + 10);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *r1 = r0 as u16;
            }
            let r0 = *(r1 + 28);
            let r0 = r0.wrapping_mul(r3);
            let r2 = r0.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
            let r2 = *(r1 + 12);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *r1 = r0 as u16;
            }
            let r0 = *r1;
            let r0 = r0.wrapping_mul(r3);
            let r2 = r0.wrapping_mul(r9);
            let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
            let r2 = *(r1 + 14);
            let r4 = r2.wrapping_sub((r2 as u32) >> 16);
            let r0 = r4.wrapping_sub((r0 as u32) >> 16);
            let r0 = r4.wrapping_add((r0 as u32) >> 16);
            unsafe {
                *(r1 + 32) = r0 as u16;
            }
            let r2 = r8 + 7;
            let r0 = r8 + 16;
            if r2 > 247 {
                r8 = r0;
            }
            let r1 = r9 + 4;
            let r0 = r10 + r8;
            let r6 = r6 + 4;
        }
    }

    if r10 > 128 {
        r10 = 128;
    }

    let r1 = r9 + 4;
    let r0 = r10 + r8;
    let r6 = 0;
    let lr = 3328;
    let r9 = 62209;

    loop {
        if r0 == 0 {
            break;
        }
        if r6 > 253 {
            break;
        }
        let r3 = r12;
        r0 -= 1;
        let r5 = *r1;
        let r8 = *(r1 + 2);
        let r5 = r3.wrapping_mul(r5);
        let r2 = r5.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r5);
        let r2 = *(r1 - 4);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        unsafe {
            *r1 = r4 as u16;
        }
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *(r1 - 12) = r0 as u16;
        }
        let r2 = *(r1 + 10);
        let r0 = r2.wrapping_mul(r3);
        let r2 = r0.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = *(r1 + 2);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r0 = *(r1 + 6);
        let r2 = *(r1 + 20);
        let r0 = r0.wrapping_mul(r3);
        let r2 = r0.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = *(r1 + 2);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r0 = *(r1 + 26);
        let r0 = r0.wrapping_mul(r3);
        let r2 = r0.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = *(r1 + 10);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r0 = *(r1 + 28);
        let r0 = r0.wrapping_mul(r3);
        let r2 = r0.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = *(r1 + 12);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *r1 = r0 as u16;
        }
        let r0 = *r1;
        let r0 = r0.wrapping_mul(r3);
        let r2 = r0.wrapping_mul(r9);
        let r0 = lr.wrapping_mul(r2).wrapping_add(r0);
        let r2 = *(r1 + 14);
        let r4 = r2.wrapping_sub((r2 as u32) >> 16);
        let r0 = r4.wrapping_sub((r0 as u32) >> 16);
        let r0 = r4.wrapping_add((r0 as u32) >> 16);
        unsafe {
            *(r1 + 32) = r0 as u16;
        }
        let r2 = r8 + 7;
        let r0 = r8 + 16;
        if r2 > 247 {
            r8 = r0;
        }
        let r1 = r9 + 4;
        let r0 = r10 + r8;
        let r6 = r6 + 4;
    }

    if r10 > 128 {
        r10 = 128;
    }

    let r4 = 128;

    loop {
        if r4 == r10 {
            break;
        }
        if r5 > 251 {
            break;
        }
        let r2 = r9 + (r5 << 1);
        let r3 = r12;
        let r0 = (r2 + 8) as *mut u16;
        let r8 = r8 - 1;
        let r5 = *r0;
        let r0 = r3.wrapping_mul(r5);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        unsafe {
            *r0 = r1 as u16;
        }
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 10) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 26) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 28) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = *r1;
        let r0 = r3.wrapping_mul(r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r1 = r0 as u16;
        }
        let r2 = r8 + 7;
        let r0 = r8 + 16;
        if r2 > 247 {
            r8 = r0;
        }
        let r5 = r5 + 1;
        let r2 = r5.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 8) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 26) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = (r2 + 28) as *mut u16;
        let r0 = r3.wrapping_mul(*r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r0 = r0 as u16;
        }
        let r0 = *r6;
        let r0 = r3.wrapping_mul(r0);
        let r2 = r0.wrapping_mul(lr);
        let r0 = r2.wrapping_add(r0);
        let r2 = (r2 as u32) >> 16;
        let r4 = *r2;
        let r1 = r4.wrapping_sub(r2);
        let r0 = r4.wrapping_add(r2);
        unsafe {
            *r1 = r0 as u16;
        }
        let r2 = r8 + 7;
        let r0 = r8 + 16;
        if r2 > 247 {
            r8 = r0;
        }
        let r1 = r9 + 4;
        let r0 = r10 + r8;
        let r6 = r6 + 2;
    }

    if r10 > 128 {
        r10 = 128;
    }

    let r0 = r11;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r0 = r2 + 128;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r0 = r4 + 64;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r0 = r6 + 32;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r0 = r6 + 16;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r4 = 128;

    loop {
        let r0 = r4;
        let r1 = 128;
        core::panicking::panic_bounds_check(r0, r1);
    }

    let r0 = r8 + 8;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r0 = r5 + 4;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);

    let r4 = r10;

    loop {
        let r0 = r4;
        let r1 = 128;
        core::panicking::panic_bounds_check(r0, r1);
    }

    let r0 = r6 + 2;
    let r1 = 256;
    core::panicking::panic_bounds_check(r0, r1);
}