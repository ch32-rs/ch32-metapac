#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose I/O. 8 lines per port."]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Port input data register (GPIOn_INDR)."]
    #[inline(always)]
    pub const fn indr(self) -> crate::common::Reg<regs::Indr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Port output data register (GPIOn_OUTDR)."]
    #[inline(always)]
    pub const fn outdr(self) -> crate::common::Reg<regs::Outdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSHR)."]
    #[inline(always)]
    pub const fn bshr(self) -> crate::common::Reg<regs::Bshr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Port bit reset register (GPIOn_BCR)."]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register."]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Port bit reset register (GPIOn_BCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Reset bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("br[0]", &self.br(0usize))
                .field("br[1]", &self.br(1usize))
                .field("br[2]", &self.br(2usize))
                .field("br[3]", &self.br(3usize))
                .field("br[4]", &self.br(4usize))
                .field("br[5]", &self.br(5usize))
                .field("br[6]", &self.br(6usize))
                .field("br[7]", &self.br(7usize))
                .finish()
        }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSHR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bshr(pub u32);
    impl Bshr {
        #[doc = "Set bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit 0."]
        #[inline(always)]
        pub const fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Bshr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bshr")
                .field("bs[0]", &self.bs(0usize))
                .field("bs[1]", &self.bs(1usize))
                .field("bs[2]", &self.bs(2usize))
                .field("bs[3]", &self.bs(3usize))
                .field("bs[4]", &self.bs(4usize))
                .field("bs[5]", &self.bs(5usize))
                .field("bs[6]", &self.bs(6usize))
                .field("bs[7]", &self.bs(7usize))
                .field("br[0]", &self.br(0usize))
                .field("br[1]", &self.br(1usize))
                .field("br[2]", &self.br(2usize))
                .field("br[3]", &self.br(3usize))
                .field("br[4]", &self.br(4usize))
                .field("br[5]", &self.br(5usize))
                .field("br[6]", &self.br(6usize))
                .field("br[7]", &self.br(7usize))
                .finish()
        }
    }
    #[doc = "Port configuration register low (GPIOn_CFGLR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfglr(pub u32);
    impl Cfglr {
        #[doc = "Port n.0 mode bits."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n.0 mode bits."]
        #[inline(always)]
        pub const fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n.0 configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cnf(&self, n: usize) -> super::vals::Cnf {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Cnf::from_bits(val as u8)
        }
        #[doc = "Port n.0 configuration bits."]
        #[inline(always)]
        pub const fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
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
    impl core::fmt::Debug for Cfglr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfglr")
                .field("mode[0]", &self.mode(0usize))
                .field("mode[1]", &self.mode(1usize))
                .field("mode[2]", &self.mode(2usize))
                .field("mode[3]", &self.mode(3usize))
                .field("mode[4]", &self.mode(4usize))
                .field("mode[5]", &self.mode(5usize))
                .field("mode[6]", &self.mode(6usize))
                .field("mode[7]", &self.mode(7usize))
                .field("cnf[0]", &self.cnf(0usize))
                .field("cnf[1]", &self.cnf(1usize))
                .field("cnf[2]", &self.cnf(2usize))
                .field("cnf[3]", &self.cnf(3usize))
                .field("cnf[4]", &self.cnf(4usize))
                .field("cnf[5]", &self.cnf(5usize))
                .field("cnf[6]", &self.cnf(6usize))
                .field("cnf[7]", &self.cnf(7usize))
                .finish()
        }
    }
    #[doc = "Port input data register (GPIOn_INDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Indr(pub u32);
    impl Indr {
        #[doc = "Port input data."]
        #[must_use]
        #[inline(always)]
        pub const fn idr(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port input data."]
        #[inline(always)]
        pub const fn set_idr(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Indr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Indr")
                .field("idr[0]", &self.idr(0usize))
                .field("idr[1]", &self.idr(1usize))
                .field("idr[2]", &self.idr(2usize))
                .field("idr[3]", &self.idr(3usize))
                .field("idr[4]", &self.idr(4usize))
                .field("idr[5]", &self.idr(5usize))
                .field("idr[6]", &self.idr(6usize))
                .field("idr[7]", &self.idr(7usize))
                .finish()
        }
    }
    #[doc = "Port configuration lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckr(pub u32);
    impl Lckr {
        #[doc = "Port A Lock bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn lck(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port A Lock bit 0."]
        #[inline(always)]
        pub const fn set_lck(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Lock key."]
        #[must_use]
        #[inline(always)]
        pub const fn lckk(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Lock key."]
        #[inline(always)]
        pub const fn set_lckk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Lckr {
        #[inline(always)]
        fn default() -> Lckr {
            Lckr(0)
        }
    }
    impl core::fmt::Debug for Lckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lckr")
                .field("lck[0]", &self.lck(0usize))
                .field("lck[1]", &self.lck(1usize))
                .field("lck[2]", &self.lck(2usize))
                .field("lck[3]", &self.lck(3usize))
                .field("lck[4]", &self.lck(4usize))
                .field("lck[5]", &self.lck(5usize))
                .field("lck[6]", &self.lck(6usize))
                .field("lck[7]", &self.lck(7usize))
                .field("lckk", &self.lckk())
                .finish()
        }
    }
    #[doc = "Port output data register (GPIOn_OUTDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Outdr(pub u32);
    impl Outdr {
        #[doc = "Port output data."]
        #[must_use]
        #[inline(always)]
        pub const fn odr(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port output data."]
        #[inline(always)]
        pub const fn set_odr(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Outdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Outdr")
                .field("odr[0]", &self.odr(0usize))
                .field("odr[1]", &self.odr(1usize))
                .field("odr[2]", &self.odr(2usize))
                .field("odr[3]", &self.odr(3usize))
                .field("odr[4]", &self.odr(4usize))
                .field("odr[5]", &self.odr(5usize))
                .field("odr[6]", &self.odr(6usize))
                .field("odr[7]", &self.odr(7usize))
                .finish()
        }
    }
}
pub mod vals {
    #[doc = "port x configuration selection, configure the corresponding port by these bits."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
}
