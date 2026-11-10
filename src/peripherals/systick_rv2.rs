#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Systick registers, 32-bit downcounter for QingKeV2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systick {
    ptr: *mut u8,
}
unsafe impl Send for Systick {}
unsafe impl Sync for Systick {}
impl Systick {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System count control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "System count status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "System counter register, 32-bit."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "System counter register, 32-bit."]
    #[inline(always)]
    pub const fn cntl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "System count compare register, 32-bit."]
    #[inline(always)]
    pub const fn cmp(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "System count compare register, 32-bit."]
    #[inline(always)]
    pub const fn cmpl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "System count control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Counter enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ste(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable control bit."]
        #[inline(always)]
        pub const fn set_ste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter interrupt enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn stie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter interrupt enable control bit."]
        #[inline(always)]
        pub const fn set_stie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counter system clock sourse selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn stclk(&self) -> super::vals::Stclk {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Stclk::from_bits(val as u8)
        }
        #[doc = "Counter system clock sourse selection bit."]
        #[inline(always)]
        pub const fn set_stclk(&mut self, val: super::vals::Stclk) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Auto reload count enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn stre(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auto reload count enable bit."]
        #[inline(always)]
        pub const fn set_stre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Software interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn swie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt enable."]
        #[inline(always)]
        pub const fn set_swie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    impl core::fmt::Debug for Ctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr")
                .field("ste", &self.ste())
                .field("stie", &self.stie())
                .field("stclk", &self.stclk())
                .field("stre", &self.stre())
                .field("swie", &self.swie())
                .finish()
        }
    }
    #[doc = "System count status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Count value compare flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cntif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Count value compare flag."]
        #[inline(always)]
        pub const fn set_cntif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr").field("cntif", &self.cntif()).finish()
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stclk {
        #[doc = "HCLK/8."]
        HCLK_DIV8 = 0x0,
        #[doc = "HCLK."]
        HCLK = 0x01,
    }
    impl Stclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stclk {
        #[inline(always)]
        fn from(val: u8) -> Stclk {
            Stclk::from_bits(val)
        }
    }
    impl From<Stclk> for u8 {
        #[inline(always)]
        fn from(val: Stclk) -> u8 {
            Stclk::to_bits(val)
        }
    }
}
