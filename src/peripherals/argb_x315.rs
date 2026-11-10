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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ARGB 0/1 Code Period Configuration Register."]
    #[inline(always)]
    pub const fn dat_arr(self) -> crate::common::Reg<regs::DatArr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ARGB RESET Code Width Configuration Register."]
    #[inline(always)]
    pub const fn rst_arr(self) -> crate::common::Reg<regs::RstArr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "High Level T0H Width Configuration Register."]
    #[inline(always)]
    pub const fn ccrl(self) -> crate::common::Reg<regs::Ccrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "1-yard high-level T1H width configuration register for ARGB."]
    #[inline(always)]
    pub const fn ccrh(self) -> crate::common::Reg<regs::Ccrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ARGB Data Refresh Cycle Length Configuration Register."]
    #[inline(always)]
    pub const fn dat_cyc(self) -> crate::common::Reg<regs::DatCyc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ARGB Data Register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "ARGB State Register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "ARGB Counter Register."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "1-yard high-level T1H width configuration register for ARGB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccrh(pub u32);
    impl Ccrh {
        #[doc = "1 yard high level T1H width."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "1 yard high level T1H width."]
        #[inline(always)]
        pub const fn set_ccrh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ccrh {
        #[inline(always)]
        fn default() -> Ccrh {
            Ccrh(0)
        }
    }
    impl core::fmt::Debug for Ccrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccrh").field("ccrh", &self.ccrh()).finish()
        }
    }
    #[doc = "High Level T0H Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccrl(pub u32);
    impl Ccrl {
        #[doc = "0 yard high level T0H width."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "0 yard high level T0H width."]
        #[inline(always)]
        pub const fn set_ccrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ccrl {
        #[inline(always)]
        fn default() -> Ccrl {
            Ccrl(0)
        }
    }
    impl core::fmt::Debug for Ccrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccrl").field("ccrl", &self.ccrl()).finish()
        }
    }
    #[doc = "ARGB Counter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "ARGB base counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "ARGB base counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr").field("cnt", &self.cnt()).finish()
        }
    }
    #[doc = "ARGB Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Send Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send Enable."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Enables."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Enables."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Send mode configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send mode configuration bits."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data Send Mode Enabling Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn lsb(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data Send Mode Enabling Bits."]
        #[inline(always)]
        pub const fn set_lsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data refresh cycle end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn frhie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data refresh cycle end interrupt enabled."]
        #[inline(always)]
        pub const fn set_frhie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RESET code sending is interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RESET code sending is interrupted."]
        #[inline(always)]
        pub const fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Send completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Send completion interrupt enable."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Byte send completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn byteie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Byte send completion interrupt enable."]
        #[inline(always)]
        pub const fn set_byteie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data Brush Sends More Than Half Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn halfie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data Brush Sends More Than Half Interrupt Enable."]
        #[inline(always)]
        pub const fn set_halfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Send buffer empty interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Send buffer empty interrupt enable."]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("te", &self.te())
                .field("dmaen", &self.dmaen())
                .field("mode", &self.mode())
                .field("lsb", &self.lsb())
                .field("frhie", &self.frhie())
                .field("rstie", &self.rstie())
                .field("tcie", &self.tcie())
                .field("byteie", &self.byteie())
                .field("halfie", &self.halfie())
                .field("txeie", &self.txeie())
                .finish()
        }
    }
    #[doc = "ARGB 0/1 Code Period Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatArr(pub u32);
    impl DatArr {
        #[doc = "0/1 yard cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub const fn set_dat_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for DatArr {
        #[inline(always)]
        fn default() -> DatArr {
            DatArr(0)
        }
    }
    impl core::fmt::Debug for DatArr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DatArr")
                .field("dat_arr", &self.dat_arr())
                .finish()
        }
    }
    #[doc = "ARGB Data Refresh Cycle Length Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatCyc(pub u32);
    impl DatCyc {
        #[doc = "ARGB data refresh cycle length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_cyc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ARGB data refresh cycle length in bytes."]
        #[inline(always)]
        pub const fn set_dat_cyc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DatCyc {
        #[inline(always)]
        fn default() -> DatCyc {
            DatCyc(0)
        }
    }
    impl core::fmt::Debug for DatCyc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DatCyc")
                .field("dat_cyc", &self.dat_cyc())
                .finish()
        }
    }
    #[doc = "ARGB Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "ARGB sends data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ARGB sends data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    impl core::fmt::Debug for Datar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Datar").field("data", &self.data()).finish()
        }
    }
    #[doc = "ARGB RESET Code Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RstArr(pub u32);
    impl RstArr {
        #[doc = "0/1 yard cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_arr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "0/1 yard cycle."]
        #[inline(always)]
        pub const fn set_rst_arr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for RstArr {
        #[inline(always)]
        fn default() -> RstArr {
            RstArr(0)
        }
    }
    impl core::fmt::Debug for RstArr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RstArr")
                .field("rst_arr", &self.rst_arr())
                .finish()
        }
    }
    #[doc = "ARGB State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "The data refresh cycle has been sent to the interrupt signal, and 1 is written to clear it."]
        #[must_use]
        #[inline(always)]
        pub const fn frhif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The data refresh cycle has been sent to the interrupt signal, and 1 is written to clear it."]
        #[inline(always)]
        pub const fn set_frhif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RESET code sends the interrupt mark, writes 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rstif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RESET code sends the interrupt mark, writes 1 to clear."]
        #[inline(always)]
        pub const fn set_rstif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Send the interrupt sign completed, write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send the interrupt sign completed, write 1 to clear."]
        #[inline(always)]
        pub const fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The byte sending completion interrupt mark, write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn byteif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The byte sending completion interrupt mark, write 1 to clear."]
        #[inline(always)]
        pub const fn set_byteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Send more than half of the interrupt flag, write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn halfif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Send more than half of the interrupt flag, write 1 to clear."]
        #[inline(always)]
        pub const fn set_halfif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Send a buffer empty interrupt flag, writing DATAR will clear the subflag."]
        #[must_use]
        #[inline(always)]
        pub const fn txeif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Send a buffer empty interrupt flag, writing DATAR will clear the subflag."]
        #[inline(always)]
        pub const fn set_txeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "State machine indication signal."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "State machine indication signal."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Send bit counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Send bit counter."]
        #[inline(always)]
        pub const fn set_bit_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Send byte counters."]
        #[must_use]
        #[inline(always)]
        pub const fn byte_cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Send byte counters."]
        #[inline(always)]
        pub const fn set_byte_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("frhif", &self.frhif())
                .field("rstif", &self.rstif())
                .field("tcif", &self.tcif())
                .field("byteif", &self.byteif())
                .field("halfif", &self.halfif())
                .field("txeif", &self.txeif())
                .field("state", &self.state())
                .field("bit_cnt", &self.bit_cnt())
                .field("byte_cnt", &self.byte_cnt())
                .finish()
        }
    }
}
