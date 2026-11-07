#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose I/O. 16 lines per port."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port configuration register low (GPIOn_CFGLR)."]
    #[inline(always)]
    pub const fn cfglr(self) -> crate::common::Reg<regs::Cfglr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port configuration register high (GPIOn_CFGHR)."]
    #[inline(always)]
    pub const fn cfghr(self) -> crate::common::Reg<regs::Cfghr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Port input data register (GPIOn_INDR)."]
    #[inline(always)]
    pub const fn indr(self) -> crate::common::Reg<regs::Indr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port output data register (GPIOn_OUTDR)."]
    #[inline(always)]
    pub const fn outdr(self) -> crate::common::Reg<regs::Outdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSHR)."]
    #[inline(always)]
    pub const fn bshr(self) -> crate::common::Reg<regs::Bshr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port bit reset register (GPIOn_BCR)."]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register."]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Port output speed register (GPIOn_SPEED)."]
    #[inline(always)]
    pub const fn speed(self) -> crate::common::Reg<regs::Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Port bit reset register (GPIOn_BCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSHR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bshr(pub u32);
    impl Bshr {
        #[doc = "Set bit 0."]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit 0."]
        #[inline(always)]
        pub fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bshr {
        #[inline(always)]
        fn default() -> Bshr {
            Bshr(0)
        }
    }
    #[doc = "Port configuration register high (GPIOn_CFGHR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfghr(pub u32);
    impl Cfghr {
        #[doc = "Port n.8 mode bits."]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n.8 mode bits."]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n.8 configuration bits."]
        #[inline(always)]
        pub const fn cnf(&self, n: usize) -> super::vals::Cnf {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Cnf::from_bits(val as u8)
        }
        #[doc = "Port n.8 configuration bits."]
        #[inline(always)]
        pub fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Cfghr {
        #[inline(always)]
        fn default() -> Cfghr {
            Cfghr(0)
        }
    }
    #[doc = "Port configuration register low (GPIOn_CFGLR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfglr(pub u32);
    impl Cfglr {
        #[doc = "Port n.0 mode bits."]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n.0 mode bits."]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n.0 configuration bits."]
        #[inline(always)]
        pub const fn cnf(&self, n: usize) -> super::vals::Cnf {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Cnf::from_bits(val as u8)
        }
        #[doc = "Port n.0 configuration bits."]
        #[inline(always)]
        pub fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Cfglr {
        #[inline(always)]
        fn default() -> Cfglr {
            Cfglr(0)
        }
    }
    #[doc = "Port input data register (GPIOn_INDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Indr(pub u32);
    impl Indr {
        #[doc = "Port input data."]
        #[inline(always)]
        pub const fn idr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port input data."]
        #[inline(always)]
        pub fn set_idr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Indr {
        #[inline(always)]
        fn default() -> Indr {
            Indr(0)
        }
    }
    #[doc = "Port configuration lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckr(pub u32);
    impl Lckr {
        #[doc = "Port A Lock bit 0."]
        #[inline(always)]
        pub const fn lck(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port A Lock bit 0."]
        #[inline(always)]
        pub fn set_lck(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Lock key."]
        #[inline(always)]
        pub const fn lckk(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Lock key."]
        #[inline(always)]
        pub fn set_lckk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Lckr {
        #[inline(always)]
        fn default() -> Lckr {
            Lckr(0)
        }
    }
    #[doc = "Port output data register (GPIOn_OUTDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Outdr(pub u32);
    impl Outdr {
        #[doc = "Port output data."]
        #[inline(always)]
        pub const fn odr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port output data."]
        #[inline(always)]
        pub fn set_odr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Outdr {
        #[inline(always)]
        fn default() -> Outdr {
            Outdr(0)
        }
    }
    #[doc = "Port output speed register (GPIOn_SPEED)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Speed(pub u32);
    impl Speed {
        #[doc = "Port n output speed bits."]
        #[inline(always)]
        pub const fn speed(&self, n: usize) -> super::vals::Speed {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Speed::from_bits(val as u8)
        }
        #[doc = "Port n output speed bits."]
        #[inline(always)]
        pub fn set_speed(&mut self, n: usize, val: super::vals::Speed) {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Speed {
        #[inline(always)]
        fn default() -> Speed {
            Speed(0)
        }
    }
}
pub mod vals {
    #[doc = "port x configuration selection, configure the corresponding port by these bits."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cnf {
        #[doc = "Analog input / push-pull output."]
        ANALOG_IN__PUSH_PULL_OUT = 0x0,
        #[doc = "Floating input / open-drain output."]
        FLOATING_IN__OPEN_DRAIN_OUT = 0x01,
        #[doc = "Input with pull-up / AF pull-down."]
        PULL_IN__AF_PUSH_PULL_OUT = 0x02,
        #[doc = "Alternate function output push-pull"]
        AF_OPEN_DRAIN_OUT = 0x03,
    }
    impl Cnf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cnf {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cnf {
        #[inline(always)]
        fn from(val: u8) -> Cnf {
            Cnf::from_bits(val)
        }
    }
    impl From<Cnf> for u8 {
        #[inline(always)]
        fn from(val: Cnf) -> u8 {
            Cnf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mode {
        #[doc = "Input mode."]
        INPUT = 0x0,
        #[doc = "Output mode."]
        OUTPUT_10MHZ = 0x01,
        #[doc = "Output mode."]
        OUTPUT_2MHZ = 0x02,
        #[doc = "Output mode."]
        OUTPUT_50MHZ = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
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
    #[doc = "Port output speed selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Speed {
        #[doc = "10 MHz."]
        LOW = 0x0,
        #[doc = "50 MHz."]
        MEDIUM = 0x01,
        #[doc = "100 MHz."]
        HIGH = 0x02,
        #[doc = "180 MHz."]
        VERY_HIGH = 0x03,
    }
    impl Speed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Speed {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Speed {
        #[inline(always)]
        fn from(val: u8) -> Speed {
            Speed::from_bits(val)
        }
    }
    impl From<Speed> for u8 {
        #[inline(always)]
        fn from(val: Speed) -> u8 {
            Speed::to_bits(val)
        }
    }
}
