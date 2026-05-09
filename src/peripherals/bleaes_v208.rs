#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BLE AES engine — only +0x04 cleanup-bit pair decoded for ADV TX path."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAes {
    ptr: *mut u8,
}
unsafe impl Send for BleAes {}
unsafe impl Sync for BleAes {}
impl BleAes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AES op status (bit0 + bit1; cleared sequentially from the BB IRQ path)."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "BLE_AES status — phase 1 / phase 2 cleanup bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "AES op cleanup phase 2 (cleared after PHASE1)."]
        #[inline(always)]
        pub const fn phase2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES op cleanup phase 2 (cleared after PHASE1)."]
        #[inline(always)]
        pub fn set_phase2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AES op cleanup phase 1 (cleared first if set)."]
        #[inline(always)]
        pub const fn phase1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AES op cleanup phase 1 (cleared first if set)."]
        #[inline(always)]
        pub fn set_phase1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
}
