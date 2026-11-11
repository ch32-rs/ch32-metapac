#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Exten configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extend {
    ptr: *mut u8,
}
unsafe impl Send for Extend {}
unsafe impl Sync for Extend {}
impl Extend {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EXTEN control register 0."]
    #[inline(always)]
    pub const fn ctr0(self) -> crate::common::Reg<regs::ExtenCtlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "EXTEN control register 1."]
    #[inline(always)]
    pub const fn ctr1(self) -> crate::common::Reg<regs::ExtenCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTEN control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr0(pub u32);
    impl ExtenCtlr0 {
        #[doc = "Whether HSI is divided."]
        #[must_use]
        #[inline(always)]
        pub const fn hsipre(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Whether HSI is divided."]
        #[inline(always)]
        pub const fn set_hsipre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LOCKUP_Eable."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP_Eable."]
        #[inline(always)]
        pub const fn set_lkupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LOCKUP RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn lkuprst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP RESET."]
        #[inline(always)]
        pub const fn set_lkuprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULLDOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ulldotrim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "ULLDOTRIM."]
        #[inline(always)]
        pub const fn set_ulldotrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "LDOTRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn ldotrim(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "LDOTRIM."]
        #[inline(always)]
        pub const fn set_ldotrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for ExtenCtlr0 {
        #[inline(always)]
        fn default() -> ExtenCtlr0 {
            ExtenCtlr0(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr0")
                .field("hsipre", &self.hsipre())
                .field("lkupen", &self.lkupen())
                .field("lkuprst", &self.lkuprst())
                .field("ulldotrim", &self.ulldotrim())
                .field("ldotrim", &self.ldotrim())
                .finish()
        }
    }
    #[doc = "EXTEN control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr1(pub u16);
    impl ExtenCtlr1 {
        #[doc = "Mapping of ADC on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_adc_rm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of ADC on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_adc_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Mapping of SPI1 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_spi1_rm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of SPI1 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_spi1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Mapping of SPI2 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_spi2_rm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of SPI2 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_spi2_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Mapping of TIM1_UP on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim1_up_rm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM1_UP on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim1_up_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Mapping of TIM2_CH4 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim2_ch4_rm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM2_CH4 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim2_ch4_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Mapping of TIM2_UP on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim2_up_rm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM2_UP on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim2_up_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Mapping of TIM3_CH1 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim3_ch1_rm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM3_CH1 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim3_ch1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Mapping of TIM3_CH3 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim3_ch3_rm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM3_CH3 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim3_ch3_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Mapping of TIM3_CH4 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim3_ch4_rm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM3_CH4 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim3_ch4_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Mapping of TIM4_CH1 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim4_ch1_rm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM4_CH1 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim4_ch1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Mapping of IM4_CH3 on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim4_ch3_rm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of IM4_CH3 on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim4_ch3_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Mapping of TIM4_UP on the DMA channel."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_tim4_up_rm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Mapping of TIM4_UP on the DMA channel."]
        #[inline(always)]
        pub const fn set_dma_tim4_up_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for ExtenCtlr1 {
        #[inline(always)]
        fn default() -> ExtenCtlr1 {
            ExtenCtlr1(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr1")
                .field("dma_adc_rm", &self.dma_adc_rm())
                .field("dma_spi1_rm", &self.dma_spi1_rm())
                .field("dma_spi2_rm", &self.dma_spi2_rm())
                .field("dma_tim1_up_rm", &self.dma_tim1_up_rm())
                .field("dma_tim2_ch4_rm", &self.dma_tim2_ch4_rm())
                .field("dma_tim2_up_rm", &self.dma_tim2_up_rm())
                .field("dma_tim3_ch1_rm", &self.dma_tim3_ch1_rm())
                .field("dma_tim3_ch3_rm", &self.dma_tim3_ch3_rm())
                .field("dma_tim3_ch4_rm", &self.dma_tim3_ch4_rm())
                .field("dma_tim4_ch1_rm", &self.dma_tim4_ch1_rm())
                .field("dma_tim4_ch3_rm", &self.dma_tim4_ch3_rm())
                .field("dma_tim4_up_rm", &self.dma_tim4_up_rm())
                .finish()
        }
    }
}
