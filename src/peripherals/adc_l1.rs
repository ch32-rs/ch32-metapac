#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog to digital converter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "sample time register 1."]
    #[inline(always)]
    pub const fn samptr1(self) -> crate::common::Reg<regs::Samptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "sample time register 2."]
    #[inline(always)]
    pub const fn samptr2(self) -> crate::common::Reg<regs::Samptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "injected channel data offset register x."]
    #[inline(always)]
    pub const fn iofr(self, n: usize) -> crate::common::Reg<regs::Iofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "watchdog higher threshold register."]
    #[inline(always)]
    pub const fn wdhtr(self) -> crate::common::Reg<regs::Wdhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "watchdog lower threshold register."]
    #[inline(always)]
    pub const fn wdltr(self) -> crate::common::Reg<regs::Wdltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "regular sequence register 1."]
    #[inline(always)]
    pub const fn rsqr1(self) -> crate::common::Reg<regs::Rsqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "regular sequence register 2."]
    #[inline(always)]
    pub const fn rsqr2(self) -> crate::common::Reg<regs::Rsqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "regular sequence register 3."]
    #[inline(always)]
    pub const fn rsqr3(self) -> crate::common::Reg<regs::Rsqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "injected sequence register."]
    #[inline(always)]
    pub const fn isqr(self) -> crate::common::Reg<regs::Isqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "injected data register 1."]
    #[inline(always)]
    pub const fn idatar(self, n: usize) -> crate::common::Reg<regs::Idatar, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize + n * 4usize) as _) }
    }
    #[doc = "regular data register_start and discharge time register."]
    #[inline(always)]
    pub const fn rdatar(self) -> crate::common::Reg<regs::Rdatar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "ADC configuration register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "ADC buffer misalignment control."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_buftrim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC buffer misalignment control."]
        #[inline(always)]
        pub const fn set_adc_buftrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "ADC low-power mode control."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_lp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC low-power mode control."]
        #[inline(always)]
        pub const fn set_adc_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC FIFO enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC FIFO enable."]
        #[inline(always)]
        pub const fn set_fifo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ADC clock duty control."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_duty_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock duty control."]
        #[inline(always)]
        pub const fn set_adc_duty_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TOUCHKEY shielded enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tkey_drv_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TOUCHKEY shielded enable."]
        #[inline(always)]
        pub const fn set_tkey_drv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TOUCHKEY shielded each channel enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tkey_drv_outen(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x03ff;
            val as u16
        }
        #[doc = "TOUCHKEY shielded each channel enable."]
        #[inline(always)]
        pub const fn set_tkey_drv_outen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 9usize)) | (((val as u32) & 0x03ff) << 9usize);
        }
        #[doc = "TOUCHKEY bias current."]
        #[must_use]
        #[inline(always)]
        pub const fn tkey_sel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "TOUCHKEY bias current."]
        #[inline(always)]
        pub const fn set_tkey_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "TOUCHKEY Wake-up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tkey_wake_en(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x03ff;
            val as u16
        }
        #[doc = "TOUCHKEY Wake-up enable."]
        #[inline(always)]
        pub const fn set_tkey_wake_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("adc_buftrim", &self.adc_buftrim())
                .field("adc_lp", &self.adc_lp())
                .field("fifo_en", &self.fifo_en())
                .field("adc_duty_en", &self.adc_duty_en())
                .field("tkey_drv_en", &self.tkey_drv_en())
                .field("tkey_drv_outen", &self.tkey_drv_outen())
                .field("tkey_sel", &self.tkey_sel())
                .field("tkey_wake_en", &self.tkey_wake_en())
                .finish()
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Analog watchdog channel select bits."]
        #[must_use]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog channel select bits."]
        #[inline(always)]
        pub const fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Interrupt enable for EOC."]
        #[must_use]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for EOC."]
        #[inline(always)]
        pub const fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[inline(always)]
        pub const fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt enable for injected channels."]
        #[must_use]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for injected channels."]
        #[inline(always)]
        pub const fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scan(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Scan mode enable."]
        #[inline(always)]
        pub const fn set_scan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable the watchdog on a single channel in scan mode."]
        #[must_use]
        #[inline(always)]
        pub const fn awdsgl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the watchdog on a single channel in scan mode."]
        #[inline(always)]
        pub const fn set_awdsgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Automatic injected group conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic injected group conversion."]
        #[inline(always)]
        pub const fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Discontinuous mode on regular channels."]
        #[must_use]
        #[inline(always)]
        pub const fn rdiscen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on regular channels."]
        #[inline(always)]
        pub const fn set_rdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Discontinuous mode on injected channels."]
        #[must_use]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on injected channels."]
        #[inline(always)]
        pub const fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Discontinuous mode channel count."]
        #[must_use]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Discontinuous mode channel count."]
        #[inline(always)]
        pub const fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "ADC calibration mode control."]
        #[must_use]
        #[inline(always)]
        pub const fn cal_mod(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ADC calibration mode control."]
        #[inline(always)]
        pub const fn set_cal_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Analog watchdog enable on injected channels."]
        #[must_use]
        #[inline(always)]
        pub const fn jawden(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog enable on injected channels."]
        #[inline(always)]
        pub const fn set_jawden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog enable on regular channels."]
        #[must_use]
        #[inline(always)]
        pub const fn awden(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog enable on regular channels."]
        #[inline(always)]
        pub const fn set_awden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "TKEY enable, including TKEY_F and TKEY_V."]
        #[must_use]
        #[inline(always)]
        pub const fn tkenable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY enable, including TKEY_F and TKEY_V."]
        #[inline(always)]
        pub const fn set_tkenable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TKEY_I enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tkitune(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY_I enable."]
        #[inline(always)]
        pub const fn set_tkitune(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "TKEY_BUF_Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bufen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY_BUF_Enable."]
        #[inline(always)]
        pub const fn set_bufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ADC_PGA."]
        #[must_use]
        #[inline(always)]
        pub const fn pga(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "ADC_PGA."]
        #[inline(always)]
        pub const fn set_pga(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    impl core::fmt::Debug for Ctlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr1")
                .field("awdch", &self.awdch())
                .field("eocie", &self.eocie())
                .field("awdie", &self.awdie())
                .field("jeocie", &self.jeocie())
                .field("scan", &self.scan())
                .field("awdsgl", &self.awdsgl())
                .field("jauto", &self.jauto())
                .field("rdiscen", &self.rdiscen())
                .field("jdiscen", &self.jdiscen())
                .field("discnum", &self.discnum())
                .field("cal_mod", &self.cal_mod())
                .field("jawden", &self.jawden())
                .field("awden", &self.awden())
                .field("tkenable", &self.tkenable())
                .field("tkitune", &self.tkitune())
                .field("bufen", &self.bufen())
                .field("pga", &self.pga())
                .finish()
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "A/D converter ON / OFF."]
        #[must_use]
        #[inline(always)]
        pub const fn adon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A/D converter ON / OFF."]
        #[inline(always)]
        pub const fn set_adon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Continuous conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous conversion."]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "A/D calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "A/D calibration."]
        #[inline(always)]
        pub const fn set_cal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Reset calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn rstcal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Reset calibration."]
        #[inline(always)]
        pub const fn set_rstcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Direct memory access mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Direct memory access mode."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data alignment."]
        #[must_use]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data alignment."]
        #[inline(always)]
        pub const fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event select for injected group."]
        #[must_use]
        #[inline(always)]
        pub const fn jextsel(&self) -> super::vals::Jextsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Jextsel::from_bits(val as u8)
        }
        #[doc = "External event select for injected group."]
        #[inline(always)]
        pub const fn set_jextsel(&mut self, val: super::vals::Jextsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "External trigger conversion mode for injected channels."]
        #[must_use]
        #[inline(always)]
        pub const fn jexttrig(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger conversion mode for injected channels."]
        #[inline(always)]
        pub const fn set_jexttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "External event select for regular group."]
        #[must_use]
        #[inline(always)]
        pub const fn extsel(&self) -> super::vals::Extsel {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Extsel::from_bits(val as u8)
        }
        #[doc = "External event select for regular group."]
        #[inline(always)]
        pub const fn set_extsel(&mut self, val: super::vals::Extsel) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "External trigger conversion mode for regular channels."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger conversion mode for regular channels."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Start conversion of injected channels."]
        #[must_use]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of injected channels."]
        #[inline(always)]
        pub const fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Start conversion of regular channels."]
        #[must_use]
        #[inline(always)]
        pub const fn swstart(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of regular channels."]
        #[inline(always)]
        pub const fn set_swstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor and VREFINT enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tsvrefe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor and VREFINT enable."]
        #[inline(always)]
        pub const fn set_tsvrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    impl core::fmt::Debug for Ctlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr2")
                .field("adon", &self.adon())
                .field("cont", &self.cont())
                .field("cal", &self.cal())
                .field("rstcal", &self.rstcal())
                .field("dma", &self.dma())
                .field("align", &self.align())
                .field("jextsel", &self.jextsel())
                .field("jexttrig", &self.jexttrig())
                .field("extsel", &self.extsel())
                .field("exttrig", &self.exttrig())
                .field("jswstart", &self.jswstart())
                .field("swstart", &self.swstart())
                .field("tsvrefe", &self.tsvrefe())
                .finish()
        }
    }
    #[doc = "injected data register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idatar(pub u32);
    impl Idatar {
        #[doc = "Injected data."]
        #[must_use]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data."]
        #[inline(always)]
        pub const fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Idatar {
        #[inline(always)]
        fn default() -> Idatar {
            Idatar(0)
        }
    }
    impl core::fmt::Debug for Idatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idatar")
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[doc = "injected channel data offset register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iofr(pub u32);
    impl Iofr {
        #[doc = "Data offset for injected channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn joffset(&self, n: usize) -> u16 {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            let val = (self.0 >> offs) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x."]
        #[inline(always)]
        pub const fn set_joffset(&mut self, n: usize, val: u16) {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            self.0 = (self.0 & !(0x0fff << offs)) | (((val as u32) & 0x0fff) << offs);
        }
    }
    impl Default for Iofr {
        #[inline(always)]
        fn default() -> Iofr {
            Iofr(0)
        }
    }
    impl core::fmt::Debug for Iofr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iofr")
                .field("joffset[0]", &self.joffset(0usize))
                .finish()
        }
    }
    #[doc = "injected sequence register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isqr(pub u32);
    impl Isqr {
        #[doc = "1st conversion in injected sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in injected sequence."]
        #[inline(always)]
        pub const fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Injected sequence length."]
        #[must_use]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Injected sequence length."]
        #[inline(always)]
        pub const fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Isqr {
        #[inline(always)]
        fn default() -> Isqr {
            Isqr(0)
        }
    }
    impl core::fmt::Debug for Isqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isqr")
                .field("jsq[0]", &self.jsq(0usize))
                .field("jsq[1]", &self.jsq(1usize))
                .field("jsq[2]", &self.jsq(2usize))
                .field("jsq[3]", &self.jsq(3usize))
                .field("jl", &self.jl())
                .finish()
        }
    }
    #[doc = "regular data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdatar(pub u32);
    impl Rdatar {
        #[doc = "Regular data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rdatar {
        #[inline(always)]
        fn default() -> Rdatar {
            Rdatar(0)
        }
    }
    impl core::fmt::Debug for Rdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdatar")
                .field("data", &self.data())
                .finish()
        }
    }
    #[doc = "regular sequence register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsqr1(pub u32);
    impl Rsqr1 {
        #[doc = "13th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "13th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Regular channel sequence length."]
        #[must_use]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length."]
        #[inline(always)]
        pub const fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Rsqr1 {
        #[inline(always)]
        fn default() -> Rsqr1 {
            Rsqr1(0)
        }
    }
    impl core::fmt::Debug for Rsqr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsqr1")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("l", &self.l())
                .finish()
        }
    }
    #[doc = "regular sequence register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsqr2(pub u32);
    impl Rsqr2 {
        #[doc = "7th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "7th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Rsqr2 {
        #[inline(always)]
        fn default() -> Rsqr2 {
            Rsqr2(0)
        }
    }
    impl core::fmt::Debug for Rsqr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsqr2")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[doc = "regular sequence register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsqr3(pub u32);
    impl Rsqr3 {
        #[doc = "1st conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Rsqr3 {
        #[inline(always)]
        fn default() -> Rsqr3 {
            Rsqr3(0)
        }
    }
    impl core::fmt::Debug for Rsqr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsqr3")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .field("sq[5]", &self.sq(5usize))
                .finish()
        }
    }
    #[doc = "sample time register 1. Only ch 18, 17, 16 are available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Samptr1(pub u32);
    impl Samptr1 {
        #[doc = "Channel 10 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel 10 sample time selection."]
        #[inline(always)]
        pub const fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Samptr1 {
        #[inline(always)]
        fn default() -> Samptr1 {
            Samptr1(0)
        }
    }
    impl core::fmt::Debug for Samptr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Samptr1")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .finish()
        }
    }
    #[doc = "sample time register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Samptr2(pub u32);
    impl Samptr2 {
        #[doc = "Channel 0 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel 0 sample time selection."]
        #[inline(always)]
        pub const fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Samptr2 {
        #[inline(always)]
        fn default() -> Samptr2 {
            Samptr2(0)
        }
    }
    impl core::fmt::Debug for Samptr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Samptr2")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .field("smp[9]", &self.smp(9usize))
                .finish()
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Analog watchdog flag."]
        #[must_use]
        #[inline(always)]
        pub const fn awd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag."]
        #[inline(always)]
        pub const fn set_awd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular channel end of conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel end of conversion."]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected channel end of conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of conversion."]
        #[inline(always)]
        pub const fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Injected channel start flag."]
        #[must_use]
        #[inline(always)]
        pub const fn jstrt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel start flag."]
        #[inline(always)]
        pub const fn set_jstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular channel start flag."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel start flag."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    impl core::fmt::Debug for Statr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Statr")
                .field("awd", &self.awd())
                .field("eoc", &self.eoc())
                .field("jeoc", &self.jeoc())
                .field("jstrt", &self.jstrt())
                .field("strt", &self.strt())
                .finish()
        }
    }
    #[doc = "watchdog higher threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdhtr(pub u32);
    impl Wdhtr {
        #[doc = "Analog watchdog higher threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog higher threshold."]
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Wdhtr {
        #[inline(always)]
        fn default() -> Wdhtr {
            Wdhtr(0)
        }
    }
    impl core::fmt::Debug for Wdhtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdhtr").field("ht", &self.ht()).finish()
        }
    }
    #[doc = "watchdog lower threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdltr(pub u32);
    impl Wdltr {
        #[doc = "Analog watchdog lower threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog lower threshold."]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Wdltr {
        #[inline(always)]
        fn default() -> Wdltr {
            Wdltr(0)
        }
    }
    impl core::fmt::Debug for Wdltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdltr").field("lt", &self.lt()).finish()
        }
    }
}
pub mod vals {
    #[doc = "External event select for regular group."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Extsel {
        #[doc = "Timer 1 capture compare 1."]
        TIM1_CC1 = 0x0,
        #[doc = "Timer 1 capture compare 2."]
        TIM1_CC2 = 0x01,
        #[doc = "Timer 1 capture compare 3."]
        TIM1_CC3 = 0x02,
        #[doc = "Timer 2 capture compare 2."]
        TIM2_CC2 = 0x03,
        #[doc = "Timer 3 TRGO event."]
        TIM3_TRGO = 0x04,
        #[doc = "Timer 4 capture compare 4."]
        TIM4_CC4 = 0x05,
        #[doc = "External interrupt line 11."]
        EXTI11 = 0x06,
        #[doc = "Software start."]
        SWSTART = 0x07,
    }
    impl Extsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extsel {
        #[inline(always)]
        fn from(val: u8) -> Extsel {
            Extsel::from_bits(val)
        }
    }
    impl From<Extsel> for u8 {
        #[inline(always)]
        fn from(val: Extsel) -> u8 {
            Extsel::to_bits(val)
        }
    }
    #[doc = "External event select for injected group."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jextsel {
        #[doc = "Timer 1 TRGO event."]
        TIM1_TRGO = 0x0,
        #[doc = "Timer 1 capture compare 4."]
        TIM1_CC4 = 0x01,
        #[doc = "Timer 2 TRGO event."]
        TIM2_TRGO = 0x02,
        #[doc = "Timer 2 capture compare 1."]
        TIM2_CC1 = 0x03,
        #[doc = "Timer 3 capture compare 4."]
        TIM3_CC4 = 0x04,
        #[doc = "Timer 4 TRGO event."]
        TIM4_TRGO = 0x05,
        #[doc = "External interrupt line 15."]
        EXTI15 = 0x06,
        #[doc = "Software start."]
        JSWSTART = 0x07,
    }
    impl Jextsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jextsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jextsel {
        #[inline(always)]
        fn from(val: u8) -> Jextsel {
            Jextsel::from_bits(val)
        }
    }
    impl From<Jextsel> for u8 {
        #[inline(always)]
        fn from(val: Jextsel) -> u8 {
            Jextsel::to_bits(val)
        }
    }
    #[doc = "Sample time selection, when ADC_LP=0"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SampleTime {
        #[doc = "1.5 cycles"]
        CYCLES1_5 = 0x0,
        #[doc = "7.5 cycles"]
        CYCLES7_5 = 0x01,
        #[doc = "13.5 cycles"]
        CYCLES13_5 = 0x02,
        #[doc = "28.5 cycles"]
        CYCLES28_5 = 0x03,
        #[doc = "41.5 cycles"]
        CYCLES41_5 = 0x04,
        #[doc = "55.5 cycles"]
        CYCLES55_5 = 0x05,
        #[doc = "71.5 cycles"]
        CYCLES71_5 = 0x06,
        #[doc = "239.5 cycles"]
        CYCLES239_5 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
}
