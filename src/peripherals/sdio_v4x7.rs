#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Secure digital input/output interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio {
    ptr: *mut u8,
}
unsafe impl Send for Sdio {}
unsafe impl Sync for Sdio {}
impl Sdio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 1:0 = PWRCTRL: Power supply control bits."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SDI clock control register (SDIO_CLKCR)."]
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::Clkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 31:0 = : Command argument."]
    #[inline(always)]
    pub const fn arg(self) -> crate::common::Reg<regs::Arg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SDIO command register (SDIO_CMD)."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SDIO command register."]
    #[inline(always)]
    pub const fn respcmd(self) -> crate::common::Reg<regs::Respcmd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 31:0 = CARDSTATUS1."]
    #[inline(always)]
    pub const fn resp(self, n: usize) -> crate::common::Reg<regs::Resp, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "Bits 31:0 = DATATIME: Data timeout period."]
    #[inline(always)]
    pub const fn dtimer(self) -> crate::common::Reg<regs::Dtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Bits 24:0 = DATALENGTH: Data length value."]
    #[inline(always)]
    pub const fn dlen(self) -> crate::common::Reg<regs::Dlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SDIO data control register (SDIO_DCTRL)."]
    #[inline(always)]
    pub const fn dctrl(self) -> crate::common::Reg<regs::Dctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Bits 24:0 = DATACOUNT: Data count value."]
    #[inline(always)]
    pub const fn dcount(self) -> crate::common::Reg<regs::Dcount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SDIO status register (SDIO_STA)."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SDIO interrupt clear register (SDIO_ICR)."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SDIO mask register (SDIO_MASK)."]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO."]
    #[inline(always)]
    pub const fn fifocnt(self) -> crate::common::Reg<regs::Fifocnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Data control register 2."]
    #[inline(always)]
    pub const fn dctrl2(self) -> crate::common::Reg<regs::Dctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
}
pub mod regs {
    #[doc = "Bits 31:0 = : Command argument."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arg(pub u32);
    impl Arg {
        #[doc = "Command argument."]
        #[inline(always)]
        pub const fn cmdarg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Command argument."]
        #[inline(always)]
        pub fn set_cmdarg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Arg {
        #[inline(always)]
        fn default() -> Arg {
            Arg(0)
        }
    }
    #[doc = "SDI clock control register (SDIO_CLKCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkcr(pub u32);
    impl Clkcr {
        #[doc = "Clock divide factor."]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock divide factor."]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Clock enable bit."]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable bit."]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Power saving configuration bit."]
        #[inline(always)]
        pub const fn pwrsav(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Power saving configuration bit."]
        #[inline(always)]
        pub fn set_pwrsav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock divider bypass enable bit."]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock divider bypass enable bit."]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wide bus mode enable bit."]
        #[inline(always)]
        pub const fn widbus(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Wide bus mode enable bit."]
        #[inline(always)]
        pub fn set_widbus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "SDIO_CK dephasing selection bit."]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO_CK dephasing selection bit."]
        #[inline(always)]
        pub fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HW Flow Control enable."]
        #[inline(always)]
        pub const fn hwfc_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HW Flow Control enable."]
        #[inline(always)]
        pub fn set_hwfc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Clkcr {
        #[inline(always)]
        fn default() -> Clkcr {
            Clkcr(0)
        }
    }
    #[doc = "SDIO command register (SDIO_CMD)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Command index."]
        #[inline(always)]
        pub const fn cmdindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Command index."]
        #[inline(always)]
        pub fn set_cmdindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Wait for response bits."]
        #[inline(always)]
        pub const fn waitresp(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Wait for response bits."]
        #[inline(always)]
        pub fn set_waitresp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "CPSM waits for interrupt request."]
        #[inline(always)]
        pub const fn waitint(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM waits for interrupt request."]
        #[inline(always)]
        pub fn set_waitint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)."]
        #[inline(always)]
        pub const fn waitpend(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal)."]
        #[inline(always)]
        pub fn set_waitpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Command path state machine (CPSM) Enable bit."]
        #[inline(always)]
        pub const fn cpsmen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine (CPSM) Enable bit."]
        #[inline(always)]
        pub fn set_cpsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O suspend command."]
        #[inline(always)]
        pub const fn sdiosuspend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O suspend command."]
        #[inline(always)]
        pub fn set_sdiosuspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable CMD completion."]
        #[inline(always)]
        pub const fn encmdcompl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CMD completion."]
        #[inline(always)]
        pub fn set_encmdcompl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "not Interrupt Enable."]
        #[inline(always)]
        pub const fn n_ien(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "not Interrupt Enable."]
        #[inline(always)]
        pub fn set_n_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CE-ATA command."]
        #[inline(always)]
        pub const fn ce_atacmd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CE-ATA command."]
        #[inline(always)]
        pub fn set_ce_atacmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    #[doc = "Bits 24:0 = DATACOUNT: Data count value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcount(pub u32);
    impl Dcount {
        #[doc = "Data count value."]
        #[inline(always)]
        pub const fn datacount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data count value."]
        #[inline(always)]
        pub fn set_datacount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dcount {
        #[inline(always)]
        fn default() -> Dcount {
            Dcount(0)
        }
    }
    #[doc = "SDIO data control register (SDIO_DCTRL)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctrl(pub u32);
    impl Dctrl {
        #[doc = "Data transfer enabled bit."]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer enabled bit."]
        #[inline(always)]
        pub fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transfer direction selection."]
        #[inline(always)]
        pub const fn dtdir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction selection."]
        #[inline(always)]
        pub fn set_dtdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
        #[inline(always)]
        pub const fn dtmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
        #[inline(always)]
        pub fn set_dtmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable bit."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable bit."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data block size."]
        #[inline(always)]
        pub const fn dblocksize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data block size."]
        #[inline(always)]
        pub fn set_dblocksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Read wait start."]
        #[inline(always)]
        pub const fn pwstart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait start."]
        #[inline(always)]
        pub fn set_pwstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Read wait stop."]
        #[inline(always)]
        pub const fn pwstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait stop."]
        #[inline(always)]
        pub fn set_pwstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read wait mode."]
        #[inline(always)]
        pub const fn rwmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read wait mode."]
        #[inline(always)]
        pub fn set_rwmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O enable functions."]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O enable functions."]
        #[inline(always)]
        pub fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dctrl {
        #[inline(always)]
        fn default() -> Dctrl {
            Dctrl(0)
        }
    }
    #[doc = "Data control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctrl2(pub u32);
    impl Dctrl2 {
        #[doc = "Data block length for an arbitrary byte length transfer."]
        #[inline(always)]
        pub const fn dblocksize2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data block length for an arbitrary byte length transfer."]
        #[inline(always)]
        pub fn set_dblocksize2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Arbitrary byte length transfer enable."]
        #[inline(always)]
        pub const fn random_len_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitrary byte length transfer enable."]
        #[inline(always)]
        pub fn set_random_len_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Slave mode enable."]
        #[inline(always)]
        pub const fn slv_mode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Slave mode enable."]
        #[inline(always)]
        pub fn set_slv_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Force data block CRC errors in slave mode."]
        #[inline(always)]
        pub const fn slv_force_err(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Force data block CRC errors in slave mode."]
        #[inline(always)]
        pub fn set_slv_force_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Data output clock phase in slave mode."]
        #[inline(always)]
        pub const fn slv_ck_phase(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Data output clock phase in slave mode."]
        #[inline(always)]
        pub fn set_slv_ck_phase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dctrl2 {
        #[inline(always)]
        fn default() -> Dctrl2 {
            Dctrl2(0)
        }
    }
    #[doc = "Bits 24:0 = DATALENGTH: Data length value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlen(pub u32);
    impl Dlen {
        #[doc = "Data length value."]
        #[inline(always)]
        pub const fn datalength(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data length value."]
        #[inline(always)]
        pub fn set_datalength(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dlen {
        #[inline(always)]
        fn default() -> Dlen {
            Dlen(0)
        }
    }
    #[doc = "Bits 31:0 = DATATIME: Data timeout period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtimer(pub u32);
    impl Dtimer {
        #[doc = "Data timeout period."]
        #[inline(always)]
        pub const fn datatime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data timeout period."]
        #[inline(always)]
        pub fn set_datatime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dtimer {
        #[inline(always)]
        fn default() -> Dtimer {
            Dtimer(0)
        }
    }
    #[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "Receive and transmit FIFO data."]
        #[inline(always)]
        pub const fn fifodata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive and transmit FIFO data."]
        #[inline(always)]
        pub fn set_fifodata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    #[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifocnt(pub u32);
    impl Fifocnt {
        #[doc = "Remaining number of words to be written to or read from the FIFO."]
        #[inline(always)]
        pub const fn fif0count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Remaining number of words to be written to or read from the FIFO."]
        #[inline(always)]
        pub fn set_fif0count(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifocnt {
        #[inline(always)]
        fn default() -> Fifocnt {
            Fifocnt(0)
        }
    }
    #[doc = "SDIO interrupt clear register (SDIO_ICR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "CCRCFAIL flag clear bit."]
        #[inline(always)]
        pub const fn ccrcfailc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCRCFAIL flag clear bit."]
        #[inline(always)]
        pub fn set_ccrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCRCFAIL flag clear bit."]
        #[inline(always)]
        pub const fn dcrcfailc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCRCFAIL flag clear bit."]
        #[inline(always)]
        pub fn set_dcrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CTIMEOUT flag clear bit."]
        #[inline(always)]
        pub const fn ctimeoutc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CTIMEOUT flag clear bit."]
        #[inline(always)]
        pub fn set_ctimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DTIMEOUT flag clear bit."]
        #[inline(always)]
        pub const fn dtimeoutc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTIMEOUT flag clear bit."]
        #[inline(always)]
        pub fn set_dtimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TXUNDERR flag clear bit."]
        #[inline(always)]
        pub const fn txunderrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TXUNDERR flag clear bit."]
        #[inline(always)]
        pub fn set_txunderrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXOVERR flag clear bit."]
        #[inline(always)]
        pub const fn rxoverrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXOVERR flag clear bit."]
        #[inline(always)]
        pub fn set_rxoverrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMDREND flag clear bit."]
        #[inline(always)]
        pub const fn cmdrendc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMDREND flag clear bit."]
        #[inline(always)]
        pub fn set_cmdrendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMDSENT flag clear bit."]
        #[inline(always)]
        pub const fn cmdsentc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMDSENT flag clear bit."]
        #[inline(always)]
        pub fn set_cmdsentc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DATAEND flag clear bit."]
        #[inline(always)]
        pub const fn dataendc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DATAEND flag clear bit."]
        #[inline(always)]
        pub fn set_dataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "STBITERR flag clear bit."]
        #[inline(always)]
        pub const fn stbiterrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "STBITERR flag clear bit."]
        #[inline(always)]
        pub fn set_stbiterrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DBCKEND flag clear bit."]
        #[inline(always)]
        pub const fn dbckendc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DBCKEND flag clear bit."]
        #[inline(always)]
        pub fn set_dbckendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDIOIT flag clear bit."]
        #[inline(always)]
        pub const fn sdioitc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIOIT flag clear bit."]
        #[inline(always)]
        pub fn set_sdioitc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CEATAEND flag clear bit."]
        #[inline(always)]
        pub const fn ceataendc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "CEATAEND flag clear bit."]
        #[inline(always)]
        pub fn set_ceataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[doc = "SDIO mask register (SDIO_MASK)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mask(pub u32);
    impl Mask {
        #[doc = "Command CRC fail interrupt enable."]
        #[inline(always)]
        pub const fn ccrcfailie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command CRC fail interrupt enable."]
        #[inline(always)]
        pub fn set_ccrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data CRC fail interrupt enable."]
        #[inline(always)]
        pub const fn dcrcfailie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data CRC fail interrupt enable."]
        #[inline(always)]
        pub fn set_dcrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command timeout interrupt enable."]
        #[inline(always)]
        pub const fn ctimeoutie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command timeout interrupt enable."]
        #[inline(always)]
        pub fn set_ctimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout interrupt enable."]
        #[inline(always)]
        pub const fn dtimeoutie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout interrupt enable."]
        #[inline(always)]
        pub fn set_dtimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tx FIFO underrun error interrupt enable."]
        #[inline(always)]
        pub const fn txunderrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO underrun error interrupt enable."]
        #[inline(always)]
        pub fn set_txunderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO overrun error interrupt enable."]
        #[inline(always)]
        pub const fn rxoverrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO overrun error interrupt enable."]
        #[inline(always)]
        pub fn set_rxoverrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received interrupt enable."]
        #[inline(always)]
        pub const fn cmdrendie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received interrupt enable."]
        #[inline(always)]
        pub fn set_cmdrendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent interrupt enable."]
        #[inline(always)]
        pub const fn cmdsentie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent interrupt enable."]
        #[inline(always)]
        pub fn set_cmdsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end interrupt enable."]
        #[inline(always)]
        pub const fn dataendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end interrupt enable."]
        #[inline(always)]
        pub fn set_dataendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start bit error interrupt enable."]
        #[inline(always)]
        pub const fn stbiterrie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit error interrupt enable."]
        #[inline(always)]
        pub fn set_stbiterrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block end interrupt enable."]
        #[inline(always)]
        pub const fn dbackendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block end interrupt enable."]
        #[inline(always)]
        pub fn set_dbackendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Command acting interrupt enable."]
        #[inline(always)]
        pub const fn cmdactie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Command acting interrupt enable."]
        #[inline(always)]
        pub fn set_cmdactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data transmit acting interrupt enable."]
        #[inline(always)]
        pub const fn txactie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmit acting interrupt enable."]
        #[inline(always)]
        pub fn set_txactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data receive acting interrupt enable."]
        #[inline(always)]
        pub const fn rxactie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Data receive acting interrupt enable."]
        #[inline(always)]
        pub fn set_rxactie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx FIFO half empty interrupt enable."]
        #[inline(always)]
        pub const fn txfifoheie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO half empty interrupt enable."]
        #[inline(always)]
        pub fn set_txfifoheie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rx FIFO half full interrupt enable."]
        #[inline(always)]
        pub const fn rxfifohfie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO half full interrupt enable."]
        #[inline(always)]
        pub fn set_rxfifohfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Tx FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn txfifofie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO full interrupt enable."]
        #[inline(always)]
        pub fn set_txfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Rx FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn rxfifofie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO full interrupt enable."]
        #[inline(always)]
        pub fn set_rxfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Tx FIFO empty interrupt enable."]
        #[inline(always)]
        pub const fn txfifoeie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty interrupt enable."]
        #[inline(always)]
        pub fn set_txfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Rx FIFO empty interrupt enable."]
        #[inline(always)]
        pub const fn rxfifoeie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO empty interrupt enable."]
        #[inline(always)]
        pub fn set_rxfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data available in Tx FIFO interrupt enable."]
        #[inline(always)]
        pub const fn txdavlie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in Tx FIFO interrupt enable."]
        #[inline(always)]
        pub fn set_txdavlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data available in Rx FIFO interrupt enable."]
        #[inline(always)]
        pub const fn rxdavlie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in Rx FIFO interrupt enable."]
        #[inline(always)]
        pub fn set_rxdavlie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO mode interrupt received interrupt enable."]
        #[inline(always)]
        pub const fn sdioitie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO mode interrupt received interrupt enable."]
        #[inline(always)]
        pub fn set_sdioitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CE-ATA command completion signal received interrupt enable."]
        #[inline(always)]
        pub const fn ceatendie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "CE-ATA command completion signal received interrupt enable."]
        #[inline(always)]
        pub fn set_ceatendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Mask {
        #[inline(always)]
        fn default() -> Mask {
            Mask(0)
        }
    }
    #[doc = "Bits 1:0 = PWRCTRL: Power supply control bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u32);
    impl Power {
        #[doc = "Power supply control bits."]
        #[inline(always)]
        pub const fn pwrctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Power supply control bits."]
        #[inline(always)]
        pub fn set_pwrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    #[doc = "Bits 31:0 = CARDSTATUS1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resp(pub u32);
    impl Resp {
        #[doc = "Card status 1."]
        #[inline(always)]
        pub const fn cardstatus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Card status 1."]
        #[inline(always)]
        pub fn set_cardstatus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Resp {
        #[inline(always)]
        fn default() -> Resp {
            Resp(0)
        }
    }
    #[doc = "SDIO command register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Respcmd(pub u32);
    impl Respcmd {
        #[doc = "Response command index."]
        #[inline(always)]
        pub const fn respcmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Response command index."]
        #[inline(always)]
        pub fn set_respcmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Respcmd {
        #[inline(always)]
        fn default() -> Respcmd {
            Respcmd(0)
        }
    }
    #[doc = "SDIO status register (SDIO_STA)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Command response received (CRC check failed)."]
        #[inline(always)]
        pub const fn ccrcfail(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check failed)."]
        #[inline(always)]
        pub fn set_ccrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data block sent/received (CRC check failed)."]
        #[inline(always)]
        pub const fn dcrcfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check failed)."]
        #[inline(always)]
        pub fn set_dcrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command response timeout."]
        #[inline(always)]
        pub const fn ctimeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command response timeout."]
        #[inline(always)]
        pub fn set_ctimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout."]
        #[inline(always)]
        pub const fn dtimeout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout."]
        #[inline(always)]
        pub fn set_dtimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit FIFO underrun error."]
        #[inline(always)]
        pub const fn txunderr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO underrun error."]
        #[inline(always)]
        pub fn set_txunderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Received FIFO overrun error."]
        #[inline(always)]
        pub const fn rxoverr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received FIFO overrun error."]
        #[inline(always)]
        pub fn set_rxoverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received (CRC check passed)."]
        #[inline(always)]
        pub const fn cmdrend(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check passed)."]
        #[inline(always)]
        pub fn set_cmdrend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent (no response required)."]
        #[inline(always)]
        pub const fn cmdsent(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent (no response required)."]
        #[inline(always)]
        pub fn set_cmdsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end (data counter, SDIDCOUNT, is zero)."]
        #[inline(always)]
        pub const fn dataend(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end (data counter, SDIDCOUNT, is zero)."]
        #[inline(always)]
        pub fn set_dataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start bit not detected on all data signals in wide bus mode."]
        #[inline(always)]
        pub const fn stbiterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start bit not detected on all data signals in wide bus mode."]
        #[inline(always)]
        pub fn set_stbiterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block sent/received (CRC check passed)."]
        #[inline(always)]
        pub const fn dbckend(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check passed)."]
        #[inline(always)]
        pub fn set_dbckend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Command transfer in progress."]
        #[inline(always)]
        pub const fn cmdact(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer in progress."]
        #[inline(always)]
        pub fn set_cmdact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data transmit in progress."]
        #[inline(always)]
        pub const fn txact(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmit in progress."]
        #[inline(always)]
        pub fn set_txact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data receive in progress."]
        #[inline(always)]
        pub const fn rxact(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Data receive in progress."]
        #[inline(always)]
        pub fn set_rxact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO."]
        #[inline(always)]
        pub const fn txfifohe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO half empty: at least 8 words can be written into the FIFO."]
        #[inline(always)]
        pub fn set_txfifohe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO."]
        #[inline(always)]
        pub const fn rxfifohf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO half full: there are at least 8 words in the FIFO."]
        #[inline(always)]
        pub fn set_rxfifohf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmit FIFO full."]
        #[inline(always)]
        pub const fn txfifof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO full."]
        #[inline(always)]
        pub fn set_txfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive FIFO full."]
        #[inline(always)]
        pub const fn rxfifof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO full."]
        #[inline(always)]
        pub fn set_rxfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Transmit FIFO empty."]
        #[inline(always)]
        pub const fn txfifoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO empty."]
        #[inline(always)]
        pub fn set_txfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receive FIFO empty."]
        #[inline(always)]
        pub const fn rxfifoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO empty."]
        #[inline(always)]
        pub fn set_rxfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data available in transmit FIFO."]
        #[inline(always)]
        pub const fn txdavl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in transmit FIFO."]
        #[inline(always)]
        pub fn set_txdavl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data available in receive FIFO."]
        #[inline(always)]
        pub const fn rxdavl(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Data available in receive FIFO."]
        #[inline(always)]
        pub fn set_rxdavl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO interrupt received."]
        #[inline(always)]
        pub const fn sdioit(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO interrupt received."]
        #[inline(always)]
        pub fn set_sdioit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CE-ATA command completion signal received for CMD61."]
        #[inline(always)]
        pub const fn ceataend(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "CE-ATA command completion signal received for CMD61."]
        #[inline(always)]
        pub fn set_ceataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
}
