#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Inter integrated circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Own address register 1."]
    #[inline(always)]
    pub const fn oaddr1(self) -> crate::common::Reg<regs::Oaddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Own address register 2."]
    #[inline(always)]
    pub const fn oaddr2(self) -> crate::common::Reg<regs::Oaddr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status register 1."]
    #[inline(always)]
    pub const fn star1(self) -> crate::common::Reg<regs::Star1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Status register 2."]
    #[inline(always)]
    pub const fn star2(self) -> crate::common::Reg<regs::Star2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Clock control register."]
    #[inline(always)]
    pub const fn ckcfgr(self) -> crate::common::Reg<regs::Ckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Raise time register."]
    #[inline(always)]
    pub const fn rtr(self) -> crate::common::Reg<regs::Rtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcfgr(pub u32);
    impl Ckcfgr {
        #[doc = "Clock control register in Fast/Standard mode (Master mode)."]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Clock control register in Fast/Standard mode (Master mode)."]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Fast mode duty cycle."]
        #[inline(always)]
        pub const fn duty(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Fast mode duty cycle."]
        #[inline(always)]
        pub fn set_duty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "I2C master mode selection."]
        #[inline(always)]
        pub const fn f_s(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "I2C master mode selection."]
        #[inline(always)]
        pub fn set_f_s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ckcfgr {
        #[inline(always)]
        fn default() -> Ckcfgr {
            Ckcfgr(0)
        }
    }
    #[doc = "Control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Peripheral enable."]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral enable."]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SMBus mode."]
        #[inline(always)]
        pub const fn smbus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus mode."]
        #[inline(always)]
        pub fn set_smbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SMBus type."]
        #[inline(always)]
        pub const fn smbtype(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus type."]
        #[inline(always)]
        pub fn set_smbtype(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ARP enable."]
        #[inline(always)]
        pub const fn enarp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ARP enable."]
        #[inline(always)]
        pub fn set_enarp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PEC enable."]
        #[inline(always)]
        pub const fn enpec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PEC enable."]
        #[inline(always)]
        pub fn set_enpec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "General call enable."]
        #[inline(always)]
        pub const fn engc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "General call enable."]
        #[inline(always)]
        pub fn set_engc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clock stretching disable (Slave mode)."]
        #[inline(always)]
        pub const fn nostretch(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock stretching disable (Slave mode)."]
        #[inline(always)]
        pub fn set_nostretch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Start generation."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Start generation."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Stop generation."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Stop generation."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge enable."]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge enable."]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge/PEC Position (for data reception)."]
        #[inline(always)]
        pub const fn pos(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge/PEC Position (for data reception)."]
        #[inline(always)]
        pub fn set_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Packet error checking."]
        #[inline(always)]
        pub const fn pec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Packet error checking."]
        #[inline(always)]
        pub fn set_pec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SMBus alert."]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus alert."]
        #[inline(always)]
        pub fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Software reset."]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset."]
        #[inline(always)]
        pub fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    #[doc = "Control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "Peripheral clock frequency."]
        #[inline(always)]
        pub const fn freq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Peripheral clock frequency."]
        #[inline(always)]
        pub fn set_freq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn iterren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub fn set_iterren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Event interrupt enable."]
        #[inline(always)]
        pub const fn itevten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Event interrupt enable."]
        #[inline(always)]
        pub fn set_itevten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Buffer interrupt enable."]
        #[inline(always)]
        pub const fn itbufen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer interrupt enable."]
        #[inline(always)]
        pub fn set_itbufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DMA requests enable."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA requests enable."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA last transfer."]
        #[inline(always)]
        pub const fn last(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA last transfer."]
        #[inline(always)]
        pub fn set_last(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "8-bit data register."]
        #[inline(always)]
        pub const fn datar(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit data register."]
        #[inline(always)]
        pub fn set_datar(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "Own address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oaddr1(pub u32);
    impl Oaddr1 {
        #[doc = "Interface address."]
        #[inline(always)]
        pub const fn add0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub fn set_add0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub const fn add7_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub fn set_add7_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub const fn add9_8(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub fn set_add9_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Must be 1."]
        #[inline(always)]
        pub const fn must1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Must be 1."]
        #[inline(always)]
        pub fn set_must1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Addressing mode (slave mode)."]
        #[inline(always)]
        pub const fn addmode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Addressing mode (slave mode)."]
        #[inline(always)]
        pub fn set_addmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Oaddr1 {
        #[inline(always)]
        fn default() -> Oaddr1 {
            Oaddr1(0)
        }
    }
    #[doc = "Own address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oaddr2(pub u32);
    impl Oaddr2 {
        #[doc = "Dual addressing mode enable."]
        #[inline(always)]
        pub const fn endual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Dual addressing mode enable."]
        #[inline(always)]
        pub fn set_endual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub const fn add2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Interface address."]
        #[inline(always)]
        pub fn set_add2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Oaddr2 {
        #[inline(always)]
        fn default() -> Oaddr2 {
            Oaddr2(0)
        }
    }
    #[doc = "Raise time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtr(pub u32);
    impl Rtr {
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)."]
        #[inline(always)]
        pub const fn trise(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Maximum rise time in Fast/Standard mode (Master mode)."]
        #[inline(always)]
        pub fn set_trise(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Rtr {
        #[inline(always)]
        fn default() -> Rtr {
            Rtr(0)
        }
    }
    #[doc = "Status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star1(pub u32);
    impl Star1 {
        #[doc = "Start bit (Master mode)."]
        #[inline(always)]
        pub const fn sb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit (Master mode)."]
        #[inline(always)]
        pub fn set_sb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address sent (master mode)/matched (slave mode)."]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address sent (master mode)/matched (slave mode)."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Byte transfer finished."]
        #[inline(always)]
        pub const fn btf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Byte transfer finished."]
        #[inline(always)]
        pub fn set_btf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "10-bit header sent (Master mode)."]
        #[inline(always)]
        pub const fn add10(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "10-bit header sent (Master mode)."]
        #[inline(always)]
        pub fn set_add10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Stop detection (slave mode)."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Stop detection (slave mode)."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data register not empty (receivers)."]
        #[inline(always)]
        pub const fn rx_ne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data register not empty (receivers)."]
        #[inline(always)]
        pub fn set_rx_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data register empty (transmitters)."]
        #[inline(always)]
        pub const fn tx_e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data register empty (transmitters)."]
        #[inline(always)]
        pub fn set_tx_e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bus error."]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bus error."]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Arbitration lost (master mode)."]
        #[inline(always)]
        pub const fn arlo(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost (master mode)."]
        #[inline(always)]
        pub fn set_arlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge failure."]
        #[inline(always)]
        pub const fn af(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge failure."]
        #[inline(always)]
        pub fn set_af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Overrun/Underrun."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/Underrun."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PEC Error in reception."]
        #[inline(always)]
        pub const fn pecerr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PEC Error in reception."]
        #[inline(always)]
        pub fn set_pecerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timeout or Tlow error."]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout or Tlow error."]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SMBus alert."]
        #[inline(always)]
        pub const fn smbalert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus alert."]
        #[inline(always)]
        pub fn set_smbalert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Star1 {
        #[inline(always)]
        fn default() -> Star1 {
            Star1(0)
        }
    }
    #[doc = "Status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star2(pub u32);
    impl Star2 {
        #[doc = "Master/slave."]
        #[inline(always)]
        pub const fn msl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master/slave."]
        #[inline(always)]
        pub fn set_msl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bus busy."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bus busy."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter/receiver."]
        #[inline(always)]
        pub const fn tra(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter/receiver."]
        #[inline(always)]
        pub fn set_tra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "General call address (Slave mode)."]
        #[inline(always)]
        pub const fn gencall(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "General call address (Slave mode)."]
        #[inline(always)]
        pub fn set_gencall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SMBus device default address (Slave mode)."]
        #[inline(always)]
        pub const fn smbdefault(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus device default address (Slave mode)."]
        #[inline(always)]
        pub fn set_smbdefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SMBus host header (Slave mode)."]
        #[inline(always)]
        pub const fn smbhost(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus host header (Slave mode)."]
        #[inline(always)]
        pub fn set_smbhost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Dual flag (Slave mode)."]
        #[inline(always)]
        pub const fn dualf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Dual flag (Slave mode)."]
        #[inline(always)]
        pub fn set_dualf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "acket error checking register."]
        #[inline(always)]
        pub const fn pec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "acket error checking register."]
        #[inline(always)]
        pub fn set_pec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Star2 {
        #[inline(always)]
        fn default() -> Star2 {
            Star2(0)
        }
    }
}
