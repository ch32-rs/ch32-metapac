#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flexible static memory controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsmc {
    ptr: *mut u8,
}
unsafe impl Send for Fsmc {}
unsafe impl Sync for Fsmc {}
impl Fsmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1."]
    #[inline(always)]
    pub const fn bcr1(self) -> crate::common::Reg<regs::Bcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register 1."]
    #[inline(always)]
    pub const fn btr1(self) -> crate::common::Reg<regs::Btr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PC Card/NAND Flash control register 2."]
    #[inline(always)]
    pub const fn pcr2(self) -> crate::common::Reg<regs::Pcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "FIFO status and interrupt register 2."]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Common memory space timing register 2."]
    #[inline(always)]
    pub const fn pmem2(self) -> crate::common::Reg<regs::Pmem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Attribute memory space timing register 2."]
    #[inline(always)]
    pub const fn patt2(self) -> crate::common::Reg<regs::Patt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "ECC result register 2."]
    #[inline(always)]
    pub const fn eccr2(self) -> crate::common::Reg<regs::Eccr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1."]
    #[inline(always)]
    pub const fn bwtr1(self) -> crate::common::Reg<regs::Bwtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[doc = "SRAM/NOR-Flash chip-select control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr1(pub u32);
    impl Bcr1 {
        #[doc = "Memory bank enable bit."]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit."]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub const fn mtyp(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory databus width."]
        #[inline(always)]
        pub const fn mwid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Memory databus width."]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable."]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable."]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit."]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit."]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit."]
        #[inline(always)]
        pub const fn waitpol(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal polarity bit."]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wait timing configuration."]
        #[inline(always)]
        pub const fn waitcfg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wait timing configuration."]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit."]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit."]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit."]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit."]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable."]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable."]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Write burst enable."]
        #[inline(always)]
        pub const fn cburstrw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Write burst enable."]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Bcr1 {
        #[inline(always)]
        fn default() -> Bcr1 {
            Bcr1(0)
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr1(pub u32);
    impl Btr1 {
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration."]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration."]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Clock divide ratio (for FSMC_CLK signal)."]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divide ratio (for FSMC_CLK signal)."]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Data latency for synchronous NOR Flash memory."]
        #[inline(always)]
        pub const fn datlat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Data latency for synchronous NOR Flash memory."]
        #[inline(always)]
        pub fn set_datlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub const fn accmod(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Btr1 {
        #[inline(always)]
        fn default() -> Btr1 {
            Btr1(0)
        }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bwtr1(pub u32);
    impl Bwtr1 {
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub const fn accmod(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Bwtr1 {
        #[inline(always)]
        fn default() -> Bwtr1 {
            Bwtr1(0)
        }
    }
    #[doc = "ECC result register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr2(pub u32);
    impl Eccr2 {
        #[doc = "ECC result."]
        #[inline(always)]
        pub const fn eccx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC result."]
        #[inline(always)]
        pub fn set_eccx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccr2 {
        #[inline(always)]
        fn default() -> Eccr2 {
            Eccr2(0)
        }
    }
    #[doc = "Attribute memory space timing register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Patt2(pub u32);
    impl Patt2 {
        #[doc = "Attribute memory x setup time."]
        #[inline(always)]
        pub const fn attsetx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory x setup time."]
        #[inline(always)]
        pub fn set_attsetx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Attribute memory x wait time."]
        #[inline(always)]
        pub const fn attwaitx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory x wait time."]
        #[inline(always)]
        pub fn set_attwaitx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Attribute memory x hold time."]
        #[inline(always)]
        pub const fn attholdx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory x hold time."]
        #[inline(always)]
        pub fn set_attholdx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Attribute memory x databus HiZ time."]
        #[inline(always)]
        pub const fn atthizx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory x databus HiZ time."]
        #[inline(always)]
        pub fn set_atthizx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Patt2 {
        #[inline(always)]
        fn default() -> Patt2 {
            Patt2(0)
        }
    }
    #[doc = "PC Card/NAND Flash control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr2(pub u32);
    impl Pcr2 {
        #[doc = "Wait feature enable bit."]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable bit."]
        #[inline(always)]
        pub fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PC Card/NAND Flash memory bank enable bit."]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PC Card/NAND Flash memory bank enable bit."]
        #[inline(always)]
        pub fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub const fn ptyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Databus width."]
        #[inline(always)]
        pub const fn pwid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Databus width."]
        #[inline(always)]
        pub fn set_pwid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "ECC computation logic enable bit."]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECC computation logic enable bit."]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CLE to RE delay."]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE delay."]
        #[inline(always)]
        pub fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "ALE to RE delay."]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "ALE to RE delay."]
        #[inline(always)]
        pub fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "ECC page size."]
        #[inline(always)]
        pub const fn eccps(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "ECC page size."]
        #[inline(always)]
        pub fn set_eccps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
    }
    impl Default for Pcr2 {
        #[inline(always)]
        fn default() -> Pcr2 {
            Pcr2(0)
        }
    }
    #[doc = "Common memory space timing register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmem2(pub u32);
    impl Pmem2 {
        #[doc = "Common memory x setup time."]
        #[inline(always)]
        pub const fn memsetx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x setup time."]
        #[inline(always)]
        pub fn set_memsetx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Common memory x wait time."]
        #[inline(always)]
        pub const fn memwaitx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x wait time."]
        #[inline(always)]
        pub fn set_memwaitx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Common memory x hold time."]
        #[inline(always)]
        pub const fn memholdx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x hold time."]
        #[inline(always)]
        pub fn set_memholdx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Common memory x databus HiZ time."]
        #[inline(always)]
        pub const fn memhizx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x databus HiZ time."]
        #[inline(always)]
        pub fn set_memhizx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pmem2 {
        #[inline(always)]
        fn default() -> Pmem2 {
            Pmem2(0)
        }
    }
    #[doc = "FIFO status and interrupt register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
}
