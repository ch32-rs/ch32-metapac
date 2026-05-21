#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Window watchdog."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdg {
    ptr: *mut u8,
}
unsafe impl Send for Wwdg {}
unsafe impl Sync for Wwdg {}
impl Wwdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register (WWDG_CR)."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Configuration register (WWDG_CFGR)."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status register (WWDG_SR)."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration register (WWDG_CFGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "7-bit window value."]
        #[inline(always)]
        pub const fn w(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit window value."]
        #[inline(always)]
        pub fn set_w(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Timer Base."]
        #[inline(always)]
        pub const fn wdgtb(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Timer Base."]
        #[inline(always)]
        pub fn set_wdgtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Early Wakeup Interrupt."]
        #[inline(always)]
        pub const fn ewi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Early Wakeup Interrupt."]
        #[inline(always)]
        pub fn set_ewi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "Control register (WWDG_CR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "7-bit counter (MSB to LSB)."]
        #[inline(always)]
        pub const fn t(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit counter (MSB to LSB)."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Activation bit."]
        #[inline(always)]
        pub const fn wdga(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Activation bit."]
        #[inline(always)]
        pub fn set_wdga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "Status register (WWDG_SR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Early Wakeup Interrupt Flag."]
        #[inline(always)]
        pub const fn ewif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Early Wakeup Interrupt Flag."]
        #[inline(always)]
        pub fn set_ewif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
}
