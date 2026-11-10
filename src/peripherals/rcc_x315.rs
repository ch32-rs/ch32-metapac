#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock configuration register (RCC_CFGR0)."]
    #[inline(always)]
    pub const fn cfgr0(self) -> crate::common::Reg<regs::Cfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock interrupt register (RCC_INTR)."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "HB2 peripheral reset register (RCC_HB2PRSTR)."]
    #[inline(always)]
    pub const fn hb2prstr(self) -> crate::common::Reg<regs::Hb2prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "HB1 peripheral reset register (RCC_HB1PRSTR)."]
    #[inline(always)]
    pub const fn hb1prstr(self) -> crate::common::Reg<regs::Hb1prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "HB Peripheral Clock enable register (RCC_HBPCENR)."]
    #[inline(always)]
    pub const fn hbpcenr(self) -> crate::common::Reg<regs::Hbpcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "HB2 peripheral clock enable register (RCC_HB2PCENR)."]
    #[inline(always)]
    pub const fn hb2pcenr(self) -> crate::common::Reg<regs::Hb2pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "HB1 peripheral clock enable register (RCC_HB1PCENR)."]
    #[inline(always)]
    pub const fn hb1pcenr(self) -> crate::common::Reg<regs::Hb1pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[inline(always)]
    pub const fn rstsckr(self) -> crate::common::Reg<regs::Rstsckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "HB reset register (RCC_PHBRSTR)."]
    #[inline(always)]
    pub const fn hbrstr(self) -> crate::common::Reg<regs::Hbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Clock configuration register2 (RCC_CFGR2)."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock configuration register (RCC_CFGR0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr0(pub u32);
    impl Cfgr0 {
        #[doc = "System clock Switch."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "System clock Switch."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "System Clock Switch Status."]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "System Clock Switch Status."]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "HB prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "HB prescaler."]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "HCLK clock pre-divider selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fpre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HCLK clock pre-divider selection."]
        #[inline(always)]
        pub const fn set_fpre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC clock prescaler (PB2)."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "ADC clock prescaler (PB2)."]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[doc = "ADC prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adcpre(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "ADC prescaler."]
        #[inline(always)]
        pub const fn set_adcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "The frequency division ratios of the UBSSS PLL 625M clock and USBHS PLL 480M clock."]
        #[must_use]
        #[inline(always)]
        pub const fn plldiv(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[doc = "The frequency division ratios of the UBSSS PLL 625M clock and USBHS PLL 480M clock."]
        #[inline(always)]
        pub const fn set_plldiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
        #[doc = "PLLCLK clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "PLLCLK clock selection."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "UTMI clock gating enable."]
        #[must_use]
        #[inline(always)]
        pub const fn utmion(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UTMI clock gating enable."]
        #[inline(always)]
        pub const fn set_utmion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Microcontroller clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn mco(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Microcontroller clock output."]
        #[inline(always)]
        pub const fn set_mco(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "PLL clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhs_src(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "PLL clock selection."]
        #[inline(always)]
        pub const fn set_pllhs_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Cfgr0 {
        #[inline(always)]
        fn default() -> Cfgr0 {
            Cfgr0(0)
        }
    }
    impl core::fmt::Debug for Cfgr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr0")
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("hpre", &self.hpre())
                .field("fpre", &self.fpre())
                .field("ppre2", &self.ppre2())
                .field("adcpre", &self.adcpre())
                .field("plldiv", &self.plldiv())
                .field("pllsrc", &self.pllsrc())
                .field("utmion", &self.utmion())
                .field("mco", &self.mco())
                .field("pllhs_src", &self.pllhs_src())
                .finish()
        }
    }
    #[doc = "Clock configuration register2 (RCC_CFGR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "USBHS PLL refrence clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhspllrefsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "USBHS PLL refrence clock selection."]
        #[inline(always)]
        pub const fn set_usbhspllrefsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "USBSS PLL refrence clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsspllrefsel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "USBSS PLL refrence clock selection."]
        #[inline(always)]
        pub const fn set_usbsspllrefsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "USB PLL clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpllsrccfg(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "USB PLL clock source selection."]
        #[inline(always)]
        pub const fn set_usbpllsrccfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "USBHS PLL clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhspllsrc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL clock source selection."]
        #[inline(always)]
        pub const fn set_usbhspllsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USBSS PLL clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsspllsrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS PLL clock source selection."]
        #[inline(always)]
        pub const fn set_usbsspllsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock stable time configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdycfg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "HSE clock stable time configuration bit."]
        #[inline(always)]
        pub const fn set_hserdycfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "USBHS PLL clock stable time configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhspllrdycfg(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "USBHS PLL clock stable time configuration bit."]
        #[inline(always)]
        pub const fn set_usbhspllrdycfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "USBSS PLL clock stable time configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsspllrdycfg(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "USBSS PLL clock stable time configuration bit."]
        #[inline(always)]
        pub const fn set_usbsspllrdycfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "ADC input clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adcsrc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC input clock selection."]
        #[inline(always)]
        pub const fn set_adcsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC clock duty cycle mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adcdutymd(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "ADC clock duty cycle mode selection."]
        #[inline(always)]
        pub const fn set_adcdutymd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "ADC input clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adcpllsrc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ADC input clock source selection."]
        #[inline(always)]
        pub const fn set_adcpllsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADC input clock source selection uses the pre-divider control bits of. either USBHS_PLL at 480MHz or USBSS_PLL at 625MHz."]
        #[must_use]
        #[inline(always)]
        pub const fn adcclkdiv(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC input clock source selection uses the pre-divider control bits of. either USBHS_PLL at 480MHz or USBSS_PLL at 625MHz."]
        #[inline(always)]
        pub const fn set_adcclkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("usbhspllrefsel", &self.usbhspllrefsel())
                .field("usbsspllrefsel", &self.usbsspllrefsel())
                .field("usbpllsrccfg", &self.usbpllsrccfg())
                .field("usbhspllsrc", &self.usbhspllsrc())
                .field("usbsspllsrc", &self.usbsspllsrc())
                .field("hserdycfg", &self.hserdycfg())
                .field("usbhspllrdycfg", &self.usbhspllrdycfg())
                .field("usbsspllrdycfg", &self.usbsspllrdycfg())
                .field("adcsrc", &self.adcsrc())
                .field("adcdutymd", &self.adcdutymd())
                .field("adcpllsrc", &self.adcpllsrc())
                .field("adcclkdiv", &self.adcclkdiv())
                .finish()
        }
    }
    #[doc = "Clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Internal High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable."]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LMP clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmon(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LMP clock enable."]
        #[inline(always)]
        pub const fn set_lpmon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable."]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock ready flag."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External High Speed clock Bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock Bypass."]
        #[inline(always)]
        pub const fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock Security System enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System enable."]
        #[inline(always)]
        pub const fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USBHS PLL clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL clock enable."]
        #[inline(always)]
        pub const fn set_usbhs_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USBHS PLL clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhs_pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL clock ready flag."]
        #[inline(always)]
        pub const fn set_usbhs_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USBSS PLL clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbss_pllon(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS PLL clock enable."]
        #[inline(always)]
        pub const fn set_usbss_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USBSS PLL clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn usbss_pllrdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS PLL clock ready flag."]
        #[inline(always)]
        pub const fn set_usbss_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PIPE clock gating enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn pipeon(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PIPE clock gating enablement."]
        #[inline(always)]
        pub const fn set_pipeon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator indication bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pi_xi_stat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator indication bit."]
        #[inline(always)]
        pub const fn set_pi_xi_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PLL clock gating enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_gate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock gating enablement."]
        #[inline(always)]
        pub const fn set_pll_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("hsion", &self.hsion())
                .field("hsirdy", &self.hsirdy())
                .field("lpmon", &self.lpmon())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("usbhs_pllon", &self.usbhs_pllon())
                .field("usbhs_pllrdy", &self.usbhs_pllrdy())
                .field("usbss_pllon", &self.usbss_pllon())
                .field("usbss_pllrdy", &self.usbss_pllrdy())
                .field("pipeon", &self.pipeon())
                .field("pi_xi_stat", &self.pi_xi_stat())
                .field("pll_gate", &self.pll_gate())
                .finish()
        }
    }
    #[doc = "HB1 peripheral clock enable register (RCC_HB1PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb1pcenr(pub u32);
    impl Hb1pcenr {
        #[doc = "Timer 2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 clock enable."]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enable."]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 clock enable."]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Window watchdog clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable."]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable."]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 clock enable."]
        #[inline(always)]
        pub const fn set_usart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Power interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable."]
        #[inline(always)]
        pub const fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Hb1pcenr {
        #[inline(always)]
        fn default() -> Hb1pcenr {
            Hb1pcenr(0)
        }
    }
    impl core::fmt::Debug for Hb1pcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hb1pcenr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("usart4en", &self.usart4en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("pwren", &self.pwren())
                .finish()
        }
    }
    #[doc = "HB1 peripheral reset register (RCC_HB1PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb1prstr(pub u32);
    impl Hb1prstr {
        #[doc = "Timer 2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset."]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 reset."]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 reset."]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Window watchdog reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset."]
        #[inline(always)]
        pub const fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset."]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset."]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 reset."]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 reset."]
        #[inline(always)]
        pub const fn set_usart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset."]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Power interface reset."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset."]
        #[inline(always)]
        pub const fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Hb1prstr {
        #[inline(always)]
        fn default() -> Hb1prstr {
            Hb1prstr(0)
        }
    }
    impl core::fmt::Debug for Hb1prstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hb1prstr")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("usart4rst", &self.usart4rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("pwrrst", &self.pwrrst())
                .finish()
        }
    }
    #[doc = "HB2 peripheral clock enable register (RCC_HB2PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb2pcenr(pub u32);
    impl Hb2pcenr {
        #[doc = "Alternate function I/O clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn afioen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O clock enable."]
        #[inline(always)]
        pub const fn set_afioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port A clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iopaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A clock enable."]
        #[inline(always)]
        pub const fn set_iopaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port B clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iopben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B clock enable."]
        #[inline(always)]
        pub const fn set_iopben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port C clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iopcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C clock enable."]
        #[inline(always)]
        pub const fn set_iopcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port D clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iopden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable."]
        #[inline(always)]
        pub const fn set_iopden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC1 interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 interface clock enable."]
        #[inline(always)]
        pub const fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface clock enable."]
        #[inline(always)]
        pub const fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 Timer clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 Timer clock enable."]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable."]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable."]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 clock enable."]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 clock enable."]
        #[inline(always)]
        pub const fn set_adc4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Hb2pcenr {
        #[inline(always)]
        fn default() -> Hb2pcenr {
            Hb2pcenr(0)
        }
    }
    impl core::fmt::Debug for Hb2pcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hb2pcenr")
                .field("afioen", &self.afioen())
                .field("iopaen", &self.iopaen())
                .field("iopben", &self.iopben())
                .field("iopcen", &self.iopcen())
                .field("iopden", &self.iopden())
                .field("adc1en", &self.adc1en())
                .field("adc2en", &self.adc2en())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("usart1en", &self.usart1en())
                .field("adc3en", &self.adc3en())
                .field("adc4en", &self.adc4en())
                .finish()
        }
    }
    #[doc = "HB2 peripheral reset register (RCC_HB2PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb2prstr(pub u32);
    impl Hb2prstr {
        #[doc = "Alternate function I/O reset."]
        #[must_use]
        #[inline(always)]
        pub const fn afiorst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O reset."]
        #[inline(always)]
        pub const fn set_afiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port A reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ioparst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset."]
        #[inline(always)]
        pub const fn set_ioparst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port B reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iopbrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset."]
        #[inline(always)]
        pub const fn set_iopbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port C reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iopcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset."]
        #[inline(always)]
        pub const fn set_iopcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port D reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iopdrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset."]
        #[inline(always)]
        pub const fn set_iopdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC 1 interface reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 interface reset."]
        #[inline(always)]
        pub const fn set_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface reset."]
        #[inline(always)]
        pub const fn set_adc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset."]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset."]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC3 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 reset."]
        #[inline(always)]
        pub const fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC4 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 reset."]
        #[inline(always)]
        pub const fn set_adc4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Hb2prstr {
        #[inline(always)]
        fn default() -> Hb2prstr {
            Hb2prstr(0)
        }
    }
    impl core::fmt::Debug for Hb2prstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hb2prstr")
                .field("afiorst", &self.afiorst())
                .field("ioparst", &self.ioparst())
                .field("iopbrst", &self.iopbrst())
                .field("iopcrst", &self.iopcrst())
                .field("iopdrst", &self.iopdrst())
                .field("adc1rst", &self.adc1rst())
                .field("adc2rst", &self.adc2rst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("usart1rst", &self.usart1rst())
                .field("adc3rst", &self.adc3rst())
                .field("adc4rst", &self.adc4rst())
                .finish()
        }
    }
    #[doc = "HB Peripheral Clock enable register (RCC_HBPCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hbpcenr(pub u32);
    impl Hbpcenr {
        #[doc = "DMA clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA clock enable."]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USBHS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhsen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS clock enable."]
        #[inline(always)]
        pub const fn set_usbhsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USBPD clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpden(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD clock enable."]
        #[inline(always)]
        pub const fn set_usbpden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ARGB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn argben(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ARGB clock enable."]
        #[inline(always)]
        pub const fn set_argben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USBSS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbssen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS clock enable."]
        #[inline(always)]
        pub const fn set_usbssen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Hbpcenr {
        #[inline(always)]
        fn default() -> Hbpcenr {
            Hbpcenr(0)
        }
    }
    impl core::fmt::Debug for Hbpcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hbpcenr")
                .field("dma1en", &self.dma1en())
                .field("crcen", &self.crcen())
                .field("usbhsen", &self.usbhsen())
                .field("usbpden", &self.usbpden())
                .field("argben", &self.argben())
                .field("usbssen", &self.usbssen())
                .finish()
        }
    }
    #[doc = "HB reset register (RCC_PHBRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hbrstr(pub u32);
    impl Hbrstr {
        #[doc = "USBHS reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbhsrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS reset."]
        #[inline(always)]
        pub const fn set_usbhsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USBPD reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpdrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD reset."]
        #[inline(always)]
        pub const fn set_usbpdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ARGB reset."]
        #[must_use]
        #[inline(always)]
        pub const fn argbrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ARGB reset."]
        #[inline(always)]
        pub const fn set_argbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USBSS reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbssrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS reset."]
        #[inline(always)]
        pub const fn set_usbssrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Hbrstr {
        #[inline(always)]
        fn default() -> Hbrstr {
            Hbrstr(0)
        }
    }
    impl core::fmt::Debug for Hbrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hbrstr")
                .field("usbhsrst", &self.usbhsrst())
                .field("usbpdrst", &self.usbpdrst())
                .field("argbrst", &self.argbrst())
                .field("usbssrst", &self.usbssrst())
                .finish()
        }
    }
    #[doc = "Clock interrupt register (RCC_INTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intr(pub u32);
    impl Intr {
        #[doc = "LSI Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE Ready Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt flag."]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Ready Interrupt Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub const fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Intr {
        #[inline(always)]
        fn default() -> Intr {
            Intr(0)
        }
    }
    impl core::fmt::Debug for Intr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intr")
                .field("lsirdyf", &self.lsirdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("cssf", &self.cssf())
                .field("lsirdyie", &self.lsirdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("lsirdyc", &self.lsirdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("cssc", &self.cssc())
                .finish()
        }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstsckr(pub u32);
    impl Rstsckr {
        #[doc = "Internal low speed oscillator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator enable."]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low speed oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator ready."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LOCKUP reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lkuprstf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP reset flag."]
        #[inline(always)]
        pub const fn set_lkuprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC3 watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3awdrstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 watchdog reset flag."]
        #[inline(always)]
        pub const fn set_adc3awdrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADC4 watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4awdrstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 watchdog reset flag."]
        #[inline(always)]
        pub const fn set_adc4awdrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "USBPD reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpdrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD reset flag."]
        #[inline(always)]
        pub const fn set_usbpdrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC1 watchdog flag."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1awdrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 watchdog flag."]
        #[inline(always)]
        pub const fn set_adc1awdrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC2 watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2awdrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 watchdog reset flag."]
        #[inline(always)]
        pub const fn set_adc2awdrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Remove reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PIN reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag."]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag."]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Rstsckr {
        #[inline(always)]
        fn default() -> Rstsckr {
            Rstsckr(0)
        }
    }
    impl core::fmt::Debug for Rstsckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rstsckr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("lkuprstf", &self.lkuprstf())
                .field("adc3awdrstf", &self.adc3awdrstf())
                .field("adc4awdrstf", &self.adc4awdrstf())
                .field("usbpdrstf", &self.usbpdrstf())
                .field("adc1awdrstf", &self.adc1awdrstf())
                .field("adc2awdrstf", &self.adc2awdrstf())
                .field("rmvf", &self.rmvf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .finish()
        }
    }
}
