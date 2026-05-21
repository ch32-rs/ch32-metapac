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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Clock configuration register (RCC_CFGR0)."]
    #[inline(always)]
    pub const fn cfgr0(self) -> crate::common::Reg<regs::Cfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PLL clock configuration register (RCC_PLLCFGR)."]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Clock interrupt register (RCC_INTR)."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "HB2 peripheral reset register (RCC_HB2PRSTR)."]
    #[inline(always)]
    pub const fn hb2prstr(self) -> crate::common::Reg<regs::Hb2prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "HB1 peripheral reset register (RCC_HB1PRSTR)."]
    #[inline(always)]
    pub const fn hb1prstr(self) -> crate::common::Reg<regs::Hb1prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "HB Peripheral Clock enable register (RCC_HBPCENR)."]
    #[inline(always)]
    pub const fn hbpcenr(self) -> crate::common::Reg<regs::Hbpcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "HB2 peripheral clock enable register (RCC_HB2PCENR)."]
    #[inline(always)]
    pub const fn hb2pcenr(self) -> crate::common::Reg<regs::Hb2pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "HB1 peripheral clock enable register (RCC_HB1PCENR)."]
    #[inline(always)]
    pub const fn hb1pcenr(self) -> crate::common::Reg<regs::Hb1pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Backup domain control register (RCC_BDCTLR)."]
    #[inline(always)]
    pub const fn bdctlr(self) -> crate::common::Reg<regs::Bdctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[inline(always)]
    pub const fn rstsckr(self) -> crate::common::Reg<regs::Rstsckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "HB reset register (RCC_PHBRSTR)."]
    #[inline(always)]
    pub const fn hbrstr(self) -> crate::common::Reg<regs::Hbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Clock configuration register2 (RCC_CFGR2)."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PLL Clock configuration register2 (RCC_PLLCFGR2)."]
    #[inline(always)]
    pub const fn pllcfgr2(self) -> crate::common::Reg<regs::Pllcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "Backup domain control register (RCC_BDCTLR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdctlr(pub u32);
    impl Bdctlr {
        #[doc = "External Low Speed oscillator enable."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator enable."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Low Speed oscillator ready."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator ready."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Low Speed oscillator bypass."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator bypass."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "calibrate the clock output selection."]
        #[inline(always)]
        pub const fn cco(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "calibrate the clock output selection."]
        #[inline(always)]
        pub fn set_cco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TAMPER pin enables pulse output."]
        #[inline(always)]
        pub const fn asoe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPER pin enables pulse output."]
        #[inline(always)]
        pub fn set_asoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TAMPER pin alarm/second pulse output."]
        #[inline(always)]
        pub const fn asos(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPER pin alarm/second pulse output."]
        #[inline(always)]
        pub fn set_asos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTC clock source selection."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "RTC clock enable."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RTC calibration value."]
        #[inline(always)]
        pub const fn rtccal(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "RTC calibration value."]
        #[inline(always)]
        pub fn set_rtccal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
        #[doc = "Backup domain software reset."]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset."]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Bdctlr {
        #[inline(always)]
        fn default() -> Bdctlr {
            Bdctlr(0)
        }
    }
    #[doc = "Clock configuration register (RCC_CFGR0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr0(pub u32);
    impl Cfgr0 {
        #[doc = "System clock switch."]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status."]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status."]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "HB (AHB) prescaler — divides SYS_CLK to produce HCLK."]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "HB (AHB) prescaler — divides SYS_CLK to produce HCLK."]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "PB1 (low-speed APB) prescaler."]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "PB1 (low-speed APB) prescaler."]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "PB2 (high-speed APB) prescaler, also feeds ADC prescaler."]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "PB2 (high-speed APB) prescaler, also feeds ADC prescaler."]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "ADC prescaler (divides PB2 clock)."]
        #[inline(always)]
        pub const fn adcpre(&self) -> super::vals::Adcpre {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Adcpre::from_bits(val as u8)
        }
        #[doc = "ADC prescaler (divides PB2 clock)."]
        #[inline(always)]
        pub fn set_adcpre(&mut self, val: super::vals::Adcpre) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "V5F core prescaler — divides SYS_CLK to produce the V5F core clock."]
        #[inline(always)]
        pub const fn fpre(&self) -> super::vals::Fpre {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Fpre::from_bits(val as u8)
        }
        #[doc = "V5F core prescaler — divides SYS_CLK to produce the V5F core clock."]
        #[inline(always)]
        pub fn set_fpre(&mut self, val: super::vals::Fpre) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "1000 Mb/s Ethernet RGMII interface and clock enable."]
        #[inline(always)]
        pub const fn rgmiion(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1000 Mb/s Ethernet RGMII interface and clock enable."]
        #[inline(always)]
        pub fn set_rgmiion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PIPE clock gating enable."]
        #[inline(always)]
        pub const fn pipeon(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PIPE clock gating enable."]
        #[inline(always)]
        pub fn set_pipeon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "UTMI clock gating enable."]
        #[inline(always)]
        pub const fn utmion(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UTMI clock gating enable."]
        #[inline(always)]
        pub fn set_utmion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Microcontroller clock output selection."]
        #[inline(always)]
        pub const fn mco(&self) -> super::vals::Mco {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mco::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output selection."]
        #[inline(always)]
        pub fn set_mco(&mut self, val: super::vals::Mco) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "ADC clock duty cycle. 0=50%, 1=75%. Only effective when ADCSRC=HCLK."]
        #[inline(always)]
        pub const fn adc_duty_sel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock duty cycle. 0=50%, 1=75%. Only effective when ADCSRC=HCLK."]
        #[inline(always)]
        pub fn set_adc_duty_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADC input clock source selection."]
        #[inline(always)]
        pub const fn adcsrc(&self) -> super::vals::Adcsrc {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Adcsrc::from_bits(val as u8)
        }
        #[doc = "ADC input clock source selection."]
        #[inline(always)]
        pub fn set_adcsrc(&mut self, val: super::vals::Adcsrc) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr0 {
        #[inline(always)]
        fn default() -> Cfgr0 {
            Cfgr0(0)
        }
    }
    #[doc = "Clock configuration register2 (RCC_CFGR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "UHSIF prescaler (1..64)."]
        #[inline(always)]
        pub const fn uhsifdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "UHSIF prescaler (1..64)."]
        #[inline(always)]
        pub fn set_uhsifdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "UHSIF clock source."]
        #[inline(always)]
        pub const fn uhsifsrc(&self) -> super::vals::Uhsifsrc {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Uhsifsrc::from_bits(val as u8)
        }
        #[doc = "UHSIF clock source."]
        #[inline(always)]
        pub fn set_uhsifsrc(&mut self, val: super::vals::Uhsifsrc) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "LTDC prescaler (1..64)."]
        #[inline(always)]
        pub const fn ltdcdiv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "LTDC prescaler (1..64)."]
        #[inline(always)]
        pub fn set_ltdcdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "LTDC clock source."]
        #[inline(always)]
        pub const fn ltdcsrc(&self) -> super::vals::Ltdcsrc {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Ltdcsrc::from_bits(val as u8)
        }
        #[doc = "LTDC clock source."]
        #[inline(always)]
        pub fn set_ltdcsrc(&mut self, val: super::vals::Ltdcsrc) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "USBFS 48 MHz prescaler."]
        #[inline(always)]
        pub const fn usbfsdiv(&self) -> super::vals::Usbfsdiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Usbfsdiv::from_bits(val as u8)
        }
        #[doc = "USBFS 48 MHz prescaler."]
        #[inline(always)]
        pub fn set_usbfsdiv(&mut self, val: super::vals::Usbfsdiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "USBFS 48 MHz clock source."]
        #[inline(always)]
        pub const fn usbfssrc(&self) -> super::vals::Usbfssrc {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Usbfssrc::from_bits(val as u8)
        }
        #[doc = "USBFS 48 MHz clock source."]
        #[inline(always)]
        pub fn set_usbfssrc(&mut self, val: super::vals::Usbfssrc) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "RNG clock source."]
        #[inline(always)]
        pub const fn rngsrc(&self) -> super::vals::ClkSrcPll {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::ClkSrcPll::from_bits(val as u8)
        }
        #[doc = "RNG clock source."]
        #[inline(always)]
        pub fn set_rngsrc(&mut self, val: super::vals::ClkSrcPll) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "I2S2 clock source."]
        #[inline(always)]
        pub const fn i2s2src(&self) -> super::vals::ClkSrcPll {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::ClkSrcPll::from_bits(val as u8)
        }
        #[doc = "I2S2 clock source."]
        #[inline(always)]
        pub fn set_i2s2src(&mut self, val: super::vals::ClkSrcPll) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "I2S3 clock source."]
        #[inline(always)]
        pub const fn i2s3src(&self) -> super::vals::ClkSrcPll {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::ClkSrcPll::from_bits(val as u8)
        }
        #[doc = "I2S3 clock source."]
        #[inline(always)]
        pub fn set_i2s3src(&mut self, val: super::vals::ClkSrcPll) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "HSADC clock source."]
        #[inline(always)]
        pub const fn hsadcsrc(&self) -> super::vals::Hsadcsrc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Hsadcsrc::from_bits(val as u8)
        }
        #[doc = "HSADC clock source."]
        #[inline(always)]
        pub fn set_hsadcsrc(&mut self, val: super::vals::Hsadcsrc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Gigabit Ethernet 125 MHz clock source."]
        #[inline(always)]
        pub const fn eth1gsrc(&self) -> super::vals::Eth1gsrc {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Eth1gsrc::from_bits(val as u8)
        }
        #[doc = "Gigabit Ethernet 125 MHz clock source."]
        #[inline(always)]
        pub fn set_eth1gsrc(&mut self, val: super::vals::Eth1gsrc) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "Clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Internal High Speed clock enable."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock ready flag."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal High Speed clock trimming."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal High Speed clock trimming."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal High Speed clock calibration (hardware-set, read-only)."]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal High Speed clock calibration (hardware-set, read-only)."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "External High Speed clock enable."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed clock ready flag."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock ready flag."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External High Speed clock Bypass."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock Bypass."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock Security System enable."]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System enable."]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USBHS PLL clock enable."]
        #[inline(always)]
        pub const fn usbhs_pllon(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL clock enable."]
        #[inline(always)]
        pub fn set_usbhs_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "USBHS PLL clock ready flag."]
        #[inline(always)]
        pub const fn usbhs_pllrdy(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL clock ready flag."]
        #[inline(always)]
        pub fn set_usbhs_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USBSS PLL clock enable."]
        #[inline(always)]
        pub const fn usbss_pllon(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS PLL clock enable."]
        #[inline(always)]
        pub fn set_usbss_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USBSS PLL clock ready flag."]
        #[inline(always)]
        pub const fn usbss_pllrdy(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS PLL clock ready flag."]
        #[inline(always)]
        pub fn set_usbss_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PLL clock enable."]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock enable."]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL clock ready flag."]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock ready flag."]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ETH PLL clock enable."]
        #[inline(always)]
        pub const fn eth_pllon(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ETH PLL clock enable."]
        #[inline(always)]
        pub fn set_eth_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ETH PLL clock ready flag."]
        #[inline(always)]
        pub const fn eth_pllrdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ETH PLL clock ready flag."]
        #[inline(always)]
        pub fn set_eth_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SERDES PLL clock enable."]
        #[inline(always)]
        pub const fn serdes_pllon(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES PLL clock enable."]
        #[inline(always)]
        pub fn set_serdes_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SERDES PLL clock ready flag."]
        #[inline(always)]
        pub const fn serdes_pllrdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES PLL clock ready flag."]
        #[inline(always)]
        pub fn set_serdes_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Upon the occurrence of an HSE failure event with CSSON enabled."]
        #[inline(always)]
        pub const fn css_hse_dis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Upon the occurrence of an HSE failure event with CSSON enabled."]
        #[inline(always)]
        pub fn set_css_hse_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "HB1 peripheral clock enable register (RCC_HB1PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb1pcenr(pub u32);
    impl Hb1pcenr {
        #[doc = "Timer 2 clock enable."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 clock enable."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enable."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enable."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 clock enable."]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 clock enable."]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 5 clock enable."]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 5 clock enable."]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 6 clock enable."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 clock enable."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 clock enable."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART 6 clock enable."]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART 6 clock enable."]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART 7 clock enable."]
        #[inline(always)]
        pub const fn usart7en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART 7 clock enable."]
        #[inline(always)]
        pub fn set_usart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USART 8 clock enable."]
        #[inline(always)]
        pub const fn usart8en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USART 8 clock enable."]
        #[inline(always)]
        pub fn set_usart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 clock enable."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM2 clock enable."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "QSPI1 clock enable."]
        #[inline(always)]
        pub const fn qspi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "QSPI1 clock enable."]
        #[inline(always)]
        pub fn set_qspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QSPI2 clock enable."]
        #[inline(always)]
        pub const fn qspi2en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "QSPI2 clock enable."]
        #[inline(always)]
        pub fn set_qspi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPI4 clock enable."]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 clock enable."]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 clock enable."]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable."]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART4 clock enable."]
        #[inline(always)]
        pub const fn usart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 clock enable."]
        #[inline(always)]
        pub fn set_usart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART5 clock enable."]
        #[inline(always)]
        pub const fn usart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART5 clock enable."]
        #[inline(always)]
        pub fn set_usart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN3 clock enable."]
        #[inline(always)]
        pub const fn can3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CAN3 clock enable."]
        #[inline(always)]
        pub fn set_can3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 clock enable."]
        #[inline(always)]
        pub const fn can1en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 clock enable."]
        #[inline(always)]
        pub fn set_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 clock enable."]
        #[inline(always)]
        pub const fn can2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 clock enable."]
        #[inline(always)]
        pub fn set_can2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup interface clock enable."]
        #[inline(always)]
        pub const fn bkpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface clock enable."]
        #[inline(always)]
        pub fn set_bkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface clock enable."]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable."]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable."]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable."]
        #[inline(always)]
        pub fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 clock enable."]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable."]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SWPMI clock enable."]
        #[inline(always)]
        pub const fn swpmien(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI clock enable."]
        #[inline(always)]
        pub fn set_swpmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hb1pcenr {
        #[inline(always)]
        fn default() -> Hb1pcenr {
            Hb1pcenr(0)
        }
    }
    #[doc = "HB1 peripheral reset register (RCC_HB1PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb1prstr(pub u32);
    impl Hb1prstr {
        #[doc = "Timer 2 reset."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 reset."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 reset."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 reset."]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 reset."]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 5 reset."]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 5 reset."]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 6 reset."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 reset."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 reset."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 reset."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART 6 reset."]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART 6 reset."]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART 7 reset."]
        #[inline(always)]
        pub const fn usart7rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART 7 reset."]
        #[inline(always)]
        pub fn set_usart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USART 8 reset."]
        #[inline(always)]
        pub const fn usart8rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USART 8 reset."]
        #[inline(always)]
        pub fn set_usart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 reset."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM2 reset."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 reset."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog reset."]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset."]
        #[inline(always)]
        pub fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "QSPI1 reset."]
        #[inline(always)]
        pub const fn qspi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "QSPI1 reset."]
        #[inline(always)]
        pub fn set_qspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QSPI2 reset."]
        #[inline(always)]
        pub const fn qspi2rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "QSPI2 reset."]
        #[inline(always)]
        pub fn set_qspi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPI4 reset."]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 reset."]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART 2 reset."]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset."]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 reset."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 reset."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 reset."]
        #[inline(always)]
        pub const fn usart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 reset."]
        #[inline(always)]
        pub fn set_usart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART 5 reset."]
        #[inline(always)]
        pub const fn usart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART 5 reset."]
        #[inline(always)]
        pub fn set_usart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN3 reset."]
        #[inline(always)]
        pub const fn can3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CAN3 reset."]
        #[inline(always)]
        pub fn set_can3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 reset."]
        #[inline(always)]
        pub const fn can1rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 reset."]
        #[inline(always)]
        pub fn set_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 reset."]
        #[inline(always)]
        pub const fn can2rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 reset."]
        #[inline(always)]
        pub fn set_can2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup interface reset."]
        #[inline(always)]
        pub const fn bkprst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface reset."]
        #[inline(always)]
        pub fn set_bkprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface reset."]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset."]
        #[inline(always)]
        pub fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface reset."]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface reset."]
        #[inline(always)]
        pub fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 reset."]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset."]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SWPMI reset."]
        #[inline(always)]
        pub const fn swpmirst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI reset."]
        #[inline(always)]
        pub fn set_swpmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hb1prstr {
        #[inline(always)]
        fn default() -> Hb1prstr {
            Hb1prstr(0)
        }
    }
    #[doc = "HB2 peripheral clock enable register (RCC_HB2PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb2pcenr(pub u32);
    impl Hb2pcenr {
        #[doc = "Alternate function I/O clock enable."]
        #[inline(always)]
        pub const fn afioen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O clock enable."]
        #[inline(always)]
        pub fn set_afioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSADC clock enable."]
        #[inline(always)]
        pub const fn hsadcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSADC clock enable."]
        #[inline(always)]
        pub fn set_hsadcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I/O port A clock enable."]
        #[inline(always)]
        pub const fn iopaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A clock enable."]
        #[inline(always)]
        pub fn set_iopaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port B clock enable."]
        #[inline(always)]
        pub const fn iopben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B clock enable."]
        #[inline(always)]
        pub fn set_iopben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port C clock enable."]
        #[inline(always)]
        pub const fn iopcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C clock enable."]
        #[inline(always)]
        pub fn set_iopcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port D clock enable."]
        #[inline(always)]
        pub const fn iopden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable."]
        #[inline(always)]
        pub fn set_iopden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I/O port E clock enable."]
        #[inline(always)]
        pub const fn iopeen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port E clock enable."]
        #[inline(always)]
        pub fn set_iopeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port F clock enable."]
        #[inline(always)]
        pub const fn iopfen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port F clock enable."]
        #[inline(always)]
        pub fn set_iopfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 interface clock enable."]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 interface clock enable."]
        #[inline(always)]
        pub fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface clock enable."]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface clock enable."]
        #[inline(always)]
        pub fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 Timer clock enable."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 Timer clock enable."]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C4 clock enable."]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable."]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SAI clock enable."]
        #[inline(always)]
        pub const fn saien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SAI clock enable."]
        #[inline(always)]
        pub fn set_saien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SDIO clock enable."]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO clock enable."]
        #[inline(always)]
        pub fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 Timer clock enable."]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM10 Timer clock enable."]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIM11 Timer clock enable."]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TIM12 Timer clock enable."]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 Timer clock enable."]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OPA and CMP clock enable."]
        #[inline(always)]
        pub const fn opcmen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OPA and CMP clock enable."]
        #[inline(always)]
        pub fn set_opcmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "DFSDM clock enable."]
        #[inline(always)]
        pub const fn dfsdmen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM clock enable."]
        #[inline(always)]
        pub fn set_dfsdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECDC clock enable."]
        #[inline(always)]
        pub const fn ecdcen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ECDC clock enable."]
        #[inline(always)]
        pub fn set_ecdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "GPHA clock enable."]
        #[inline(always)]
        pub const fn gphaen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "GPHA clock enable."]
        #[inline(always)]
        pub fn set_gphaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LTDC clock enable."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "I3C clock enable."]
        #[inline(always)]
        pub const fn i3cen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "I3C clock enable."]
        #[inline(always)]
        pub fn set_i3cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hb2pcenr {
        #[inline(always)]
        fn default() -> Hb2pcenr {
            Hb2pcenr(0)
        }
    }
    #[doc = "HB2 peripheral reset register (RCC_HB2PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hb2prstr(pub u32);
    impl Hb2prstr {
        #[doc = "Alternate function I/O reset."]
        #[inline(always)]
        pub const fn afiorst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O reset."]
        #[inline(always)]
        pub fn set_afiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSADC reset."]
        #[inline(always)]
        pub const fn hsadcrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSADC reset."]
        #[inline(always)]
        pub fn set_hsadcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port A reset."]
        #[inline(always)]
        pub const fn ioparst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset."]
        #[inline(always)]
        pub fn set_ioparst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port B reset."]
        #[inline(always)]
        pub const fn iopbrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset."]
        #[inline(always)]
        pub fn set_iopbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port C reset."]
        #[inline(always)]
        pub const fn iopcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset."]
        #[inline(always)]
        pub fn set_iopcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port D reset."]
        #[inline(always)]
        pub const fn iopdrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset."]
        #[inline(always)]
        pub fn set_iopdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port E reset."]
        #[inline(always)]
        pub const fn ioperst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset."]
        #[inline(always)]
        pub fn set_ioperst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port F reset."]
        #[inline(always)]
        pub const fn iopfrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset."]
        #[inline(always)]
        pub fn set_iopfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC 1 interface reset."]
        #[inline(always)]
        pub const fn adc1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 interface reset."]
        #[inline(always)]
        pub fn set_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface reset."]
        #[inline(always)]
        pub const fn adc2rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface reset."]
        #[inline(always)]
        pub fn set_adc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer reset."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 reset."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer reset."]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer reset."]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C4 reset."]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset."]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SAI reset."]
        #[inline(always)]
        pub const fn sairst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SAI reset."]
        #[inline(always)]
        pub fn set_sairst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SDIO reset."]
        #[inline(always)]
        pub const fn sdiorst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO reset."]
        #[inline(always)]
        pub fn set_sdiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 timer reset."]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 timer reset."]
        #[inline(always)]
        pub fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM10 timer reset."]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 timer reset."]
        #[inline(always)]
        pub fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIM11 timer reset."]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 timer reset."]
        #[inline(always)]
        pub fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TIM12 timer reset."]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 timer reset."]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OPA and CMP reset."]
        #[inline(always)]
        pub const fn opcmrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OPA and CMP reset."]
        #[inline(always)]
        pub fn set_opcmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "DFSDM reset."]
        #[inline(always)]
        pub const fn dfsdmrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM reset."]
        #[inline(always)]
        pub fn set_dfsdmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECDC reset."]
        #[inline(always)]
        pub const fn ecdcrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ECDC reset."]
        #[inline(always)]
        pub fn set_ecdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "GPHA reset."]
        #[inline(always)]
        pub const fn gpharst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "GPHA reset."]
        #[inline(always)]
        pub fn set_gpharst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LTDC reset."]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC reset."]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "I3C reset."]
        #[inline(always)]
        pub const fn i3crst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "I3C reset."]
        #[inline(always)]
        pub fn set_i3crst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hb2prstr {
        #[inline(always)]
        fn default() -> Hb2prstr {
            Hb2prstr(0)
        }
    }
    #[doc = "HB Peripheral Clock enable register (RCC_HBPCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hbpcenr(pub u32);
    impl Hbpcenr {
        #[doc = "DMA clock enable."]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA clock enable."]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clock enable."]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable."]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FMC clock enable."]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FMC clock enable."]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RNG clock enable."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SDMMC clock enable."]
        #[inline(always)]
        pub const fn sdmmcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC clock enable."]
        #[inline(always)]
        pub fn set_sdmmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "USBHS clock enable."]
        #[inline(always)]
        pub const fn usbhsen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS clock enable."]
        #[inline(always)]
        pub fn set_usbhsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USBSS clock enable."]
        #[inline(always)]
        pub const fn usbssen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS clock enable."]
        #[inline(always)]
        pub fn set_usbssen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DVP clock enable."]
        #[inline(always)]
        pub const fn dvpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DVP clock enable."]
        #[inline(always)]
        pub fn set_dvpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Ethernet MAC clock enable."]
        #[inline(always)]
        pub const fn ethmacen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable."]
        #[inline(always)]
        pub fn set_ethmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USBFS_OTG_FS clock enable."]
        #[inline(always)]
        pub const fn otgfsen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS_OTG_FS clock enable."]
        #[inline(always)]
        pub fn set_otgfsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "UHSIF clock enable."]
        #[inline(always)]
        pub const fn uhsifen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "UHSIF clock enable."]
        #[inline(always)]
        pub fn set_uhsifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USBPD clock enable."]
        #[inline(always)]
        pub const fn usbpden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD clock enable."]
        #[inline(always)]
        pub fn set_usbpden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SERDES clock enable."]
        #[inline(always)]
        pub const fn serdesen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES clock enable."]
        #[inline(always)]
        pub fn set_serdesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PIOC clock enable."]
        #[inline(always)]
        pub const fn piocen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PIOC clock enable."]
        #[inline(always)]
        pub fn set_piocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Hbpcenr {
        #[inline(always)]
        fn default() -> Hbpcenr {
            Hbpcenr(0)
        }
    }
    #[doc = "HB reset register (RCC_PHBRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hbrstr(pub u32);
    impl Hbrstr {
        #[doc = "DMA1 reset."]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 reset."]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 reset."]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset."]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMC reset."]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FMC reset."]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RNG reset."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RNG reset."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SDMMC reset."]
        #[inline(always)]
        pub const fn sdmmcrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC reset."]
        #[inline(always)]
        pub fn set_sdmmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "USBHS reset."]
        #[inline(always)]
        pub const fn usbhsrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS reset."]
        #[inline(always)]
        pub fn set_usbhsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USBSS reset."]
        #[inline(always)]
        pub const fn usbssrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBSS reset."]
        #[inline(always)]
        pub fn set_usbssrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DVP reset."]
        #[inline(always)]
        pub const fn dvprst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DVP reset."]
        #[inline(always)]
        pub fn set_dvprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Ethernet MAC reset."]
        #[inline(always)]
        pub const fn ethmacrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC reset."]
        #[inline(always)]
        pub fn set_ethmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USBFS_OTG_FS eset."]
        #[inline(always)]
        pub const fn otgfsrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS_OTG_FS eset."]
        #[inline(always)]
        pub fn set_otgfsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "UHSIF reset."]
        #[inline(always)]
        pub const fn uhsifrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "UHSIF reset."]
        #[inline(always)]
        pub fn set_uhsifrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USBPD reset."]
        #[inline(always)]
        pub const fn usbpdrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD reset."]
        #[inline(always)]
        pub fn set_usbpdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SERDES reset."]
        #[inline(always)]
        pub const fn serdesrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES reset."]
        #[inline(always)]
        pub fn set_serdesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PIOC reset."]
        #[inline(always)]
        pub const fn piocrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PIOC reset."]
        #[inline(always)]
        pub fn set_piocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Hbrstr {
        #[inline(always)]
        fn default() -> Hbrstr {
            Hbrstr(0)
        }
    }
    #[doc = "Clock interrupt register (RCC_INTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intr(pub u32);
    impl Intr {
        #[doc = "LSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE Ready Interrupt flag."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE Ready Interrupt flag."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL Ready Interrupt flag."]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ETH Ready Interrupt flag."]
        #[inline(always)]
        pub const fn ethpllrdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ETH Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_ethpllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SERDES_PLL Ready Interrupt flag."]
        #[inline(always)]
        pub const fn serdespllrdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES_PLL Ready Interrupt flag."]
        #[inline(always)]
        pub fn set_serdespllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ETHPLL Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn ethpllrdyie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ETHPLL Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_ethpllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SERDESPLL Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn serdespllrdyie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SERDESPLL Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_serdespllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ETH PLL Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn ethpllrdyc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ETH PLL Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_ethpllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SERDES Ready Interrupt Clear."]
        #[inline(always)]
        pub const fn serdespllrdyc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SERDES Ready Interrupt Clear."]
        #[inline(always)]
        pub fn set_serdespllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear."]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Intr {
        #[inline(always)]
        fn default() -> Intr {
            Intr(0)
        }
    }
    #[doc = "PLL clock configuration register (RCC_PLLCFGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "Main PLL multiplication factor. Writable only while PLLON=0."]
        #[inline(always)]
        pub const fn pllmul(&self) -> super::vals::Pllmul {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Pllmul::from_bits(val as u8)
        }
        #[doc = "Main PLL multiplication factor. Writable only while PLLON=0."]
        #[inline(always)]
        pub fn set_pllmul(&mut self, val: super::vals::Pllmul) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "PLL input clock source. Writable only while PLLON=0."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL input clock source. Writable only while PLLON=0."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "PLL input prescaler (1..64). Writable only while PLLON=0."]
        #[inline(always)]
        pub const fn pll_src_div(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "PLL input prescaler (1..64). Writable only while PLLON=0."]
        #[inline(always)]
        pub fn set_pll_src_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "System-clock-to-PLL selection. Writable only while SYSPLL_GATE=0."]
        #[inline(always)]
        pub const fn syspll_sel(&self) -> super::vals::SyspllSel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::SyspllSel::from_bits(val as u8)
        }
        #[doc = "System-clock-to-PLL selection. Writable only while SYSPLL_GATE=0."]
        #[inline(always)]
        pub fn set_syspll_sel(&mut self, val: super::vals::SyspllSel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
        #[doc = "System-clock-to-PLL gate. Must be set before switching SYSCLK to PLL output."]
        #[inline(always)]
        pub const fn syspll_gate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "System-clock-to-PLL gate. Must be set before switching SYSCLK to PLL output."]
        #[inline(always)]
        pub fn set_syspll_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    #[doc = "PLL Clock configuration register2 (RCC_PLLCFGR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr2(pub u32);
    impl Pllcfgr2 {
        #[doc = "USBHS PLL input clock source."]
        #[inline(always)]
        pub const fn usbhspllsrc(&self) -> super::vals::Usbhspllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usbhspllsrc::from_bits(val as u8)
        }
        #[doc = "USBHS PLL input clock source."]
        #[inline(always)]
        pub fn set_usbhspllsrc(&mut self, val: super::vals::Usbhspllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USBHS_PLL reference clock frequency. Writable only while USBHS_PLLON=0."]
        #[inline(always)]
        pub const fn usbhspll_refsel(&self) -> super::vals::UsbhspllRefsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::UsbhspllRefsel::from_bits(val as u8)
        }
        #[doc = "USBHS_PLL reference clock frequency. Writable only while USBHS_PLLON=0."]
        #[inline(always)]
        pub fn set_usbhspll_refsel(&mut self, val: super::vals::UsbhspllRefsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USBSS_PLL reference clock frequency. Writable only while USBSS_PLLON=0."]
        #[inline(always)]
        pub const fn usbsspll_refsel(&self) -> super::vals::UsbsspllRefsel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::UsbsspllRefsel::from_bits(val as u8)
        }
        #[doc = "USBSS_PLL reference clock frequency. Writable only while USBSS_PLLON=0."]
        #[inline(always)]
        pub fn set_usbsspll_refsel(&mut self, val: super::vals::UsbsspllRefsel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "USBHS_PLL input prescaler from SYS_PLL (1..32). Writable only while USBHS_PLLON=0."]
        #[inline(always)]
        pub const fn usbhspll_in_div(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "USBHS_PLL input prescaler from SYS_PLL (1..32). Writable only while USBHS_PLLON=0."]
        #[inline(always)]
        pub fn set_usbhspll_in_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "SERDES_PLL multiplication factor. Writable only while SERDES_PLLON=0."]
        #[inline(always)]
        pub const fn serdespll_mul(&self) -> super::vals::SerdespllMul {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::SerdespllMul::from_bits(val as u8)
        }
        #[doc = "SERDES_PLL multiplication factor. Writable only while SERDES_PLLON=0."]
        #[inline(always)]
        pub fn set_serdespll_mul(&mut self, val: super::vals::SerdespllMul) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Pllcfgr2 {
        #[inline(always)]
        fn default() -> Pllcfgr2 {
            Pllcfgr2(0)
        }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstsckr(pub u32);
    impl Rstsckr {
        #[doc = "Internal low speed oscillator enable."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator enable."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low speed oscillator ready."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator ready."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PIN reset flag."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag."]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag."]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "LOCKUP reset flag."]
        #[inline(always)]
        pub const fn lkuprstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP reset flag."]
        #[inline(always)]
        pub fn set_lkuprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rstsckr {
        #[inline(always)]
        fn default() -> Rstsckr {
            Rstsckr(0)
        }
    }
}
pub mod vals {
    #[doc = "ADC prescaler (divides PB2 clock)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcpre {
        #[doc = "PB2 divided by 2."]
        DIV2 = 0x0,
        #[doc = "PB2 divided by 4."]
        DIV4 = 0x01,
        #[doc = "PB2 divided by 6."]
        DIV6 = 0x02,
        #[doc = "PB2 divided by 8."]
        DIV8 = 0x03,
    }
    impl Adcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcpre {
        #[inline(always)]
        fn from(val: u8) -> Adcpre {
            Adcpre::from_bits(val)
        }
    }
    impl From<Adcpre> for u8 {
        #[inline(always)]
        fn from(val: Adcpre) -> u8 {
            Adcpre::to_bits(val)
        }
    }
    #[doc = "ADC input clock source selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcsrc {
        #[doc = "HCLK feeds the ADC prescaler."]
        HCLK = 0x0,
        #[doc = "USBHS_PLL (480 MHz) feeds the ADC prescaler."]
        USBHS_PLL = 0x01,
    }
    impl Adcsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcsrc {
        #[inline(always)]
        fn from(val: u8) -> Adcsrc {
            Adcsrc::from_bits(val)
        }
    }
    impl From<Adcsrc> for u8 {
        #[inline(always)]
        fn from(val: Adcsrc) -> u8 {
            Adcsrc::to_bits(val)
        }
    }
    #[doc = "Single-bit clock source selecting between SYSCLK and PLL_CLK."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ClkSrcPll {
        #[doc = "SYSCLK selected."]
        SYSCLK = 0x0,
        #[doc = "PLL_CLK selected."]
        PLL_CLK = 0x01,
    }
    impl ClkSrcPll {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClkSrcPll {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClkSrcPll {
        #[inline(always)]
        fn from(val: u8) -> ClkSrcPll {
            ClkSrcPll::from_bits(val)
        }
    }
    impl From<ClkSrcPll> for u8 {
        #[inline(always)]
        fn from(val: ClkSrcPll) -> u8 {
            ClkSrcPll::to_bits(val)
        }
    }
    #[doc = "Gigabit Ethernet 125 MHz clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eth1gsrc {
        #[doc = "PLL_CLK selected."]
        PLL_CLK = 0x0,
        #[doc = "USBSS_PLL selected."]
        USBSS_PLL = 0x01,
        #[doc = "ETH_PLL divided by 4 selected."]
        ETH_PLL_DIV4 = 0x02,
        #[doc = "SERDES_PLL divided by 8 selected."]
        SERDES_PLL_DIV8 = 0x03,
    }
    impl Eth1gsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eth1gsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eth1gsrc {
        #[inline(always)]
        fn from(val: u8) -> Eth1gsrc {
            Eth1gsrc::from_bits(val)
        }
    }
    impl From<Eth1gsrc> for u8 {
        #[inline(always)]
        fn from(val: Eth1gsrc) -> u8 {
            Eth1gsrc::to_bits(val)
        }
    }
    #[doc = "V5F core clock prescaler. Note 1xx encodes \"divide by 4\" (the MSB is don't-care for that case)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fpre {
        #[doc = "V5F clock not divided."]
        DIV1 = 0x0,
        #[doc = "V5F clock divided by 2."]
        DIV2 = 0x01,
        #[doc = "V5F clock divided by 4."]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Fpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpre {
        #[inline(always)]
        fn from(val: u8) -> Fpre {
            Fpre::from_bits(val)
        }
    }
    impl From<Fpre> for u8 {
        #[inline(always)]
        fn from(val: Fpre) -> u8 {
            Fpre::to_bits(val)
        }
    }
    #[doc = "HB (AHB) prescaler."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hpre {
        #[doc = "SYS_CLK not divided."]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYS_CLK divided by 2."]
        DIV2 = 0x08,
        #[doc = "SYS_CLK divided by 4."]
        DIV4 = 0x09,
        #[doc = "SYS_CLK divided by 8."]
        DIV8 = 0x0a,
        #[doc = "SYS_CLK divided by 16."]
        DIV16 = 0x0b,
        #[doc = "SYS_CLK divided by 64."]
        DIV64 = 0x0c,
        #[doc = "SYS_CLK divided by 128."]
        DIV128 = 0x0d,
        #[doc = "SYS_CLK divided by 256."]
        DIV256 = 0x0e,
        #[doc = "SYS_CLK divided by 512."]
        DIV512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[doc = "HSADC clock source. Encoding follows the WCH SDK header (RCC_HSADCSource_* in ch32h417_rcc.h) and the HSADC example which sets value 0 to obtain PLL_CLK. RM V1.7 section 3.4.13 lists the encoding for values 00 / 01 swapped (SYSCLK / PLL_CLK) — believed to be a doc typo since the SDK example is the canonical shipping code."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hsadcsrc {
        #[doc = "PLL_CLK selected."]
        PLL_CLK = 0x0,
        #[doc = "SYSCLK selected."]
        SYSCLK = 0x01,
        #[doc = "USBHS_PLL (480 MHz) selected."]
        USBHS_PLL = 0x02,
        #[doc = "ETH_PLL (500 MHz) selected."]
        ETH_PLL = 0x03,
    }
    impl Hsadcsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsadcsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsadcsrc {
        #[inline(always)]
        fn from(val: u8) -> Hsadcsrc {
            Hsadcsrc::from_bits(val)
        }
    }
    impl From<Hsadcsrc> for u8 {
        #[inline(always)]
        fn from(val: Hsadcsrc) -> u8 {
            Hsadcsrc::to_bits(val)
        }
    }
    #[doc = "LTDC clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ltdcsrc {
        #[doc = "PLL_CLK selected."]
        PLL_CLK = 0x0,
        #[doc = "SERDES_PLL divided by 2 selected."]
        SERDES_PLL_DIV2 = 0x01,
        #[doc = "ETH_PLL selected."]
        ETH_PLL = 0x02,
        #[doc = "USBHS_PLL selected."]
        USBHS_PLL = 0x03,
    }
    impl Ltdcsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ltdcsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ltdcsrc {
        #[inline(always)]
        fn from(val: u8) -> Ltdcsrc {
            Ltdcsrc::from_bits(val)
        }
    }
    impl From<Ltdcsrc> for u8 {
        #[inline(always)]
        fn from(val: Ltdcsrc) -> u8 {
            Ltdcsrc::to_bits(val)
        }
    }
    #[doc = "Microcontroller clock output (MCO pin) source selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mco {
        #[doc = "No clock output (encoded values 0000-0011)."]
        NO_CLK = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "SYSCLK output."]
        SYSCLK = 0x04,
        #[doc = "HSI (25 MHz internal RC) output."]
        HSI = 0x05,
        #[doc = "HSE output."]
        HSE = 0x06,
        #[doc = "PLL clock divided by 2 output."]
        PLL_DIV2 = 0x07,
        #[doc = "UTMI clock output."]
        UTMI = 0x08,
        #[doc = "USBSS_PLL divided by 2 output."]
        USBSS_PLL_DIV2 = 0x09,
        #[doc = "ETH_PLL divided by 8 output."]
        ETH_PLL_DIV8 = 0x0a,
        #[doc = "SERDES_PLL divided by 16 output."]
        SERDES_PLL_DIV16 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mco {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco {
        #[inline(always)]
        fn from(val: u8) -> Mco {
            Mco::from_bits(val)
        }
    }
    impl From<Mco> for u8 {
        #[inline(always)]
        fn from(val: Mco) -> u8 {
            Mco::to_bits(val)
        }
    }
    #[doc = "Main PLL multiplication factor (writable only while PLLON=0)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllmul {
        #[doc = "PLL x 4."]
        MUL4 = 0x0,
        #[doc = "PLL x 6."]
        MUL6 = 0x01,
        #[doc = "PLL x 7."]
        MUL7 = 0x02,
        #[doc = "PLL x 8."]
        MUL8 = 0x03,
        #[doc = "PLL x 8.5."]
        MUL8_5 = 0x04,
        #[doc = "PLL x 9."]
        MUL9 = 0x05,
        #[doc = "PLL x 9.5."]
        MUL9_5 = 0x06,
        #[doc = "PLL x 10."]
        MUL10 = 0x07,
        #[doc = "PLL x 10.5."]
        MUL10_5 = 0x08,
        #[doc = "PLL x 11."]
        MUL11 = 0x09,
        #[doc = "PLL x 11.5."]
        MUL11_5 = 0x0a,
        #[doc = "PLL x 12."]
        MUL12 = 0x0b,
        #[doc = "PLL x 12.5."]
        MUL12_5 = 0x0c,
        #[doc = "PLL x 13."]
        MUL13 = 0x0d,
        #[doc = "PLL x 14."]
        MUL14 = 0x0e,
        #[doc = "PLL x 15."]
        MUL15 = 0x0f,
        #[doc = "PLL x 16."]
        MUL16 = 0x10,
        #[doc = "PLL x 17."]
        MUL17 = 0x11,
        #[doc = "PLL x 18."]
        MUL18 = 0x12,
        #[doc = "PLL x 19."]
        MUL19 = 0x13,
        #[doc = "PLL x 20."]
        MUL20 = 0x14,
        #[doc = "PLL x 22."]
        MUL22 = 0x15,
        #[doc = "PLL x 24."]
        MUL24 = 0x16,
        #[doc = "PLL x 26."]
        MUL26 = 0x17,
        #[doc = "PLL x 28."]
        MUL28 = 0x18,
        #[doc = "PLL x 30."]
        MUL30 = 0x19,
        #[doc = "PLL x 32."]
        MUL32 = 0x1a,
        #[doc = "PLL x 34."]
        MUL34 = 0x1b,
        #[doc = "PLL x 36."]
        MUL36 = 0x1c,
        #[doc = "PLL x 38."]
        MUL38 = 0x1d,
        #[doc = "PLL x 40."]
        MUL40 = 0x1e,
        #[doc = "PLL x 59."]
        MUL59 = 0x1f,
    }
    impl Pllmul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmul {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmul {
        #[inline(always)]
        fn from(val: u8) -> Pllmul {
            Pllmul::from_bits(val)
        }
    }
    impl From<Pllmul> for u8 {
        #[inline(always)]
        fn from(val: Pllmul) -> u8 {
            Pllmul::to_bits(val)
        }
    }
    #[doc = "Main PLL input clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsrc {
        #[doc = "HSI selected as PLL input."]
        HSI = 0x0,
        #[doc = "HSE selected as PLL input."]
        HSE = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "USBHS_PLL (480 MHz) selected as PLL input."]
        USBHS_PLL = 0x04,
        #[doc = "ETH_PLL (500 MHz) selected as PLL input."]
        ETH_PLL = 0x05,
        #[doc = "USBSS_PLL (125 MHz) selected as PLL input."]
        USBSS_PLL = 0x06,
        #[doc = "SERDES_PLL divided by 2 selected as PLL input."]
        SERDES_PLL_DIV2 = 0x07,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[doc = "PB (APB) prescaler."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ppre {
        #[doc = "HCLK not divided."]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2."]
        DIV2 = 0x04,
        #[doc = "HCLK divided by 4."]
        DIV4 = 0x05,
        #[doc = "HCLK divided by 8."]
        DIV8 = 0x06,
        #[doc = "HCLK divided by 16."]
        DIV16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[doc = "RTC clock source selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rtcsel {
        #[doc = "No clock."]
        NO_CLK = 0x0,
        #[doc = "LSE oscillator selected as RTC clock."]
        LSE = 0x01,
        #[doc = "LSI oscillator selected as RTC clock."]
        LSI = 0x02,
        #[doc = "HSE oscillator divided by 512 selected as RTC clock."]
        HSE_DIV512 = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[doc = "SERDES_PLL multiplication factor."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SerdespllMul {
        #[doc = "SERDES_PLL x 25."]
        MUL25 = 0x0,
        #[doc = "SERDES_PLL x 28."]
        MUL28 = 0x01,
        #[doc = "SERDES_PLL x 30."]
        MUL30 = 0x02,
        #[doc = "SERDES_PLL x 32."]
        MUL32 = 0x03,
        #[doc = "SERDES_PLL x 35."]
        MUL35 = 0x04,
        #[doc = "SERDES_PLL x 38."]
        MUL38 = 0x05,
        #[doc = "SERDES_PLL x 40."]
        MUL40 = 0x06,
        #[doc = "SERDES_PLL x 45."]
        MUL45 = 0x07,
        #[doc = "SERDES_PLL x 50."]
        MUL50 = 0x08,
        #[doc = "SERDES_PLL x 56."]
        MUL56 = 0x09,
        #[doc = "SERDES_PLL x 60."]
        MUL60 = 0x0a,
        #[doc = "SERDES_PLL x 64."]
        MUL64 = 0x0b,
        #[doc = "SERDES_PLL x 70."]
        MUL70 = 0x0c,
        #[doc = "SERDES_PLL x 76."]
        MUL76 = 0x0d,
        #[doc = "SERDES_PLL x 80."]
        MUL80 = 0x0e,
        #[doc = "SERDES_PLL x 90."]
        MUL90 = 0x0f,
    }
    impl SerdespllMul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SerdespllMul {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SerdespllMul {
        #[inline(always)]
        fn from(val: u8) -> SerdespllMul {
            SerdespllMul::from_bits(val)
        }
    }
    impl From<SerdespllMul> for u8 {
        #[inline(always)]
        fn from(val: SerdespllMul) -> u8 {
            SerdespllMul::to_bits(val)
        }
    }
    #[doc = "System clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sw {
        #[doc = "HSI selected as system clock."]
        HSI = 0x0,
        #[doc = "HSE selected as system clock."]
        HSE = 0x01,
        #[doc = "PLL selected as system clock."]
        PLL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
    #[doc = "System-clock-to-PLL output selection (writable only while SYSPLL_GATE=0)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SyspllSel {
        #[doc = "PLL_CLK (covers any encoding 0xx)."]
        PLL_CLK = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "USBHS_PLL (480 MHz)."]
        USBHS_PLL = 0x04,
        #[doc = "ETH_PLL (500 MHz)."]
        ETH_PLL = 0x05,
        #[doc = "SERDES_PLL divided by 2."]
        SERDES_PLL_DIV2 = 0x06,
        #[doc = "USBSS_PLL (125 MHz)."]
        USBSS_PLL = 0x07,
    }
    impl SyspllSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyspllSel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyspllSel {
        #[inline(always)]
        fn from(val: u8) -> SyspllSel {
            SyspllSel::from_bits(val)
        }
    }
    impl From<SyspllSel> for u8 {
        #[inline(always)]
        fn from(val: SyspllSel) -> u8 {
            SyspllSel::to_bits(val)
        }
    }
    #[doc = "UHSIF clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Uhsifsrc {
        #[doc = "SYSCLK selected."]
        SYSCLK = 0x0,
        #[doc = "PLL_CLK selected."]
        PLL_CLK = 0x01,
        #[doc = "USBHS_PLL (480 MHz) selected."]
        USBHS_PLL = 0x02,
        #[doc = "ETH_PLL (500 MHz) selected."]
        ETH_PLL = 0x03,
    }
    impl Uhsifsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uhsifsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Uhsifsrc {
        #[inline(always)]
        fn from(val: u8) -> Uhsifsrc {
            Uhsifsrc::from_bits(val)
        }
    }
    impl From<Uhsifsrc> for u8 {
        #[inline(always)]
        fn from(val: Uhsifsrc) -> u8 {
            Uhsifsrc::to_bits(val)
        }
    }
    #[doc = "USBFS 48 MHz prescaler. Selects integer or half-integer divisors of the source clock."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usbfsdiv {
        #[doc = "Divide by 1."]
        DIV1 = 0x0,
        #[doc = "Divide by 2."]
        DIV2 = 0x01,
        #[doc = "Divide by 3."]
        DIV3 = 0x02,
        #[doc = "Divide by 4."]
        DIV4 = 0x03,
        #[doc = "Divide by 5."]
        DIV5 = 0x04,
        #[doc = "Divide by 6."]
        DIV6 = 0x05,
        #[doc = "Divide by 8."]
        DIV8 = 0x06,
        #[doc = "Divide by 10."]
        DIV10 = 0x07,
        #[doc = "Divide by 1.5."]
        DIV1_5 = 0x08,
        #[doc = "Divide by 2.5."]
        DIV2_5 = 0x09,
        #[doc = "Divide by 3.5."]
        DIV3_5 = 0x0a,
        #[doc = "Divide by 4.5."]
        DIV4_5 = 0x0b,
        #[doc = "Divide by 5.5."]
        DIV5_5 = 0x0c,
        #[doc = "Divide by 6.5."]
        DIV6_5 = 0x0d,
        #[doc = "Divide by 7.5."]
        DIV7_5 = 0x0e,
        #[doc = "Divide by 9.5."]
        DIV9_5 = 0x0f,
    }
    impl Usbfsdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbfsdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbfsdiv {
        #[inline(always)]
        fn from(val: u8) -> Usbfsdiv {
            Usbfsdiv::from_bits(val)
        }
    }
    impl From<Usbfsdiv> for u8 {
        #[inline(always)]
        fn from(val: Usbfsdiv) -> u8 {
            Usbfsdiv::to_bits(val)
        }
    }
    #[doc = "USBFS 48 MHz clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usbfssrc {
        #[doc = "PLL clock selected."]
        PLL = 0x0,
        #[doc = "USBHS_PLL clock selected."]
        USBHS_PLL = 0x01,
    }
    impl Usbfssrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbfssrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbfssrc {
        #[inline(always)]
        fn from(val: u8) -> Usbfssrc {
            Usbfssrc::from_bits(val)
        }
    }
    impl From<Usbfssrc> for u8 {
        #[inline(always)]
        fn from(val: Usbfssrc) -> u8 {
            Usbfssrc::to_bits(val)
        }
    }
    #[doc = "USBHS_PLL reference clock frequency."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum UsbhspllRefsel {
        #[doc = "25 MHz."]
        F25MHZ = 0x0,
        #[doc = "20 MHz."]
        F20MHZ = 0x01,
        #[doc = "24 MHz."]
        F24MHZ = 0x02,
        #[doc = "32 MHz."]
        F32MHZ = 0x03,
    }
    impl UsbhspllRefsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbhspllRefsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbhspllRefsel {
        #[inline(always)]
        fn from(val: u8) -> UsbhspllRefsel {
            UsbhspllRefsel::from_bits(val)
        }
    }
    impl From<UsbhspllRefsel> for u8 {
        #[inline(always)]
        fn from(val: UsbhspllRefsel) -> u8 {
            UsbhspllRefsel::to_bits(val)
        }
    }
    #[doc = "USBHS_PLL input clock source."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usbhspllsrc {
        #[doc = "HSE selected as USBHS_PLL input."]
        HSE = 0x0,
        #[doc = "HSI selected as USBHS_PLL input."]
        HSI = 0x01,
        #[doc = "ETHCLK_20M selected as USBHS_PLL input."]
        ETHCLK_20M = 0x02,
        #[doc = "SYS_PLL_CLK divided by USBHSPLL_IN_DIV selected as USBHS_PLL input."]
        SYS_PLL_DIV = 0x03,
    }
    impl Usbhspllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbhspllsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbhspllsrc {
        #[inline(always)]
        fn from(val: u8) -> Usbhspllsrc {
            Usbhspllsrc::from_bits(val)
        }
    }
    impl From<Usbhspllsrc> for u8 {
        #[inline(always)]
        fn from(val: Usbhspllsrc) -> u8 {
            Usbhspllsrc::to_bits(val)
        }
    }
    #[doc = "USBSS_PLL reference clock frequency."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum UsbsspllRefsel {
        #[doc = "20 MHz."]
        F20MHZ = 0x0,
        #[doc = "24 MHz."]
        F24MHZ = 0x01,
        #[doc = "25 MHz."]
        F25MHZ = 0x02,
        #[doc = "30 MHz."]
        F30MHZ = 0x03,
        #[doc = "32 MHz."]
        F32MHZ = 0x04,
        #[doc = "40 MHz."]
        F40MHZ = 0x05,
        #[doc = "60 MHz."]
        F60MHZ = 0x06,
        #[doc = "80 MHz."]
        F80MHZ = 0x07,
    }
    impl UsbsspllRefsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbsspllRefsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbsspllRefsel {
        #[inline(always)]
        fn from(val: u8) -> UsbsspllRefsel {
            UsbsspllRefsel::from_bits(val)
        }
    }
    impl From<UsbsspllRefsel> for u8 {
        #[inline(always)]
        fn from(val: UsbsspllRefsel) -> u8 {
            UsbsspllRefsel::to_bits(val)
        }
    }
}
