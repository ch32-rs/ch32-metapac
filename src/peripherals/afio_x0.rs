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
    #[doc = "External interrupt configuration register (AFIO_EXTICRx)."]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "CTLR."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "CTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "UDM_PUE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_pue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "UDM_PUE configuration."]
        #[inline(always)]
        pub const fn set_udm_pue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "UDP_PUE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pue(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "UDP_PUE configuration."]
        #[inline(always)]
        pub const fn set_udp_pue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "USB_PHY_V33 configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_phy_v33(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY_V33 configuration."]
        #[inline(always)]
        pub const fn set_usb_phy_v33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB_IOEN configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_ioen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB_IOEN configuration."]
        #[inline(always)]
        pub const fn set_usb_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USBPD_PHY_V33 configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_phy_v33(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD_PHY_V33 configuration."]
        #[inline(always)]
        pub const fn set_usbpd_phy_v33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USBPD_IN_HVT configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_in_hvt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD_IN_HVT configuration."]
        #[inline(always)]
        pub const fn set_usbpd_in_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UDP_BC_VSRC configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "UDP_BC_VSRC configuration."]
        #[inline(always)]
        pub const fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "UDM_BC_VSRC configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "UDM_BC_VSRC configuration."]
        #[inline(always)]
        pub const fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "UDP_BC_CMPO configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "UDP_BC_CMPO configuration."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UDM_BC_CMPO configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UDM_BC_CMPO configuration."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PA3_FILT_EN configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pa3_filt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_FILT_EN configuration."]
        #[inline(always)]
        pub const fn set_pa3_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PA4_FILT_EN configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pa4_filt_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_FILT_EN configuration."]
        #[inline(always)]
        pub const fn set_pa4_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB5_FILT_EN configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pb5_filt_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_FILT_EN configuration."]
        #[inline(always)]
        pub const fn set_pb5_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB6_FILT_EN configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pb6_filt_en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_FILT_EN configuration."]
        #[inline(always)]
        pub const fn set_pb6_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("udm_pue", &self.udm_pue())
                .field("udp_pue", &self.udp_pue())
                .field("usb_phy_v33", &self.usb_phy_v33())
                .field("usb_ioen", &self.usb_ioen())
                .field("usbpd_phy_v33", &self.usbpd_phy_v33())
                .field("usbpd_in_hvt", &self.usbpd_in_hvt())
                .field("udp_bc_vsrc", &self.udp_bc_vsrc())
                .field("udm_bc_vsrc", &self.udm_bc_vsrc())
                .field("udp_bc_cmpo", &self.udp_bc_cmpo())
                .field("udm_bc_cmpo", &self.udm_bc_cmpo())
                .field("pa3_filt_en", &self.pa3_filt_en())
                .field("pa4_filt_en", &self.pa4_filt_en())
                .field("pb5_filt_en", &self.pb5_filt_en())
                .field("pb6_filt_en", &self.pb6_filt_en())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register (AFIO_EXTICRx)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "EXTI configuration."]
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
        #[doc = "SPI1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_rm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SPI1 remapping."]
        #[inline(always)]
        pub const fn set_spi1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "I2C1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_rm(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "I2C1 remapping."]
        #[inline(always)]
        pub const fn set_i2c1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "USART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_rm(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "USART1 remapping."]
        #[inline(always)]
        pub const fn set_usart1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "USART2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_rm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "USART2 remapping."]
        #[inline(always)]
        pub const fn set_usart2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "USART3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 remapping."]
        #[inline(always)]
        pub const fn set_usart3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "USART4 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4_rm(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "USART4 remapping."]
        #[inline(always)]
        pub const fn set_usart4_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "TIM1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_rm(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "TIM1 remapping."]
        #[inline(always)]
        pub const fn set_tim1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "TIM2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_rm(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "TIM2 remapping."]
        #[inline(always)]
        pub const fn set_tim2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "TIM3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_rm(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "TIM3 remapping."]
        #[inline(always)]
        pub const fn set_tim3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "IO2W__RM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn pioc_rm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IO2W__RM remapping."]
        #[inline(always)]
        pub const fn set_pioc_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SW_CFG Configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "SW_CFG Configuration."]
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
                .field("usart4_rm", &self.usart4_rm())
                .field("tim1_rm", &self.tim1_rm())
                .field("tim2_rm", &self.tim2_rm())
                .field("tim3_rm", &self.tim3_rm())
                .field("pioc_rm", &self.pioc_rm())
                .field("sw_cfg", &self.sw_cfg())
                .finish()
        }
    }
}
