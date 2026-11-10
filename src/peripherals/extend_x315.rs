#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extended configuration."]
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
    #[doc = "Configuration extension control register."]
    #[inline(always)]
    pub const fn exten_ctr(self) -> crate::common::Reg<regs::ExtenCtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "System control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtr(pub u32);
    impl ExtenCtr {
        #[doc = "SPI1 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_dma_rm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_spi1_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SPI2 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2_dma_rm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_spi2_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3_dma_rm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_spi3_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART1 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_dma_rm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_usart1_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART2 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_dma_rm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_usart2_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART3 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3_dma_rm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_usart3_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART4 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4_dma_rm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_usart4_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C2 DMA channel mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2_dma_rm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 DMA channel mapping."]
        #[inline(always)]
        pub const fn set_i2c2_dma_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan cycle counter output ADC selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_scan_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Scan cycle counter output ADC selection."]
        #[inline(always)]
        pub const fn set_adc_scan_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ExtenCtr {
        #[inline(always)]
        fn default() -> ExtenCtr {
            ExtenCtr(0)
        }
    }
    impl core::fmt::Debug for ExtenCtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtr")
                .field("spi1_dma_rm", &self.spi1_dma_rm())
                .field("spi2_dma_rm", &self.spi2_dma_rm())
                .field("spi3_dma_rm", &self.spi3_dma_rm())
                .field("usart1_dma_rm", &self.usart1_dma_rm())
                .field("usart2_dma_rm", &self.usart2_dma_rm())
                .field("usart3_dma_rm", &self.usart3_dma_rm())
                .field("usart4_dma_rm", &self.usart4_dma_rm())
                .field("i2c2_dma_rm", &self.i2c2_dma_rm())
                .field("adc_scan_sel", &self.adc_scan_sel())
                .finish()
        }
    }
}
