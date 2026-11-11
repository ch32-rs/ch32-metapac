#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Controller area network."]
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
    #[doc = "CAN Master control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CAN master status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CAN transmit status register."]
    #[inline(always)]
    pub const fn tstatr(self) -> crate::common::Reg<regs::Tstatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CAN receive FIFO 0 register."]
    #[inline(always)]
    pub const fn rfifo0(self) -> crate::common::Reg<regs::Rfifo0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CAN receive FIFO 1 register."]
    #[inline(always)]
    pub const fn rfifo1(self) -> crate::common::Reg<regs::Rfifo1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CAN interrupt enable register."]
    #[inline(always)]
    pub const fn intenr(self) -> crate::common::Reg<regs::Intenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CAN error status register."]
    #[inline(always)]
    pub const fn errsr(self) -> crate::common::Reg<regs::Errsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CAN bit timing register."]
    #[inline(always)]
    pub const fn btimr(self) -> crate::common::Reg<regs::Btimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CAN time trigger control register."]
    #[inline(always)]
    pub const fn ttctlr(self) -> crate::common::Reg<regs::Ttctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CAN Time Trigger Count Value Register."]
    #[inline(always)]
    pub const fn ttcnt(self) -> crate::common::Reg<regs::Ttcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CAN offline recovery error counter."]
    #[inline(always)]
    pub const fn terr_cnt(self) -> crate::common::Reg<regs::TerrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CAN TX mailbox identifier register."]
    #[inline(always)]
    pub const fn txmir(self, n: usize) -> crate::common::Reg<regs::Txmir, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN mailbox data length control and time stamp register."]
    #[inline(always)]
    pub const fn txmdtr(self, n: usize) -> crate::common::Reg<regs::Txmdtr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN mailbox data low register."]
    #[inline(always)]
    pub const fn txmdlr(self, n: usize) -> crate::common::Reg<regs::Txmdlr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN mailbox data high register."]
    #[inline(always)]
    pub const fn txmdhr(self, n: usize) -> crate::common::Reg<regs::Txmdhr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize + n * 16usize) as _)
        }
    }
    #[doc = "CAN receive FIFO mailbox identifier register."]
    #[inline(always)]
    pub const fn rxmir(self, n: usize) -> crate::common::Reg<regs::Rxmir, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN receive FIFO mailbox data length control and time stamp register."]
    #[inline(always)]
    pub const fn rxmdtr(self, n: usize) -> crate::common::Reg<regs::Rxmdtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN receive FIFO mailbox data low register."]
    #[inline(always)]
    pub const fn rxmdlr(self, n: usize) -> crate::common::Reg<regs::Rxmdlr, crate::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize + n * 16usize) as _)
        }
    }
    #[doc = "CAN receive FIFO mailbox data high register."]
    #[inline(always)]
    pub const fn rxmdhr(self, n: usize) -> crate::common::Reg<regs::Rxmdhr, crate::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize + n * 16usize) as _)
        }
    }
    #[doc = "CAN filter master register."]
    #[inline(always)]
    pub const fn fctlr(self) -> crate::common::Reg<regs::Fctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "CAN filter mode register."]
    #[inline(always)]
    pub const fn fmcfgr(self) -> crate::common::Reg<regs::Fmcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "CAN filter scale register."]
    #[inline(always)]
    pub const fn fscfgr(self) -> crate::common::Reg<regs::Fscfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "CAN filter FIFO assignment register."]
    #[inline(always)]
    pub const fn fafifor(self) -> crate::common::Reg<regs::Fafifor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "CAN filter activation register."]
    #[inline(always)]
    pub const fn fwr(self) -> crate::common::Reg<regs::Fwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Filter bank registers (14 banks, 2 words each)."]
    #[inline(always)]
    pub const fn fr(self, n: usize) -> crate::common::Reg<regs::Fr, crate::common::RW> {
        assert!(n < 28usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
}
pub mod regs {
    #[doc = "CAN bit timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btimr(pub u32);
    impl Btimr {
        #[doc = "minimum time unit length setting value."]
        #[must_use]
        #[inline(always)]
        pub const fn brp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "minimum time unit length setting value."]
        #[inline(always)]
        pub const fn set_brp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "CLAS_LONG_TS1=0;TS1 is TS\\[3:0\\](4bit);CLAS_LONG_TS1=1,TS1 is. TS\\[1:0\\]+BTR_TS1_T\\[15:12\\](6bit)."]
        #[must_use]
        #[inline(always)]
        pub const fn btr_ts1_t(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "CLAS_LONG_TS1=0;TS1 is TS\\[3:0\\](4bit);CLAS_LONG_TS1=1,TS1 is. TS\\[1:0\\]+BTR_TS1_T\\[15:12\\](6bit)."]
        #[inline(always)]
        pub const fn set_btr_ts1_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Time segment 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ts1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Time segment 1."]
        #[inline(always)]
        pub const fn set_ts1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Time segment 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ts2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Time segment 2."]
        #[inline(always)]
        pub const fn set_ts2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Resynchronization jump width."]
        #[must_use]
        #[inline(always)]
        pub const fn sjw(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Resynchronization jump width."]
        #[inline(always)]
        pub const fn set_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Loop back mode (debug)."]
        #[must_use]
        #[inline(always)]
        pub const fn lbkm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Loop back mode (debug)."]
        #[inline(always)]
        pub const fn set_lbkm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Silent mode (debug)."]
        #[must_use]
        #[inline(always)]
        pub const fn silm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Silent mode (debug)."]
        #[inline(always)]
        pub const fn set_silm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Btimr {
        #[inline(always)]
        fn default() -> Btimr {
            Btimr(0)
        }
    }
    impl core::fmt::Debug for Btimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btimr")
                .field("brp", &self.brp())
                .field("btr_ts1_t", &self.btr_ts1_t())
                .field("ts1", &self.ts1())
                .field("ts2", &self.ts2())
                .field("sjw", &self.sjw())
                .field("lbkm", &self.lbkm())
                .field("silm", &self.silm())
                .finish()
        }
    }
    #[doc = "CAN Master control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Initialization request."]
        #[must_use]
        #[inline(always)]
        pub const fn inrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization request."]
        #[inline(always)]
        pub const fn set_inrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sleep mode request bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep mode request bit."]
        #[inline(always)]
        pub const fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit FIFO priority."]
        #[must_use]
        #[inline(always)]
        pub const fn txfp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO priority."]
        #[inline(always)]
        pub const fn set_txfp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive FIFO locked mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rflm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO locked mode."]
        #[inline(always)]
        pub const fn set_rflm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No automatic retransmission."]
        #[must_use]
        #[inline(always)]
        pub const fn nart(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No automatic retransmission."]
        #[inline(always)]
        pub const fn set_nart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Automatic wakeup mode."]
        #[must_use]
        #[inline(always)]
        pub const fn awum(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic wakeup mode."]
        #[inline(always)]
        pub const fn set_awum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic bus-off management."]
        #[must_use]
        #[inline(always)]
        pub const fn abom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bus-off management."]
        #[inline(always)]
        pub const fn set_abom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Time triggered communication mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ttcm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Time triggered communication mode."]
        #[inline(always)]
        pub const fn set_ttcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Software master reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Software master reset."]
        #[inline(always)]
        pub const fn set_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Debug freeze."]
        #[must_use]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Debug freeze."]
        #[inline(always)]
        pub const fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    impl core::fmt::Debug for Ctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr")
                .field("inrq", &self.inrq())
                .field("sleep", &self.sleep())
                .field("txfp", &self.txfp())
                .field("rflm", &self.rflm())
                .field("nart", &self.nart())
                .field("awum", &self.awum())
                .field("abom", &self.abom())
                .field("ttcm", &self.ttcm())
                .field("rst", &self.rst())
                .field("dbf", &self.dbf())
                .finish()
        }
    }
    #[doc = "CAN error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errsr(pub u32);
    impl Errsr {
        #[doc = "Error warning flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ewgf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error warning flag."]
        #[inline(always)]
        pub const fn set_ewgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Error passive flag."]
        #[must_use]
        #[inline(always)]
        pub const fn epvf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive flag."]
        #[inline(always)]
        pub const fn set_epvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bus-off flag."]
        #[must_use]
        #[inline(always)]
        pub const fn boff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-off flag."]
        #[inline(always)]
        pub const fn set_boff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Last error code."]
        #[must_use]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Last error code."]
        #[inline(always)]
        pub const fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Least significant byte of the 9-bit transmit error counter."]
        #[must_use]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Least significant byte of the 9-bit transmit error counter."]
        #[inline(always)]
        pub const fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Receive error counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Receive error counter."]
        #[inline(always)]
        pub const fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Errsr {
        #[inline(always)]
        fn default() -> Errsr {
            Errsr(0)
        }
    }
    impl core::fmt::Debug for Errsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errsr")
                .field("ewgf", &self.ewgf())
                .field("epvf", &self.epvf())
                .field("boff", &self.boff())
                .field("lec", &self.lec())
                .field("tec", &self.tec())
                .field("rec", &self.rec())
                .finish()
        }
    }
    #[doc = "CAN filter FIFO assignment register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fafifor(pub u32);
    impl Fafifor {
        #[doc = "FFA"]
        #[must_use]
        #[inline(always)]
        pub const fn ffa(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FFA"]
        #[inline(always)]
        pub const fn set_ffa(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fafifor {
        #[inline(always)]
        fn default() -> Fafifor {
            Fafifor(0)
        }
    }
    impl core::fmt::Debug for Fafifor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fafifor")
                .field("ffa[0]", &self.ffa(0usize))
                .field("ffa[1]", &self.ffa(1usize))
                .field("ffa[2]", &self.ffa(2usize))
                .field("ffa[3]", &self.ffa(3usize))
                .field("ffa[4]", &self.ffa(4usize))
                .field("ffa[5]", &self.ffa(5usize))
                .field("ffa[6]", &self.ffa(6usize))
                .field("ffa[7]", &self.ffa(7usize))
                .field("ffa[8]", &self.ffa(8usize))
                .field("ffa[9]", &self.ffa(9usize))
                .field("ffa[10]", &self.ffa(10usize))
                .field("ffa[11]", &self.ffa(11usize))
                .field("ffa[12]", &self.ffa(12usize))
                .field("ffa[13]", &self.ffa(13usize))
                .finish()
        }
    }
    #[doc = "CAN filter master register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fctlr(pub u32);
    impl Fctlr {
        #[doc = "Filter init mode."]
        #[must_use]
        #[inline(always)]
        pub const fn finit(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Filter init mode."]
        #[inline(always)]
        pub const fn set_finit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Fctlr {
        #[inline(always)]
        fn default() -> Fctlr {
            Fctlr(0)
        }
    }
    impl core::fmt::Debug for Fctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fctlr")
                .field("finit", &self.finit())
                .finish()
        }
    }
    #[doc = "CAN filter mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmcfgr(pub u32);
    impl Fmcfgr {
        #[doc = "FBM"]
        #[must_use]
        #[inline(always)]
        pub const fn fbm(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FBM"]
        #[inline(always)]
        pub const fn set_fbm(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fmcfgr {
        #[inline(always)]
        fn default() -> Fmcfgr {
            Fmcfgr(0)
        }
    }
    impl core::fmt::Debug for Fmcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmcfgr")
                .field("fbm[0]", &self.fbm(0usize))
                .field("fbm[1]", &self.fbm(1usize))
                .field("fbm[2]", &self.fbm(2usize))
                .field("fbm[3]", &self.fbm(3usize))
                .field("fbm[4]", &self.fbm(4usize))
                .field("fbm[5]", &self.fbm(5usize))
                .field("fbm[6]", &self.fbm(6usize))
                .field("fbm[7]", &self.fbm(7usize))
                .field("fbm[8]", &self.fbm(8usize))
                .field("fbm[9]", &self.fbm(9usize))
                .field("fbm[10]", &self.fbm(10usize))
                .field("fbm[11]", &self.fbm(11usize))
                .field("fbm[12]", &self.fbm(12usize))
                .field("fbm[13]", &self.fbm(13usize))
                .finish()
        }
    }
    #[doc = "CAN filter bank register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fr(pub u32);
    impl Fr {
        #[doc = "Filter bits."]
        #[must_use]
        #[inline(always)]
        pub const fn fb(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter bits."]
        #[inline(always)]
        pub const fn set_fb(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fr {
        #[inline(always)]
        fn default() -> Fr {
            Fr(0)
        }
    }
    impl core::fmt::Debug for Fr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fr")
                .field("fb[0]", &self.fb(0usize))
                .field("fb[1]", &self.fb(1usize))
                .field("fb[2]", &self.fb(2usize))
                .field("fb[3]", &self.fb(3usize))
                .field("fb[4]", &self.fb(4usize))
                .field("fb[5]", &self.fb(5usize))
                .field("fb[6]", &self.fb(6usize))
                .field("fb[7]", &self.fb(7usize))
                .field("fb[8]", &self.fb(8usize))
                .field("fb[9]", &self.fb(9usize))
                .field("fb[10]", &self.fb(10usize))
                .field("fb[11]", &self.fb(11usize))
                .field("fb[12]", &self.fb(12usize))
                .field("fb[13]", &self.fb(13usize))
                .field("fb[14]", &self.fb(14usize))
                .field("fb[15]", &self.fb(15usize))
                .field("fb[16]", &self.fb(16usize))
                .field("fb[17]", &self.fb(17usize))
                .field("fb[18]", &self.fb(18usize))
                .field("fb[19]", &self.fb(19usize))
                .field("fb[20]", &self.fb(20usize))
                .field("fb[21]", &self.fb(21usize))
                .field("fb[22]", &self.fb(22usize))
                .field("fb[23]", &self.fb(23usize))
                .field("fb[24]", &self.fb(24usize))
                .field("fb[25]", &self.fb(25usize))
                .field("fb[26]", &self.fb(26usize))
                .field("fb[27]", &self.fb(27usize))
                .field("fb[28]", &self.fb(28usize))
                .field("fb[29]", &self.fb(29usize))
                .field("fb[30]", &self.fb(30usize))
                .field("fb[31]", &self.fb(31usize))
                .finish()
        }
    }
    #[doc = "CAN filter scale register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fscfgr(pub u32);
    impl Fscfgr {
        #[doc = "FSC"]
        #[must_use]
        #[inline(always)]
        pub const fn fsc(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FSC"]
        #[inline(always)]
        pub const fn set_fsc(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fscfgr {
        #[inline(always)]
        fn default() -> Fscfgr {
            Fscfgr(0)
        }
    }
    impl core::fmt::Debug for Fscfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fscfgr")
                .field("fsc[0]", &self.fsc(0usize))
                .field("fsc[1]", &self.fsc(1usize))
                .field("fsc[2]", &self.fsc(2usize))
                .field("fsc[3]", &self.fsc(3usize))
                .field("fsc[4]", &self.fsc(4usize))
                .field("fsc[5]", &self.fsc(5usize))
                .field("fsc[6]", &self.fsc(6usize))
                .field("fsc[7]", &self.fsc(7usize))
                .field("fsc[8]", &self.fsc(8usize))
                .field("fsc[9]", &self.fsc(9usize))
                .field("fsc[10]", &self.fsc(10usize))
                .field("fsc[11]", &self.fsc(11usize))
                .field("fsc[12]", &self.fsc(12usize))
                .field("fsc[13]", &self.fsc(13usize))
                .finish()
        }
    }
    #[doc = "CAN filter activation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fwr(pub u32);
    impl Fwr {
        #[doc = "FACT"]
        #[must_use]
        #[inline(always)]
        pub const fn fact(&self, n: usize) -> bool {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FACT"]
        #[inline(always)]
        pub const fn set_fact(&mut self, n: usize, val: bool) {
            assert!(n < 14usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fwr {
        #[inline(always)]
        fn default() -> Fwr {
            Fwr(0)
        }
    }
    impl core::fmt::Debug for Fwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fwr")
                .field("fact[0]", &self.fact(0usize))
                .field("fact[1]", &self.fact(1usize))
                .field("fact[2]", &self.fact(2usize))
                .field("fact[3]", &self.fact(3usize))
                .field("fact[4]", &self.fact(4usize))
                .field("fact[5]", &self.fact(5usize))
                .field("fact[6]", &self.fact(6usize))
                .field("fact[7]", &self.fact(7usize))
                .field("fact[8]", &self.fact(8usize))
                .field("fact[9]", &self.fact(9usize))
                .field("fact[10]", &self.fact(10usize))
                .field("fact[11]", &self.fact(11usize))
                .field("fact[12]", &self.fact(12usize))
                .field("fact[13]", &self.fact(13usize))
                .finish()
        }
    }
    #[doc = "CAN interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intenr(pub u32);
    impl Intenr {
        #[doc = "Transmit mailbox empty interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmeie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox empty interrupt enable."]
        #[inline(always)]
        pub const fn set_tmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpie0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub const fn set_fmpie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ffie0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_ffie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fovie0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub const fn set_fovie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpie1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO message pending interrupt enable."]
        #[inline(always)]
        pub const fn set_fmpie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ffie1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_ffie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fovie1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overrun interrupt enable."]
        #[inline(always)]
        pub const fn set_fovie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Error warning interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ewgie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Error warning interrupt enable."]
        #[inline(always)]
        pub const fn set_ewgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Error passive interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn epvie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive interrupt enable."]
        #[inline(always)]
        pub const fn set_epvie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Bus-off interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bofie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-off interrupt enable."]
        #[inline(always)]
        pub const fn set_bofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Last error code interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lecie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Last error code interrupt enable."]
        #[inline(always)]
        pub const fn set_lecie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Wakeup interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt enable."]
        #[inline(always)]
        pub const fn set_wkuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Sleep interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn slkie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep interrupt enable."]
        #[inline(always)]
        pub const fn set_slkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Intenr {
        #[inline(always)]
        fn default() -> Intenr {
            Intenr(0)
        }
    }
    impl core::fmt::Debug for Intenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intenr")
                .field("tmeie", &self.tmeie())
                .field("fmpie0", &self.fmpie0())
                .field("ffie0", &self.ffie0())
                .field("fovie0", &self.fovie0())
                .field("fmpie1", &self.fmpie1())
                .field("ffie1", &self.ffie1())
                .field("fovie1", &self.fovie1())
                .field("ewgie", &self.ewgie())
                .field("epvie", &self.epvie())
                .field("bofie", &self.bofie())
                .field("lecie", &self.lecie())
                .field("errie", &self.errie())
                .field("wkuie", &self.wkuie())
                .field("slkie", &self.slkie())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfifo0(pub u32);
    impl Rfifo0 {
        #[doc = "FIFO 0 message pending."]
        #[must_use]
        #[inline(always)]
        pub const fn fmp0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO 0 message pending."]
        #[inline(always)]
        pub const fn set_fmp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "FIFO 0 full."]
        #[must_use]
        #[inline(always)]
        pub const fn full0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 full."]
        #[inline(always)]
        pub const fn set_full0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO 0 overrun."]
        #[must_use]
        #[inline(always)]
        pub const fn fovr0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 overrun."]
        #[inline(always)]
        pub const fn set_fovr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Release FIFO 0 output mailbox."]
        #[must_use]
        #[inline(always)]
        pub const fn rfom0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Release FIFO 0 output mailbox."]
        #[inline(always)]
        pub const fn set_rfom0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Rfifo0 {
        #[inline(always)]
        fn default() -> Rfifo0 {
            Rfifo0(0)
        }
    }
    impl core::fmt::Debug for Rfifo0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfifo0")
                .field("fmp0", &self.fmp0())
                .field("full0", &self.full0())
                .field("fovr0", &self.fovr0())
                .field("rfom0", &self.rfom0())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfifo1(pub u32);
    impl Rfifo1 {
        #[doc = "FIFO 1 message pending."]
        #[must_use]
        #[inline(always)]
        pub const fn fmp1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO 1 message pending."]
        #[inline(always)]
        pub const fn set_fmp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "FIFO 1 full."]
        #[must_use]
        #[inline(always)]
        pub const fn full1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 1 full."]
        #[inline(always)]
        pub const fn set_full1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO 1 overrun."]
        #[must_use]
        #[inline(always)]
        pub const fn fovr1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 1 overrun."]
        #[inline(always)]
        pub const fn set_fovr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Release FIFO 1 output mailbox."]
        #[must_use]
        #[inline(always)]
        pub const fn rfom1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Release FIFO 1 output mailbox."]
        #[inline(always)]
        pub const fn set_rfom1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Rfifo1 {
        #[inline(always)]
        fn default() -> Rfifo1 {
            Rfifo1(0)
        }
    }
    impl core::fmt::Debug for Rfifo1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfifo1")
                .field("fmp1", &self.fmp1())
                .field("full1", &self.full1())
                .field("fovr1", &self.fovr1())
                .field("rfom1", &self.rfom1())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO mailbox data high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdhr(pub u32);
    impl Rxmdhr {
        #[doc = "DATA4."]
        #[must_use]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DATA4."]
        #[inline(always)]
        pub const fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DATA5."]
        #[must_use]
        #[inline(always)]
        pub const fn data5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "DATA5."]
        #[inline(always)]
        pub const fn set_data5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "DATA6."]
        #[must_use]
        #[inline(always)]
        pub const fn data6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "DATA6."]
        #[inline(always)]
        pub const fn set_data6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "DATA7."]
        #[must_use]
        #[inline(always)]
        pub const fn data7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "DATA7."]
        #[inline(always)]
        pub const fn set_data7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rxmdhr {
        #[inline(always)]
        fn default() -> Rxmdhr {
            Rxmdhr(0)
        }
    }
    impl core::fmt::Debug for Rxmdhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxmdhr")
                .field("data4", &self.data4())
                .field("data5", &self.data5())
                .field("data6", &self.data6())
                .field("data7", &self.data7())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO mailbox data low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdlr(pub u32);
    impl Rxmdlr {
        #[doc = "Data Byte 0."]
        #[must_use]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 0."]
        #[inline(always)]
        pub const fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data Byte 1."]
        #[must_use]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 1."]
        #[inline(always)]
        pub const fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data Byte 2."]
        #[must_use]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 2."]
        #[inline(always)]
        pub const fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data Byte 3."]
        #[must_use]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data Byte 3."]
        #[inline(always)]
        pub const fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rxmdlr {
        #[inline(always)]
        fn default() -> Rxmdlr {
            Rxmdlr(0)
        }
    }
    impl core::fmt::Debug for Rxmdlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxmdlr")
                .field("data0", &self.data0())
                .field("data1", &self.data1())
                .field("data2", &self.data2())
                .field("data3", &self.data3())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO mailbox data length control and time stamp register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmdtr(pub u32);
    impl Rxmdtr {
        #[doc = "Data length code."]
        #[must_use]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data length code."]
        #[inline(always)]
        pub const fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "BRS bit of the received frame."]
        #[must_use]
        #[inline(always)]
        pub const fn brs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BRS bit of the received frame."]
        #[inline(always)]
        pub const fn set_brs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ESI bit of the received frame."]
        #[must_use]
        #[inline(always)]
        pub const fn esi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ESI bit of the received frame."]
        #[inline(always)]
        pub const fn set_esi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RES bit of the received frame."]
        #[must_use]
        #[inline(always)]
        pub const fn res(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RES bit of the received frame."]
        #[inline(always)]
        pub const fn set_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Filter match index."]
        #[must_use]
        #[inline(always)]
        pub const fn fmi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Filter match index."]
        #[inline(always)]
        pub const fn set_fmi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Message time stamp."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Rxmdtr {
        #[inline(always)]
        fn default() -> Rxmdtr {
            Rxmdtr(0)
        }
    }
    impl core::fmt::Debug for Rxmdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxmdtr")
                .field("dlc", &self.dlc())
                .field("brs", &self.brs())
                .field("esi", &self.esi())
                .field("res", &self.res())
                .field("fmi", &self.fmi())
                .field("time", &self.time())
                .finish()
        }
    }
    #[doc = "CAN receive FIFO mailbox identifier register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmir(pub u32);
    impl Rxmir {
        #[doc = "Remote transmission request."]
        #[must_use]
        #[inline(always)]
        pub const fn rtr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub const fn set_rtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Identifier extension."]
        #[must_use]
        #[inline(always)]
        pub const fn ide(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub const fn set_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "extended identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub const fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "Standard identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn stid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub const fn set_stid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Rxmir {
        #[inline(always)]
        fn default() -> Rxmir {
            Rxmir(0)
        }
    }
    impl core::fmt::Debug for Rxmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxmir")
                .field("rtr", &self.rtr())
                .field("ide", &self.ide())
                .field("exid", &self.exid())
                .field("stid", &self.stid())
                .finish()
        }
    }
    #[doc = "CAN master status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Initialization acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn inak(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization acknowledge."]
        #[inline(always)]
        pub const fn set_inak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sleep acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn slak(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep acknowledge."]
        #[inline(always)]
        pub const fn set_slak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn erri(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt."]
        #[inline(always)]
        pub const fn set_erri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn wkui(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt."]
        #[inline(always)]
        pub const fn set_wkui(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Sleep acknowledge interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn slaki(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep acknowledge interrupt."]
        #[inline(always)]
        pub const fn set_slaki(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit mode."]
        #[must_use]
        #[inline(always)]
        pub const fn txm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mode."]
        #[inline(always)]
        pub const fn set_txm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rxm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive mode."]
        #[inline(always)]
        pub const fn set_rxm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Last sample point."]
        #[must_use]
        #[inline(always)]
        pub const fn samp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Last sample point."]
        #[inline(always)]
        pub const fn set_samp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rx signal."]
        #[must_use]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Rx signal."]
        #[inline(always)]
        pub const fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    impl core::fmt::Debug for Statr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Statr")
                .field("inak", &self.inak())
                .field("slak", &self.slak())
                .field("erri", &self.erri())
                .field("wkui", &self.wkui())
                .field("slaki", &self.slaki())
                .field("txm", &self.txm())
                .field("rxm", &self.rxm())
                .field("samp", &self.samp())
                .field("rx", &self.rx())
                .finish()
        }
    }
    #[doc = "CAN offline recovery error counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TerrCnt(pub u32);
    impl TerrCnt {
        #[doc = "Offline recovery error count values."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_err_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Offline recovery error count values."]
        #[inline(always)]
        pub const fn set_tx_err_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for TerrCnt {
        #[inline(always)]
        fn default() -> TerrCnt {
            TerrCnt(0)
        }
    }
    impl core::fmt::Debug for TerrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TerrCnt")
                .field("tx_err_cnt", &self.tx_err_cnt())
                .finish()
        }
    }
    #[doc = "CAN transmit status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstatr(pub u32);
    impl Tstatr {
        #[doc = "Request completed mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn rqcp0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox0."]
        #[inline(always)]
        pub const fn set_rqcp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmission OK of mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn txok0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox0."]
        #[inline(always)]
        pub const fn set_txok0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Arbitration lost for mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn alst0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox0."]
        #[inline(always)]
        pub const fn set_alst0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmission error of mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn terr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox0."]
        #[inline(always)]
        pub const fn set_terr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Abort request for mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn abrq0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox0."]
        #[inline(always)]
        pub const fn set_abrq0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Request completed mailbox1."]
        #[must_use]
        #[inline(always)]
        pub const fn rqcp1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox1."]
        #[inline(always)]
        pub const fn set_rqcp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission OK of mailbox1."]
        #[must_use]
        #[inline(always)]
        pub const fn txok1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox1."]
        #[inline(always)]
        pub const fn set_txok1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Arbitration lost for mailbox1."]
        #[must_use]
        #[inline(always)]
        pub const fn alst1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox1."]
        #[inline(always)]
        pub const fn set_alst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Transmission error of mailbox1."]
        #[must_use]
        #[inline(always)]
        pub const fn terr1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox1."]
        #[inline(always)]
        pub const fn set_terr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Abort request for mailbox 1."]
        #[must_use]
        #[inline(always)]
        pub const fn abrq1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox 1."]
        #[inline(always)]
        pub const fn set_abrq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Request completed mailbox2."]
        #[must_use]
        #[inline(always)]
        pub const fn rqcp2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Request completed mailbox2."]
        #[inline(always)]
        pub const fn set_rqcp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Transmission OK of mailbox 2."]
        #[must_use]
        #[inline(always)]
        pub const fn txok2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission OK of mailbox 2."]
        #[inline(always)]
        pub const fn set_txok2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Arbitration lost for mailbox 2."]
        #[must_use]
        #[inline(always)]
        pub const fn alst2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration lost for mailbox 2."]
        #[inline(always)]
        pub const fn set_alst2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Transmission error of mailbox 2."]
        #[must_use]
        #[inline(always)]
        pub const fn terr2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission error of mailbox 2."]
        #[inline(always)]
        pub const fn set_terr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Abort request for mailbox 2."]
        #[must_use]
        #[inline(always)]
        pub const fn abrq2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request for mailbox 2."]
        #[inline(always)]
        pub const fn set_abrq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Mailbox code."]
        #[must_use]
        #[inline(always)]
        pub const fn code(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Mailbox code."]
        #[inline(always)]
        pub const fn set_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Transmit mailbox 0 empty."]
        #[must_use]
        #[inline(always)]
        pub const fn tme0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 0 empty."]
        #[inline(always)]
        pub const fn set_tme0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Transmit mailbox 1 empty."]
        #[must_use]
        #[inline(always)]
        pub const fn tme1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 1 empty."]
        #[inline(always)]
        pub const fn set_tme1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Transmit mailbox 2 empty."]
        #[must_use]
        #[inline(always)]
        pub const fn tme2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox 2 empty."]
        #[inline(always)]
        pub const fn set_tme2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Lowest priority flag for mailbox0."]
        #[must_use]
        #[inline(always)]
        pub const fn low0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox0."]
        #[inline(always)]
        pub const fn set_low0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Lowest priority flag for mailbox1."]
        #[must_use]
        #[inline(always)]
        pub const fn low1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox1."]
        #[inline(always)]
        pub const fn set_low1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Lowest priority flag for mailbox2."]
        #[must_use]
        #[inline(always)]
        pub const fn low2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox2."]
        #[inline(always)]
        pub const fn set_low2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tstatr {
        #[inline(always)]
        fn default() -> Tstatr {
            Tstatr(0)
        }
    }
    impl core::fmt::Debug for Tstatr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tstatr")
                .field("rqcp0", &self.rqcp0())
                .field("txok0", &self.txok0())
                .field("alst0", &self.alst0())
                .field("terr0", &self.terr0())
                .field("abrq0", &self.abrq0())
                .field("rqcp1", &self.rqcp1())
                .field("txok1", &self.txok1())
                .field("alst1", &self.alst1())
                .field("terr1", &self.terr1())
                .field("abrq1", &self.abrq1())
                .field("rqcp2", &self.rqcp2())
                .field("txok2", &self.txok2())
                .field("alst2", &self.alst2())
                .field("terr2", &self.terr2())
                .field("abrq2", &self.abrq2())
                .field("code", &self.code())
                .field("tme0", &self.tme0())
                .field("tme1", &self.tme1())
                .field("tme2", &self.tme2())
                .field("low0", &self.low0())
                .field("low1", &self.low1())
                .field("low2", &self.low2())
                .finish()
        }
    }
    #[doc = "CAN Time Trigger Count Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcnt(pub u32);
    impl Ttcnt {
        #[doc = "time-triggered count value."]
        #[must_use]
        #[inline(always)]
        pub const fn timcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "time-triggered count value."]
        #[inline(always)]
        pub const fn set_timcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ttcnt {
        #[inline(always)]
        fn default() -> Ttcnt {
            Ttcnt(0)
        }
    }
    impl core::fmt::Debug for Ttcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttcnt")
                .field("timcnt", &self.timcnt())
                .finish()
        }
    }
    #[doc = "CAN time trigger control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttctlr(pub u32);
    impl Ttctlr {
        #[doc = "Internal counter count end value."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Internal counter count end value."]
        #[inline(always)]
        pub const fn set_timcmv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Internal counter reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn timrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal counter reset control."]
        #[inline(always)]
        pub const fn set_timrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Time-triggered mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Time-triggered mode selection."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ttctlr {
        #[inline(always)]
        fn default() -> Ttctlr {
            Ttctlr(0)
        }
    }
    impl core::fmt::Debug for Ttctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttctlr")
                .field("timcmv", &self.timcmv())
                .field("timrst", &self.timrst())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[doc = "CAN mailbox data high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdhr(pub u32);
    impl Txmdhr {
        #[doc = "Data byte 4."]
        #[must_use]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 4."]
        #[inline(always)]
        pub const fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data byte 5."]
        #[must_use]
        #[inline(always)]
        pub const fn data5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 5."]
        #[inline(always)]
        pub const fn set_data5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data byte 6."]
        #[must_use]
        #[inline(always)]
        pub const fn data6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 6."]
        #[inline(always)]
        pub const fn set_data6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data byte 7."]
        #[must_use]
        #[inline(always)]
        pub const fn data7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 7."]
        #[inline(always)]
        pub const fn set_data7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Txmdhr {
        #[inline(always)]
        fn default() -> Txmdhr {
            Txmdhr(0)
        }
    }
    impl core::fmt::Debug for Txmdhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txmdhr")
                .field("data4", &self.data4())
                .field("data5", &self.data5())
                .field("data6", &self.data6())
                .field("data7", &self.data7())
                .finish()
        }
    }
    #[doc = "CAN mailbox data low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdlr(pub u32);
    impl Txmdlr {
        #[doc = "Data byte 0."]
        #[must_use]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub const fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data byte 1."]
        #[must_use]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 1."]
        #[inline(always)]
        pub const fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data byte 2."]
        #[must_use]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 2."]
        #[inline(always)]
        pub const fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data byte 3."]
        #[must_use]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 3."]
        #[inline(always)]
        pub const fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Txmdlr {
        #[inline(always)]
        fn default() -> Txmdlr {
            Txmdlr(0)
        }
    }
    impl core::fmt::Debug for Txmdlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txmdlr")
                .field("data0", &self.data0())
                .field("data1", &self.data1())
                .field("data2", &self.data2())
                .field("data3", &self.data3())
                .finish()
        }
    }
    #[doc = "CAN mailbox data length control and time stamp register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmdtr(pub u32);
    impl Txmdtr {
        #[doc = "Data length code."]
        #[must_use]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data length code."]
        #[inline(always)]
        pub const fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Transmit global time."]
        #[must_use]
        #[inline(always)]
        pub const fn tgt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit global time."]
        #[inline(always)]
        pub const fn set_tgt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Message time stamp."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Message time stamp."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Txmdtr {
        #[inline(always)]
        fn default() -> Txmdtr {
            Txmdtr(0)
        }
    }
    impl core::fmt::Debug for Txmdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txmdtr")
                .field("dlc", &self.dlc())
                .field("tgt", &self.tgt())
                .field("time", &self.time())
                .finish()
        }
    }
    #[doc = "CAN TX mailbox identifier register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmir(pub u32);
    impl Txmir {
        #[doc = "Transmit mailbox request."]
        #[must_use]
        #[inline(always)]
        pub const fn txrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mailbox request."]
        #[inline(always)]
        pub const fn set_txrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Remote transmission request."]
        #[must_use]
        #[inline(always)]
        pub const fn rtr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote transmission request."]
        #[inline(always)]
        pub const fn set_rtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Identifier extension."]
        #[must_use]
        #[inline(always)]
        pub const fn ide(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Identifier extension."]
        #[inline(always)]
        pub const fn set_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "extended identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "extended identifier."]
        #[inline(always)]
        pub const fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "Standard identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn stid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "Standard identifier."]
        #[inline(always)]
        pub const fn set_stid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Txmir {
        #[inline(always)]
        fn default() -> Txmir {
            Txmir(0)
        }
    }
    impl core::fmt::Debug for Txmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txmir")
                .field("txrq", &self.txrq())
                .field("rtr", &self.rtr())
                .field("ide", &self.ide())
                .field("exid", &self.exid())
                .field("stid", &self.stid())
                .finish()
        }
    }
}
