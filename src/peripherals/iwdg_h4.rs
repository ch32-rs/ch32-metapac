#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Independent watchdog."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdg {
    ptr: *mut u8,
}
unsafe impl Send for Iwdg {}
unsafe impl Sync for Iwdg {}
impl Iwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Key register (IWDG_CTLR)."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Prescaler register (IWDG_PSCR)."]
    #[inline(always)]
    pub const fn pscr(self) -> crate::common::Reg<regs::Pscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Reload register (IWDG_RLDR)."]
    #[inline(always)]
    pub const fn rldr(self) -> crate::common::Reg<regs::Rldr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Status register (IWDG_SR)."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Key register (IWDG_CTLR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Key value."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Key value."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "Prescaler register (IWDG_PSCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pscr(pub u32);
    impl Pscr {
        #[doc = "Prescaler divider."]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Prescaler divider."]
        #[inline(always)]
        pub fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pscr {
        #[inline(always)]
        fn default() -> Pscr {
            Pscr(0)
        }
    }
    #[doc = "Reload register (IWDG_RLDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rldr(pub u32);
    impl Rldr {
        #[doc = "Watchdog counter reload value."]
        #[inline(always)]
        pub const fn rl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Watchdog counter reload value."]
        #[inline(always)]
        pub fn set_rl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Rldr {
        #[inline(always)]
        fn default() -> Rldr {
            Rldr(0)
        }
    }
    #[doc = "Status register (IWDG_SR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Watchdog prescaler value update."]
        #[inline(always)]
        pub const fn pvu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog prescaler value update."]
        #[inline(always)]
        pub fn set_pvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Watchdog counter reload value update."]
        #[inline(always)]
        pub const fn rvu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog counter reload value update."]
        #[inline(always)]
        pub fn set_rvu(&mut self, val: bool) {
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
