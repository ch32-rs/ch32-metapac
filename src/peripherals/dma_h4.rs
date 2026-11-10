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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel number-of-data register (DMA_CNTR)."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel peripheral address register (DMA_PADDR)."]
    #[inline(always)]
    pub const fn paddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel memory address register (DMA_MADDR, also used as memory0 in double-buffer mode)."]
    #[inline(always)]
    pub const fn maddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel second memory address (DMA_M1ADDR, double-buffer mode only)."]
    #[inline(always)]
    pub const fn m1addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt flag clear register."]
    #[inline(always)]
    pub const fn intfcr(self) -> crate::common::Reg<regs::Intfcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CFGR / CNTR / PADDR / MADDR / M1ADDR. Stride 20 bytes per channel."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x08usize + n * 20usize) as _) }
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
    #[doc = "DMA interrupt flag clear register (DMA_INTFCR). Per-channel clear bits (write 1 to clear)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfcr(pub u32);
    impl Intfcr {
        #[doc = "Channel global interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cgif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt clear."]
        #[inline(always)]
        pub const fn set_cgif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete clear."]
        #[must_use]
        #[inline(always)]
        pub const fn ctcif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete clear."]
        #[inline(always)]
        pub const fn set_ctcif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half-transfer complete clear."]
        #[must_use]
        #[inline(always)]
        pub const fn chtif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half-transfer complete clear."]
        #[inline(always)]
        pub const fn set_chtif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cteif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error clear."]
        #[inline(always)]
        pub const fn set_cteif(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for Intfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intfcr")
                .field("cgif[0]", &self.cgif(0usize))
                .field("cgif[1]", &self.cgif(1usize))
                .field("cgif[2]", &self.cgif(2usize))
                .field("cgif[3]", &self.cgif(3usize))
                .field("cgif[4]", &self.cgif(4usize))
                .field("cgif[5]", &self.cgif(5usize))
                .field("cgif[6]", &self.cgif(6usize))
                .field("cgif[7]", &self.cgif(7usize))
                .field("ctcif[0]", &self.ctcif(0usize))
                .field("ctcif[1]", &self.ctcif(1usize))
                .field("ctcif[2]", &self.ctcif(2usize))
                .field("ctcif[3]", &self.ctcif(3usize))
                .field("ctcif[4]", &self.ctcif(4usize))
                .field("ctcif[5]", &self.ctcif(5usize))
                .field("ctcif[6]", &self.ctcif(6usize))
                .field("ctcif[7]", &self.ctcif(7usize))
                .field("chtif[0]", &self.chtif(0usize))
                .field("chtif[1]", &self.chtif(1usize))
                .field("chtif[2]", &self.chtif(2usize))
                .field("chtif[3]", &self.chtif(3usize))
                .field("chtif[4]", &self.chtif(4usize))
                .field("chtif[5]", &self.chtif(5usize))
                .field("chtif[6]", &self.chtif(6usize))
                .field("chtif[7]", &self.chtif(7usize))
                .field("cteif[0]", &self.cteif(0usize))
                .field("cteif[1]", &self.cteif(1usize))
                .field("cteif[2]", &self.cteif(2usize))
                .field("cteif[3]", &self.cteif(3usize))
                .field("cteif[4]", &self.cteif(4usize))
                .field("cteif[5]", &self.cteif(5usize))
                .field("cteif[6]", &self.cteif(6usize))
                .field("cteif[7]", &self.cteif(7usize))
                .finish()
        }
    }
    #[doc = "DMA interrupt status register (DMA_INTFR). Per-channel global / TC / HT / TE flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Channel global interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub const fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub const fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half-transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half-transfer complete flag."]
        #[inline(always)]
        pub const fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub const fn set_teif(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for Intfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intfr")
                .field("gif[0]", &self.gif(0usize))
                .field("gif[1]", &self.gif(1usize))
                .field("gif[2]", &self.gif(2usize))
                .field("gif[3]", &self.gif(3usize))
                .field("gif[4]", &self.gif(4usize))
                .field("gif[5]", &self.gif(5usize))
                .field("gif[6]", &self.gif(6usize))
                .field("gif[7]", &self.gif(7usize))
                .field("tcif[0]", &self.tcif(0usize))
                .field("tcif[1]", &self.tcif(1usize))
                .field("tcif[2]", &self.tcif(2usize))
                .field("tcif[3]", &self.tcif(3usize))
                .field("tcif[4]", &self.tcif(4usize))
                .field("tcif[5]", &self.tcif(5usize))
                .field("tcif[6]", &self.tcif(6usize))
                .field("tcif[7]", &self.tcif(7usize))
                .field("htif[0]", &self.htif(0usize))
                .field("htif[1]", &self.htif(1usize))
                .field("htif[2]", &self.htif(2usize))
                .field("htif[3]", &self.htif(3usize))
                .field("htif[4]", &self.htif(4usize))
                .field("htif[5]", &self.htif(5usize))
                .field("htif[6]", &self.htif(6usize))
                .field("htif[7]", &self.htif(7usize))
                .field("teif[0]", &self.teif(0usize))
                .field("teif[1]", &self.teif(1usize))
                .field("teif[2]", &self.teif(2usize))
                .field("teif[3]", &self.teif(3usize))
                .field("teif[4]", &self.teif(4usize))
                .field("teif[5]", &self.teif(5usize))
                .field("teif[6]", &self.teif(6usize))
                .field("teif[7]", &self.teif(7usize))
                .finish()
        }
    }
}
