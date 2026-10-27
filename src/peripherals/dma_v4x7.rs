#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Channel cluster: CFGR, CNTR, PADDR and MADDR registers."]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA channel number of data register."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA channel peripheral address register."]
    #[inline(always)]
    pub const fn paddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA channel memory address register."]
    #[inline(always)]
    pub const fn maddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "DMA1 controller (13 channels)."]
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
    #[doc = "DMA1 interrupt status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA1 interrupt flag clear register."]
    #[inline(always)]
    pub const fn intfcr(self) -> crate::common::Reg<regs::Intfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CFGR, CNTR, PADDR and MADDR registers."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 7usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x08usize + n * 20usize) as _) }
    }
    #[doc = "Channel cluster: CFGR, CNTR, PADDR and MADDR registers."]
    #[inline(always)]
    pub const fn ech(self, n: usize) -> Ch {
        assert!(n < 6usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x90usize + n * 16usize) as _) }
    }
    #[doc = "DMA1 extended interrupt status register."]
    #[inline(always)]
    pub const fn extem_intfr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "DMA1 extended interrupt flag clear register."]
    #[inline(always)]
    pub const fn extem_intfcr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
}
#[doc = "DMA2 controller (11 channels)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2 {
    ptr: *mut u8,
}
unsafe impl Send for Dma2 {}
unsafe impl Sync for Dma2 {}
impl Dma2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA2 interrupt status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA2 interrupt flag clear register."]
    #[inline(always)]
    pub const fn intfcr(self) -> crate::common::Reg<regs::Intfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel cluster: CFGR, CNTR, PADDR and MADDR registers."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 7usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x08usize + n * 20usize) as _) }
    }
    #[doc = "Channel cluster: CFGR, CNTR, PADDR and MADDR registers."]
    #[inline(always)]
    pub const fn ech(self, n: usize) -> Ch {
        assert!(n < 4usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x90usize + n * 16usize) as _) }
    }
    #[doc = "DMA2 extended interrupt status register."]
    #[inline(always)]
    pub const fn extem_intfr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "DMA2 extended interrupt flag clear register."]
    #[inline(always)]
    pub const fn extem_intfcr(self) -> crate::common::Reg<regs::ExtemIntfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA channel configuration register."]
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
        #[doc = "Half transfer interrupt enable."]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Half transfer interrupt enable."]
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
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Data transfer direction."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
        pub const fn psize(&self) -> super::vals::Size {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Peripheral size."]
        #[inline(always)]
        pub fn set_psize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Memory size."]
        #[inline(always)]
        pub const fn msize(&self) -> super::vals::Size {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Memory size."]
        #[inline(always)]
        pub fn set_msize(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Channel priority level."]
        #[inline(always)]
        pub const fn pl(&self) -> super::vals::Pl {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pl::from_bits(val as u8)
        }
        #[doc = "Channel priority level."]
        #[inline(always)]
        pub fn set_pl(&mut self, val: super::vals::Pl) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Memory to memory mode enable."]
        #[inline(always)]
        pub const fn mem2mem(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory mode enable."]
        #[inline(always)]
        pub fn set_mem2mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "DMA channel number of data register."]
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
    #[doc = "DMA extended interrupt status register, channels 8..13. Index 0 is channel 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtemIntfr(pub u32);
    impl ExtemIntfr {
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half transfer complete flag."]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half transfer complete flag."]
        #[inline(always)]
        pub fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub fn set_teif(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ExtemIntfr {
        #[inline(always)]
        fn default() -> ExtemIntfr {
            ExtemIntfr(0)
        }
    }
    #[doc = "DMA interrupt status register, channels 1..7. Index 0 is channel 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub const fn gif(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel global interrupt flag."]
        #[inline(always)]
        pub fn set_gif(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub const fn tcif(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer complete flag."]
        #[inline(always)]
        pub fn set_tcif(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel half transfer complete flag."]
        #[inline(always)]
        pub const fn htif(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel half transfer complete flag."]
        #[inline(always)]
        pub fn set_htif(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub const fn teif(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Channel transfer error flag."]
        #[inline(always)]
        pub fn set_teif(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
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
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "Read from peripheral."]
        FROMPERIPHERAL = 0x0,
        #[doc = "Read from memory."]
        FROMMEMORY = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pl {
        #[doc = "Low priority."]
        LOW = 0x0,
        #[doc = "Medium priority."]
        MEDIUM = 0x01,
        #[doc = "High priority."]
        HIGH = 0x02,
        #[doc = "Very high priority."]
        VERYHIGH = 0x03,
    }
    impl Pl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pl {
        #[inline(always)]
        fn from(val: u8) -> Pl {
            Pl::from_bits(val)
        }
    }
    impl From<Pl> for u8 {
        #[inline(always)]
        fn from(val: Pl) -> u8 {
            Pl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Size {
        #[doc = "8-bit size."]
        BITS8 = 0x0,
        #[doc = "16-bit size."]
        BITS16 = 0x01,
        #[doc = "32-bit size."]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Size {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Size {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Size {
        #[inline(always)]
        fn from(val: u8) -> Size {
            Size::from_bits(val)
        }
    }
    impl From<Size> for u8 {
        #[inline(always)]
        fn from(val: Size) -> u8 {
            Size::to_bits(val)
        }
    }
}
