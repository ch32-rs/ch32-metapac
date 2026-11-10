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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PSRAM time series register."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PSRAM DATA Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PSRAM Address Register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PSRAM Command Word 0 Configuration Register."]
    #[inline(always)]
    pub const fn cmd0_cfg(self) -> crate::common::Reg<regs::Cmd0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PSRAM Command Word 1 Configuration Register."]
    #[inline(always)]
    pub const fn cmd1_cfg(self) -> crate::common::Reg<regs::Cmd1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "PSRAM DMA Control Register."]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "PSRAM Data Transmission Quantity Register."]
    #[inline(always)]
    pub const fn data_num(self) -> crate::common::Reg<regs::DataNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Start address register of memory during DMA transmission."]
    #[inline(always)]
    pub const fn dma_mem_addr(self) -> crate::common::Reg<regs::DmaMemAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Start Address Register of PSRAM during DMA Transmission."]
    #[inline(always)]
    pub const fn dma_psram_addr(self) -> crate::common::Reg<regs::DmaPsramAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PSRAM Interrupt State Register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PSRAM Interrupt Error Enable Register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "PSRAM controller state machine register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PSRAM CE Pulse Configuration Register."]
    #[inline(always)]
    pub const fn cecfg(self) -> crate::common::Reg<regs::Cecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "PSRAM Read/Write Delay Register."]
    #[inline(always)]
    pub const fn latency(self) -> crate::common::Reg<regs::Latency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "PSRAM Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Inform PSRAM the nest operation."]
        #[must_use]
        #[inline(always)]
        pub const fn mw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Inform PSRAM the nest operation."]
        #[inline(always)]
        pub const fn set_mw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Commands executed."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Commands executed."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "The register 0 address to be manipulated."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_addr0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The register 0 address to be manipulated."]
        #[inline(always)]
        pub const fn set_mr_addr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr")
                .field("busy", &self.busy())
                .field("mw", &self.mw())
                .field("cmd", &self.cmd())
                .field("mr_addr0", &self.mr_addr0())
                .finish()
        }
    }
    #[doc = "PSRAM CE Pulse Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cecfg(pub u32);
    impl Cecfg {
        #[doc = "Maximum duration of CE low pulse. (excluding withdrawal of half sleep): T= N*PSRAM clock period."]
        #[must_use]
        #[inline(always)]
        pub const fn celpd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Maximum duration of CE low pulse. (excluding withdrawal of half sleep): T= N*PSRAM clock period."]
        #[inline(always)]
        pub const fn set_celpd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "CE pulse width detection enables."]
        #[must_use]
        #[inline(always)]
        pub const fn cepwen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CE pulse width detection enables."]
        #[inline(always)]
        pub const fn set_cepwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cecfg {
        #[inline(always)]
        fn default() -> Cecfg {
            Cecfg(0)
        }
    }
    impl core::fmt::Debug for Cecfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cecfg")
                .field("celpd", &self.celpd())
                .field("cepwen", &self.cepwen())
                .finish()
        }
    }
    #[doc = "PSRAM Command Word 0 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd0Cfg(pub u32);
    impl Cmd0Cfg {
        #[doc = "Configurable synchronous read commands."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous read commands."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Configurable synchronous write commands."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous write commands."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Configurable synchronous read (linear burst) commands."]
        #[must_use]
        #[inline(always)]
        pub const fn sr_lb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous read (linear burst) commands."]
        #[inline(always)]
        pub const fn set_sr_lb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Configurable synchronous write (linear burst) commands."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_lb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable synchronous write (linear burst) commands."]
        #[inline(always)]
        pub const fn set_sw_lb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Cmd0Cfg {
        #[inline(always)]
        fn default() -> Cmd0Cfg {
            Cmd0Cfg(0)
        }
    }
    impl core::fmt::Debug for Cmd0Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd0Cfg")
                .field("sr", &self.sr())
                .field("sw", &self.sw())
                .field("sr_lb", &self.sr_lb())
                .field("sw_lb", &self.sw_lb())
                .finish()
        }
    }
    #[doc = "PSRAM Command Word 1 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd1Cfg(pub u32);
    impl Cmd1Cfg {
        #[doc = "Configurable register read commands."]
        #[must_use]
        #[inline(always)]
        pub const fn mregr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable register read commands."]
        #[inline(always)]
        pub const fn set_mregr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Configurable register write commands."]
        #[must_use]
        #[inline(always)]
        pub const fn mregw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable register write commands."]
        #[inline(always)]
        pub const fn set_mregw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Configurable global reset commands."]
        #[must_use]
        #[inline(always)]
        pub const fn glrst(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable global reset commands."]
        #[inline(always)]
        pub const fn set_glrst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Configurable refresh command command."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Configurable refresh command command."]
        #[inline(always)]
        pub const fn set_refresh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Cmd1Cfg {
        #[inline(always)]
        fn default() -> Cmd1Cfg {
            Cmd1Cfg(0)
        }
    }
    impl core::fmt::Debug for Cmd1Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd1Cfg")
                .field("mregr", &self.mregr())
                .field("mregw", &self.mregw())
                .field("glrst", &self.glrst())
                .field("refresh", &self.refresh())
                .finish()
        }
    }
    #[doc = "PSRAM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Empty Read FIFO Method Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfifo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Empty Read FIFO Method Select."]
        #[inline(always)]
        pub const fn set_cfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA write operation for clearing FIFO mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dfifo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA write operation for clearing FIFO mode Selection."]
        #[inline(always)]
        pub const fn set_dfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PSRAM capacity configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cap_cfg(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "PSRAM capacity configuration."]
        #[inline(always)]
        pub const fn set_cap_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "PSRAM exits the current low-power state enable."]
        #[must_use]
        #[inline(always)]
        pub const fn exit_lpmd(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PSRAM exits the current low-power state enable."]
        #[inline(always)]
        pub const fn set_exit_lpmd(&mut self, val: bool) {
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
                .field("cfifo", &self.cfifo())
                .field("dfifo", &self.dfifo())
                .field("cap_cfg", &self.cap_cfg())
                .field("exit_lpmd", &self.exit_lpmd())
                .finish()
        }
    }
    #[doc = "PSRAM DATA Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u16);
    impl Data {
        #[doc = "Low-byte data written or read."]
        #[must_use]
        #[inline(always)]
        pub const fn rw_ldata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Low-byte data written or read."]
        #[inline(always)]
        pub const fn set_rw_ldata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "High-byte data written or read."]
        #[must_use]
        #[inline(always)]
        pub const fn rw_hdata(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "High-byte data written or read."]
        #[inline(always)]
        pub const fn set_rw_hdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data")
                .field("rw_ldata", &self.rw_ldata())
                .field("rw_hdata", &self.rw_hdata())
                .finish()
        }
    }
    #[doc = "PSRAM Data Transmission Quantity Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataNum(pub u32);
    impl DataNum {
        #[doc = "Number of data transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn data_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of data transfers."]
        #[inline(always)]
        pub const fn set_data_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DataNum {
        #[inline(always)]
        fn default() -> DataNum {
            DataNum(0)
        }
    }
    impl core::fmt::Debug for DataNum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DataNum")
                .field("data_num", &self.data_num())
                .finish()
        }
    }
    #[doc = "Start address register of memory during DMA transmission."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaMemAddr(pub u32);
    impl DmaMemAddr {
        #[doc = "The start address of the memory (single-byte alignment) when DMA is in transit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_mem_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the memory (single-byte alignment) when DMA is in transit."]
        #[inline(always)]
        pub const fn set_dma_mem_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaMemAddr {
        #[inline(always)]
        fn default() -> DmaMemAddr {
            DmaMemAddr(0)
        }
    }
    impl core::fmt::Debug for DmaMemAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaMemAddr")
                .field("dma_mem_addr", &self.dma_mem_addr())
                .finish()
        }
    }
    #[doc = "Start Address Register of PSRAM during DMA Transmission."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaPsramAddr(pub u32);
    impl DmaPsramAddr {
        #[doc = "The start address of the PSRAM when DMA is transmitted (32 bytes aligned in 4 mode and 16 bytes aligned in 2 mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_psram_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the PSRAM when DMA is transmitted (32 bytes aligned in 4 mode and 16 bytes aligned in 2 mode)."]
        #[inline(always)]
        pub const fn set_dma_psram_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaPsramAddr {
        #[inline(always)]
        fn default() -> DmaPsramAddr {
            DmaPsramAddr(0)
        }
    }
    impl core::fmt::Debug for DmaPsramAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaPsramAddr")
                .field("dma_psram_addr", &self.dma_psram_addr())
                .finish()
        }
    }
    #[doc = "PSRAM DMA Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacr(pub u32);
    impl Dmacr {
        #[doc = "DMA Enables."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transmission direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data transmission direction."]
        #[inline(always)]
        pub const fn set_dat_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Memory Data Width."]
        #[must_use]
        #[inline(always)]
        pub const fn memory_size(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Memory Data Width."]
        #[inline(always)]
        pub const fn set_memory_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "DMA burst length."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_onebl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub const fn set_dma_onebl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Pause time between bursts, in HCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn towbl_tout(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Pause time between bursts, in HCLK."]
        #[inline(always)]
        pub const fn set_towbl_tout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Dmacr {
        #[inline(always)]
        fn default() -> Dmacr {
            Dmacr(0)
        }
    }
    impl core::fmt::Debug for Dmacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacr")
                .field("dmaen", &self.dmaen())
                .field("dat_dir", &self.dat_dir())
                .field("memory_size", &self.memory_size())
                .field("dma_onebl", &self.dma_onebl())
                .field("towbl_tout", &self.towbl_tout())
                .finish()
        }
    }
    #[doc = "PSRAM Interrupt Error Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "DMA bus transmission completion enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA bus transmission completion enable."]
        #[inline(always)]
        pub const fn set_dmaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CEM error interrupt marker enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ceneien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CEM error interrupt marker enable."]
        #[inline(always)]
        pub const fn set_ceneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aeien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub const fn set_aeien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("dmaten", &self.dmaten())
                .field("ceneien", &self.ceneien())
                .field("aeien", &self.aeien())
                .finish()
        }
    }
    #[doc = "PSRAM Interrupt State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "DMA bus transmission completion flag, write 1 to clear zero."]
        #[must_use]
        #[inline(always)]
        pub const fn dmatf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA bus transmission completion flag, write 1 to clear zero."]
        #[inline(always)]
        pub const fn set_dmatf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CEM Error Interrupt Flag, write 1 to clear zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ceneif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CEM Error Interrupt Flag, write 1 to clear zero."]
        #[inline(always)]
        pub const fn set_ceneif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address error interrupt flag, address exceeds MEM capacity."]
        #[must_use]
        #[inline(always)]
        pub const fn aeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt flag, address exceeds MEM capacity."]
        #[inline(always)]
        pub const fn set_aeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("dmatf", &self.dmatf())
                .field("ceneif", &self.ceneif())
                .field("aeif", &self.aeif())
                .finish()
        }
    }
    #[doc = "PSRAM Read/Write Delay Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latency(pub u32);
    impl Latency {
        #[doc = "Read the delay."]
        #[must_use]
        #[inline(always)]
        pub const fn rlatency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read the delay."]
        #[inline(always)]
        pub const fn set_rlatency(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Write the delay."]
        #[must_use]
        #[inline(always)]
        pub const fn wlatency(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Write the delay."]
        #[inline(always)]
        pub const fn set_wlatency(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Latency {
        #[inline(always)]
        fn default() -> Latency {
            Latency(0)
        }
    }
    impl core::fmt::Debug for Latency {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latency")
                .field("rlatency", &self.rlatency())
                .field("wlatency", &self.wlatency())
                .finish()
        }
    }
    #[doc = "PSRAM controller state machine register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "PSRAM controller state machine."]
        #[must_use]
        #[inline(always)]
        pub const fn psram_state(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "PSRAM controller state machine."]
        #[inline(always)]
        pub const fn set_psram_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("psram_state", &self.psram_state())
                .finish()
        }
    }
    #[doc = "PSRAM time series register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timing(pub u32);
    impl Timing {
        #[doc = "Minimum time for read/write cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Minimum time for read/write cycles."]
        #[inline(always)]
        pub const fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The minimum interval between two consecutive bursts."]
        #[must_use]
        #[inline(always)]
        pub const fn tcph(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The minimum interval between two consecutive bursts."]
        #[inline(always)]
        pub const fn set_tcph(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Sustained pulse width when exiting low power CE_n."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Sustained pulse width when exiting low power CE_n."]
        #[inline(always)]
        pub const fn set_txlpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timing {
        #[inline(always)]
        fn default() -> Timing {
            Timing(0)
        }
    }
    impl core::fmt::Debug for Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timing")
                .field("trc", &self.trc())
                .field("tcph", &self.tcph())
                .field("txlpd", &self.txlpd())
                .finish()
        }
    }
}
