#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Controller area network (bxCAN-compatible). 3 TX mailboxes + 2 RX FIFOs + 42 filter banks shared across CAN1/CAN2/CAN3 instances."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CAN master control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CAN master status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CAN transmit status register."]
    #[inline(always)]
    pub const fn tstatr(self) -> crate::common::Reg<regs::Tstatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CAN receive FIFO register (RX FIFO 0 / 1)."]
    #[inline(always)]
    pub const fn rfifo(self, n: usize) -> crate::common::Reg<regs::Rfifo, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 4usize) as _) }
    }
    #[doc = "CAN interrupt enable register."]
    #[inline(always)]
    pub const fn intenr(self) -> crate::common::Reg<regs::Intenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CAN error status register."]
    #[inline(always)]
    pub const fn errsr(self) -> crate::common::Reg<regs::Errsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CAN bit timing register."]
    #[inline(always)]
    pub const fn btimr(self) -> crate::common::Reg<regs::Btimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "CAN time-trigger control register."]
    #[inline(always)]
    pub const fn ttctlr(self) -> crate::common::Reg<regs::Ttctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "CAN time-trigger counter value."]
    #[inline(always)]
    pub const fn ttcnt(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "CAN error counter register."]
    #[inline(always)]
    pub const fn terr_cnt(self) -> crate::common::Reg<regs::TerrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "CAN TX mailbox cluster: TXMIR/TXMDTR/TXMDLR/TXMDHR per mailbox, 3 mailboxes."]
    #[inline(always)]
    pub const fn tx(self, n: usize) -> Tx {
        assert!(n < 3usize);
        unsafe { Tx::from_ptr(self.ptr.add(0x0180usize + n * 16usize) as _) }
    }
    #[doc = "CAN RX FIFO mailbox cluster: RXMIR/RXMDTR/RXMDLR/RXMDHR per FIFO, 2 FIFOs."]
    #[inline(always)]
    pub const fn rx(self, n: usize) -> Rx {
        assert!(n < 2usize);
        unsafe { Rx::from_ptr(self.ptr.add(0x01b0usize + n * 16usize) as _) }
    }
    #[doc = "Filter master register."]
    #[inline(always)]
    pub const fn fctlr(self) -> crate::common::Reg<regs::Fctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Filter mode configuration (banks owned by CAN1/CAN2)."]
    #[inline(always)]
    pub const fn fmcfgr(self) -> crate::common::Reg<regs::Fmcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Filter mode configuration (banks owned by CAN3)."]
    #[inline(always)]
    pub const fn fmcfgr_can3(self) -> crate::common::Reg<regs::FmcfgrCan3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Filter scale configuration (banks owned by CAN1/CAN2)."]
    #[inline(always)]
    pub const fn fscfgr(self) -> crate::common::Reg<regs::Fscfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Filter scale configuration (banks owned by CAN3)."]
    #[inline(always)]
    pub const fn fscfgr_can3(self) -> crate::common::Reg<regs::FscfgrCan3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "Filter assignment to FIFO (banks owned by CAN1/CAN2)."]
    #[inline(always)]
    pub const fn fafifor(self) -> crate::common::Reg<regs::Fafifor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "Filter assignment to FIFO (banks owned by CAN3)."]
    #[inline(always)]
    pub const fn fafifor_can3(self) -> crate::common::Reg<regs::FafiforCan3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "Filter activation (banks owned by CAN1/CAN2)."]
    #[inline(always)]
    pub const fn fwr(self) -> crate::common::Reg<regs::Fwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "Filter activation (banks owned by CAN3)."]
    #[inline(always)]
    pub const fn fwr_can3(self) -> crate::common::Reg<regs::FwrCan3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "Filter bank: FR1 + FR2 per bank, 42 banks shared by CAN1/CAN2/CAN3."]
    #[inline(always)]
    pub const fn fb(self, n: usize) -> Fb {
        assert!(n < 42usize);
        unsafe { Fb::from_ptr(self.ptr.add(0x0240usize + n * 8usize) as _) }
    }
}
#[doc = "Filter bank (two 32-bit registers; layout depends on FSCFGR.FSC bit for this bank)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fb {
    ptr: *mut u8,
}
unsafe impl Send for Fb {}
unsafe impl Sync for Fb {}
impl Fb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Filter bank register 1."]
    #[inline(always)]
    pub const fn fr1(self) -> crate::common::Reg<regs::Fr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Filter bank register 2."]
    #[inline(always)]
    pub const fn fr2(self) -> crate::common::Reg<regs::Fr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "CAN RX FIFO mailbox (read-only; identifier / data length+timestamp / data low / data high)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx {
    ptr: *mut u8,
}
unsafe impl Send for Rx {}
unsafe impl Sync for Rx {}
impl Rx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RX FIFO mailbox identifier register."]
    #[inline(always)]
    pub const fn rxmir(self) -> crate::common::Reg<regs::Rxmir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RX FIFO mailbox data length control and timestamp register."]
    #[inline(always)]
    pub const fn rxmdtr(self) -> crate::common::Reg<regs::Rxmdtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RX FIFO mailbox data low register."]
    #[inline(always)]
    pub const fn rxmdlr(self) -> crate::common::Reg<regs::Rxmdlr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RX FIFO mailbox data high register."]
    #[inline(always)]
    pub const fn rxmdhr(self) -> crate::common::Reg<regs::Rxmdhr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "CAN TX mailbox (identifier / data length+timestamp / data low / data high)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx {
    ptr: *mut u8,
}
unsafe impl Send for Tx {}
unsafe impl Sync for Tx {}
impl Tx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TX mailbox identifier register."]
    #[inline(always)]
    pub const fn txmir(self) -> crate::common::Reg<regs::Txmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TX mailbox data length control and timestamp register."]
    #[inline(always)]
    pub const fn txmdtr(self) -> crate::common::Reg<regs::Txmdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TX mailbox data low register."]
    #[inline(always)]
    pub const fn txmdlr(self) -> crate::common::Reg<regs::Txmdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TX mailbox data high register."]
    #[inline(always)]
    pub const fn txmdhr(self) -> crate::common::Reg<regs::Txmdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "CAN bit timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btimr(pub u32);
    impl Btimr {
        #[doc = "minimum time unit length setting value."]
        #[inline(always)]
        pub const fn brp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "minimum time unit length setting value."]
        #[inline(always)]
        pub fn set_brp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "CLAS_LONG_TS1=0;TS1 is TS\\[3:0\\](4bit);CLAS_LONG_TS1=1,TS1 is. TS\\[1:0\\]+BTR_TS1_T\\[15:12\\](6bit)."]
        #[inline(always)]
        pub const fn btr_ts1_t(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "CLAS_LONG_TS1=0;TS1 is TS\\[3:0\\](4bit);CLAS_LONG_TS1=1,TS1 is. TS\\[1:0\\]+BTR_TS1_T\\[15:12\\](6bit)."]
        #[inline(always)]
        pub fn set_btr_ts1_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Time segment 1."]
        #[inline(always)]
        pub const fn ts1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Time segment 1."]
        #[inline(always)]
        pub fn set_ts1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Time segment 2."]
        #[inline(always)]
        pub const fn ts2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Time segment 2."]
        #[inline(always)]
        pub fn set_ts2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Resynchronization jump width."]
        #[inline(always)]
        pub const fn sjw(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Resynchronization jump width."]
        #[inline(always)]
        pub fn set_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Loop back mode (debug)."]
        #[inline(always)]
        pub const fn lbkm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Loop back mode (debug)."]
        #[inline(always)]
        pub fn set_lbkm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Silent mode (debug)."]
        #[inline(always)]
        pub const fn silm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Silent mode (debug)."]
        #[inline(always)]
        pub fn set_silm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Btimr {
        #[inline(always)]
        fn default() -> Btimr {
            Btimr(0)
        }
    }
    #[doc = "CAN Master control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Initialization request."]
        #[inline(always)]
        pub const fn inrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization request."]
        #[inline(always)]
        pub fn set_inrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sleep mode request bit."]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep mode request bit."]
        #[inline(always)]
        pub fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit FIFO priority."]
        #[inline(always)]
        pub const fn txfp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO priority."]
        #[inline(always)]
        pub fn set_txfp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive FIFO locked mode."]
        #[inline(always)]
        pub const fn rflm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO locked mode."]
        #[inline(always)]
        pub fn set_rflm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No automatic retransmission."]
        #[inline(always)]
        pub const fn nart(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No automatic retransmission."]
        #[inline(always)]
        pub fn set_nart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Automatic wakeup mode."]
        #[inline(always)]
        pub const fn awum(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic wakeup mode."]
        #[inline(always)]
        pub fn set_awum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic bus-off management."]
        #[inline(always)]
        pub const fn abom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bus-off management."]
        #[inline(always)]
        pub fn set_abom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Time triggered communication mode."]
        #[inline(always)]
        pub const fn ttcm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Time triggered communication mode."]
        #[inline(always)]
        pub fn set_ttcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Software master reset."]
        #[inline(always)]
        pub const fn rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Software master reset."]
        #[inline(always)]
        pub fn set_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Debug freeze."]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Debug freeze."]
        #[inline(always)]
        pub fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Configure CAN offline recovery time."]
        #[inline(always)]
        pub const fn cfgcanm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Configure CAN offline recovery time."]
        #[inline(always)]
        pub fn set_cfgcanm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "CAN error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errsr(pub u32);
    impl Errsr {
        #[doc = "Error warning flag."]
        #[inline(always)]
        pub const fn ewgf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error warning flag."]
        #[inline(always)]
        pub fn set_ewgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Error passive flag."]
        #[inline(always)]
        pub const fn epvf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive flag."]
        #[inline(always)]
        pub fn set_epvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bus-off flag."]
        #[inline(always)]
        pub const fn boff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-off flag."]
        #[inline(always)]
        pub fn set_boff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Last error code."]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Last error code."]
        #[inline(always)]
        pub fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Least significant byte of the 9-bit transmit error counter."]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Least significant byte of the 9-bit transmit error counter."]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Receive error counter."]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Receive error counter."]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Errsr {
        #[inline(always)]
        fn default() -> Errsr {
            Errsr(0)
        }
    }
    #[doc = "CAN filter FIFO assignment register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fafifor(pub u32);
    impl Fafifor {
        #[doc = "Filter FIFO assignment for filter 0."]
        #[inline(always)]
        pub const fn ffa0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 0."]
        #[inline(always)]
        pub fn set_ffa0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter FIFO assignment for filter 1."]
        #[inline(always)]
        pub const fn ffa1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 1."]
        #[inline(always)]
        pub fn set_ffa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter FIFO assignment for filter 2."]
        #[inline(always)]
        pub const fn ffa2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 2."]
        #[inline(always)]
        pub fn set_ffa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter FIFO assignment for filter 3."]
        #[inline(always)]
        pub const fn ffa3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 3."]
        #[inline(always)]
        pub fn set_ffa3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter FIFO assignment for filter 4."]
        #[inline(always)]
        pub const fn ffa4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 4."]
        #[inline(always)]
        pub fn set_ffa4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter FIFO assignment for filter 5."]
        #[inline(always)]
        pub const fn ffa5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 5."]
        #[inline(always)]
        pub fn set_ffa5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter FIFO assignment for filter 6."]
        #[inline(always)]
        pub const fn ffa6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 6."]
        #[inline(always)]
        pub fn set_ffa6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter FIFO assignment for filter 7."]
        #[inline(always)]
        pub const fn ffa7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 7."]
        #[inline(always)]
        pub fn set_ffa7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter FIFO assignment for filter 8."]
        #[inline(always)]
        pub const fn ffa8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 8."]
        #[inline(always)]
        pub fn set_ffa8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter FIFO assignment for filter 9."]
        #[inline(always)]
        pub const fn ffa9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 9."]
        #[inline(always)]
        pub fn set_ffa9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter FIFO assignment for filter 10."]
        #[inline(always)]
        pub const fn ffa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 10."]
        #[inline(always)]
        pub fn set_ffa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter FIFO assignment for filter 11."]
        #[inline(always)]
        pub const fn ffa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 11."]
        #[inline(always)]
        pub fn set_ffa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter FIFO assignment for filter 12."]
        #[inline(always)]
        pub const fn ffa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 12."]
        #[inline(always)]
        pub fn set_ffa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter FIFO assignment for filter 13."]
        #[inline(always)]
        pub const fn ffa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 13."]
        #[inline(always)]
        pub fn set_ffa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter FIFO assignment for filter 14."]
        #[inline(always)]
        pub const fn ffa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 14."]
        #[inline(always)]
        pub fn set_ffa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter FIFO assignment for filter 15."]
        #[inline(always)]
        pub const fn ffa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 15."]
        #[inline(always)]
        pub fn set_ffa15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter FIFO assignment for filter 16."]
        #[inline(always)]
        pub const fn ffa16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 16."]
        #[inline(always)]
        pub fn set_ffa16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter FIFO assignment for filter 17."]
        #[inline(always)]
        pub const fn ffa17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 17."]
        #[inline(always)]
        pub fn set_ffa17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter FIFO assignment for filter 18."]
        #[inline(always)]
        pub const fn ffa18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 18."]
        #[inline(always)]
        pub fn set_ffa18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter FIFO assignment for filter 19."]
        #[inline(always)]
        pub const fn ffa19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 19."]
        #[inline(always)]
        pub fn set_ffa19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter FIFO assignment for filter 20."]
        #[inline(always)]
        pub const fn ffa20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 20."]
        #[inline(always)]
        pub fn set_ffa20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter FIFO assignment for filter 21."]
        #[inline(always)]
        pub const fn ffa21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 21."]
        #[inline(always)]
        pub fn set_ffa21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter FIFO assignment for filter 22."]
        #[inline(always)]
        pub const fn ffa22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 22."]
        #[inline(always)]
        pub fn set_ffa22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter FIFO assignment for filter 23."]
        #[inline(always)]
        pub const fn ffa23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 23."]
        #[inline(always)]
        pub fn set_ffa23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Fafifor {
        #[inline(always)]
        fn default() -> Fafifor {
            Fafifor(0)
        }
    }
    #[doc = "CAN filter FIFO assignment register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FafiforCan3(pub u32);
    impl FafiforCan3 {
        #[doc = "Filter FIFO assignment for filter 28."]
        #[inline(always)]
        pub const fn ffa28(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 28."]
        #[inline(always)]
        pub fn set_ffa28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter FIFO assignment for filter 29."]
        #[inline(always)]
        pub const fn ffa29(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 29."]
        #[inline(always)]
        pub fn set_ffa29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter FIFO assignment for filter 30."]
        #[inline(always)]
        pub const fn ffa30(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 30."]
        #[inline(always)]
        pub fn set_ffa30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter FIFO assignment for filter 31."]
        #[inline(always)]
        pub const fn ffa31(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 31."]
        #[inline(always)]
        pub fn set_ffa31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter FIFO assignment for filter 32."]
        #[inline(always)]
        pub const fn ffa32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 32."]
        #[inline(always)]
        pub fn set_ffa32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter FIFO assignment for filter 33."]
        #[inline(always)]
        pub const fn ffa33(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 33."]
        #[inline(always)]
        pub fn set_ffa33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter FIFO assignment for filter 34."]
        #[inline(always)]
        pub const fn ffa34(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 34."]
        #[inline(always)]
        pub fn set_ffa34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter FIFO assignment for filter 35."]
        #[inline(always)]
        pub const fn ffa35(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 35."]
        #[inline(always)]
        pub fn set_ffa35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter FIFO assignment for filter 36."]
        #[inline(always)]
        pub const fn ffa36(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 36."]
        #[inline(always)]
        pub fn set_ffa36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter FIFO assignment for filter 37."]
        #[inline(always)]
        pub const fn ffa37(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 37."]
        #[inline(always)]
        pub fn set_ffa37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter FIFO assignment for filter 38."]
        #[inline(always)]
        pub const fn ffa38(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 38."]
        #[inline(always)]
        pub fn set_ffa38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter FIFO assignment for filter 39."]
        #[inline(always)]
        pub const fn ffa39(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 39."]
        #[inline(always)]
        pub fn set_ffa39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter FIFO assignment for filter 40."]
        #[inline(always)]
        pub const fn ffa40(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 40."]
        #[inline(always)]
        pub fn set_ffa40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter FIFO assignment for filter 41."]
        #[inline(always)]
        pub const fn ffa41(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 41."]
        #[inline(always)]
        pub fn set_ffa41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for FafiforCan3 {
        #[inline(always)]
        fn default() -> FafiforCan3 {
            FafiforCan3(0)
        }
    }
    #[doc = "CAN filter master register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fctlr(pub u32);
    impl Fctlr {
        #[doc = "Filter init mode."]
        #[inline(always)]
        pub const fn finit(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter init mode."]
        #[inline(always)]
        pub fn set_finit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CAN2 start bank."]
        #[inline(always)]
        pub const fn can2sb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "CAN2 start bank."]
        #[inline(always)]
        pub fn set_can2sb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "CAN3 start bank."]
        #[inline(always)]
        pub const fn can3sb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "CAN3 start bank."]
        #[inline(always)]
        pub fn set_can3sb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Fctlr {
        #[inline(always)]
        fn default() -> Fctlr {
            Fctlr(0)
        }
    }
    #[doc = "CAN filter mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmcfgr(pub u32);
    impl Fmcfgr {
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Fmcfgr {
        #[inline(always)]
        fn default() -> Fmcfgr {
            Fmcfgr(0)
        }
    }
    #[doc = "CAN filter mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmcfgrCan3(pub u32);
    impl FmcfgrCan3 {
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm28(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm29(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm30(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm31(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm33(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm34(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm35(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm36(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm37(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm38(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm39(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm40(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub const fn fbm41(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter mode."]
        #[inline(always)]
        pub fn set_fbm41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for FmcfgrCan3 {
        #[inline(always)]
        fn default() -> FmcfgrCan3 {
            FmcfgrCan3(0)
        }
    }
    #[doc = "Filter bank 0 register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fr1(pub u32);
    impl Fr1 {
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Fr1 {
        #[inline(always)]
        fn default() -> Fr1 {
            Fr1(0)
        }
    }
    #[doc = "Filter bank 0 register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fr2(pub u32);
    impl Fr2 {
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn fb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub fn set_fb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Fr2 {
        #[inline(always)]
        fn default() -> Fr2 {
            Fr2(0)
        }
    }
    #[doc = "CAN filter scale register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fscfgr(pub u32);
    impl Fscfgr {
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Fscfgr {
        #[inline(always)]
        fn default() -> Fscfgr {
            Fscfgr(0)
        }
    }
    #[doc = "CAN filter scale register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FscfgrCan3(pub u32);
    impl FscfgrCan3 {
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc28(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc29(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc30(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc31(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc33(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc34(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc35(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc36(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc37(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc38(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc39(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc40(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub const fn fsc41(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration."]
        #[inline(always)]
        pub fn set_fsc41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for FscfgrCan3 {
        #[inline(always)]
        fn default() -> FscfgrCan3 {
            FscfgrCan3(0)
        }
    }
    #[doc = "CAN filter activation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fwr(pub u32);
    impl Fwr {
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Fwr {
        #[inline(always)]
        fn default() -> Fwr {
            Fwr(0)
        }
    }
    #[doc = "CAN filter activation register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FwrCan3(pub u32);
    impl FwrCan3 {
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact28(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact29(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact30(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact31(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact33(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact34(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact35(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact36(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact37(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact38(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact39(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact40(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub const fn fact41(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Filter active."]
        #[inline(always)]
        pub fn set_fact41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for FwrCan3 {
        #[inline(always)]
        fn default() -> FwrCan3 {
            FwrCan3(0)
        }
    }
    #[doc = "CAN interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intenr(pub u32);
    impl Intenr {
        #[doc = "Transmit mailbox empty interrupt enable."]
        #[inline(always)]
        pub const fn tmeie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox empty interrupt enable."]
        #[inline(always)]
        pub fn set_tmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub const fn fmpie0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub fn set_fmpie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn ffie0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub fn set_ffie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub const fn fovie0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub fn set_fovie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub const fn fmpie1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub fn set_fmpie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn ffie1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub fn set_ffie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub const fn fovie1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub fn set_fovie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Error warning interrupt enable."]
        #[inline(always)]
        pub const fn ewgie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Error warning interrupt enable."]
        #[inline(always)]
        pub fn set_ewgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Error passive interrupt enable."]
        #[inline(always)]
        pub const fn epvie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive interrupt enable."]
        #[inline(always)]
        pub fn set_epvie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Bus-off interrupt enable."]
        #[inline(always)]
        pub const fn bofie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-off interrupt enable."]
        #[inline(always)]
        pub fn set_bofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Last error code interrupt enable."]
        #[inline(always)]
        pub const fn lecie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Last error code interrupt enable."]
        #[inline(always)]
        pub fn set_lecie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Wakeup interrupt enable."]
        #[inline(always)]
        pub const fn wkuie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt enable."]
        #[inline(always)]
        pub fn set_wkuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Sleep interrupt enable."]
        #[inline(always)]
        pub const fn slkie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep interrupt enable."]
        #[inline(always)]
        pub fn set_slkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Intenr {
        #[inline(always)]
        fn default() -> Intenr {
            Intenr(0)
        }
    }
    #[doc = "CAN receive FIFO 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfifo(pub u32);
    impl Rfifo {
        #[doc = "FIFO 0 message pending."]
        #[inline(always)]
        pub const fn fmp0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO 0 message pending."]
        #[inline(always)]
        pub fn set_fmp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "FIFO 0 full."]
        #[inline(always)]
        pub const fn full0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 full."]
        #[inline(always)]
        pub fn set_full0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO 0 overrun."]
        #[inline(always)]
        pub const fn fovr0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 overrun."]
        #[inline(always)]
        pub fn set_fovr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Release FIFO 0 output mailbox."]
        #[inline(always)]
        pub const fn rfom0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Release FIFO 0 output mailbox."]
        #[inline(always)]
        pub fn set_rfom0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Rfifo {
        #[inline(always)]
        fn default() -> Rfifo {
            Rfifo(0)
        }
    }
    #[doc = "CAN receive FIFO mailbox data high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdhr(pub u32);
    impl Rxmdhr {
        #[doc = "DATA4."]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DATA4."]
        #[inline(always)]
        pub fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DATA5."]
        #[inline(always)]
        pub const fn data5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "DATA5."]
        #[inline(always)]
        pub fn set_data5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "DATA6."]
        #[inline(always)]
        pub const fn data6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "DATA6."]
        #[inline(always)]
        pub fn set_data6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "DATA7."]
        #[inline(always)]
        pub const fn data7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "DATA7."]
        #[inline(always)]
        pub fn set_data7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rxmdhr {
        #[inline(always)]
        fn default() -> Rxmdhr {
            Rxmdhr(0)
        }
    }
    #[doc = "CAN receive FIFO mailbox data low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdlr(pub u32);
    impl Rxmdlr {
        #[doc = "Data Byte 0."]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 0."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data Byte 1."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 1."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data Byte 2."]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 2."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data Byte 3."]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 3."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rxmdlr {
        #[inline(always)]
        fn default() -> Rxmdlr {
            Rxmdlr(0)
        }
    }
    #[doc = "CAN receive FIFO mailbox data length control and time stamp register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdtr(pub u32);
    impl Rxmdtr {
        #[doc = "Data length code."]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data length code."]
        #[inline(always)]
        pub fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "BRS bit of the received frame."]
        #[inline(always)]
        pub const fn brs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BRS bit of the received frame."]
        #[inline(always)]
        pub fn set_brs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ESI bit of the received frame."]
        #[inline(always)]
        pub const fn esi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ESI bit of the received frame."]
        #[inline(always)]
        pub fn set_esi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RES bit of the received frame."]
        #[inline(always)]
        pub const fn res(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RES bit of the received frame."]
        #[inline(always)]
        pub fn set_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter match index."]
        #[inline(always)]
        pub const fn fmi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Filter match index."]
        #[inline(always)]
        pub fn set_fmi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Rxmdtr {
        #[inline(always)]
        fn default() -> Rxmdtr {
            Rxmdtr(0)
        }
    }
    #[doc = "CAN receive FIFO mailbox identifier register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmir(pub u32);
    impl Rxmir {
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub const fn rtr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub const fn ide(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub fn set_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub const fn stid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub fn set_stid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Rxmir {
        #[inline(always)]
        fn default() -> Rxmir {
            Rxmir(0)
        }
    }
    #[doc = "CAN master status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Initialization acknowledge."]
        #[inline(always)]
        pub const fn inak(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization acknowledge."]
        #[inline(always)]
        pub fn set_inak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sleep acknowledge."]
        #[inline(always)]
        pub const fn slak(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep acknowledge."]
        #[inline(always)]
        pub fn set_slak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error interrupt."]
        #[inline(always)]
        pub const fn erri(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt."]
        #[inline(always)]
        pub fn set_erri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup interrupt."]
        #[inline(always)]
        pub const fn wkui(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt."]
        #[inline(always)]
        pub fn set_wkui(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Sleep acknowledge interrupt."]
        #[inline(always)]
        pub const fn slaki(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep acknowledge interrupt."]
        #[inline(always)]
        pub fn set_slaki(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit mode."]
        #[inline(always)]
        pub const fn txm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mode."]
        #[inline(always)]
        pub fn set_txm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive mode."]
        #[inline(always)]
        pub const fn rxm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive mode."]
        #[inline(always)]
        pub fn set_rxm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Last sample point."]
        #[inline(always)]
        pub const fn samp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Last sample point."]
        #[inline(always)]
        pub fn set_samp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rx signal."]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Rx signal."]
        #[inline(always)]
        pub fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    #[doc = "CAN offline recovery error counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TerrCnt(pub u32);
    impl TerrCnt {
        #[doc = "Offline recovery error count values."]
        #[inline(always)]
        pub const fn tx_err_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Offline recovery error count values."]
        #[inline(always)]
        pub fn set_tx_err_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for TerrCnt {
        #[inline(always)]
        fn default() -> TerrCnt {
            TerrCnt(0)
        }
    }
    #[doc = "CAN transmit status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstatr(pub u32);
    impl Tstatr {
        #[doc = "Request completed mailbox0."]
        #[inline(always)]
        pub const fn rqcp0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox0."]
        #[inline(always)]
        pub fn set_rqcp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmission OK of mailbox0."]
        #[inline(always)]
        pub const fn txok0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox0."]
        #[inline(always)]
        pub fn set_txok0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Arbitration lost for mailbox0."]
        #[inline(always)]
        pub const fn alst0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox0."]
        #[inline(always)]
        pub fn set_alst0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmission error of mailbox0."]
        #[inline(always)]
        pub const fn terr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox0."]
        #[inline(always)]
        pub fn set_terr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Abort request for mailbox0."]
        #[inline(always)]
        pub const fn abrq0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox0."]
        #[inline(always)]
        pub fn set_abrq0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Request completed mailbox1."]
        #[inline(always)]
        pub const fn rqcp1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox1."]
        #[inline(always)]
        pub fn set_rqcp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission OK of mailbox1."]
        #[inline(always)]
        pub const fn txok1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox1."]
        #[inline(always)]
        pub fn set_txok1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Arbitration lost for mailbox1."]
        #[inline(always)]
        pub const fn alst1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox1."]
        #[inline(always)]
        pub fn set_alst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Transmission error of mailbox1."]
        #[inline(always)]
        pub const fn terr1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox1."]
        #[inline(always)]
        pub fn set_terr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Abort request for mailbox 1."]
        #[inline(always)]
        pub const fn abrq1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox 1."]
        #[inline(always)]
        pub fn set_abrq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Request completed mailbox2."]
        #[inline(always)]
        pub const fn rqcp2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox2."]
        #[inline(always)]
        pub fn set_rqcp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Transmission OK of mailbox 2."]
        #[inline(always)]
        pub const fn txok2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox 2."]
        #[inline(always)]
        pub fn set_txok2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Arbitration lost for mailbox 2."]
        #[inline(always)]
        pub const fn alst2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox 2."]
        #[inline(always)]
        pub fn set_alst2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Transmission error of mailbox 2."]
        #[inline(always)]
        pub const fn terr2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox 2."]
        #[inline(always)]
        pub fn set_terr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Abort request for mailbox 2."]
        #[inline(always)]
        pub const fn abrq2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox 2."]
        #[inline(always)]
        pub fn set_abrq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Mailbox code."]
        #[inline(always)]
        pub const fn code(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Mailbox code."]
        #[inline(always)]
        pub fn set_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Transmit mailbox 0 empty."]
        #[inline(always)]
        pub const fn tme0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 0 empty."]
        #[inline(always)]
        pub fn set_tme0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Transmit mailbox 1 empty."]
        #[inline(always)]
        pub const fn tme1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 1 empty."]
        #[inline(always)]
        pub fn set_tme1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Transmit mailbox 2 empty."]
        #[inline(always)]
        pub const fn tme2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 2 empty."]
        #[inline(always)]
        pub fn set_tme2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Lowest priority flag for mailbox0."]
        #[inline(always)]
        pub const fn low0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox0."]
        #[inline(always)]
        pub fn set_low0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Lowest priority flag for mailbox1."]
        #[inline(always)]
        pub const fn low1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox1."]
        #[inline(always)]
        pub fn set_low1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Lowest priority flag for mailbox2."]
        #[inline(always)]
        pub const fn low2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox2."]
        #[inline(always)]
        pub fn set_low2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tstatr {
        #[inline(always)]
        fn default() -> Tstatr {
            Tstatr(0)
        }
    }
    #[doc = "CAN time trigger control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttctlr(pub u32);
    impl Ttctlr {
        #[doc = "Internal counter count end value."]
        #[inline(always)]
        pub const fn timcmv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Internal counter count end value."]
        #[inline(always)]
        pub fn set_timcmv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Internal counter reset control."]
        #[inline(always)]
        pub const fn timrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal counter reset control."]
        #[inline(always)]
        pub fn set_timrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Time-triggered mode selection."]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Time-triggered mode selection."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ttctlr {
        #[inline(always)]
        fn default() -> Ttctlr {
            Ttctlr(0)
        }
    }
    #[doc = "CAN mailbox data high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdhr(pub u32);
    impl Txmdhr {
        #[doc = "Data byte 4."]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 4."]
        #[inline(always)]
        pub fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data byte 5."]
        #[inline(always)]
        pub const fn data5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 5."]
        #[inline(always)]
        pub fn set_data5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data byte 6."]
        #[inline(always)]
        pub const fn data6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 6."]
        #[inline(always)]
        pub fn set_data6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data byte 7."]
        #[inline(always)]
        pub const fn data7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 7."]
        #[inline(always)]
        pub fn set_data7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Txmdhr {
        #[inline(always)]
        fn default() -> Txmdhr {
            Txmdhr(0)
        }
    }
    #[doc = "CAN mailbox data low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdlr(pub u32);
    impl Txmdlr {
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data byte 1."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 1."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data byte 2."]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 2."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data byte 3."]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 3."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Txmdlr {
        #[inline(always)]
        fn default() -> Txmdlr {
            Txmdlr(0)
        }
    }
    #[doc = "CAN mailbox data length control and time stamp register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdtr(pub u32);
    impl Txmdtr {
        #[doc = "Data length code."]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data length code."]
        #[inline(always)]
        pub fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Transmit global time."]
        #[inline(always)]
        pub const fn tgt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit global time."]
        #[inline(always)]
        pub fn set_tgt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Txmdtr {
        #[inline(always)]
        fn default() -> Txmdtr {
            Txmdtr(0)
        }
    }
    #[doc = "CAN TX mailbox identifier register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmir(pub u32);
    impl Txmir {
        #[doc = "Transmit mailbox request."]
        #[inline(always)]
        pub const fn txrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox request."]
        #[inline(always)]
        pub fn set_txrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub const fn rtr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub const fn ide(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub fn set_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub const fn stid_exid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub fn set_stid_exid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Txmir {
        #[inline(always)]
        fn default() -> Txmir {
            Txmir(0)
        }
    }
}
