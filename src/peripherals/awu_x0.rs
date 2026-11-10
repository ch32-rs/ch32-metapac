#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "AWU configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Awu {
    ptr: *mut u8,
}
unsafe impl Send for Awu {}
unsafe impl Sync for Awu {}
impl Awu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status Control register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AWU Window register."]
    #[inline(always)]
    pub const fn wr(self) -> crate::common::Reg<regs::Wr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PSC."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Status Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u16);
    impl Csr {
        #[doc = "AWU Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AWU Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr").field("en", &self.en()).finish()
        }
    }
    #[doc = "PSC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psc(pub u16);
    impl Psc {
        #[doc = "AWU_TBR value."]
        #[must_use]
        #[inline(always)]
        pub const fn tbr(&self) -> super::vals::Prescaler {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Prescaler::from_bits(val as u8)
        }
        #[doc = "AWU_TBR value."]
        #[inline(always)]
        pub const fn set_tbr(&mut self, val: super::vals::Prescaler) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for Psc {
        #[inline(always)]
        fn default() -> Psc {
            Psc(0)
        }
    }
    impl core::fmt::Debug for Psc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psc").field("tbr", &self.tbr()).finish()
        }
    }
    #[doc = "AWU Window register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wr(pub u16);
    impl Wr {
        #[doc = "AWU_APR value."]
        #[must_use]
        #[inline(always)]
        pub const fn apr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "AWU_APR value."]
        #[inline(always)]
        pub const fn set_apr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
    }
    impl Default for Wr {
        #[inline(always)]
        fn default() -> Wr {
            Wr(0)
        }
    }
    impl core::fmt::Debug for Wr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wr").field("apr", &self.apr()).finish()
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Prescaler {
        #[doc = "DIV1."]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "DIV2."]
        DIV2 = 0x02,
        #[doc = "DIV4."]
        DIV4 = 0x03,
        #[doc = "DIV8."]
        DIV8 = 0x04,
        #[doc = "DIV16."]
        DIV16 = 0x05,
        #[doc = "DIV32."]
        DIV32 = 0x06,
        #[doc = "DIV64."]
        DIV64 = 0x07,
        #[doc = "DIV128."]
        DIV128 = 0x08,
        #[doc = "DIV256."]
        DIV256 = 0x09,
        #[doc = "DIV512."]
        DIV512 = 0x0a,
        #[doc = "DIV1024."]
        DIV1024 = 0x0b,
        #[doc = "DIV2048."]
        DIV2048 = 0x0c,
        #[doc = "DIV4096."]
        DIV4096 = 0x0d,
        #[doc = "DIV10240."]
        DIV10240 = 0x0e,
        #[doc = "DIV61440."]
        DIV61440 = 0x0f,
    }
    impl Prescaler {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prescaler {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prescaler {
        #[inline(always)]
        fn from(val: u8) -> Prescaler {
            Prescaler::from_bits(val)
        }
    }
    impl From<Prescaler> for u8 {
        #[inline(always)]
        fn from(val: Prescaler) -> u8 {
            Prescaler::to_bits(val)
        }
    }
}
