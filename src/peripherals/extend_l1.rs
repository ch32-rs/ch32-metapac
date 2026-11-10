#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "EXTEND configuration. (EXTEN)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extend {
    ptr: *mut u8,
}
unsafe impl Send for Extend {}
unsafe impl Sync for Extend {}
impl Extend {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EXTEN control register."]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTEN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr(pub u32);
    impl Ctr {
        #[doc = "Whether HSI is divided."]
        #[must_use]
        #[inline(always)]
        pub const fn hsipre(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Whether HSI is divided."]
        #[inline(always)]
        pub const fn set_hsipre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LOCKUP_Eable."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP_Eable."]
        #[inline(always)]
        pub const fn set_lkupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LOCKUP RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn lkuprst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP RESET."]
        #[inline(always)]
        pub const fn set_lkuprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULLDOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ulldotrim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "ULLDOTRIM."]
        #[inline(always)]
        pub const fn set_ulldotrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "LDOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ldotrim(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "LDOTRIM."]
        #[inline(always)]
        pub const fn set_ldotrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Ctr {
        #[inline(always)]
        fn default() -> Ctr {
            Ctr(0)
        }
    }
    impl core::fmt::Debug for Ctr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctr")
                .field("hsipre", &self.hsipre())
                .field("lkupen", &self.lkupen())
                .field("lkuprst", &self.lkuprst())
                .field("ulldotrim", &self.ulldotrim())
                .field("ldotrim", &self.ldotrim())
                .finish()
        }
    }
}
