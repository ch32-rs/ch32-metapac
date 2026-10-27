#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ARGB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Argb {
    ptr: *mut u8,
}
unsafe impl Send for Argb {}
unsafe impl Sync for Argb {}
impl Argb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ARGB Control Register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ARGB 0/1 Code Period Configuration Register."]
    #[inline(always)]
    pub const fn dat_arr(self) -> crate::common::Reg<regs::DatArr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ARGB RESET Code Width Configuration Register."]
    #[inline(always)]
    pub const fn rst_arr(self) -> crate::common::Reg<regs::RstArr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "High Level T0H Width Configuration Register."]
    #[inline(always)]
    pub const fn ccrl(self) -> crate::common::Reg<regs::Ccrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "1-yard high-level T1H width configuration register for ARGB."]
    #[inline(always)]
    pub const fn ccrh(self) -> crate::common::Reg<regs::Ccrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ARGB Data Refresh Cycle Length Configuration Register."]
    #[inline(always)]
    pub const fn dat_cyc(self) -> crate::common::Reg<regs::DatCyc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ARGB Data Register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "ARGB State Register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ARGB Counter Register."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "1-yard high-level T1H width configuration register for ARGB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccrh(pub u32);
    impl Ccrh {
        #[doc = "1 yard high level T1H width."]
        #[inline(always)]
        pub const fn ccrh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "1 yard high level T1H width."]
        #[inline(always)]
        pub fn set_ccrh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Ccrh {
        #[inline(always)]
        fn default() -> Ccrh {
            Ccrh(0)
        }
    }
    #[doc = "High Level T0H Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccrl(pub u32);
    impl Ccrl {
        #[doc = "0 yard high level T0H width."]
        #[inline(always)]
        pub const fn ccrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "0 yard high level T0H width."]
        #[inline(always)]
        pub fn set_ccrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Ccrl {
        #[inline(always)]
        fn default() -> Ccrl {
            Ccrl(0)
        }
    }
    #[doc = "ARGB Counter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "ARGB base counter."]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ARGB base counter."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    #[doc = "ARGB Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Send Enable."]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send Enable."]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Send mode configuration bits."]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send mode configuration bits."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data Send Mode Enabling Bits."]
        #[inline(always)]
        pub const fn lsb(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data Send Mode Enabling Bits."]
        #[inline(always)]
        pub fn set_lsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data refresh cycle end interrupt enabled."]
        #[inline(always)]
        pub const fn frhie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data refresh cycle end interrupt enabled."]
        #[inline(always)]
        pub fn set_frhie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RESET code sending is interrupted."]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RESET code sending is interrupted."]
        #[inline(always)]
        pub fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Send completion interrupt enable."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Send completion interrupt enable."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Byte send completion interrupt enable."]
        #[inline(always)]
        pub const fn byteie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Byte send completion interrupt enable."]
        #[inline(always)]
        pub fn set_byteie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data Brush Sends More Than Half Interrupt Enable."]
        #[inline(always)]
        pub const fn halfie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data Brush Sends More Than Half Interrupt Enable."]
        #[inline(always)]
        pub fn set_halfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Send buffer empty interrupt enable."]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Send buffer empty interrupt enable."]
        #[inline(always)]
        pub fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "ARGB 0/1 Code Period Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatArr(pub u32);
    impl DatArr {
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub const fn dat_arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub fn set_dat_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DatArr {
        #[inline(always)]
        fn default() -> DatArr {
            DatArr(0)
        }
    }
    #[doc = "ARGB Data Refresh Cycle Length Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatCyc(pub u32);
    impl DatCyc {
        #[doc = "ARGB data refresh cycle length in bytes."]
        #[inline(always)]
        pub const fn dat_cyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "ARGB data refresh cycle length in bytes."]
        #[inline(always)]
        pub fn set_dat_cyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for DatCyc {
        #[inline(always)]
        fn default() -> DatCyc {
            DatCyc(0)
        }
    }
    #[doc = "ARGB Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "ARGB sends data."]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ARGB sends data."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "ARGB RESET Code Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RstArr(pub u32);
    impl RstArr {
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub const fn rst_arr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub fn set_rst_arr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for RstArr {
        #[inline(always)]
        fn default() -> RstArr {
            RstArr(0)
        }
    }
    #[doc = "ARGB State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "The data refresh cycle has been sent to the interrupt signal, and 1 is written to clear it."]
        #[inline(always)]
        pub const fn frhif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The data refresh cycle has been sent to the interrupt signal, and 1 is written to clear it."]
        #[inline(always)]
        pub fn set_frhif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RESET code sends the interrupt mark, writes 1 to clear."]
        #[inline(always)]
        pub const fn rstif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RESET code sends the interrupt mark, writes 1 to clear."]
        #[inline(always)]
        pub fn set_rstif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Send the interrupt sign completed, write 1 to clear."]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send the interrupt sign completed, write 1 to clear."]
        #[inline(always)]
        pub fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The byte sending completion interrupt mark, write 1 to clear."]
        #[inline(always)]
        pub const fn byteif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The byte sending completion interrupt mark, write 1 to clear."]
        #[inline(always)]
        pub fn set_byteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Send more than half of the interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn halfif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Send more than half of the interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_halfif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Send a buffer empty interrupt flag, writing DATAR will clear the subflag."]
        #[inline(always)]
        pub const fn txeif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Send a buffer empty interrupt flag, writing DATAR will clear the subflag."]
        #[inline(always)]
        pub fn set_txeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Send bit counter."]
        #[inline(always)]
        pub const fn bit_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Send bit counter."]
        #[inline(always)]
        pub fn set_bit_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Send byte counters."]
        #[inline(always)]
        pub const fn byte_cnt(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Send byte counters."]
        #[inline(always)]
        pub fn set_byte_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
}
