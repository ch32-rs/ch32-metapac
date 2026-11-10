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
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration register (AFIO_EXTICR)."]
    #[inline(always)]
    pub const fn exticr(self) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "External interrupt configuration register (AFIO_EXTICR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI0 configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "EXTI0 configuration."]
        #[inline(always)]
        pub const fn set_exti(&mut self, n: usize, val: u8) {
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
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .field("exti[4]", &self.exti(4usize))
                .field("exti[5]", &self.exti(5usize))
                .field("exti[6]", &self.exti(6usize))
                .field("exti[7]", &self.exti(7usize))
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr1(pub u32);
    impl Pcfr1 {
        #[doc = "SPI1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_rm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub const fn set_spi1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_rm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn set_i2c1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_rm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub const fn set_usart1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_rm(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub const fn set_tim1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "TIM2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_rm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub const fn set_tim2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Port A1/Port A2 mapping on OSCIN/OSCOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12_rm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port A1/Port A2 mapping on OSCIN/OSCOUT."]
        #[inline(always)]
        pub const fn set_pa12_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC 1 External trigger injected conversion remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1_etrginj_rm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 External trigger injected conversion remapping."]
        #[inline(always)]
        pub const fn set_adc1_etrginj_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC 1 external trigger regular conversion remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1_etrgreg_rm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 external trigger regular conversion remapping."]
        #[inline(always)]
        pub const fn set_adc1_etrgreg_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_rm1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub const fn set_usart1_rm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_rm1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn set_i2c1_rm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "TIM1_CH1 channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_ch1_rm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1_CH1 channel selection."]
        #[inline(always)]
        pub const fn set_tim1_ch1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Serial wire JTAG configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn swcfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub const fn set_swcfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Pcfr1 {
        #[inline(always)]
        fn default() -> Pcfr1 {
            Pcfr1(0)
        }
    }
    impl core::fmt::Debug for Pcfr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcfr1")
                .field("spi1_rm", &self.spi1_rm())
                .field("i2c1_rm", &self.i2c1_rm())
                .field("usart1_rm", &self.usart1_rm())
                .field("tim1_rm", &self.tim1_rm())
                .field("tim2_rm", &self.tim2_rm())
                .field("pa12_rm", &self.pa12_rm())
                .field("adc1_etrginj_rm", &self.adc1_etrginj_rm())
                .field("adc1_etrgreg_rm", &self.adc1_etrgreg_rm())
                .field("usart1_rm1", &self.usart1_rm1())
                .field("i2c1_rm1", &self.i2c1_rm1())
                .field("tim1_ch1_rm", &self.tim1_ch1_rm())
                .field("swcfg", &self.swcfg())
                .finish()
        }
    }
}
