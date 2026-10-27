#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Improved inter-integrated circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register, master message form."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control register, CCC message form."]
    #[inline(always)]
    pub const fn ctlr_ccc(self) -> crate::common::Reg<regs::CtlrCcc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Receive data byte register."]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Transmit data byte register."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "IBI payload data register."]
    #[inline(always)]
    pub const fn ibidr(self) -> crate::common::Reg<regs::Ibidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Reset register."]
    #[inline(always)]
    pub const fn reset(self) -> crate::common::Reg<regs::Reset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Status error register."]
    #[inline(always)]
    pub const fn stater(self) -> crate::common::Reg<regs::Stater, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Received message register."]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Event register."]
    #[inline(always)]
    pub const fn evr(self) -> crate::common::Reg<regs::Evr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn intenr(self) -> crate::common::Reg<regs::Intenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Clear event register."]
    #[inline(always)]
    pub const fn cevr(self) -> crate::common::Reg<regs::Cevr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Own device characteristics register."]
    #[inline(always)]
    pub const fn owndevr(self) -> crate::common::Reg<regs::Owndevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Target device characteristics register 1. Index 0 is DEVR1."]
    #[inline(always)]
    pub const fn devr(self, n: usize) -> crate::common::Reg<regs::Devr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "Target device characteristics register 3. Index 0 is DEVR3."]
    #[inline(always)]
    pub const fn devrx(self, n: usize) -> crate::common::Reg<regs::Devrx, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize + n * 4usize) as _) }
    }
    #[doc = "Timing register 0."]
    #[inline(always)]
    pub const fn timingr0(self) -> crate::common::Reg<regs::Timingr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Timing register 1."]
    #[inline(always)]
    pub const fn timingr1(self) -> crate::common::Reg<regs::Timingr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Timing register 2."]
    #[inline(always)]
    pub const fn timingr2(self) -> crate::common::Reg<regs::Timingr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Get maximum data speed register."]
    #[inline(always)]
    pub const fn getmdsr(self) -> crate::common::Reg<regs::Getmdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clear event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cevr(pub u32);
    impl Cevr {
        #[doc = "Clear the frame complete flag."]
        #[inline(always)]
        pub const fn cfcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the frame complete flag."]
        #[inline(always)]
        pub fn set_cfcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear the target-initiated read end flag."]
        #[inline(always)]
        pub const fn crxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the target-initiated read end flag."]
        #[inline(always)]
        pub fn set_crxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear the error flag."]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the error flag."]
        #[inline(always)]
        pub fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear the in-band interrupt request flag."]
        #[inline(always)]
        pub const fn cibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the in-band interrupt request flag."]
        #[inline(always)]
        pub fn set_cibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Clear the hot-join request flag."]
        #[inline(always)]
        pub const fn chjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the hot-join request flag."]
        #[inline(always)]
        pub fn set_chjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Cevr {
        #[inline(always)]
        fn default() -> Cevr {
            Cevr(0)
        }
    }
    #[doc = "Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "I3C enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I3C enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Controller initiative, set when acting as master."]
        #[inline(always)]
        pub const fn crinit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Controller initiative, set when acting as master."]
        #[inline(always)]
        pub fn set_crinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Do not arbitrate, skip arbitration when starting a frame."]
        #[inline(always)]
        pub const fn noarbh(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Do not arbitrate, skip arbitration when starting a frame."]
        #[inline(always)]
        pub fn set_noarbh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Stop pattern reset."]
        #[inline(always)]
        pub const fn rstptrn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Stop pattern reset."]
        #[inline(always)]
        pub fn set_rstptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Exit pattern."]
        #[inline(always)]
        pub const fn exitptrn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Exit pattern."]
        #[inline(always)]
        pub fn set_exitptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bus pull-up enable."]
        #[inline(always)]
        pub const fn pullup_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Bus pull-up enable."]
        #[inline(always)]
        pub fn set_pullup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Hot-join request acknowledge."]
        #[inline(always)]
        pub const fn hjack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join request acknowledge."]
        #[inline(always)]
        pub fn set_hjack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive FIFO DMA enable."]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO DMA enable."]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive FIFO flush."]
        #[inline(always)]
        pub const fn rxflush(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO flush."]
        #[inline(always)]
        pub fn set_rxflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive FIFO threshold."]
        #[inline(always)]
        pub const fn rxthres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO threshold."]
        #[inline(always)]
        pub fn set_rxthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Transmit FIFO DMA enable."]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO DMA enable."]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit FIFO flush."]
        #[inline(always)]
        pub const fn txflush(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO flush."]
        #[inline(always)]
        pub fn set_txflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit FIFO threshold."]
        #[inline(always)]
        pub const fn txthres(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO threshold."]
        #[inline(always)]
        pub fn set_txthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Target mode enable."]
        #[inline(always)]
        pub const fn tmode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Target mode enable."]
        #[inline(always)]
        pub fn set_tmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Control FIFO DMA enable."]
        #[inline(always)]
        pub const fn cdmaen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO DMA enable."]
        #[inline(always)]
        pub fn set_cdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Control FIFO flush."]
        #[inline(always)]
        pub const fn cflush(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO flush."]
        #[inline(always)]
        pub fn set_cflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Start frame transfer. Write 1 to launch a frame."]
        #[inline(always)]
        pub const fn tsfset(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Start frame transfer. Write 1 to launch a frame."]
        #[inline(always)]
        pub fn set_tsfset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "Control register, master message form."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Byte count of the message, linear encoding up to 64 KiB - 1."]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte count of the message, linear encoding up to 64 KiB - 1."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Read (1) or write (0) message when acting as master."]
        #[inline(always)]
        pub const fn rnw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Read (1) or write (0) message when acting as master."]
        #[inline(always)]
        pub fn set_rnw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "7-bit I3C dynamic or I2C static target address."]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit I3C dynamic or I2C static target address."]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "Last message of the frame, terminated by a stop rather than a repeated start."]
        #[inline(always)]
        pub const fn mend(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Last message of the frame, terminated by a stop rather than a repeated start."]
        #[inline(always)]
        pub fn set_mend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "Control register, CCC message form."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtlrCcc(pub u32);
    impl CtlrCcc {
        #[doc = "Byte count of the message, linear encoding up to 64 KiB - 1."]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte count of the message, linear encoding up to 64 KiB - 1."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "8-bit common command code."]
        #[inline(always)]
        pub const fn ccc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit common command code."]
        #[inline(always)]
        pub fn set_ccc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Message type."]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "Last message of the frame, terminated by a stop rather than a repeated start."]
        #[inline(always)]
        pub const fn mend(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Last message of the frame, terminated by a stop rather than a repeated start."]
        #[inline(always)]
        pub fn set_mend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CtlrCcc {
        #[inline(always)]
        fn default() -> CtlrCcc {
            CtlrCcc(0)
        }
    }
    #[doc = "Target device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr(pub u32);
    impl Devr {
        #[doc = "Assigned 7-bit dynamic address of the target."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Assigned 7-bit dynamic address of the target."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "In-band interrupt request acknowledge."]
        #[inline(always)]
        pub const fn ibiack(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "In-band interrupt request acknowledge."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "In-band interrupt payload enable."]
        #[inline(always)]
        pub const fn ibiden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "In-band interrupt payload enable."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Suspend or stop the transfer on a received in-band interrupt."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend or stop the transfer on a received in-band interrupt."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Write to the dynamic address disabled."]
        #[inline(always)]
        pub const fn dis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Write to the dynamic address disabled."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr {
        #[inline(always)]
        fn default() -> Devr {
            Devr(0)
        }
    }
    #[doc = "Target device characteristics register, with controller-role acknowledge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devrx(pub u32);
    impl Devrx {
        #[doc = "Assigned 7-bit dynamic address of the target."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Assigned 7-bit dynamic address of the target."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "Controller-role request acknowledge."]
        #[inline(always)]
        pub const fn crack(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role request acknowledge."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "In-band interrupt payload enable."]
        #[inline(always)]
        pub const fn ibiden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "In-band interrupt payload enable."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Suspend or stop the transfer on a received in-band interrupt."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend or stop the transfer on a received in-band interrupt."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Write to the dynamic address disabled."]
        #[inline(always)]
        pub const fn dis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Write to the dynamic address disabled."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devrx {
        #[inline(always)]
        fn default() -> Devrx {
            Devrx(0)
        }
    }
    #[doc = "Event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evr(pub u32);
    impl Evr {
        #[doc = "Control FIFO empty flag."]
        #[inline(always)]
        pub const fn cfef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO empty flag."]
        #[inline(always)]
        pub fn set_cfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit FIFO empty flag."]
        #[inline(always)]
        pub const fn txfef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO empty flag."]
        #[inline(always)]
        pub fn set_txfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Control FIFO not full flag."]
        #[inline(always)]
        pub const fn cfnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO not full flag."]
        #[inline(always)]
        pub fn set_cfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit FIFO not full flag."]
        #[inline(always)]
        pub const fn txfnff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO not full flag."]
        #[inline(always)]
        pub fn set_txfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receive FIFO not empty flag."]
        #[inline(always)]
        pub const fn rxfnef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO not empty flag."]
        #[inline(always)]
        pub fn set_rxfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmit FIFO last byte transmitted."]
        #[inline(always)]
        pub const fn txlastf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO last byte transmitted."]
        #[inline(always)]
        pub fn set_txlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive FIFO last byte received."]
        #[inline(always)]
        pub const fn rxlastf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO last byte received."]
        #[inline(always)]
        pub fn set_rxlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame complete flag."]
        #[inline(always)]
        pub const fn fcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame complete flag."]
        #[inline(always)]
        pub fn set_fcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Target-initiated read ended."]
        #[inline(always)]
        pub const fn rxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Target-initiated read ended."]
        #[inline(always)]
        pub fn set_rxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Error flag."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Error flag."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "In-band interrupt request flag."]
        #[inline(always)]
        pub const fn ibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "In-band interrupt request flag."]
        #[inline(always)]
        pub fn set_ibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hot-join request flag."]
        #[inline(always)]
        pub const fn hjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join request flag."]
        #[inline(always)]
        pub fn set_hjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Evr {
        #[inline(always)]
        fn default() -> Evr {
            Evr(0)
        }
    }
    #[doc = "Get maximum data speed register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getmdsr(pub u32);
    impl Getmdsr {
        #[doc = "Target supports clock stretching only."]
        #[inline(always)]
        pub const fn tsco(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Target supports clock stretching only."]
        #[inline(always)]
        pub fn set_tsco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Getmdsr {
        #[inline(always)]
        fn default() -> Getmdsr {
            Getmdsr(0)
        }
    }
    #[doc = "IBI payload data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ibidr(pub u32);
    impl Ibidr {
        #[doc = "In-band interrupt payload byte 0."]
        #[inline(always)]
        pub const fn ibidb(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "In-band interrupt payload byte 0."]
        #[inline(always)]
        pub fn set_ibidb(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Ibidr {
        #[inline(always)]
        fn default() -> Ibidr {
            Ibidr(0)
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intenr(pub u32);
    impl Intenr {
        #[doc = "Control FIFO not full interrupt enable."]
        #[inline(always)]
        pub const fn cfnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO not full interrupt enable."]
        #[inline(always)]
        pub fn set_cfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit FIFO not full interrupt enable."]
        #[inline(always)]
        pub const fn txfnfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO not full interrupt enable."]
        #[inline(always)]
        pub fn set_txfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receive FIFO not empty interrupt enable."]
        #[inline(always)]
        pub const fn rxfneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO not empty interrupt enable."]
        #[inline(always)]
        pub fn set_rxfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Frame complete interrupt enable."]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame complete interrupt enable."]
        #[inline(always)]
        pub fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Target-initiated read end interrupt enable."]
        #[inline(always)]
        pub const fn rxtgtendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Target-initiated read end interrupt enable."]
        #[inline(always)]
        pub fn set_rxtgtendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "In-band interrupt request enable."]
        #[inline(always)]
        pub const fn ibiie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "In-band interrupt request enable."]
        #[inline(always)]
        pub fn set_ibiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hot-join request interrupt enable."]
        #[inline(always)]
        pub const fn hjie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join request interrupt enable."]
        #[inline(always)]
        pub fn set_hjie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Intenr {
        #[inline(always)]
        fn default() -> Intenr {
            Intenr(0)
        }
    }
    #[doc = "Own device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Owndevr(pub u32);
    impl Owndevr {
        #[doc = "Dynamic address is valid, when acting as target."]
        #[inline(always)]
        pub const fn daval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic address is valid, when acting as target."]
        #[inline(always)]
        pub fn set_daval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "7-bit dynamic address."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit dynamic address."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Owndevr {
        #[inline(always)]
        fn default() -> Owndevr {
            Owndevr(0)
        }
    }
    #[doc = "Receive data byte register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdr(pub u32);
    impl Rdr {
        #[doc = "Received data byte."]
        #[inline(always)]
        pub const fn rdb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Received data byte."]
        #[inline(always)]
        pub fn set_rdb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rdr {
        #[inline(always)]
        fn default() -> Rdr {
            Rdr(0)
        }
    }
    #[doc = "Reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reset(pub u32);
    impl Reset {
        #[doc = "Reset the HS target interface."]
        #[inline(always)]
        pub const fn hst_sie_rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset the HS target interface."]
        #[inline(always)]
        pub fn set_hst_sie_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Reset {
        #[inline(always)]
        fn default() -> Reset {
            Reset(0)
        }
    }
    #[doc = "Received message register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmr(pub u32);
    impl Rmr {
        #[doc = "IBI remaining data count."]
        #[inline(always)]
        pub const fn ibirdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "IBI remaining data count."]
        #[inline(always)]
        pub fn set_ibirdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Request code or common command code of the received message."]
        #[inline(always)]
        pub const fn rcode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Request code or common command code of the received message."]
        #[inline(always)]
        pub fn set_rcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Dynamic address of the initiator."]
        #[inline(always)]
        pub const fn radd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "Dynamic address of the initiator."]
        #[inline(always)]
        pub fn set_radd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for Rmr {
        #[inline(always)]
        fn default() -> Rmr {
            Rmr(0)
        }
    }
    #[doc = "Status error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stater(pub u32);
    impl Stater {
        #[doc = "Common command code error."]
        #[inline(always)]
        pub const fn coderr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Common command code error."]
        #[inline(always)]
        pub fn set_coderr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Parity error."]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error."]
        #[inline(always)]
        pub fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data FIFO overrun."]
        #[inline(always)]
        pub const fn dovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data FIFO overrun."]
        #[inline(always)]
        pub fn set_dovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Control FIFO overrun."]
        #[inline(always)]
        pub const fn covr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Control FIFO overrun."]
        #[inline(always)]
        pub fn set_covr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Address phase not acknowledged."]
        #[inline(always)]
        pub const fn anack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Address phase not acknowledged."]
        #[inline(always)]
        pub fn set_anack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data phase not acknowledged."]
        #[inline(always)]
        pub const fn dnack(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data phase not acknowledged."]
        #[inline(always)]
        pub fn set_dnack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data error."]
        #[inline(always)]
        pub const fn derr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data error."]
        #[inline(always)]
        pub fn set_derr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Stater {
        #[inline(always)]
        fn default() -> Stater {
            Stater(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Transferred byte count of the current or last frame."]
        #[inline(always)]
        pub const fn xdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Transferred byte count of the current or last frame."]
        #[inline(always)]
        pub fn set_xdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Frame transfer aborted."]
        #[inline(always)]
        pub const fn abt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame transfer aborted."]
        #[inline(always)]
        pub fn set_abt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Direction of the current frame, 1 for read."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Direction of the current frame, 1 for read."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Master or manufacturer identifier."]
        #[inline(always)]
        pub const fn mid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Master or manufacturer identifier."]
        #[inline(always)]
        pub fn set_mid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    #[doc = "Transmit data byte register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdr(pub u32);
    impl Tdr {
        #[doc = "Transmit data byte."]
        #[inline(always)]
        pub const fn tdb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit data byte."]
        #[inline(always)]
        pub fn set_tdb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Tdr {
        #[inline(always)]
        fn default() -> Tdr {
            Tdr(0)
        }
    }
    #[doc = "Timing register 0, bus timing in the different bus modes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr0(pub u32);
    impl Timingr0 {
        #[doc = "SCL low period, push-pull mode."]
        #[inline(always)]
        pub const fn scll_pp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low period, push-pull mode."]
        #[inline(always)]
        pub fn set_scll_pp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "SCL high period, I3C mode."]
        #[inline(always)]
        pub const fn sclh_i3c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high period, I3C mode."]
        #[inline(always)]
        pub fn set_sclh_i3c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "SCL low period, open-drain mode."]
        #[inline(always)]
        pub const fn scll_od(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low period, open-drain mode."]
        #[inline(always)]
        pub fn set_scll_od(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SCL high period, I2C mode."]
        #[inline(always)]
        pub const fn sclh_i2c(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high period, I2C mode."]
        #[inline(always)]
        pub fn set_sclh_i2c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timingr0 {
        #[inline(always)]
        fn default() -> Timingr0 {
            Timingr0(0)
        }
    }
    #[doc = "Timing register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr1(pub u32);
    impl Timingr1 {
        #[doc = "Bus available condition timing."]
        #[inline(always)]
        pub const fn aval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bus available condition timing."]
        #[inline(always)]
        pub fn set_aval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Bus available to start condition delay."]
        #[inline(always)]
        pub const fn asncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Bus available to start condition delay."]
        #[inline(always)]
        pub fn set_asncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Free condition timing."]
        #[inline(always)]
        pub const fn free(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Free condition timing."]
        #[inline(always)]
        pub fn set_free(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "SDA hold time."]
        #[inline(always)]
        pub const fn sda_hd(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDA hold time."]
        #[inline(always)]
        pub fn set_sda_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Timingr1 {
        #[inline(always)]
        fn default() -> Timingr1 {
            Timingr1(0)
        }
    }
    #[doc = "Timing register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr2(pub u32);
    impl Timingr2 {
        #[doc = "Stalling of the transfer is enabled."]
        #[inline(always)]
        pub const fn stallt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stalling of the transfer is enabled."]
        #[inline(always)]
        pub fn set_stallt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stalling on data phase."]
        #[inline(always)]
        pub const fn stalld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stalling on data phase."]
        #[inline(always)]
        pub fn set_stalld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Stalling on control word."]
        #[inline(always)]
        pub const fn stallc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Stalling on control word."]
        #[inline(always)]
        pub fn set_stallc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Stalling on acknowledge."]
        #[inline(always)]
        pub const fn stalla(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Stalling on acknowledge."]
        #[inline(always)]
        pub fn set_stalla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Stalling time."]
        #[inline(always)]
        pub const fn stall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stalling time."]
        #[inline(always)]
        pub fn set_stall(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Timingr2 {
        #[inline(always)]
        fn default() -> Timingr2 {
            Timingr2(0)
        }
    }
}
