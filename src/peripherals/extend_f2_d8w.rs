#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extend configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extend {
    ptr: *mut u8,
}
unsafe impl Send for Extend {}
unsafe impl Sync for Extend {}
impl Extend {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EXTEND register."]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTEND register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr(pub u32);
    impl Ctr {
        #[doc = "USBD Lowspeed Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbdls(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USBD Lowspeed Enable."]
        #[inline(always)]
        pub const fn set_usbdls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USBD pullup Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbdpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USBD pullup Enable."]
        #[inline(always)]
        pub const fn set_usbdpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ETH 10M Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eth_10m_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ETH 10M Enable."]
        #[inline(always)]
        pub const fn set_eth_10m_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ETH RGMII Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eth_rgmii_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ETH RGMII Enable."]
        #[inline(always)]
        pub const fn set_eth_rgmii_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Whether HSI is divided."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_hsi_pre(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Whether HSI is divided."]
        #[inline(always)]
        pub const fn set_pll_hsi_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LOCKUP_Eable."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP_Eable."]
        #[inline(always)]
        pub const fn set_lockup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LOCKUP RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup_rstf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP RESET."]
        #[inline(always)]
        pub const fn set_lockup_rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULLDO_TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ulldo_trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "ULLDO_TRIM."]
        #[inline(always)]
        pub const fn set_ulldo_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "LDO_TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo_trim(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "LDO_TRIM."]
        #[inline(always)]
        pub const fn set_ldo_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Keep HSE oscillating in low-power modes (D8W only)."]
        #[must_use]
        #[inline(always)]
        pub const fn hsekplp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Keep HSE oscillating in low-power modes (D8W only)."]
        #[inline(always)]
        pub const fn set_hsekplp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ctr {
        #[inline(always)]
        fn default() -> Ctr {
            Ctr(0)
        }
    }
    impl core::fmt::Debug for Ctr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctr")
                .field("usbdls", &self.usbdls())
                .field("usbdpu", &self.usbdpu())
                .field("eth_10m_en", &self.eth_10m_en())
                .field("eth_rgmii_en", &self.eth_rgmii_en())
                .field("pll_hsi_pre", &self.pll_hsi_pre())
                .field("lockup_en", &self.lockup_en())
                .field("lockup_rstf", &self.lockup_rstf())
                .field("ulldo_trim", &self.ulldo_trim())
                .field("ldo_trim", &self.ldo_trim())
                .field("hsekplp", &self.hsekplp())
                .finish()
        }
    }
}
