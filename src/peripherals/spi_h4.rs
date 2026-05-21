#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Serial peripheral interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "data register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CRCR polynomial register."]
    #[inline(always)]
    pub const fn crcr(self) -> crate::common::Reg<regs::Crcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RX CRC register."]
    #[inline(always)]
    pub const fn rcrcr(self) -> crate::common::Reg<regs::Rcrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TX CRC register."]
    #[inline(always)]
    pub const fn tcrcr(self) -> crate::common::Reg<regs::Tcrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SPI_I2S configure register."]
    #[inline(always)]
    pub const fn i2scfgr(self) -> crate::common::Reg<regs::I2scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SPI_I2S prescaler register."]
    #[inline(always)]
    pub const fn i2spr(self) -> crate::common::Reg<regs::I2spr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "high speed control register."]
    #[inline(always)]
    pub const fn hscr(self) -> crate::common::Reg<regs::Hscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "CRCR polynomial register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcr(pub u32);
    impl Crcr {
        #[doc = "CRC polynomial register."]
        #[inline(always)]
        pub const fn crcpoly(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CRC polynomial register."]
        #[inline(always)]
        pub fn set_crcpoly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Crcr {
        #[inline(always)]
        fn default() -> Crcr {
            Crcr(0)
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Clock phase."]
        #[inline(always)]
        pub const fn cpha(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock phase."]
        #[inline(always)]
        pub fn set_cpha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock polarity."]
        #[inline(always)]
        pub const fn cpol(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock polarity."]
        #[inline(always)]
        pub fn set_cpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master selection."]
        #[inline(always)]
        pub const fn mstr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master selection."]
        #[inline(always)]
        pub fn set_mstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Baud rate control."]
        #[inline(always)]
        pub const fn br(&self) -> super::vals::BaudRate {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::BaudRate::from_bits(val as u8)
        }
        #[doc = "Baud rate control."]
        #[inline(always)]
        pub fn set_br(&mut self, val: super::vals::BaudRate) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "SPI enable."]
        #[inline(always)]
        pub const fn spe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI enable."]
        #[inline(always)]
        pub fn set_spe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Frame format."]
        #[inline(always)]
        pub const fn lsbfirst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Frame format."]
        #[inline(always)]
        pub fn set_lsbfirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Internal slave select."]
        #[inline(always)]
        pub const fn ssi(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Internal slave select."]
        #[inline(always)]
        pub fn set_ssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Software slave management."]
        #[inline(always)]
        pub const fn ssm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Software slave management."]
        #[inline(always)]
        pub fn set_ssm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive only."]
        #[inline(always)]
        pub const fn rxonly(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Receive only."]
        #[inline(always)]
        pub fn set_rxonly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data frame format."]
        #[inline(always)]
        pub const fn dff(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data frame format."]
        #[inline(always)]
        pub fn set_dff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CRC transfer next."]
        #[inline(always)]
        pub const fn crcnext(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC transfer next."]
        #[inline(always)]
        pub fn set_crcnext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hardware CRC calculation enable."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware CRC calculation enable."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Output enable in bidirectional mode."]
        #[inline(always)]
        pub const fn bidioe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Output enable in bidirectional mode."]
        #[inline(always)]
        pub fn set_bidioe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Bidirectional data mode enable."]
        #[inline(always)]
        pub const fn bidimode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Bidirectional data mode enable."]
        #[inline(always)]
        pub fn set_bidimode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "Rx buffer DMA enable."]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx buffer DMA enable."]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tx buffer DMA enable."]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx buffer DMA enable."]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SS output enable."]
        #[inline(always)]
        pub const fn ssoe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SS output enable."]
        #[inline(always)]
        pub fn set_ssoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX buffer not empty interrupt enable."]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX buffer not empty interrupt enable."]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Tx buffer empty interrupt enable."]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Tx buffer empty interrupt enable."]
        #[inline(always)]
        pub fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    #[doc = "data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "Data register."]
        #[inline(always)]
        pub const fn datar(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data register."]
        #[inline(always)]
        pub fn set_datar(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "high speed control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hscr(pub u32);
    impl Hscr {
        #[doc = "High speed mode read enable."]
        #[inline(always)]
        pub const fn hsrxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "High speed mode read enable."]
        #[inline(always)]
        pub fn set_hsrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High-speed receive enable 2 (CH32H4 only)."]
        #[inline(always)]
        pub const fn hsrxen2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed receive enable 2 (CH32H4 only)."]
        #[inline(always)]
        pub fn set_hsrxen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Hscr {
        #[inline(always)]
        fn default() -> Hscr {
            Hscr(0)
        }
    }
    #[doc = "SPI_I2S configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2scfgr(pub u32);
    impl I2scfgr {
        #[doc = "Channel length (number of bits per audio channel)."]
        #[inline(always)]
        pub const fn chlen(&self) -> super::vals::Chlen {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Chlen::from_bits(val as u8)
        }
        #[doc = "Channel length (number of bits per audio channel)."]
        #[inline(always)]
        pub fn set_chlen(&mut self, val: super::vals::Chlen) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "DATLEN\\[1:0\\]
bits (Data length to be transferred)."]
        #[inline(always)]
        pub const fn datlen(&self) -> super::vals::I2sdatlen {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::I2sdatlen::from_bits(val as u8)
        }
        #[doc = "DATLEN\\[1:0\\]
bits (Data length to be transferred)."]
        #[inline(always)]
        pub fn set_datlen(&mut self, val: super::vals::I2sdatlen) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "steady state clock polarity."]
        #[inline(always)]
        pub const fn ckpol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "steady state clock polarity."]
        #[inline(always)]
        pub fn set_ckpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I2SSTD\\[1:0\\]
bits (I2S standard selection)."]
        #[inline(always)]
        pub const fn i2sstd(&self) -> super::vals::I2sstd {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::I2sstd::from_bits(val as u8)
        }
        #[doc = "I2SSTD\\[1:0\\]
bits (I2S standard selection)."]
        #[inline(always)]
        pub fn set_i2sstd(&mut self, val: super::vals::I2sstd) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "PCM frame synchronization."]
        #[inline(always)]
        pub const fn pcmsync(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PCM frame synchronization."]
        #[inline(always)]
        pub fn set_pcmsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "I2SCFG\\[1:0\\]
bits (I2S configuration mode)."]
        #[inline(always)]
        pub const fn i2scfg(&self) -> super::vals::I2scfg {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2scfg::from_bits(val as u8)
        }
        #[doc = "I2SCFG\\[1:0\\]
bits (I2S configuration mode)."]
        #[inline(always)]
        pub fn set_i2scfg(&mut self, val: super::vals::I2scfg) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2S Enable."]
        #[inline(always)]
        pub const fn i2se(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2S Enable."]
        #[inline(always)]
        pub fn set_i2se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2S mode selection."]
        #[inline(always)]
        pub const fn i2smod(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2S mode selection."]
        #[inline(always)]
        pub fn set_i2smod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for I2scfgr {
        #[inline(always)]
        fn default() -> I2scfgr {
            I2scfgr(0)
        }
    }
    #[doc = "SPI_I2S prescaler register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2spr(pub u32);
    impl I2spr {
        #[doc = "I2SDIV\\[7:0\\]
bits (I2S Linear prescaler)."]
        #[inline(always)]
        pub const fn i2sdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "I2SDIV\\[7:0\\]
bits (I2S Linear prescaler)."]
        #[inline(always)]
        pub fn set_i2sdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Odd factor for the prescaler."]
        #[inline(always)]
        pub const fn odd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Odd factor for the prescaler."]
        #[inline(always)]
        pub fn set_odd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Master clock output enable."]
        #[inline(always)]
        pub const fn mckoe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock output enable."]
        #[inline(always)]
        pub fn set_mckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for I2spr {
        #[inline(always)]
        fn default() -> I2spr {
            I2spr(0)
        }
    }
    #[doc = "RX CRC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcrcr(pub u32);
    impl Rcrcr {
        #[doc = "Rx CRC register."]
        #[inline(always)]
        pub const fn rxcrc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Rx CRC register."]
        #[inline(always)]
        pub fn set_rxcrc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rcrcr {
        #[inline(always)]
        fn default() -> Rcrcr {
            Rcrcr(0)
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Receive buffer not empty."]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer not empty."]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit buffer empty."]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer empty."]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel side."]
        #[inline(always)]
        pub const fn chsid(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel side."]
        #[inline(always)]
        pub fn set_chsid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Underrun flag."]
        #[inline(always)]
        pub const fn udr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Underrun flag."]
        #[inline(always)]
        pub fn set_udr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error flag."]
        #[inline(always)]
        pub const fn crcerr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error flag."]
        #[inline(always)]
        pub fn set_crcerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Mode fault."]
        #[inline(always)]
        pub const fn modf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mode fault."]
        #[inline(always)]
        pub fn set_modf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Overrun flag."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    #[doc = "TX CRC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcrcr(pub u32);
    impl Tcrcr {
        #[doc = "Tx CRC register."]
        #[inline(always)]
        pub const fn txcrc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Tx CRC register."]
        #[inline(always)]
        pub fn set_txcrc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tcrcr {
        #[inline(always)]
        fn default() -> Tcrcr {
            Tcrcr(0)
        }
    }
}
pub mod vals {
    #[doc = "Baud rate control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BaudRate {
        #[doc = "fPCLK/2"]
        DIV_2 = 0x0,
        #[doc = "fPCLK/4"]
        DIV_4 = 0x01,
        #[doc = "fPCLK/8"]
        DIV_8 = 0x02,
        #[doc = "fPCLK/16"]
        DIV_16 = 0x03,
        #[doc = "fPCLK/32"]
        DIV_32 = 0x04,
        #[doc = "fPCLK/64"]
        DIV_64 = 0x05,
        #[doc = "fPCLK/128"]
        DIV_128 = 0x06,
        #[doc = "fPCLK/256"]
        DIV_256 = 0x07,
    }
    impl BaudRate {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BaudRate {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BaudRate {
        #[inline(always)]
        fn from(val: u8) -> BaudRate {
            BaudRate::from_bits(val)
        }
    }
    impl From<BaudRate> for u8 {
        #[inline(always)]
        fn from(val: BaudRate) -> u8 {
            BaudRate::to_bits(val)
        }
    }
    #[doc = "Channel length (number of bits per audio channel)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Chlen {
        #[doc = "16-bit data length."]
        BIT16 = 0x0,
        #[doc = "32-bit data length."]
        BIT32 = 0x01,
    }
    impl Chlen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chlen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chlen {
        #[inline(always)]
        fn from(val: u8) -> Chlen {
            Chlen::from_bits(val)
        }
    }
    impl From<Chlen> for u8 {
        #[inline(always)]
        fn from(val: Chlen) -> u8 {
            Chlen::to_bits(val)
        }
    }
    #[doc = "I2S mode selection"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2scfg {
        #[doc = "Slave transmit."]
        SLAVETX = 0x0,
        #[doc = "Slave receive."]
        SLAVERX = 0x01,
        #[doc = "Master transmit."]
        MASTERTX = 0x02,
        #[doc = "Master receive."]
        MASTERRX = 0x03,
    }
    impl I2scfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2scfg {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2scfg {
        #[inline(always)]
        fn from(val: u8) -> I2scfg {
            I2scfg::from_bits(val)
        }
    }
    impl From<I2scfg> for u8 {
        #[inline(always)]
        fn from(val: I2scfg) -> u8 {
            I2scfg::to_bits(val)
        }
    }
    #[doc = "Data length to be transferred."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2sdatlen {
        #[doc = "16-bit data length."]
        BIT16 = 0x0,
        #[doc = "24-bit data length."]
        BIT24 = 0x01,
        #[doc = "32-bit data length."]
        BIT32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2sdatlen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2sdatlen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2sdatlen {
        #[inline(always)]
        fn from(val: u8) -> I2sdatlen {
            I2sdatlen::from_bits(val)
        }
    }
    impl From<I2sdatlen> for u8 {
        #[inline(always)]
        fn from(val: I2sdatlen) -> u8 {
            I2sdatlen::to_bits(val)
        }
    }
    #[doc = "I2S standard selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2sstd {
        #[doc = "I2S Philips standard."]
        PHILIPS = 0x0,
        #[doc = "MSB justified standard, left justified."]
        MSB = 0x01,
        #[doc = "LSB justified standard, right justified."]
        LSB = 0x02,
        #[doc = "PCM standard."]
        PCM = 0x03,
    }
    impl I2sstd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2sstd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2sstd {
        #[inline(always)]
        fn from(val: u8) -> I2sstd {
            I2sstd::from_bits(val)
        }
    }
    impl From<I2sstd> for u8 {
        #[inline(always)]
        fn from(val: I2sstd) -> u8 {
            I2sstd::to_bits(val)
        }
    }
}
