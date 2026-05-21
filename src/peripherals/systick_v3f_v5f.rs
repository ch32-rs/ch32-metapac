#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Systick registers for V3F + V5F dual-core (currently CH32H417). Two independent 32-bit up/down counters with per-counter core ID (CID) routing for interrupts."]
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
    #[doc = "System count control register for counter 0."]
    #[inline(always)]
    pub const fn ctlr_0(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "System counter interrupt status register (shared by both counters)."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "System counter 0 count register."]
    #[inline(always)]
    pub const fn cnt_0(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "System counter 0 compare register."]
    #[inline(always)]
    pub const fn cmp_0(self) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "System count control register for counter 1."]
    #[inline(always)]
    pub const fn ctlr_1(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "System counter 1 count register."]
    #[inline(always)]
    pub const fn cnt_1(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "System counter 1 compare register."]
    #[inline(always)]
    pub const fn cmp_1(self) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
}
pub mod regs {
    #[doc = "System count compare register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "32-bit compare value (used as reload value when counting down)."]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32-bit compare value (used as reload value when counting down)."]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    #[doc = "System counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "32-bit Systick counter value."]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32-bit Systick counter value."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    #[doc = "System count control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter interrupt enable."]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter interrupt enable."]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Counter clock source selection."]
        #[inline(always)]
        pub const fn no_rtc(&self) -> super::vals::Stclk {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Stclk::from_bits(val as u8)
        }
        #[doc = "Counter clock source selection."]
        #[inline(always)]
        pub fn set_no_rtc(&mut self, val: super::vals::Stclk) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Auto reload enable."]
        #[inline(always)]
        pub const fn auto_reload(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auto reload enable."]
        #[inline(always)]
        pub fn set_auto_reload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Count mode."]
        #[inline(always)]
        pub const fn down_mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Count mode."]
        #[inline(always)]
        pub fn set_down_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core ID for routing the counter interrupt to a specific hart. Auto-updated when EN is set to the operating hart's core ID."]
        #[inline(always)]
        pub const fn cid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Core ID for routing the counter interrupt to a specific hart. Auto-updated when EN is set to the operating hart's core ID."]
        #[inline(always)]
        pub fn set_cid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "System counter interrupt status register (shared)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Systick0 interrupt flag."]
        #[inline(always)]
        pub const fn isr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Systick0 interrupt flag."]
        #[inline(always)]
        pub fn set_isr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Systick1 interrupt flag."]
        #[inline(always)]
        pub const fn isr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Systick1 interrupt flag."]
        #[inline(always)]
        pub fn set_isr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
