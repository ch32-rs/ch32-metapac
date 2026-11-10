#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose I/O. 24 lines per port."]
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
    #[doc = "Port configuration register high (GPIOn_CFGHR)."]
    #[inline(always)]
    pub const fn cfghr(self) -> crate::common::Reg<regs::Cfghr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Port input data register (GPIOn_INDR)."]
    #[inline(always)]
    pub const fn indr(self) -> crate::common::Reg<regs::Indr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Port output data register (GPIOn_OUTDR)."]
    #[inline(always)]
    pub const fn outdr(self) -> crate::common::Reg<regs::Outdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSHR)."]
    #[inline(always)]
    pub const fn bshr(self) -> crate::common::Reg<regs::Bshr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Port bit reset register (GPIOn_BCR)."]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register."]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Port configuration register extension (GPIOn_CFGXR)."]
    #[inline(always)]
    pub const fn cfgxr(self) -> crate::common::Reg<regs::Cfgxr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSXR)."]
    #[inline(always)]
    pub const fn bsxr(self) -> crate::common::Reg<regs::Bsxr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
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
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
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
                .field("br[8]", &self.br(8usize))
                .field("br[9]", &self.br(9usize))
                .field("br[10]", &self.br(10usize))
                .field("br[11]", &self.br(11usize))
                .field("br[12]", &self.br(12usize))
                .field("br[13]", &self.br(13usize))
                .field("br[14]", &self.br(14usize))
                .field("br[15]", &self.br(15usize))
                .field("br[16]", &self.br(16usize))
                .field("br[17]", &self.br(17usize))
                .field("br[18]", &self.br(18usize))
                .field("br[19]", &self.br(19usize))
                .field("br[20]", &self.br(20usize))
                .field("br[21]", &self.br(21usize))
                .field("br[22]", &self.br(22usize))
                .field("br[23]", &self.br(23usize))
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
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit 0."]
        #[inline(always)]
        pub const fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 0."]
        #[inline(always)]
        pub const fn set_br(&mut self, n: usize, val: bool) {
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
                .field("bs[8]", &self.bs(8usize))
                .field("bs[9]", &self.bs(9usize))
                .field("bs[10]", &self.bs(10usize))
                .field("bs[11]", &self.bs(11usize))
                .field("bs[12]", &self.bs(12usize))
                .field("bs[13]", &self.bs(13usize))
                .field("bs[14]", &self.bs(14usize))
                .field("bs[15]", &self.bs(15usize))
                .field("br[0]", &self.br(0usize))
                .field("br[1]", &self.br(1usize))
                .field("br[2]", &self.br(2usize))
                .field("br[3]", &self.br(3usize))
                .field("br[4]", &self.br(4usize))
                .field("br[5]", &self.br(5usize))
                .field("br[6]", &self.br(6usize))
                .field("br[7]", &self.br(7usize))
                .field("br[8]", &self.br(8usize))
                .field("br[9]", &self.br(9usize))
                .field("br[10]", &self.br(10usize))
                .field("br[11]", &self.br(11usize))
                .field("br[12]", &self.br(12usize))
                .field("br[13]", &self.br(13usize))
                .field("br[14]", &self.br(14usize))
                .field("br[15]", &self.br(15usize))
                .finish()
        }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSXR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsxr(pub u32);
    impl Bsxr {
        #[doc = "Set bit 16."]
        #[must_use]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit 16."]
        #[inline(always)]
        pub const fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit 16."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit 16."]
        #[inline(always)]
        pub const fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bsxr {
        #[inline(always)]
        fn default() -> Bsxr {
            Bsxr(0)
        }
    }
    impl core::fmt::Debug for Bsxr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsxr")
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
    #[doc = "Port configuration register high (GPIOn_CFGHR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfghr(pub u32);
    impl Cfghr {
        #[doc = "Port n.8 mode bits."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n.8 mode bits."]
        #[inline(always)]
        pub const fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n.8 configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cnf(&self, n: usize) -> super::vals::Cnf {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Cnf::from_bits(val as u8)
        }
        #[doc = "Port n.8 configuration bits."]
        #[inline(always)]
        pub const fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
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
    impl core::fmt::Debug for Cfghr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfghr")
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
    #[doc = "Port configuration register extension (GPIOn_CFGXR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgxr(pub u32);
    impl Cfgxr {
        #[doc = "Port n.16 mode bits."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n.16 mode bits."]
        #[inline(always)]
        pub const fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n.16 configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cnf(&self, n: usize) -> super::vals::Cnf {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Cnf::from_bits(val as u8)
        }
        #[doc = "Port n.16 configuration bits."]
        #[inline(always)]
        pub const fn set_cnf(&mut self, n: usize, val: super::vals::Cnf) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Cfgxr {
        #[inline(always)]
        fn default() -> Cfgxr {
            Cfgxr(0)
        }
    }
    impl core::fmt::Debug for Cfgxr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgxr")
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
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port input data."]
        #[inline(always)]
        pub const fn set_idr(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
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
                .field("idr[8]", &self.idr(8usize))
                .field("idr[9]", &self.idr(9usize))
                .field("idr[10]", &self.idr(10usize))
                .field("idr[11]", &self.idr(11usize))
                .field("idr[12]", &self.idr(12usize))
                .field("idr[13]", &self.idr(13usize))
                .field("idr[14]", &self.idr(14usize))
                .field("idr[15]", &self.idr(15usize))
                .field("idr[16]", &self.idr(16usize))
                .field("idr[17]", &self.idr(17usize))
                .field("idr[18]", &self.idr(18usize))
                .field("idr[19]", &self.idr(19usize))
                .field("idr[20]", &self.idr(20usize))
                .field("idr[21]", &self.idr(21usize))
                .field("idr[22]", &self.idr(22usize))
                .field("idr[23]", &self.idr(23usize))
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
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port A Lock bit 0."]
        #[inline(always)]
        pub const fn set_lck(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Lock key."]
        #[must_use]
        #[inline(always)]
        pub const fn lckk(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Lock key."]
        #[inline(always)]
        pub const fn set_lckk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("lck[8]", &self.lck(8usize))
                .field("lck[9]", &self.lck(9usize))
                .field("lck[10]", &self.lck(10usize))
                .field("lck[11]", &self.lck(11usize))
                .field("lck[12]", &self.lck(12usize))
                .field("lck[13]", &self.lck(13usize))
                .field("lck[14]", &self.lck(14usize))
                .field("lck[15]", &self.lck(15usize))
                .field("lck[16]", &self.lck(16usize))
                .field("lck[17]", &self.lck(17usize))
                .field("lck[18]", &self.lck(18usize))
                .field("lck[19]", &self.lck(19usize))
                .field("lck[20]", &self.lck(20usize))
                .field("lck[21]", &self.lck(21usize))
                .field("lck[22]", &self.lck(22usize))
                .field("lck[23]", &self.lck(23usize))
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
            assert!(n < 24usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port output data."]
        #[inline(always)]
        pub const fn set_odr(&mut self, n: usize, val: bool) {
            assert!(n < 24usize);
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
                .field("odr[8]", &self.odr(8usize))
                .field("odr[9]", &self.odr(9usize))
                .field("odr[10]", &self.odr(10usize))
                .field("odr[11]", &self.odr(11usize))
                .field("odr[12]", &self.odr(12usize))
                .field("odr[13]", &self.odr(13usize))
                .field("odr[14]", &self.odr(14usize))
                .field("odr[15]", &self.odr(15usize))
                .field("odr[16]", &self.odr(16usize))
                .field("odr[17]", &self.odr(17usize))
                .field("odr[18]", &self.odr(18usize))
                .field("odr[19]", &self.odr(19usize))
                .field("odr[20]", &self.odr(20usize))
                .field("odr[21]", &self.odr(21usize))
                .field("odr[22]", &self.odr(22usize))
                .field("odr[23]", &self.odr(23usize))
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
        #[doc = "Floating input, no Open Drain output."]
        FLOATING_IN__OPEN_DRAIN_OUT = 0x01,
        #[doc = "Input with pull-up or pull down / AF pull output"]
        PULL_IN__AF_PUSH_PULL_OUT = 0x02,
        _RESERVED_3 = 0x03,
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
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
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
