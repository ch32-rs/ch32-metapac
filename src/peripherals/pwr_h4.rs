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
    #[doc = "Power control register (PWR_CSR)."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power control register (PWR_CSR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "PVD Output."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVD Output."]
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VIO18 power supply initial status indicator."]
        #[must_use]
        #[inline(always)]
        pub const fn vio18_sr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "VIO18 power supply initial status indicator."]
        #[inline(always)]
        pub const fn set_vio18_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
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
            f.debug_struct("Csr")
                .field("pvdo", &self.pvdo())
                .field("vio18_sr", &self.vio18_sr())
                .finish()
        }
    }
    #[doc = "Power control register (PWR_CTRL)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Low Power Deep Sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn lpds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low Power Deep Sleep."]
        #[inline(always)]
        pub const fn set_lpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power Voltage Detector Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Power Voltage Detector Enable."]
        #[inline(always)]
        pub const fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PVD Level Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "PVD Level Selection."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "Disable Backup Domain write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Backup Domain write protection."]
        #[inline(always)]
        pub const fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VIO18 power regulation mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_sw_cr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VIO18 power regulation mode selection."]
        #[inline(always)]
        pub const fn set_vio_sw_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "VIO18 power supply adjustment position."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel_vio18(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "VIO18 power supply adjustment position."]
        #[inline(always)]
        pub const fn set_vsel_vio18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
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
                .field("lpds", &self.lpds())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("vio_sw_cr", &self.vio_sw_cr())
                .field("vsel_vio18", &self.vsel_vio18())
                .finish()
        }
    }
}
