#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extend configuration."]
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
    #[doc = "Configure the extended control register."]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configure the extended key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configure the extended control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr(pub u32);
    impl Ctr {
        #[doc = "Configure the PLL clock delay time."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_cfg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Configure the PLL clock delay time."]
        #[inline(always)]
        pub const fn set_pll_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "LOCKUP_Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP_Enable."]
        #[inline(always)]
        pub const fn set_lockup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LOCKUP RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup_reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP RESET."]
        #[inline(always)]
        pub const fn set_lockup_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LDO_TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo_trim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LDO_TRIM."]
        #[inline(always)]
        pub const fn set_ldo_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "FLASH clock trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn flash_clk_trim(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "FLASH clock trimming."]
        #[inline(always)]
        pub const fn set_flash_clk_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[doc = "Control Register write enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Control Register write enable."]
        #[inline(always)]
        pub const fn set_wr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Control Register write lock."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_lock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Control Register write lock."]
        #[inline(always)]
        pub const fn set_wr_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OPA Enalbe."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPA Enalbe."]
        #[inline(always)]
        pub const fn set_opa_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPA negative end channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_nsel(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OPA negative end channel selection."]
        #[inline(always)]
        pub const fn set_opa_nsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OPA positive end channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_psel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OPA positive end channel selection."]
        #[inline(always)]
        pub const fn set_opa_psel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("pll_cfg", &self.pll_cfg())
                .field("lockup_en", &self.lockup_en())
                .field("lockup_reset", &self.lockup_reset())
                .field("ldo_trim", &self.ldo_trim())
                .field("flash_clk_trim", &self.flash_clk_trim())
                .field("wr_en", &self.wr_en())
                .field("wr_lock", &self.wr_lock())
                .field("opa_en", &self.opa_en())
                .field("opa_nsel", &self.opa_nsel())
                .field("opa_psel", &self.opa_psel())
                .finish()
        }
    }
    #[doc = "Configure the extended key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Write key value."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write key value."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
}
