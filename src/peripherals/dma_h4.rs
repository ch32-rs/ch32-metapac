#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA channel cluster (CFGR / CNTR / PADDR / MADDR / M1ADDR). M1ADDR holds the second memory address when double-buffer mode is enabled."]
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
    #[doc = "Channel configuration register (DMA_CFGR)."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Channel number-of-data register (DMA_CNTR)."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel peripheral address register (DMA_PADDR)."]
    #[inline(always)]
    pub const fn paddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Channel memory address register (DMA_MADDR, also used as memory0 in double-buffer mode)."]
    #[inline(always)]
    pub const fn maddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Channel second memory address (DMA_M1ADDR, double-buffer mode only)."]
    #[inline(always)]
    pub const fn m1addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
#[doc = "DMA controller (8 channels per instance). H4 introduces double-buffer mode via M1ADDR and routes requests via DMAMUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn intfcr(self) -> crate::common::Reg<regs::Intfcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CFGR / CNTR / PADDR / MADDR / M1ADDR. Stride 20 bytes per channel."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x08usize + n * 20usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA channel configuration register (DMA_CFGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Channel enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Half Transfer interrupt enable."]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Half Transfer interrupt enable."]
        #[inline(always)]
        pub fn set_htie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transfer error interrupt enable."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data transfer direction."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Circular mode."]
        #[inline(always)]
        pub const fn circ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Circular mode."]
        #[inline(always)]
        pub fn set_circ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral increment mode."]
        #[inline(always)]
        pub const fn pinc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral increment mode."]
        #[inline(always)]
        pub fn set_pinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Memory increment mode."]
        #[inline(always)]
        pub const fn minc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Memory increment mode."]
        #[inline(always)]
        pub fn set_minc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Peripheral size."]
        #[inline(always)]
        pub const fn psize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Peripheral size."]
        #[inline(always)]
        pub fn set_psize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Memory size."]
        #[inline(always)]
        pub const fn msize(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Memory size."]
        #[inline(always)]
        pub fn set_msize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Channel Priority level."]
        #[inline(always)]
        pub const fn pl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Channel Priority level."]
        #[inline(always)]
        pub fn set_pl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Memory to memory mode."]
        #[inline(always)]
        pub const fn mem2mem(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory mode."]
        #[inline(always)]
        pub fn set_mem2mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Double buffer mode enable bit."]
        #[inline(always)]
        pub const fn double_mode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode enable bit."]
        #[inline(always)]
        pub fn set_double_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Memory address selection setting."]
        #[inline(always)]
        pub const fn flag_cur_mem(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Memory address selection setting."]
        #[inline(always)]
        pub fn set_flag_cur_mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "DMA channel 1 number of data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "Number of data to transfer."]
        #[inline(always)]
        pub const fn ndt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data to transfer."]
        #[inline(always)]
        pub fn set_ndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    #[doc = "DMA interrupt flag clear register (DMA_INTFCR). Per-channel clear bits (write 1 to clear)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfcr(pub u32);
    impl Intfcr {
        #[doc = "Channel global interrupt clear."]
        #[inline(always)]
        pub const fn cgif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt clear."]
        #[inline(always)]
        pub fn set_cgif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete clear."]
        #[inline(always)]
        pub const fn ctcif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete clear."]
        #[inline(always)]
        pub fn set_ctcif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half-transfer complete clear."]
        #[inline(always)]
        pub const fn chtif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half-transfer complete clear."]
        #[inline(always)]
        pub fn set_chtif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error clear."]
        #[inline(always)]
        pub const fn cteif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error clear."]
        #[inline(always)]
        pub fn set_cteif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intfcr {
        #[inline(always)]
        fn default() -> Intfcr {
            Intfcr(0)
        }
    }
    #[doc = "DMA interrupt status register (DMA_INTFR). Per-channel global / TC / HT / TE flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half-transfer complete flag."]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half-transfer complete flag."]
        #[inline(always)]
        pub fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub fn set_teif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intfr {
        #[inline(always)]
        fn default() -> Intfr {
            Intfr(0)
        }
    }
}
