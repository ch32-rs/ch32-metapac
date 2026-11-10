#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Channel cluster: CFGR, CNTR, PADDR, MADDR and M1ADDR registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA channel configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA channel number of data register."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA channel peripheral address register."]
    #[inline(always)]
    pub const fn paddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA channel memory address register."]
    #[inline(always)]
    pub const fn maddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DMA channel second memory address register."]
    #[inline(always)]
    pub const fn m1addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "DMA1 controller (11 channels)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1 {
    ptr: *mut u8,
}
unsafe impl Send for Dma1 {}
unsafe impl Sync for Dma1 {}
impl Dma1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA interrupt status register (DMA_INTFR)."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA interrupt flag clear register (DMA_INTFCR)."]
    #[inline(always)]
    pub const fn intfcr(self) -> crate::common::Reg<regs::Intfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CFGR, CNTR, PADDR, MADDR and M1ADDR registers."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 11usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x08usize + n * 20usize) as _) }
    }
    #[doc = "DMA extends the interrupt status register."]
    #[inline(always)]
    pub const fn extem_intfr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "DMA extends the interrupt flag clear register."]
    #[inline(always)]
    pub const fn extem_intfcr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA channel configuration register (DMA_CFGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Channel enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Half Transfer interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Half Transfer interrupt enable."]
        #[inline(always)]
        pub const fn set_htie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transfer error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data transfer direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Circular mode."]
        #[must_use]
        #[inline(always)]
        pub const fn circ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Circular mode."]
        #[inline(always)]
        pub const fn set_circ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral increment mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pinc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral increment mode."]
        #[inline(always)]
        pub const fn set_pinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Memory increment mode."]
        #[must_use]
        #[inline(always)]
        pub const fn minc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Memory increment mode."]
        #[inline(always)]
        pub const fn set_minc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Peripheral size."]
        #[must_use]
        #[inline(always)]
        pub const fn psize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Peripheral size."]
        #[inline(always)]
        pub const fn set_psize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Memory size."]
        #[must_use]
        #[inline(always)]
        pub const fn msize(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Memory size."]
        #[inline(always)]
        pub const fn set_msize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Channel Priority level."]
        #[must_use]
        #[inline(always)]
        pub const fn pl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Channel Priority level."]
        #[inline(always)]
        pub const fn set_pl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Memory to memory mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mem2mem(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory mode."]
        #[inline(always)]
        pub const fn set_mem2mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Double buffer mode enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn double_mode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode enable bit."]
        #[inline(always)]
        pub const fn set_double_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Memory address selection setting."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_cur_mem(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Memory address selection setting."]
        #[inline(always)]
        pub const fn set_flag_cur_mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("en", &self.en())
                .field("tcie", &self.tcie())
                .field("htie", &self.htie())
                .field("teie", &self.teie())
                .field("dir", &self.dir())
                .field("circ", &self.circ())
                .field("pinc", &self.pinc())
                .field("minc", &self.minc())
                .field("psize", &self.psize())
                .field("msize", &self.msize())
                .field("pl", &self.pl())
                .field("mem2mem", &self.mem2mem())
                .field("double_mode", &self.double_mode())
                .field("flag_cur_mem", &self.flag_cur_mem())
                .finish()
        }
    }
    #[doc = "DMA channel 1 number of data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "Number of data to transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn ndt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data to transfer."]
        #[inline(always)]
        pub const fn set_ndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr").field("ndt", &self.ndt()).finish()
        }
    }
    #[doc = "DMA extends the interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtemIntfr(pub u32);
    impl ExtemIntfr {
        #[doc = "Channel 9 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif9(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 9 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 9 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 9 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel 9 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif9(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 9 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel 9 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif9(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 9 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel 10 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif10(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 10 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel 10 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif10(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 10 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 10 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif10(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 10 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel 10 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif10(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 10 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel 11 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif11(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 11 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel 11 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif11(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 11 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel 11 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif11(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 11 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel 11 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 11 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for ExtemIntfr {
        #[inline(always)]
        fn default() -> ExtemIntfr {
            ExtemIntfr(0)
        }
    }
    impl core::fmt::Debug for ExtemIntfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtemIntfr")
                .field("gif9", &self.gif9())
                .field("tcif9", &self.tcif9())
                .field("htif9", &self.htif9())
                .field("teif9", &self.teif9())
                .field("gif10", &self.gif10())
                .field("tcif10", &self.tcif10())
                .field("htif10", &self.htif10())
                .field("teif10", &self.teif10())
                .field("gif11", &self.gif11())
                .field("tcif11", &self.tcif11())
                .field("htif11", &self.htif11())
                .field("teif11", &self.teif11())
                .finish()
        }
    }
    #[doc = "DMA interrupt status register (DMA_INTFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Channel 1 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 1 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel 1 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel 1 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel 2 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 2 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel 2 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 2 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 2 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 2 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel 2 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 2 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel 3 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 3 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel 3 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 3 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel 3 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 3 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel 3 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 3 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel 4 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 4 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Channel 4 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 4 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Channel 4 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 4 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Channel 4 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 4 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Channel 5 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 5 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Channel 5 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif5(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 5 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Channel 5 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif5(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 5 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Channel 5 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif5(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 5 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Channel 6 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif6(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 6 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Channel 6 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif6(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 6 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Channel 6 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 6 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Channel 6 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif6(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 6 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Channel 7 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif7(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 7 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Channel 7 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif7(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 7 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Channel 7 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif7(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 7 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Channel 7 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif7(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 7 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Channel 8 Global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif8(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 8 Global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Channel 8 Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif8(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 8 Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_tcif8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Channel 8 Half Transfer Complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif8(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 8 Half Transfer Complete flag."]
        #[inline(always)]
        pub const fn set_htif8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Channel 8 Transfer Error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif8(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 8 Transfer Error flag."]
        #[inline(always)]
        pub const fn set_teif8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Intfr {
        #[inline(always)]
        fn default() -> Intfr {
            Intfr(0)
        }
    }
    impl core::fmt::Debug for Intfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intfr")
                .field("gif1", &self.gif1())
                .field("tcif1", &self.tcif1())
                .field("htif1", &self.htif1())
                .field("teif1", &self.teif1())
                .field("gif2", &self.gif2())
                .field("tcif2", &self.tcif2())
                .field("htif2", &self.htif2())
                .field("teif2", &self.teif2())
                .field("gif3", &self.gif3())
                .field("tcif3", &self.tcif3())
                .field("htif3", &self.htif3())
                .field("teif3", &self.teif3())
                .field("gif4", &self.gif4())
                .field("tcif4", &self.tcif4())
                .field("htif4", &self.htif4())
                .field("teif4", &self.teif4())
                .field("gif5", &self.gif5())
                .field("tcif5", &self.tcif5())
                .field("htif5", &self.htif5())
                .field("teif5", &self.teif5())
                .field("gif6", &self.gif6())
                .field("tcif6", &self.tcif6())
                .field("htif6", &self.htif6())
                .field("teif6", &self.teif6())
                .field("gif7", &self.gif7())
                .field("tcif7", &self.tcif7())
                .field("htif7", &self.htif7())
                .field("teif7", &self.teif7())
                .field("gif8", &self.gif8())
                .field("tcif8", &self.tcif8())
                .field("htif8", &self.htif8())
                .field("teif8", &self.teif8())
                .finish()
        }
    }
}
