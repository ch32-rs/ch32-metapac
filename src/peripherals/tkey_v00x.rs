#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Touch key. Aliases ADC IDATAR1/RDATAR when ADC TKENABLE=1; enable via ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tkey {
    ptr: *mut u8,
}
unsafe impl Send for Tkey {}
unsafe impl Sync for Tkey {}
impl Tkey {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "charge time configuration register."]
    #[inline(always)]
    pub const fn tkey_chg(self) -> crate::common::Reg<regs::TkeyChg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "start and discharge time register."]
    #[inline(always)]
    pub const fn tkey_dischg(self) -> crate::common::Reg<regs::TkeyDischg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "data register."]
    #[inline(always)]
    pub const fn tkey_dr(self) -> crate::common::Reg<regs::TkeyDr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "charge time configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyChg(pub u32);
    impl TkeyChg {
        #[doc = "Touch key charge time."]
        #[inline(always)]
        pub const fn tkcharge(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Touch key charge time."]
        #[inline(always)]
        pub fn set_tkcharge(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for TkeyChg {
        #[inline(always)]
        fn default() -> TkeyChg {
            TkeyChg(0)
        }
    }
    #[doc = "start and discharge time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyDischg(pub u32);
    impl TkeyDischg {
        #[doc = "Touch key start and discharge time."]
        #[inline(always)]
        pub const fn tkact_dcg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Touch key start and discharge time."]
        #[inline(always)]
        pub fn set_tkact_dcg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for TkeyDischg {
        #[inline(always)]
        fn default() -> TkeyDischg {
            TkeyDischg(0)
        }
    }
    #[doc = "data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyDr(pub u32);
    impl TkeyDr {
        #[doc = "Converted data."]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Converted data."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TkeyDr {
        #[inline(always)]
        fn default() -> TkeyDr {
            TkeyDr(0)
        }
    }
}
