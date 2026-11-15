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
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
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
        #[doc = "Regulator in low-power mode during Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn lpds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Regulator in low-power mode during Stop."]
        #[inline(always)]
        pub const fn set_lpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enter Standby instead of Stop on deep sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn pdds(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Standby instead of Stop on deep sleep."]
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
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "PVD level selection."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
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
        #[doc = "Keep the 2K RAM powered in Standby (the 20K RAM on V20x D6)."]
        #[must_use]
        #[inline(always)]
        pub const fn r2ksty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Keep the 2K RAM powered in Standby (the 20K RAM on V20x D6)."]
        #[inline(always)]
        pub const fn set_r2ksty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Keep the 30K RAM powered in Standby (D8 parts)."]
        #[must_use]
        #[inline(always)]
        pub const fn r30ksty(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Keep the 30K RAM powered in Standby (D8 parts)."]
        #[inline(always)]
        pub const fn set_r30ksty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Keep the 2K RAM powered from VBAT in Standby (the 20K RAM on V20x D6)."]
        #[must_use]
        #[inline(always)]
        pub const fn r2kvbat(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Keep the 2K RAM powered from VBAT in Standby (the 20K RAM on V20x D6)."]
        #[inline(always)]
        pub const fn set_r2kvbat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Keep the 30K RAM powered from VBAT in Standby (D8 parts)."]
        #[must_use]
        #[inline(always)]
        pub const fn r30kvbat(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Keep the 30K RAM powered from VBAT in Standby (D8 parts)."]
        #[inline(always)]
        pub const fn set_r30kvbat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "RAM low-voltage mode in Stop (with LPDS)."]
        #[must_use]
        #[inline(always)]
        pub const fn ramlv(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "RAM low-voltage mode in Stop (with LPDS)."]
        #[inline(always)]
        pub const fn set_ramlv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("r2ksty", &self.r2ksty())
                .field("r30ksty", &self.r30ksty())
                .field("r2kvbat", &self.r2kvbat())
                .field("r30kvbat", &self.r30kvbat())
                .field("ramlv", &self.ramlv())
                .finish()
        }
    }
}
