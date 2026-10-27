#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PSRAM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psram {
    ptr: *mut u8,
}
unsafe impl Send for Psram {}
unsafe impl Sync for Psram {}
impl Psram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PSRAM control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PSRAM time series register."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PSRAM DATA Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PSRAM Address Register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PSRAM Command Word 0 Configuration Register."]
    #[inline(always)]
    pub const fn cmd0_cfg(self) -> crate::common::Reg<regs::Cmd0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PSRAM Command Word 1 Configuration Register."]
    #[inline(always)]
    pub const fn cmd1_cfg(self) -> crate::common::Reg<regs::Cmd1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PSRAM DMA Control Register."]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PSRAM Data Transmission Quantity Register."]
    #[inline(always)]
    pub const fn data_num(self) -> crate::common::Reg<regs::DataNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Start address register of memory during DMA transmission."]
    #[inline(always)]
    pub const fn dma_mem_addr(self) -> crate::common::Reg<regs::DmaMemAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Start Address Register of PSRAM during DMA Transmission."]
    #[inline(always)]
    pub const fn dma_psram_addr(self) -> crate::common::Reg<regs::DmaPsramAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PSRAM Interrupt State Register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PSRAM Interrupt Error Enable Register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "PSRAM controller state machine register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PSRAM CE Pulse Configuration Register."]
    #[inline(always)]
    pub const fn cecfg(self) -> crate::common::Reg<regs::Cecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PSRAM Read/Write Delay Register."]
    #[inline(always)]
    pub const fn latency(self) -> crate::common::Reg<regs::Latency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "PSRAM Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "BUSY."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BUSY."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Inform PSRAM the nest operation."]
        #[inline(always)]
        pub const fn mw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Inform PSRAM the nest operation."]
        #[inline(always)]
        pub fn set_mw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Commands executed."]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Commands executed."]
        #[inline(always)]
        pub fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "The register 0 address to be manipulated."]
        #[inline(always)]
        pub const fn mr_addr0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The register 0 address to be manipulated."]
        #[inline(always)]
        pub fn set_mr_addr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    #[doc = "PSRAM CE Pulse Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cecfg(pub u32);
    impl Cecfg {
        #[doc = "Maximum duration of CE low pulse. (excluding withdrawal of half sleep): T= N*PSRAM clock period."]
        #[inline(always)]
        pub const fn celpd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Maximum duration of CE low pulse. (excluding withdrawal of half sleep): T= N*PSRAM clock period."]
        #[inline(always)]
        pub fn set_celpd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "CE pulse width detection enables."]
        #[inline(always)]
        pub const fn cepwen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CE pulse width detection enables."]
        #[inline(always)]
        pub fn set_cepwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cecfg {
        #[inline(always)]
        fn default() -> Cecfg {
            Cecfg(0)
        }
    }
    #[doc = "PSRAM Command Word 0 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd0Cfg(pub u32);
    impl Cmd0Cfg {
        #[doc = "Configurable synchronous read commands."]
        #[inline(always)]
        pub const fn sr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous read commands."]
        #[inline(always)]
        pub fn set_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Configurable synchronous write commands."]
        #[inline(always)]
        pub const fn sw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous write commands."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Configurable synchronous read (linear burst) commands."]
        #[inline(always)]
        pub const fn sr_lb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous read (linear burst) commands."]
        #[inline(always)]
        pub fn set_sr_lb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Configurable synchronous write (linear burst) commands."]
        #[inline(always)]
        pub const fn sw_lb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous write (linear burst) commands."]
        #[inline(always)]
        pub fn set_sw_lb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Cmd0Cfg {
        #[inline(always)]
        fn default() -> Cmd0Cfg {
            Cmd0Cfg(0)
        }
    }
    #[doc = "PSRAM Command Word 1 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd1Cfg(pub u32);
    impl Cmd1Cfg {
        #[doc = "Configurable register read commands."]
        #[inline(always)]
        pub const fn mregr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable register read commands."]
        #[inline(always)]
        pub fn set_mregr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Configurable register write commands."]
        #[inline(always)]
        pub const fn mregw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable register write commands."]
        #[inline(always)]
        pub fn set_mregw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Configurable global reset commands."]
        #[inline(always)]
        pub const fn glrst(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable global reset commands."]
        #[inline(always)]
        pub fn set_glrst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Configurable refresh command command."]
        #[inline(always)]
        pub const fn refresh(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable refresh command command."]
        #[inline(always)]
        pub fn set_refresh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Cmd1Cfg {
        #[inline(always)]
        fn default() -> Cmd1Cfg {
            Cmd1Cfg(0)
        }
    }
    #[doc = "PSRAM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Empty Read FIFO Method Select."]
        #[inline(always)]
        pub const fn cfifo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Empty Read FIFO Method Select."]
        #[inline(always)]
        pub fn set_cfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA write operation for clearing FIFO mode Selection."]
        #[inline(always)]
        pub const fn dfifo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA write operation for clearing FIFO mode Selection."]
        #[inline(always)]
        pub fn set_dfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PSRAM capacity configuration."]
        #[inline(always)]
        pub const fn cap_cfg(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "PSRAM capacity configuration."]
        #[inline(always)]
        pub fn set_cap_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "PSRAM exits the current low-power state enable."]
        #[inline(always)]
        pub const fn exit_lpmd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PSRAM exits the current low-power state enable."]
        #[inline(always)]
        pub fn set_exit_lpmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "PSRAM DATA Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u16);
    impl Data {
        #[doc = "Low-byte data written or read."]
        #[inline(always)]
        pub const fn rw_ldata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Low-byte data written or read."]
        #[inline(always)]
        pub fn set_rw_ldata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "High-byte data written or read."]
        #[inline(always)]
        pub const fn rw_hdata(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "High-byte data written or read."]
        #[inline(always)]
        pub fn set_rw_hdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    #[doc = "PSRAM Data Transmission Quantity Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataNum(pub u32);
    impl DataNum {
        #[doc = "Number of data transfers."]
        #[inline(always)]
        pub const fn data_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data transfers."]
        #[inline(always)]
        pub fn set_data_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DataNum {
        #[inline(always)]
        fn default() -> DataNum {
            DataNum(0)
        }
    }
    #[doc = "Start address register of memory during DMA transmission."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaMemAddr(pub u32);
    impl DmaMemAddr {
        #[doc = "The start address of the memory (single-byte alignment) when DMA is in transit."]
        #[inline(always)]
        pub const fn dma_mem_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the memory (single-byte alignment) when DMA is in transit."]
        #[inline(always)]
        pub fn set_dma_mem_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaMemAddr {
        #[inline(always)]
        fn default() -> DmaMemAddr {
            DmaMemAddr(0)
        }
    }
    #[doc = "Start Address Register of PSRAM during DMA Transmission."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaPsramAddr(pub u32);
    impl DmaPsramAddr {
        #[doc = "The start address of the PSRAM when DMA is transmitted (32 bytes aligned in 4 mode and 16 bytes aligned in 2 mode)."]
        #[inline(always)]
        pub const fn dma_psram_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the PSRAM when DMA is transmitted (32 bytes aligned in 4 mode and 16 bytes aligned in 2 mode)."]
        #[inline(always)]
        pub fn set_dma_psram_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaPsramAddr {
        #[inline(always)]
        fn default() -> DmaPsramAddr {
            DmaPsramAddr(0)
        }
    }
    #[doc = "PSRAM DMA Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacr(pub u32);
    impl Dmacr {
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transmission direction."]
        #[inline(always)]
        pub const fn dat_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmission direction."]
        #[inline(always)]
        pub fn set_dat_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Memory Data Width."]
        #[inline(always)]
        pub const fn memory_size(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Memory Data Width."]
        #[inline(always)]
        pub fn set_memory_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub const fn dma_onebl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub fn set_dma_onebl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Pause time between bursts, in HCLK."]
        #[inline(always)]
        pub const fn towbl_tout(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Pause time between bursts, in HCLK."]
        #[inline(always)]
        pub fn set_towbl_tout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Dmacr {
        #[inline(always)]
        fn default() -> Dmacr {
            Dmacr(0)
        }
    }
    #[doc = "PSRAM Interrupt Error Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "DMA bus transmission completion enable."]
        #[inline(always)]
        pub const fn dmaten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA bus transmission completion enable."]
        #[inline(always)]
        pub fn set_dmaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CEM error interrupt marker enable."]
        #[inline(always)]
        pub const fn ceneien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CEM error interrupt marker enable."]
        #[inline(always)]
        pub fn set_ceneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub const fn aeien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub fn set_aeien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "PSRAM Interrupt State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "DMA bus transmission completion flag, write 1 to clear zero."]
        #[inline(always)]
        pub const fn dmatf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA bus transmission completion flag, write 1 to clear zero."]
        #[inline(always)]
        pub fn set_dmatf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CEM Error Interrupt Flag, write 1 to clear zero."]
        #[inline(always)]
        pub const fn ceneif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CEM Error Interrupt Flag, write 1 to clear zero."]
        #[inline(always)]
        pub fn set_ceneif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address error interrupt flag, address exceeds MEM capacity."]
        #[inline(always)]
        pub const fn aeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt flag, address exceeds MEM capacity."]
        #[inline(always)]
        pub fn set_aeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "PSRAM Read/Write Delay Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latency(pub u32);
    impl Latency {
        #[doc = "Read the delay."]
        #[inline(always)]
        pub const fn rlatency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read the delay."]
        #[inline(always)]
        pub fn set_rlatency(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Write the delay."]
        #[inline(always)]
        pub const fn wlatency(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Write the delay."]
        #[inline(always)]
        pub fn set_wlatency(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Latency {
        #[inline(always)]
        fn default() -> Latency {
            Latency(0)
        }
    }
    #[doc = "PSRAM controller state machine register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "PSRAM controller state machine."]
        #[inline(always)]
        pub const fn psram_state(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "PSRAM controller state machine."]
        #[inline(always)]
        pub fn set_psram_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "PSRAM time series register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timing(pub u32);
    impl Timing {
        #[doc = "Minimum time for read/write cycles."]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Minimum time for read/write cycles."]
        #[inline(always)]
        pub fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The minimum interval between two consecutive bursts."]
        #[inline(always)]
        pub const fn tcph(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The minimum interval between two consecutive bursts."]
        #[inline(always)]
        pub fn set_tcph(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Sustained pulse width when exiting low power CE_n."]
        #[inline(always)]
        pub const fn txlpd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Sustained pulse width when exiting low power CE_n."]
        #[inline(always)]
        pub fn set_txlpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timing {
        #[inline(always)]
        fn default() -> Timing {
            Timing(0)
        }
    }
}
