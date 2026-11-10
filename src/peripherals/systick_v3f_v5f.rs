#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Systick registers for the CH32H4 dual-core SysTick. Two independent 32-bit counters, indexed by hart, and one status register shared by both."]
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
    pub const fn ctlr(self, n: usize) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 128usize) as _) }
    }
    #[doc = "System counter interrupt status register. Shared by both counters."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "System counter count register."]
    #[inline(always)]
    pub const fn cnt(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 128usize) as _)
        }
    }
    #[doc = "System count compare register."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 128usize) as _)
        }
    }
}
pub mod regs {
    #[doc = "System count control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ste(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn set_ste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn stie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter interrupt enable."]
        #[inline(always)]
        pub const fn set_stie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counter clock source. Both counters are clocked from the V3F HCLK domain."]
        #[must_use]
        #[inline(always)]
        pub const fn stclk(&self) -> super::vals::Stclk {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Stclk::from_bits(val as u8)
        }
        #[doc = "Counter clock source. Both counters are clocked from the V3F HCLK domain."]
        #[inline(always)]
        pub const fn set_stclk(&mut self, val: super::vals::Stclk) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Auto reload enable. Leave clear for a free-running counter."]
        #[must_use]
        #[inline(always)]
        pub const fn stre(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auto reload enable. Leave clear for a free-running counter."]
        #[inline(always)]
        pub const fn set_stre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Count mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Count mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Hart the counter interrupt is routed to. Updated to the operating hart when STE is set."]
        #[must_use]
        #[inline(always)]
        pub const fn cid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Hart the counter interrupt is routed to. Updated to the operating hart when STE is set."]
        #[inline(always)]
        pub const fn set_cid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("mode", &self.mode())
                .field("cid", &self.cid())
                .finish()
        }
    }
    #[doc = "System counter interrupt status register (shared)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Counter 0 compare flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cntif0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter 0 compare flag."]
        #[inline(always)]
        pub const fn set_cntif0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter 1 compare flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cntif1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter 1 compare flag."]
        #[inline(always)]
        pub const fn set_cntif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            f.debug_struct("Sr")
                .field("cntif0", &self.cntif0())
                .field("cntif1", &self.cntif1())
                .finish()
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mode {
        #[doc = "Upcount."]
        UPCOUNT = 0x0,
        #[doc = "Downcount."]
        DOWNCOUNT = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stclk {
        #[doc = "V3F HCLK/8."]
        HCLK_DIV8 = 0x0,
        #[doc = "V3F HCLK."]
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
