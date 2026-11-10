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
    #[doc = "control register 1/TKEY_V_CTLR."]
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
    pub const fn iofr1(self) -> crate::common::Reg<regs::Iofr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "injected channel data offset register x."]
    #[inline(always)]
    pub const fn iofr2(self) -> crate::common::Reg<regs::Iofr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "injected channel data offset register x."]
    #[inline(always)]
    pub const fn iofr3(self) -> crate::common::Reg<regs::Iofr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "injected channel data offset register x."]
    #[inline(always)]
    pub const fn iofr4(self) -> crate::common::Reg<regs::Iofr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
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
    #[doc = "injected data register x."]
    #[inline(always)]
    pub const fn idatar1(self) -> crate::common::Reg<regs::Idatar1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "injected data register x."]
    #[inline(always)]
    pub const fn idatar2(self) -> crate::common::Reg<regs::Idatar2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "injected data register x."]
    #[inline(always)]
    pub const fn idatar3(self) -> crate::common::Reg<regs::Idatar3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "injected data register x."]
    #[inline(always)]
    pub const fn idatar4(self) -> crate::common::Reg<regs::Idatar4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "regular data register."]
    #[inline(always)]
    pub const fn rdatar(self) -> crate::common::Reg<regs::Rdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "ADC sampling time register."]
    #[inline(always)]
    pub const fn smpmd(self) -> crate::common::Reg<regs::Smpmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ADC programmable sampling time register."]
    #[inline(always)]
    pub const fn udp(self) -> crate::common::Reg<regs::Udp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ADC cascade configure register."]
    #[inline(always)]
    pub const fn concfg(self) -> crate::common::Reg<regs::Concfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "ADC scanning count configuration register."]
    #[inline(always)]
    pub const fn scancnt(self) -> crate::common::Reg<regs::Scancnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC cascade configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Concfg(pub u32);
    impl Concfg {
        #[doc = "rule channel cascade trigger delay."]
        #[must_use]
        #[inline(always)]
        pub const fn regudly(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "rule channel cascade trigger delay."]
        #[inline(always)]
        pub const fn set_regudly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "injection of channel-level cascading trigger delay."]
        #[must_use]
        #[inline(always)]
        pub const fn injedly(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "injection of channel-level cascading trigger delay."]
        #[inline(always)]
        pub const fn set_injedly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "effective injection of trigger sequence number configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn injeseq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "effective injection of trigger sequence number configuration."]
        #[inline(always)]
        pub const fn set_injeseq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "inject trigger to configure the exchange cycle period."]
        #[must_use]
        #[inline(always)]
        pub const fn injenum(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "inject trigger to configure the exchange cycle period."]
        #[inline(always)]
        pub const fn set_injenum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for Concfg {
        #[inline(always)]
        fn default() -> Concfg {
            Concfg(0)
        }
    }
    impl core::fmt::Debug for Concfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Concfg")
                .field("regudly", &self.regudly())
                .field("injedly", &self.injedly())
                .field("injeseq", &self.injeseq())
                .field("injenum", &self.injenum())
                .finish()
        }
    }
    #[doc = "control register 1/TKEY_V_CTLR."]
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
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on regular channels."]
        #[inline(always)]
        pub const fn set_discen(&mut self, val: bool) {
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
        #[doc = "DOUBLE_MODE enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dualmod(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DOUBLE_MODE enable."]
        #[inline(always)]
        pub const fn set_dualmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
        #[doc = "ADC buffer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bufen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ADC buffer enable."]
        #[inline(always)]
        pub const fn set_bufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ADC watchdog reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn awd_rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADC watchdog reset enable."]
        #[inline(always)]
        pub const fn set_awd_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "simulation module reset enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ana_rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "simulation module reset enabled."]
        #[inline(always)]
        pub const fn set_ana_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Channel pre-handoff is closed."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_pre(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pre-handoff is closed."]
        #[inline(always)]
        pub const fn set_sw_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("discen", &self.discen())
                .field("jdiscen", &self.jdiscen())
                .field("discnum", &self.discnum())
                .field("dualmod", &self.dualmod())
                .field("jawden", &self.jawden())
                .field("awden", &self.awden())
                .field("bufen", &self.bufen())
                .field("awd_rst", &self.awd_rst())
                .field("ana_rst", &self.ana_rst())
                .field("sw_pre", &self.sw_pre())
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
        #[doc = "calibration voltage configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cal_vol(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "calibration voltage configuration."]
        #[inline(always)]
        pub const fn set_cal_vol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "the hardware automatic calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn cal_auto(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "the hardware automatic calibration."]
        #[inline(always)]
        pub const fn set_cal_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "External event select for injected group."]
        #[inline(always)]
        pub const fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "External event select for regular group."]
        #[inline(always)]
        pub const fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
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
        #[doc = "internal voltage channel enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "internal voltage channel enable bit."]
        #[inline(always)]
        pub const fn set_vrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "enhancement mode configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn enhance(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "enhancement mode configuration bit."]
        #[inline(always)]
        pub const fn set_enhance(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "rule channel cascade enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn regucon(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "rule channel cascade enable bit."]
        #[inline(always)]
        pub const fn set_regucon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "inject channel cascading enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn injecon(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "inject channel cascading enable bit."]
        #[inline(always)]
        pub const fn set_injecon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADC preamplifier selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn conts(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "ADC preamplifier selection bit."]
        #[inline(always)]
        pub const fn set_conts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
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
                .field("cal_vol", &self.cal_vol())
                .field("cal_auto", &self.cal_auto())
                .field("dma", &self.dma())
                .field("align", &self.align())
                .field("jextsel", &self.jextsel())
                .field("jexttrig", &self.jexttrig())
                .field("extsel", &self.extsel())
                .field("exttrig", &self.exttrig())
                .field("jswstart", &self.jswstart())
                .field("swstart", &self.swstart())
                .field("vrefe", &self.vrefe())
                .field("enhance", &self.enhance())
                .field("regucon", &self.regucon())
                .field("injecon", &self.injecon())
                .field("conts", &self.conts())
                .finish()
        }
    }
    #[doc = "injected data register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idatar1(pub u32);
    impl Idatar1 {
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
    impl Default for Idatar1 {
        #[inline(always)]
        fn default() -> Idatar1 {
            Idatar1(0)
        }
    }
    impl core::fmt::Debug for Idatar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idatar1")
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[doc = "injected data register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idatar2(pub u32);
    impl Idatar2 {
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
    impl Default for Idatar2 {
        #[inline(always)]
        fn default() -> Idatar2 {
            Idatar2(0)
        }
    }
    impl core::fmt::Debug for Idatar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idatar2")
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[doc = "injected data register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idatar3(pub u32);
    impl Idatar3 {
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
    impl Default for Idatar3 {
        #[inline(always)]
        fn default() -> Idatar3 {
            Idatar3(0)
        }
    }
    impl core::fmt::Debug for Idatar3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idatar3")
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[doc = "injected data register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idatar4(pub u32);
    impl Idatar4 {
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
    impl Default for Idatar4 {
        #[inline(always)]
        fn default() -> Idatar4 {
            Idatar4(0)
        }
    }
    impl core::fmt::Debug for Idatar4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idatar4")
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[doc = "injected channel data offset register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iofr1(pub u32);
    impl Iofr1 {
        #[doc = "Data offset for injected channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn joffset1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x."]
        #[inline(always)]
        pub const fn set_joffset1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Iofr1 {
        #[inline(always)]
        fn default() -> Iofr1 {
            Iofr1(0)
        }
    }
    impl core::fmt::Debug for Iofr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iofr1")
                .field("joffset1", &self.joffset1())
                .finish()
        }
    }
    #[doc = "injected channel data offset register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iofr2(pub u32);
    impl Iofr2 {
        #[doc = "Data offset for injected channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn joffset2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x."]
        #[inline(always)]
        pub const fn set_joffset2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Iofr2 {
        #[inline(always)]
        fn default() -> Iofr2 {
            Iofr2(0)
        }
    }
    impl core::fmt::Debug for Iofr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iofr2")
                .field("joffset2", &self.joffset2())
                .finish()
        }
    }
    #[doc = "injected channel data offset register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iofr3(pub u32);
    impl Iofr3 {
        #[doc = "Data offset for injected channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn joffset3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x."]
        #[inline(always)]
        pub const fn set_joffset3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Iofr3 {
        #[inline(always)]
        fn default() -> Iofr3 {
            Iofr3(0)
        }
    }
    impl core::fmt::Debug for Iofr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iofr3")
                .field("joffset3", &self.joffset3())
                .finish()
        }
    }
    #[doc = "injected channel data offset register x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iofr4(pub u32);
    impl Iofr4 {
        #[doc = "Data offset for injected channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn joffset4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x."]
        #[inline(always)]
        pub const fn set_joffset4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Iofr4 {
        #[inline(always)]
        fn default() -> Iofr4 {
            Iofr4(0)
        }
    }
    impl core::fmt::Debug for Iofr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iofr4")
                .field("joffset4", &self.joffset4())
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
        pub const fn jsq1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in injected sequence."]
        #[inline(always)]
        pub const fn set_jsq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "2nd conversion in injected sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn jsq2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "2nd conversion in injected sequence."]
        #[inline(always)]
        pub const fn set_jsq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "3rd conversion in injected sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn jsq3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "3rd conversion in injected sequence."]
        #[inline(always)]
        pub const fn set_jsq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "4th conversion in injected sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn jsq4(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "4th conversion in injected sequence."]
        #[inline(always)]
        pub const fn set_jsq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
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
                .field("jsq1", &self.jsq1())
                .field("jsq2", &self.jsq2())
                .field("jsq3", &self.jsq3())
                .field("jsq4", &self.jsq4())
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
        #[doc = "converter data."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2data(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "converter data."]
        #[inline(always)]
        pub const fn set_adc2data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("adc2data", &self.adc2data())
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
        pub const fn sq13(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "13th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "14th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq14(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "14th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "15th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq15(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "15th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "16th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq16(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "16th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
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
                .field("sq13", &self.sq13())
                .field("sq14", &self.sq14())
                .field("sq15", &self.sq15())
                .field("sq16", &self.sq16())
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
        pub const fn sq7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "7th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "8th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq8(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "8th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "9th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq9(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "9th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "10th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq10(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "10th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "11th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq11(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "11th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "12th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq12(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "12th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
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
                .field("sq7", &self.sq7())
                .field("sq8", &self.sq8())
                .field("sq9", &self.sq9())
                .field("sq10", &self.sq10())
                .field("sq11", &self.sq11())
                .field("sq12", &self.sq12())
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
        pub const fn sq1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "2nd conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "2nd conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "3rd conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "3rd conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "4th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq4(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "4th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "5th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "5th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "6th conversion in regular sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn sq6(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "6th conversion in regular sequence."]
        #[inline(always)]
        pub const fn set_sq6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
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
                .field("sq1", &self.sq1())
                .field("sq2", &self.sq2())
                .field("sq3", &self.sq3())
                .field("sq4", &self.sq4())
                .field("sq5", &self.sq5())
                .field("sq6", &self.sq6())
                .finish()
        }
    }
    #[doc = "sample time register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Samptr1(pub u32);
    impl Samptr1 {
        #[doc = "Channel 10 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 10 sample time selection."]
        #[inline(always)]
        pub const fn set_smp10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Channel 11 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp11(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 11 sample time selection."]
        #[inline(always)]
        pub const fn set_smp11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Channel 12 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp12(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 12 sample time selection."]
        #[inline(always)]
        pub const fn set_smp12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Channel 13 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp13(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 13 sample time selection."]
        #[inline(always)]
        pub const fn set_smp13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
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
                .field("smp10", &self.smp10())
                .field("smp11", &self.smp11())
                .field("smp12", &self.smp12())
                .field("smp13", &self.smp13())
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
        pub const fn smp0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 0 sample time selection."]
        #[inline(always)]
        pub const fn set_smp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Channel 1 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 1 sample time selection."]
        #[inline(always)]
        pub const fn set_smp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Channel 2 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp2(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 2 sample time selection."]
        #[inline(always)]
        pub const fn set_smp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Channel 3 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp3(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 3 sample time selection."]
        #[inline(always)]
        pub const fn set_smp3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Channel 4 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 4 sample time selection."]
        #[inline(always)]
        pub const fn set_smp4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Channel 5 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp5(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 5 sample time selection."]
        #[inline(always)]
        pub const fn set_smp5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "Channel 6 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp6(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 6 sample time selection."]
        #[inline(always)]
        pub const fn set_smp6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "Channel 7 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp7(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 7 sample time selection."]
        #[inline(always)]
        pub const fn set_smp7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Channel 8 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 8 sample time selection."]
        #[inline(always)]
        pub const fn set_smp8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Channel 9 sample time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn smp9(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x07;
            val as u8
        }
        #[doc = "Channel 9 sample time selection."]
        #[inline(always)]
        pub const fn set_smp9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
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
                .field("smp0", &self.smp0())
                .field("smp1", &self.smp1())
                .field("smp2", &self.smp2())
                .field("smp3", &self.smp3())
                .field("smp4", &self.smp4())
                .field("smp5", &self.smp5())
                .field("smp6", &self.smp6())
                .field("smp7", &self.smp7())
                .field("smp8", &self.smp8())
                .field("smp9", &self.smp9())
                .finish()
        }
    }
    #[doc = "ADC scanning count configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scancnt(pub u32);
    impl Scancnt {
        #[doc = "scan channel source configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn scscfg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "scan channel source configuration bit."]
        #[inline(always)]
        pub const fn set_scscfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn scanoe0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 0."]
        #[inline(always)]
        pub const fn set_scanoe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn scanoe1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 1."]
        #[inline(always)]
        pub const fn set_scanoe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn scanoe2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output enable bit of the scanning cycle counter bit 2."]
        #[inline(always)]
        pub const fn set_scanoe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Scancnt {
        #[inline(always)]
        fn default() -> Scancnt {
            Scancnt(0)
        }
    }
    impl core::fmt::Debug for Scancnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scancnt")
                .field("scscfg", &self.scscfg())
                .field("scanoe0", &self.scanoe0())
                .field("scanoe1", &self.scanoe1())
                .field("scanoe2", &self.scanoe2())
                .finish()
        }
    }
    #[doc = "ADC sampling time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpmd(pub u32);
    impl Smpmd {
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "channel sampling mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn smpmd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "channel sampling mode configuration."]
        #[inline(always)]
        pub const fn set_smpmd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Smpmd {
        #[inline(always)]
        fn default() -> Smpmd {
            Smpmd(0)
        }
    }
    impl core::fmt::Debug for Smpmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpmd")
                .field("smpmd0", &self.smpmd0())
                .field("smpmd1", &self.smpmd1())
                .field("smpmd2", &self.smpmd2())
                .field("smpmd3", &self.smpmd3())
                .field("smpmd4", &self.smpmd4())
                .field("smpmd5", &self.smpmd5())
                .field("smpmd6", &self.smpmd6())
                .field("smpmd7", &self.smpmd7())
                .field("smpmd8", &self.smpmd8())
                .field("smpmd9", &self.smpmd9())
                .field("smpmd10", &self.smpmd10())
                .field("smpmd11", &self.smpmd11())
                .field("smpmd12", &self.smpmd12())
                .field("smpmd13", &self.smpmd13())
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
        #[doc = "ADC reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn adcrstf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reset flag."]
        #[inline(always)]
        pub const fn set_adcrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "calibration deviation value."]
        #[must_use]
        #[inline(always)]
        pub const fn delta(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "calibration deviation value."]
        #[inline(always)]
        pub const fn set_delta(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
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
                .field("adcrstf", &self.adcrstf())
                .field("delta", &self.delta())
                .finish()
        }
    }
    #[doc = "ADC programmable sampling time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Udp(pub u32);
    impl Udp {
        #[doc = "user programmable sampling time."]
        #[must_use]
        #[inline(always)]
        pub const fn smpudp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "user programmable sampling time."]
        #[inline(always)]
        pub const fn set_smpudp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "calibration channel sampling time."]
        #[must_use]
        #[inline(always)]
        pub const fn smpcal(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "calibration channel sampling time."]
        #[inline(always)]
        pub const fn set_smpcal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Udp {
        #[inline(always)]
        fn default() -> Udp {
            Udp(0)
        }
    }
    impl core::fmt::Debug for Udp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Udp")
                .field("smpudp", &self.smpudp())
                .field("smpcal", &self.smpcal())
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
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Analog watchdog higher threshold."]
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
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
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Analog watchdog lower threshold."]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
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
