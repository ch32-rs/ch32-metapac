#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power control register (PWR_CTRL)."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Automatic wake-up control state register (PWR_AWUCSR)."]
    #[inline(always)]
    pub const fn awucsr(self) -> crate::common::Reg<regs::Awucsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Automatic wake window comparison value register (PWR_AWUWR)."]
    #[inline(always)]
    pub const fn awuwr(self) -> crate::common::Reg<regs::Awuwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Automatic wake-up control state register (PWR_AWUCSR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awucsr(pub u32);
    impl Awucsr {
        #[doc = "Automatic wake-up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn awuen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic wake-up enable."]
        #[inline(always)]
        pub const fn set_awuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Awucsr {
        #[inline(always)]
        fn default() -> Awucsr {
            Awucsr(0)
        }
    }
    impl core::fmt::Debug for Awucsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awucsr")
                .field("awuen", &self.awuen())
                .finish()
        }
    }
    #[doc = "Automatic wake window comparison value register (PWR_AWUWR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awuwr(pub u32);
    impl Awuwr {
        #[doc = "AWU window value."]
        #[must_use]
        #[inline(always)]
        pub const fn awuwr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "AWU window value."]
        #[inline(always)]
        pub const fn set_awuwr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Awuwr {
        #[inline(always)]
        fn default() -> Awuwr {
            Awuwr(0)
        }
    }
    impl core::fmt::Debug for Awuwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awuwr")
                .field("awuwr", &self.awuwr())
                .finish()
        }
    }
    #[doc = "Power control register (PWR_CTRL)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Power Down Deep Sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn pdds(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power Down Deep Sleep."]
        #[inline(always)]
        pub const fn set_pdds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ISINK Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isinken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ISINK Enable."]
        #[inline(always)]
        pub const fn set_isinken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PB4 function Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_io_vhv(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "PB4 function Selection."]
        #[inline(always)]
        pub const fn set_sel_io_vhv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "PMU over-temperature protection Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pmuproen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PMU over-temperature protection Enable."]
        #[inline(always)]
        pub const fn set_pmuproen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("pdds", &self.pdds())
                .field("isinken", &self.isinken())
                .field("sel_io_vhv", &self.sel_io_vhv())
                .field("pmuproen", &self.pmuproen())
                .finish()
        }
    }
}
