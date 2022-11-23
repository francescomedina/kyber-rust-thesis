#![no_std]

use cortex_m_semihosting::hprintln;
use stm32f3::stm32f303::FLASH;

pub struct Flash {
    flash: FLASH,
    pub sector: u8, // u8 = 1 Byte
}

#[derive(Debug, Clone)]
pub struct FlashError {
    status: u16,
}

///All errors contain raw value of the FLASH_SR status register (lower 16b)
impl Flash {
    pub fn new(flash: FLASH, sector: u8) -> Self {
        debug_assert!(sector < 255, "invalid sector {}", sector);

        let flash = Flash {
            flash,
            sector,
        };

        flash.init();

        flash
    }

    fn init(&self) {
        self.flash.keyr.write(|w| w.fkeyr().bits(0x4567_0123));
        self.flash.keyr.write(|w| w.fkeyr().bits(0xCDEF_89AB));
        // self.flash.cr.modify(|_, w| w.pg().set_bit());
    }

    pub fn erase(&self, offset: usize, size: usize) -> Result<(), u16> {
        while self.flash.sr.read().bsy().bit_is_set() {}

        self.flash.cr.modify(|_, w| {
            w.per().set_bit()
            // unsafe { w.snb().bits(self.sector) }
        });
        let target = Flash::get_address(self, offset , 0) as *mut u32;
        // self.flash.cr.modify(|_, w| w.strt().set_bit());
        for i in 0..size as isize / 4 {
            self.flash.ar.write(|w| unsafe { w.bits(*target.offset(i)) });

            self.flash.cr.modify(|_, w| w.strt().set_bit());

            while self.flash.sr.read().bsy().bit_is_set() {}

            self.flash.cr.modify(|_, w| w.strt().clear_bit());

            let status = self.flash.sr.read();
            if status.wrprterr().bit_is_set() {
                self.flash.sr.modify(|_, w| w.wrprterr().set_bit());
                return Err(status.bits() as u16);
            }
        }

        self.flash.cr.modify(|_, w| {
            w.per().clear_bit()
            // unsafe { w.snb().bits(self.sector) }
        });

        Ok(())
    }

    fn get_address(&self, offset: usize, access_size: usize) -> usize {
        let (size, address) = match self.sector { // sector 112 -> start from 0x08038000 and it has 32KB of available FLASH memory
            //RM0316 Rev 8 page 65
            0..=127 => (0x800, 0x0800_0000 + self.sector as usize * 0x800), //0x800 => 2KB
            _ => panic!("invalid sector {}", self.sector),
        };

        // debug_assert!(offset + access_size < 0x0804_0000, "access beyond sector limits");

        address + offset
    }

    pub fn write<T>(&self, offset: usize, data: &T) -> Result<(), u16> {
        let size = core::mem::size_of::<T>();
        let src_ptr = (data as *const T) as *const u16;
        // let dest_ptr = Flash::get_address(self, offset, size) as *mut u16;
        let dest_ptr = Flash::get_address(self, offset, size) as *mut u16;

        debug_assert!(size % 4 == 0, "data size not 4-byte aligned");
        debug_assert!(src_ptr as usize % 4 == 0, "data address not 4-byte aligned");

        while self.flash.sr.read().bsy().bit_is_set() {}

        //check if register operations can be moved out of the loop
        for i in 0..size as isize / 2 {
            self.flash.cr.modify(|_, w| w.pg().set_bit());
            unsafe {
                // let asd = *src_ptr.offset(i);
                *dest_ptr.offset(i) = *src_ptr.offset(i);
            }
            while self.flash.sr.read().bsy().bit_is_set() {}

            let status = self.flash.sr.read();
            if status.wrprterr().bit_is_set()
                || status.pgerr().bit_is_set()
            {
                self.flash.sr.write(|w| unsafe { w.bits(0xFFFF) });
                return Err(status.bits() as u16);
            }
        }

        self.flash.cr.modify(|_, w| w.pg().clear_bit());

        Ok(())
    }

    pub fn read<T>(&self, offset: usize) -> T {
        let size = core::mem::size_of::<T>();
        let ptr = Flash::get_address(self, offset, size) as *const u8;
        unsafe { core::ptr::read(ptr as *const _) }
    }

    pub fn read_into<T>(&self, offset: usize, dest: &mut T) {
        let size = core::mem::size_of::<T>();
        let ptr = Flash::get_address(self, offset, size) as *const u8;
        unsafe { core::ptr::copy_nonoverlapping(ptr as *const _, dest, 1); };
    }
}