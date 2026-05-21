#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "High speed ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsadc {
    ptr: *mut u8,
}
unsafe impl Send for Hsadc {}
unsafe impl Sync for Hsadc {}
impl Hsadc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "High-speed ADC configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "HSADC Control Register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "HSADC Control Register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "HSADC Status Register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "HSADC Data Register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "HSADC DMA Receive Address Register 0."]
    #[inline(always)]
    pub const fn addr0(self) -> crate::common::Reg<regs::Addr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "HSADC DMA Receive Address Register 1."]
    #[inline(always)]
    pub const fn addr1(self) -> crate::common::Reg<regs::Addr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSADC DMA Receive Address Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr0(pub u32);
    impl Addr0 {
        #[doc = "The DMA transmission address is configured with the 0 configuration bit."]
        #[inline(always)]
        pub const fn dma_addr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The DMA transmission address is configured with the 0 configuration bit."]
        #[inline(always)]
        pub fn set_dma_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addr0 {
        #[inline(always)]
        fn default() -> Addr0 {
            Addr0(0)
        }
    }
    #[doc = "HSADC DMA Receive Address Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr1(pub u32);
    impl Addr1 {
        #[doc = "DMA transport address 1 configuration bit."]
        #[inline(always)]
        pub const fn dma_addr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DMA transport address 1 configuration bit."]
        #[inline(always)]
        pub fn set_dma_addr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addr1 {
        #[inline(always)]
        fn default() -> Addr1 {
            Addr1(0)
        }
    }
    #[doc = "High-speed ADC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "High-speed ADC enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed ADC enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct Storage Access (DMA) mode enable."]
        #[inline(always)]
        pub const fn dmaen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Direct Storage Access (DMA) mode enable."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "First Transition Establishment Time Configuration Bit."]
        #[inline(always)]
        pub const fn setup(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "First Transition Establishment Time Configuration Bit."]
        #[inline(always)]
        pub fn set_setup(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Ping Pong storage mode enable."]
        #[inline(always)]
        pub const fn ppmode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ping Pong storage mode enable."]
        #[inline(always)]
        pub fn set_ppmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DMA transmission length configuration bit."]
        #[inline(always)]
        pub const fn burst_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transmission length configuration bit."]
        #[inline(always)]
        pub fn set_burst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DMA transmission length configuration bit."]
        #[inline(always)]
        pub const fn dma_len(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA transmission length configuration bit."]
        #[inline(always)]
        pub fn set_dma_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "HSADC Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Initiating High-Speed ADC Conversion."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initiating High-Speed ADC Conversion."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Abort a burst transmission."]
        #[inline(always)]
        pub const fn burst_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Abort a burst transmission."]
        #[inline(always)]
        pub fn set_burst_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transition Completion Interrupt Enables."]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transition Completion Interrupt Enables."]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transition Completion Interrupt Enables."]
        #[inline(always)]
        pub const fn dmaie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transition Completion Interrupt Enables."]
        #[inline(always)]
        pub fn set_dmaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Interrupt enables when the burst transmission is complete."]
        #[inline(always)]
        pub const fn burstie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enables when the burst transmission is complete."]
        #[inline(always)]
        pub fn set_burstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    #[doc = "HSADC Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "The final DMA transmission length configuration bit for burst transmission.. If the number of data transmitted in the final burst is not aligned with the DMA transmission length, the DMA transmission length is configured."]
        #[inline(always)]
        pub const fn burst_dma_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The final DMA transmission length configuration bit for burst transmission.. If the number of data transmitted in the final burst is not aligned with the DMA transmission length, the DMA transmission length is configured."]
        #[inline(always)]
        pub fn set_burst_dma_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Burst transmission length configuration."]
        #[inline(always)]
        pub const fn burst_len(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Burst transmission length configuration."]
        #[inline(always)]
        pub fn set_burst_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    #[doc = "HSADC Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "Convert data register."]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Convert data register."]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "HSADC Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Transition Complete Interrupt flag."]
        #[inline(always)]
        pub const fn eocif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transition Complete Interrupt flag."]
        #[inline(always)]
        pub fn set_eocif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Transmission Complete Interrupt flag."]
        #[inline(always)]
        pub const fn dmaif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Transmission Complete Interrupt flag."]
        #[inline(always)]
        pub fn set_dmaif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Burst transfer completed interrupt flag."]
        #[inline(always)]
        pub const fn burstif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Burst transfer completed interrupt flag."]
        #[inline(always)]
        pub fn set_burstif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The data register is not empty flag, that is, the conversion is completed. and the data is stored in the data register, which is the position bit; A read operation on the data register clears the bit."]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The data register is not empty flag, that is, the conversion is completed. and the data is stored in the data register, which is the position bit; A read operation on the data register clears the bit."]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Ping-pong storage mode cache address indicator bit."]
        #[inline(always)]
        pub const fn pp_addr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Ping-pong storage mode cache address indicator bit."]
        #[inline(always)]
        pub fn set_pp_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receives a FIFO non-null status flag."]
        #[inline(always)]
        pub const fn fifo_rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receives a FIFO non-null status flag."]
        #[inline(always)]
        pub fn set_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive FIFO full status flags."]
        #[inline(always)]
        pub const fn fifo_full(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO full status flags."]
        #[inline(always)]
        pub fn set_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receives the FIFO overflow status flag."]
        #[inline(always)]
        pub const fn fifo_ov(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Receives the FIFO overflow status flag."]
        #[inline(always)]
        pub fn set_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receives the FIFO current count value."]
        #[inline(always)]
        pub const fn fifo_cnt(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "Receives the FIFO current count value."]
        #[inline(always)]
        pub fn set_fifo_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
}
