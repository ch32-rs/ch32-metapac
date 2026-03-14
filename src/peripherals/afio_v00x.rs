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
    #[doc = "External interrupt configuration register (AFIO_EXTICR)."]
    #[inline(always)]
    pub const fn exticr(self) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "External interrupt configuration register (AFIO_EXTICR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "EXTI configuration."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
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
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub const fn spi1_rm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub fn set_spi1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn i2c1_rm(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub fn set_i2c1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub const fn usart1_rm(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub fn set_usart1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub const fn tim1_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub fn set_tim1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub const fn tim2_rm(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub fn set_tim2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Port A1/Port A2 mapping on OSCIN/OSCOUT."]
        #[inline(always)]
        pub const fn pa1pa2_rm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port A1/Port A2 mapping on OSCIN/OSCOUT."]
        #[inline(always)]
        pub fn set_pa1pa2_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC 1 External trigger injected conversion remapping."]
        #[inline(always)]
        pub const fn adc_etrginj_rm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 External trigger injected conversion remapping."]
        #[inline(always)]
        pub fn set_adc_etrginj_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC 1 external trigger regular conversion remapping."]
        #[inline(always)]
        pub const fn adc_etrgreg_rm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 external trigger regular conversion remapping."]
        #[inline(always)]
        pub fn set_adc_etrgreg_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART2 remapping."]
        #[inline(always)]
        pub const fn usart2_rm(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "USART2 remapping."]
        #[inline(always)]
        pub fn set_usart2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "configure the I/O port with the SW function and tracing function."]
        #[inline(always)]
        pub const fn swcfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "configure the I/O port with the SW function and tracing function."]
        #[inline(always)]
        pub fn set_swcfg(&mut self, val: u8) {
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
