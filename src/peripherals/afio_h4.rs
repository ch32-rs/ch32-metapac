#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Alternate function I/O."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afio {
    ptr: *mut u8,
}
unsafe impl Send for Afio {}
unsafe impl Sync for Afio {}
impl Afio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO alternate-function selection. 6 ports (A..F) x 2 halves (low pins 0..7, high pins 8..15) = 12 registers. Index as `gpio_afr(port_idx * 2 + (pin / 8)).afr(pin % 8) = af_num`, where port_idx is A=0, B=1, ..., F=5."]
    #[inline(always)]
    pub const fn gpio_afr(self, n: usize) -> crate::common::Reg<regs::Afr, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "External interrupt configuration register (AFIO_EXTICRx). EXTICR\\[0\\]
holds EXTI lines 0..7, EXTICR\\[1\\]
holds lines 8..15."]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "GPIO alternate-function selection. Each 4-bit field selects the AF for one pin (8 pins per register)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afr(pub u32);
    impl Afr {
        #[doc = "AF selection for the pin."]
        #[inline(always)]
        pub const fn afr(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "AF selection for the pin."]
        #[inline(always)]
        pub fn set_afr(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Afr {
        #[inline(always)]
        fn default() -> Afr {
            Afr(0)
        }
    }
    #[doc = "External interrupt configuration register (AFIO_EXTICRx). Each 4-bit field selects which GPIO port drives the corresponding EXTI line."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "GPIO port that drives the EXTI line."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> super::vals::ExtiPort {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::ExtiPort::from_bits(val as u8)
        }
        #[doc = "GPIO port that drives the EXTI line."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: super::vals::ExtiPort) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr1(pub u32);
    impl Pcfr1 {
        #[doc = "PD0PD1 remapping."]
        #[inline(always)]
        pub const fn pd0pd1_rm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PD0PD1 remapping."]
        #[inline(always)]
        pub fn set_pd0pd1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC1_ETRGREG remapping."]
        #[inline(always)]
        pub const fn adc1_etrgreg_rm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1_ETRGREG remapping."]
        #[inline(always)]
        pub fn set_adc1_etrgreg_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1_ETRGINJ remapping."]
        #[inline(always)]
        pub const fn adc1_etrginj_rm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1_ETRGINJ remapping."]
        #[inline(always)]
        pub fn set_adc1_etrginj_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC2_ETRGREG remapping."]
        #[inline(always)]
        pub const fn adc2_etrgreg_rm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2_ETRGREG remapping."]
        #[inline(always)]
        pub fn set_adc2_etrgreg_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC2_ETRGINJ remapping."]
        #[inline(always)]
        pub const fn adc2_etrginj_rm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2_ETRGINJ remapping."]
        #[inline(always)]
        pub fn set_adc2_etrginj_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UHSIF_CLK remapping."]
        #[inline(always)]
        pub const fn uhsif_clk_rm(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "UHSIF_CLK remapping."]
        #[inline(always)]
        pub fn set_uhsif_clk_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "UHSIF_PORT remapping."]
        #[inline(always)]
        pub const fn uhsif_port_rm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "UHSIF_PORT remapping."]
        #[inline(always)]
        pub fn set_uhsif_port_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "SDMMC remapping."]
        #[inline(always)]
        pub const fn sdmmc_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "SDMMC remapping."]
        #[inline(always)]
        pub fn set_sdmmc_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIM2ITR1 remapping."]
        #[inline(always)]
        pub const fn tim2itr1_rm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2ITR1 remapping."]
        #[inline(always)]
        pub fn set_tim2itr1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VIO18 IO speed configuration at low voltage."]
        #[inline(always)]
        pub const fn vio18_io_hslv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VIO18 IO speed configuration at low voltage."]
        #[inline(always)]
        pub fn set_vio18_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VIO33 IO speed configuration at low voltage."]
        #[inline(always)]
        pub const fn vio33_io_hslv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VIO33 IO speed configuration at low voltage."]
        #[inline(always)]
        pub fn set_vio33_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "VDD33 IO speed configuration at low voltage."]
        #[inline(always)]
        pub const fn vdd33_io_hslv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VDD33 IO speed configuration at low voltage."]
        #[inline(always)]
        pub fn set_vdd33_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CC pin input channel threshold adjustment."]
        #[inline(always)]
        pub const fn usbpd_cc_hvt(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CC pin input channel threshold adjustment."]
        #[inline(always)]
        pub fn set_usbpd_cc_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub const fn sw_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub fn set_sw_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Pcfr1 {
        #[inline(always)]
        fn default() -> Pcfr1 {
            Pcfr1(0)
        }
    }
}
pub mod vals {
    #[doc = "GPIO port selection for an EXTI line (encoding matches SDK GPIO_PortSourceGPIO*). Only EXTI lines 0..15 are GPIO-routable; lines 16..26 are peripheral wake events."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ExtiPort {
        #[doc = "GPIOA pin selected."]
        PA = 0x0,
        #[doc = "GPIOB pin selected."]
        PB = 0x01,
        #[doc = "GPIOC pin selected."]
        PC = 0x02,
        #[doc = "GPIOD pin selected."]
        PD = 0x03,
        #[doc = "GPIOE pin selected."]
        PE = 0x04,
        #[doc = "GPIOF pin selected (CH32H4 family only)."]
        PF = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl ExtiPort {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtiPort {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtiPort {
        #[inline(always)]
        fn from(val: u8) -> ExtiPort {
            ExtiPort::from_bits(val)
        }
    }
    impl From<ExtiPort> for u8 {
        #[inline(always)]
        fn from(val: ExtiPort) -> u8 {
            ExtiPort::to_bits(val)
        }
    }
}
