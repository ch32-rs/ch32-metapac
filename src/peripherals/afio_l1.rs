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
    #[doc = "Event Control Register (AFIO_ECR)."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register 1 (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)."]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "AFIO control register (AFIO_CR)."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)."]
    #[inline(always)]
    pub const fn pcfr2(self) -> crate::common::Reg<regs::Pcfr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "AFIO control register (AFIO_CR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "PD pin PB6/PD7 High threshold input mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_in_hvt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PD pin PB6/PD7 High threshold input mode."]
        #[inline(always)]
        pub const fn set_usbpd_in_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA12/UDP pin BC source voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC source voltage enable."]
        #[inline(always)]
        pub const fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PA11/UDM pin BC source voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC source voltage enable."]
        #[inline(always)]
        pub const fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PA12/UDP pin BC protocol comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC protocol comparator enable."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PA11/UDM pin BC protocol comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC protocol comparator enable."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PA12/UDP pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PA11/UDM pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("usbpd_in_hvt", &self.usbpd_in_hvt())
                .field("udp_bc_vsrc", &self.udp_bc_vsrc())
                .field("udm_bc_vsrc", &self.udm_bc_vsrc())
                .field("udp_bc_cmpe", &self.udp_bc_cmpe())
                .field("udm_bc_cmpe", &self.udm_bc_cmpe())
                .field("udp_bc_cmpo", &self.udp_bc_cmpo())
                .field("udm_bc_cmpo", &self.udm_bc_cmpo())
                .finish()
        }
    }
    #[doc = "Event Control Register (AFIO_ECR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Pin selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pin selection."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Port selection."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Port selection."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event Output Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn evoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event Output Enable."]
        #[inline(always)]
        pub const fn set_evoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    impl core::fmt::Debug for Ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecr")
                .field("pin", &self.pin())
                .field("port", &self.port())
                .field("evoe", &self.evoe())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI12 configuration."]
        #[inline(always)]
        pub const fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
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
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register 1 (AFIO_PCFR1)."]
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
        #[doc = "USART2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_rm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 remapping."]
        #[inline(always)]
        pub const fn set_usart2_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3_rm(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 remapping."]
        #[inline(always)]
        pub const fn set_usart3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
        #[doc = "TIM3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_rm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 remapping."]
        #[inline(always)]
        pub const fn set_tim3_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM4 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4_rm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 remapping."]
        #[inline(always)]
        pub const fn set_tim4_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn can_rm(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CAN remapping."]
        #[inline(always)]
        pub const fn set_can_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01_rm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT."]
        #[inline(always)]
        pub const fn set_pd01_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Serial wire JTAG configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub const fn set_sw_cfg(&mut self, val: u8) {
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
                .field("usart2_rm", &self.usart2_rm())
                .field("usart3_rm", &self.usart3_rm())
                .field("tim1_rm", &self.tim1_rm())
                .field("tim2_rm", &self.tim2_rm())
                .field("tim3_rm", &self.tim3_rm())
                .field("tim4_rm", &self.tim4_rm())
                .field("can_rm", &self.can_rm())
                .field("pd01_rm", &self.pd01_rm())
                .field("sw_cfg", &self.sw_cfg())
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr2(pub u32);
    impl Pcfr2 {
        #[doc = "USART4 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4_rm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 remapping."]
        #[inline(always)]
        pub const fn set_usart4_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_rm_h(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 remapping."]
        #[inline(always)]
        pub const fn set_usart2_rm_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_rm_h(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub const fn set_usart1_rm_h(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "TIM2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_rm_h(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub const fn set_tim2_rm_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TIM1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_rm_h(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub const fn set_tim1_rm_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_rm_h(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn set_i2c1_rm_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SPI1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_rm_h(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub const fn set_spi1_rm_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "LPTIM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim_rm(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM remapping."]
        #[inline(always)]
        pub const fn set_lptim_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Pcfr2 {
        #[inline(always)]
        fn default() -> Pcfr2 {
            Pcfr2(0)
        }
    }
    impl core::fmt::Debug for Pcfr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcfr2")
                .field("usart4_rm", &self.usart4_rm())
                .field("usart2_rm_h", &self.usart2_rm_h())
                .field("usart1_rm_h", &self.usart1_rm_h())
                .field("tim2_rm_h", &self.tim2_rm_h())
                .field("tim1_rm_h", &self.tim1_rm_h())
                .field("i2c1_rm_h", &self.i2c1_rm_h())
                .field("spi1_rm_h", &self.spi1_rm_h())
                .field("lptim_rm", &self.lptim_rm())
                .finish()
        }
    }
}
