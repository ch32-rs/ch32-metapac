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
        #[doc = "EXTI line configuration (00=PA, 01=PB, 10=PC)."]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "EXTI line configuration (00=PA, 01=PB, 10=PC)."]
        #[inline(always)]
        pub const fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 16usize);
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
                .field("exti[8]", &self.exti(8usize))
                .field("exti[9]", &self.exti(9usize))
                .field("exti[10]", &self.exti(10usize))
                .field("exti[11]", &self.exti(11usize))
                .field("exti[12]", &self.exti(12usize))
                .field("exti[13]", &self.exti(13usize))
                .field("exti[14]", &self.exti(14usize))
                .field("exti[15]", &self.exti(15usize))
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr1(pub u32);
    impl Pcfr1 {
        #[doc = "I2C1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_rm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn set_i2c1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "UART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1_rm(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "UART1 remapping."]
        #[inline(always)]
        pub const fn set_uart1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "SPI1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_rm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub const fn set_spi1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TIM1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_rm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub const fn set_tim1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "TIM2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub const fn set_tim2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIM3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_rm(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "TIM3 remapping."]
        #[inline(always)]
        pub const fn set_tim3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "ADC External trigger injected conversion remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_etrgin_rm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC External trigger injected conversion remapping."]
        #[inline(always)]
        pub const fn set_adc_etrgin_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PB5 and PB6 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn pb5pb6_rm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB5 and PB6 remapping."]
        #[inline(always)]
        pub const fn set_pb5pb6_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB9 Vdd8 diode output is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ho_dio_en0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB9 Vdd8 diode output is enabled."]
        #[inline(always)]
        pub const fn set_ho_dio_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB11 Vdd8 diode output is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ho_dio_en1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 Vdd8 diode output is enabled."]
        #[inline(always)]
        pub const fn set_ho_dio_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB13 Vdd8 diode output is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ho_dio_en2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB13 Vdd8 diode output is enabled."]
        #[inline(always)]
        pub const fn set_ho_dio_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB15 Vdd8 diode output is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ho_dio_en3(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 Vdd8 diode output is enabled."]
        #[inline(always)]
        pub const fn set_ho_dio_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("i2c1_rm", &self.i2c1_rm())
                .field("uart1_rm", &self.uart1_rm())
                .field("spi1_rm", &self.spi1_rm())
                .field("tim1_rm", &self.tim1_rm())
                .field("tim2_rm", &self.tim2_rm())
                .field("tim3_rm", &self.tim3_rm())
                .field("adc_etrgin_rm", &self.adc_etrgin_rm())
                .field("pb5pb6_rm", &self.pb5pb6_rm())
                .field("ho_dio_en0", &self.ho_dio_en0())
                .field("ho_dio_en1", &self.ho_dio_en1())
                .field("ho_dio_en2", &self.ho_dio_en2())
                .field("ho_dio_en3", &self.ho_dio_en3())
                .field("swcfg", &self.swcfg())
                .finish()
        }
    }
}
