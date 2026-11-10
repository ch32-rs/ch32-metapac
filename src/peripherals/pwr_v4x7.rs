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
    #[doc = "Power control register (PWR_CTLR)."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power control/status register (PWR_CSR)."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Wake-up flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag."]
        #[inline(always)]
        pub const fn set_wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Standby flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Standby flag."]
        #[inline(always)]
        pub const fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD output."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD output."]
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable WKUP pin."]
        #[must_use]
        #[inline(always)]
        pub const fn ewup(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WKUP pin."]
        #[inline(always)]
        pub const fn set_ewup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("wuf", &self.wuf())
                .field("sbf", &self.sbf())
                .field("pvdo", &self.pvdo())
                .field("ewup", &self.ewup())
                .finish()
        }
    }
    #[doc = "Power control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Low-power deep sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn lpds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power deep sleep."]
        #[inline(always)]
        pub const fn set_lpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power-down deep sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn pdds(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power-down deep sleep."]
        #[inline(always)]
        pub const fn set_pdds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wake-up flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag."]
        #[inline(always)]
        pub const fn set_cwuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear standby flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag."]
        #[inline(always)]
        pub const fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Power voltage detector enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable."]
        #[inline(always)]
        pub const fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PVD level selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "PVD level selection."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Disable backup domain write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection."]
        #[inline(always)]
        pub const fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VDD18 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo18_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VDD18 enable."]
        #[inline(always)]
        pub const fn set_ldo18_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "32K SRAM retention in standby."]
        #[must_use]
        #[inline(always)]
        pub const fn r32ksty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "32K SRAM retention in standby."]
        #[inline(always)]
        pub const fn set_r32ksty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "32K SRAM supplied from VBAT."]
        #[must_use]
        #[inline(always)]
        pub const fn r32kvbat(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "32K SRAM supplied from VBAT."]
        #[inline(always)]
        pub const fn set_r32kvbat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM low-voltage mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ramlv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM low-voltage mode enable."]
        #[inline(always)]
        pub const fn set_ramlv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "VDDK voltage regulation position."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo_vddk(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "VDDK voltage regulation position."]
        #[inline(always)]
        pub const fn set_ldo_vddk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "VDD12A voltage regulation position."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo_vdd12a(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x07;
            val as u8
        }
        #[doc = "VDD12A voltage regulation position."]
        #[inline(always)]
        pub const fn set_ldo_vdd12a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
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
                .field("pdds", &self.pdds())
                .field("cwuf", &self.cwuf())
                .field("csbf", &self.csbf())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("ldo18_en", &self.ldo18_en())
                .field("r32ksty", &self.r32ksty())
                .field("r32kvbat", &self.r32kvbat())
                .field("ramlv", &self.ramlv())
                .field("ldo_vddk", &self.ldo_vddk())
                .field("ldo_vdd12a", &self.ldo_vdd12a())
                .finish()
        }
    }
}
