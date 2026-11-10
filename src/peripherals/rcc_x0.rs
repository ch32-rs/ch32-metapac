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
    #[doc = "APB2 peripheral reset register (RCC_APB2PRSTR)."]
    #[inline(always)]
    pub const fn apb2prstr(self) -> crate::common::Reg<regs::Apb2prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1PRSTR)."]
    #[inline(always)]
    pub const fn apb1prstr(self) -> crate::common::Reg<regs::Apb1prstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)."]
    #[inline(always)]
    pub const fn ahbpcenr(self) -> crate::common::Reg<regs::Ahbpcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)."]
    #[inline(always)]
    pub const fn apb2pcenr(self) -> crate::common::Reg<regs::Apb2pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1PCENR)."]
    #[inline(always)]
    pub const fn apb1pcenr(self) -> crate::common::Reg<regs::Apb1pcenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[inline(always)]
    pub const fn rstsckr(self) -> crate::common::Reg<regs::Rstsckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB reset register (RCC_APHBRSTR)."]
    #[inline(always)]
    pub const fn ahbrstr(self) -> crate::common::Reg<regs::Ahbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbpcenr(pub u32);
    impl Ahbpcenr {
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
        #[doc = "SRAM interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sramen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable."]
        #[inline(always)]
        pub const fn set_sramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USBFS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS clock enable."]
        #[inline(always)]
        pub const fn set_usbfsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USBPD clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD clock enable."]
        #[inline(always)]
        pub const fn set_usbpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahbpcenr {
        #[inline(always)]
        fn default() -> Ahbpcenr {
            Ahbpcenr(0)
        }
    }
    impl core::fmt::Debug for Ahbpcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbpcenr")
                .field("dma1en", &self.dma1en())
                .field("sramen", &self.sramen())
                .field("usbfsen", &self.usbfsen())
                .field("usbpd", &self.usbpd())
                .finish()
        }
    }
    #[doc = "AHB reset register (RCC_APHBRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrstr(pub u32);
    impl Ahbrstr {
        #[doc = "USBFS reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfsrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS reset."]
        #[inline(always)]
        pub const fn set_usbfsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PIOC reset."]
        #[must_use]
        #[inline(always)]
        pub const fn piocrst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PIOC reset."]
        #[inline(always)]
        pub const fn set_piocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USBPD reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpdrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD reset."]
        #[inline(always)]
        pub const fn set_usbpdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahbrstr {
        #[inline(always)]
        fn default() -> Ahbrstr {
            Ahbrstr(0)
        }
    }
    impl core::fmt::Debug for Ahbrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbrstr")
                .field("usbfsrst", &self.usbfsrst())
                .field("piocrst", &self.piocrst())
                .field("usbpdrst", &self.usbpdrst())
                .finish()
        }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1pcenr(pub u32);
    impl Apb1pcenr {
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
        #[doc = "USART 2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 clock enable."]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 clock enable."]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 clock enable."]
        #[inline(always)]
        pub const fn set_usart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C 1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 clock enable."]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
    impl Default for Apb1pcenr {
        #[inline(always)]
        fn default() -> Apb1pcenr {
            Apb1pcenr(0)
        }
    }
    impl core::fmt::Debug for Apb1pcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1pcenr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("wwdgen", &self.wwdgen())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("usart4en", &self.usart4en())
                .field("i2c1en", &self.i2c1en())
                .field("pwren", &self.pwren())
                .finish()
        }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1prstr(pub u32);
    impl Apb1prstr {
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
    impl Default for Apb1prstr {
        #[inline(always)]
        fn default() -> Apb1prstr {
            Apb1prstr(0)
        }
    }
    impl core::fmt::Debug for Apb1prstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1prstr")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("usart4rst", &self.usart4rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("pwrrst", &self.pwrrst())
                .finish()
        }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2pcenr(pub u32);
    impl Apb2pcenr {
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
    }
    impl Default for Apb2pcenr {
        #[inline(always)]
        fn default() -> Apb2pcenr {
            Apb2pcenr(0)
        }
    }
    impl core::fmt::Debug for Apb2pcenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2pcenr")
                .field("afioen", &self.afioen())
                .field("iopaen", &self.iopaen())
                .field("iopben", &self.iopben())
                .field("iopcen", &self.iopcen())
                .field("adc1en", &self.adc1en())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("usart1en", &self.usart1en())
                .finish()
        }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2PRSTR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2prstr(pub u32);
    impl Apb2prstr {
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
    }
    impl Default for Apb2prstr {
        #[inline(always)]
        fn default() -> Apb2prstr {
            Apb2prstr(0)
        }
    }
    impl core::fmt::Debug for Apb2prstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2prstr")
                .field("afiorst", &self.afiorst())
                .field("ioparst", &self.ioparst())
                .field("iopbrst", &self.iopbrst())
                .field("iopcrst", &self.iopcrst())
                .field("adc1rst", &self.adc1rst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("usart1rst", &self.usart1rst())
                .finish()
        }
    }
    #[doc = "Clock configuration register (RCC_CFGR0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr0(pub u32);
    impl Cfgr0 {
        #[doc = "AHB prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler."]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Microcontroller clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn mco(&self) -> super::vals::Mco {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mco::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output."]
        #[inline(always)]
        pub const fn set_mco(&mut self, val: super::vals::Mco) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
                .field("hpre", &self.hpre())
                .field("mco", &self.mco())
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
        #[doc = "Internal High Speed clock trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal High Speed clock trimming."]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal High Speed clock Calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal High Speed clock Calibration."]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                .field("hsitrim", &self.hsitrim())
                .field("hsical", &self.hsical())
                .finish()
        }
    }
    #[doc = "Control/status register (RCC_RSTSCKR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstsckr(pub u32);
    impl Rstsckr {
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
        #[doc = "OPA reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn oparstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "OPA reset flag."]
        #[inline(always)]
        pub const fn set_oparstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
        #[doc = "Low-power reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("rmvf", &self.rmvf())
                .field("oparstf", &self.oparstf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
}
pub mod vals {
    #[doc = "AHB prescaler."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hpre {
        #[doc = "SYSCLK not divided."]
        DIV1 = 0x0,
        #[doc = "SYSCLK divided by 2."]
        DIV2 = 0x01,
        #[doc = "SYSCLK divided by 3."]
        DIV3 = 0x02,
        #[doc = "SYSCLK divided by 4."]
        DIV4 = 0x03,
        #[doc = "SYSCLK divided by 5."]
        DIV5 = 0x04,
        #[doc = "SYSCLK divided by 6."]
        DIV6 = 0x05,
        #[doc = "SYSCLK divided by 7."]
        DIV7 = 0x06,
        #[doc = "SYSCLK divided by 8."]
        DIV8 = 0x07,
        #[doc = "SYSCLK divided by 2."]
        DIV2_ALT = 0x08,
        #[doc = "SYSCLK divided by 4."]
        DIV4_ALT = 0x09,
        #[doc = "SYSCLK divided by 8."]
        DIV8_ALT = 0x0a,
        #[doc = "SYSCLK divided by 16."]
        DIV16 = 0x0b,
        #[doc = "SYSCLK divided by 32."]
        DIV32 = 0x0c,
        #[doc = "SYSCLK divided by 64."]
        DIV64 = 0x0d,
        #[doc = "SYSCLK divided by 128."]
        DIV128 = 0x0e,
        #[doc = "SYSCLK divided by 256."]
        DIV256 = 0x0f,
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
    #[doc = "Microcontroller clock output."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mco {
        #[doc = "No clock output."]
        NOCLK = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "SYSCLK selected."]
        SYSCLK = 0x04,
        #[doc = "HSI selected."]
        HSI = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco {
            unsafe { core::mem::transmute(val & 0x07) }
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
}
