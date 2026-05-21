#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm {}
unsafe impl Sync for Dfsdm {}
impl Dfsdm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "channel configuration 0 register."]
    #[inline(always)]
    pub const fn ch0cfgr1(self) -> crate::common::Reg<regs::Ch0cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "channel configuration 1 register."]
    #[inline(always)]
    pub const fn ch1cfgr1(self) -> crate::common::Reg<regs::Ch1cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "channel configuration 0 register."]
    #[inline(always)]
    pub const fn ch0cfgr2(self) -> crate::common::Reg<regs::Ch0cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "channel configuration 1 register."]
    #[inline(always)]
    pub const fn ch1cfgr2(self) -> crate::common::Reg<regs::Ch1cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "analog watchdog and short-circuit detector register."]
    #[inline(always)]
    pub const fn ch0awscdr(self) -> crate::common::Reg<regs::Ch0awscdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "analog watchdog and short-circuit detector register."]
    #[inline(always)]
    pub const fn ch1awscdr(self) -> crate::common::Reg<regs::Ch1awscdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "channel watchdog filter data register."]
    #[inline(always)]
    pub const fn ch0wdatr(self) -> crate::common::Reg<regs::Ch0wdatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "channel watchdog filter data register."]
    #[inline(always)]
    pub const fn ch1wdatr(self) -> crate::common::Reg<regs::Ch1wdatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "channel data input register."]
    #[inline(always)]
    pub const fn ch0datinr(self) -> crate::common::Reg<regs::Ch0datinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "channel data input register."]
    #[inline(always)]
    pub const fn ch1datinr(self) -> crate::common::Reg<regs::Ch1datinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn dfsdm_flt0cr1(self) -> crate::common::Reg<regs::DfsdmFlt0cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn dfsdm_flt1cr1(self) -> crate::common::Reg<regs::DfsdmFlt1cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn dfsdm_flt0cr2(self) -> crate::common::Reg<regs::DfsdmFlt0cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn dfsdm_flt1cr2(self) -> crate::common::Reg<regs::DfsdmFlt1cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "interrupt and status register."]
    #[inline(always)]
    pub const fn dfsdm_flt0isr(self) -> crate::common::Reg<regs::DfsdmFlt0isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "interrupt and status register."]
    #[inline(always)]
    pub const fn dfsdm_flt1isr(self) -> crate::common::Reg<regs::DfsdmFlt1isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "interrupt flag clear register."]
    #[inline(always)]
    pub const fn dfsdm_flt0icr(self) -> crate::common::Reg<regs::DfsdmFlt0icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "interrupt flag clear register."]
    #[inline(always)]
    pub const fn dfsdm_flt1icr(self) -> crate::common::Reg<regs::DfsdmFlt1icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "injected channel group selection register."]
    #[inline(always)]
    pub const fn dfsdm_flt0jchgr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0jchgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "injected channel group selection register."]
    #[inline(always)]
    pub const fn dfsdm_flt1jchgr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1jchgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "control register 3."]
    #[inline(always)]
    pub const fn dfsdm_flt0fcr3(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0fcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "control register 3."]
    #[inline(always)]
    pub const fn dfsdm_flt1fcr3(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1fcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "data register for injected group."]
    #[inline(always)]
    pub const fn dfsdm_flt0jdatar(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0jdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "data register for injected group."]
    #[inline(always)]
    pub const fn dfsdm_flt1jdatar(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1jdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "data register for the regular channel."]
    #[inline(always)]
    pub const fn dfsdm_flt0rdatar(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0rdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "data register for the regular channel."]
    #[inline(always)]
    pub const fn dfsdm_flt1rdatar(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1rdatar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "analog watchdog high threshold register."]
    #[inline(always)]
    pub const fn dfsdm_flt0awhtr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0awhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "analog watchdog high threshold register."]
    #[inline(always)]
    pub const fn dfsdm_flt1awhtr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1awhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "analog watchdog low threshold register."]
    #[inline(always)]
    pub const fn dfsdm_flt0awltr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0awltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "analog watchdog low threshold register."]
    #[inline(always)]
    pub const fn dfsdm_flt1awltr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1awltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "analog watchdog status register."]
    #[inline(always)]
    pub const fn dfsdm_flt0awsr(self) -> crate::common::Reg<regs::DfsdmFlt0awsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "analog watchdog status register."]
    #[inline(always)]
    pub const fn dfsdm_flt1awsr(self) -> crate::common::Reg<regs::DfsdmFlt1awsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "analog watchdog clear flag register."]
    #[inline(always)]
    pub const fn dfsdm_flt0awcfr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0awcfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "analog watchdog clear flag register."]
    #[inline(always)]
    pub const fn dfsdm_flt1awcfr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1awcfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Extremes detector maximum register."]
    #[inline(always)]
    pub const fn dfsdm_flt0exmax(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0exmax, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Extremes detector maximum register."]
    #[inline(always)]
    pub const fn dfsdm_flt1exmax(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1exmax, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Extremes detector minimum register."]
    #[inline(always)]
    pub const fn dfsdm_flt0exmin(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0exmin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Extremes detector minimum register."]
    #[inline(always)]
    pub const fn dfsdm_flt1exmin(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1exmin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "conversion timer register."]
    #[inline(always)]
    pub const fn dfsdm_flt0cnvtimr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt0cnvtimr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "conversion timer register."]
    #[inline(always)]
    pub const fn dfsdm_flt1cnvtimr(
        self,
    ) -> crate::common::Reg<regs::DfsdmFlt1cnvtimr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
}
pub mod regs {
    #[doc = "analog watchdog and short-circuit detector register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0awscdr(pub u32);
    impl Ch0awscdr {
        #[doc = "Short Circuit Detector Threshold for Channel 0."]
        #[inline(always)]
        pub const fn scdt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Short Circuit Detector Threshold for Channel 0."]
        #[inline(always)]
        pub fn set_scdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Channel 0 Short Circuit Detector Open Signal Distribution."]
        #[inline(always)]
        pub const fn bkscd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel 0 Short Circuit Detector Open Signal Distribution."]
        #[inline(always)]
        pub fn set_bkscd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Channel 0 analog watchdog filter oversampling rate."]
        #[inline(always)]
        pub const fn awfosr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Channel 0 analog watchdog filter oversampling rate."]
        #[inline(always)]
        pub fn set_awfosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Channel 0 analog watchdog Sinc filter order."]
        #[inline(always)]
        pub const fn awford(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 0 analog watchdog Sinc filter order."]
        #[inline(always)]
        pub fn set_awford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Ch0awscdr {
        #[inline(always)]
        fn default() -> Ch0awscdr {
            Ch0awscdr(0)
        }
    }
    #[doc = "channel configuration 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0cfgr1(pub u32);
    impl Ch0cfgr1 {
        #[doc = "Channel 0 Serial Interface Type."]
        #[inline(always)]
        pub const fn sitp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 Serial Interface Type."]
        #[inline(always)]
        pub fn set_sitp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 0 SPI Clock Selection."]
        #[inline(always)]
        pub const fn spicksel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 0 SPI Clock Selection."]
        #[inline(always)]
        pub fn set_spicksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Channel 0 Short Circuit Detector Enables."]
        #[inline(always)]
        pub const fn scden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 Short Circuit Detector Enables."]
        #[inline(always)]
        pub fn set_scden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 0 Clock Missing Detector Enables."]
        #[inline(always)]
        pub const fn ckaben(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 Clock Missing Detector Enables."]
        #[inline(always)]
        pub fn set_ckaben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel 0 enables."]
        #[inline(always)]
        pub const fn chen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 enables."]
        #[inline(always)]
        pub fn set_chen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel Input Selection."]
        #[inline(always)]
        pub const fn chinsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Input Selection."]
        #[inline(always)]
        pub fn set_chinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel 0 Input Data Multiplexer."]
        #[inline(always)]
        pub const fn datmpx(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 0 Input Data Multiplexer."]
        #[inline(always)]
        pub fn set_datmpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "R32_DFSDM_CHyDATINR register data encapsulation mode."]
        #[inline(always)]
        pub const fn datpack(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "R32_DFSDM_CHyDATINR register data encapsulation mode."]
        #[inline(always)]
        pub fn set_datpack(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Output Serial Clock Divider."]
        #[inline(always)]
        pub const fn ckoutdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Output Serial Clock Divider."]
        #[inline(always)]
        pub fn set_ckoutdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Output Serial Clock Source Selection."]
        #[inline(always)]
        pub const fn ckoutsrc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Output Serial Clock Source Selection."]
        #[inline(always)]
        pub fn set_ckoutsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "The DFSDM interface is globally enabled."]
        #[inline(always)]
        pub const fn dfsdmen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The DFSDM interface is globally enabled."]
        #[inline(always)]
        pub fn set_dfsdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch0cfgr1 {
        #[inline(always)]
        fn default() -> Ch0cfgr1 {
            Ch0cfgr1(0)
        }
    }
    #[doc = "channel configuration 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0cfgr2(pub u32);
    impl Ch0cfgr2 {
        #[doc = "Channel 0 data right shift."]
        #[inline(always)]
        pub const fn dtrbs(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Channel 0 data right shift."]
        #[inline(always)]
        pub fn set_dtrbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Channel 0 24-bit calibration offset."]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Channel 0 24-bit calibration offset."]
        #[inline(always)]
        pub fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Ch0cfgr2 {
        #[inline(always)]
        fn default() -> Ch0cfgr2 {
            Ch0cfgr2(0)
        }
    }
    #[doc = "channel data input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0datinr(pub u32);
    impl Ch0datinr {
        #[doc = "Channel Y Input Data."]
        #[inline(always)]
        pub const fn indat0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel Y Input Data."]
        #[inline(always)]
        pub fn set_indat0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Input data for channel y or channel y+1."]
        #[inline(always)]
        pub const fn indat1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Input data for channel y or channel y+1."]
        #[inline(always)]
        pub fn set_indat1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ch0datinr {
        #[inline(always)]
        fn default() -> Ch0datinr {
            Ch0datinr(0)
        }
    }
    #[doc = "channel watchdog filter data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0wdatr(pub u32);
    impl Ch0wdatr {
        #[doc = "Enter channel 0 watchdog data."]
        #[inline(always)]
        pub const fn wdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Enter channel 0 watchdog data."]
        #[inline(always)]
        pub fn set_wdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch0wdatr {
        #[inline(always)]
        fn default() -> Ch0wdatr {
            Ch0wdatr(0)
        }
    }
    #[doc = "analog watchdog and short-circuit detector register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1awscdr(pub u32);
    impl Ch1awscdr {
        #[doc = "Short Circuit Detector Threshold for Channel 1."]
        #[inline(always)]
        pub const fn scdt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Short Circuit Detector Threshold for Channel 1."]
        #[inline(always)]
        pub fn set_scdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Channel 1 Short Circuit Detector Open Signal Distribution."]
        #[inline(always)]
        pub const fn bkscd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel 1 Short Circuit Detector Open Signal Distribution."]
        #[inline(always)]
        pub fn set_bkscd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Channel 1 analog watchdog filter oversampling rate."]
        #[inline(always)]
        pub const fn awfosr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Channel 1 analog watchdog filter oversampling rate."]
        #[inline(always)]
        pub fn set_awfosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Channel 1 analog watchdog Sinc filter order."]
        #[inline(always)]
        pub const fn awford(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 1 analog watchdog Sinc filter order."]
        #[inline(always)]
        pub fn set_awford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Ch1awscdr {
        #[inline(always)]
        fn default() -> Ch1awscdr {
            Ch1awscdr(0)
        }
    }
    #[doc = "channel configuration 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1cfgr1(pub u32);
    impl Ch1cfgr1 {
        #[doc = "Channel 1 Serial Interface Type."]
        #[inline(always)]
        pub const fn sitp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Serial Interface Type."]
        #[inline(always)]
        pub fn set_sitp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 1 SPI Clock Selection."]
        #[inline(always)]
        pub const fn spicksel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 1 SPI Clock Selection."]
        #[inline(always)]
        pub fn set_spicksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Channel 1 Short Circuit Detector Enables."]
        #[inline(always)]
        pub const fn scden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Short Circuit Detector Enables."]
        #[inline(always)]
        pub fn set_scden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 1 Clock Missing Detector Enables."]
        #[inline(always)]
        pub const fn ckaben(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Clock Missing Detector Enables."]
        #[inline(always)]
        pub fn set_ckaben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel 1 enables."]
        #[inline(always)]
        pub const fn chen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 enables."]
        #[inline(always)]
        pub fn set_chen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel Input Selection."]
        #[inline(always)]
        pub const fn chinsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Input Selection."]
        #[inline(always)]
        pub fn set_chinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel 1 Input Data Multiplexer."]
        #[inline(always)]
        pub const fn datmpx(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Channel 1 Input Data Multiplexer."]
        #[inline(always)]
        pub fn set_datmpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "R32_DFSDM_CHyDATINR register data encapsulation mode."]
        #[inline(always)]
        pub const fn datpack(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "R32_DFSDM_CHyDATINR register data encapsulation mode."]
        #[inline(always)]
        pub fn set_datpack(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Output Serial Clock Divider."]
        #[inline(always)]
        pub const fn ckoutdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Output Serial Clock Divider."]
        #[inline(always)]
        pub fn set_ckoutdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Output Serial Clock Source Selection."]
        #[inline(always)]
        pub const fn ckoutsrc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Output Serial Clock Source Selection."]
        #[inline(always)]
        pub fn set_ckoutsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "The DFSDM interface is globally enabled."]
        #[inline(always)]
        pub const fn dfsdmen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The DFSDM interface is globally enabled."]
        #[inline(always)]
        pub fn set_dfsdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch1cfgr1 {
        #[inline(always)]
        fn default() -> Ch1cfgr1 {
            Ch1cfgr1(0)
        }
    }
    #[doc = "channel configuration 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1cfgr2(pub u32);
    impl Ch1cfgr2 {
        #[doc = "Channel 1 data right shift."]
        #[inline(always)]
        pub const fn dtrbs(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Channel 1 data right shift."]
        #[inline(always)]
        pub fn set_dtrbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Channel 1 24-bit calibration offset."]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Channel 1 24-bit calibration offset."]
        #[inline(always)]
        pub fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Ch1cfgr2 {
        #[inline(always)]
        fn default() -> Ch1cfgr2 {
            Ch1cfgr2(0)
        }
    }
    #[doc = "channel data input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1datinr(pub u32);
    impl Ch1datinr {
        #[doc = "Channel 1 Input Data."]
        #[inline(always)]
        pub const fn indat0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel 1 Input Data."]
        #[inline(always)]
        pub fn set_indat0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Input data for channel 1 or channel 2."]
        #[inline(always)]
        pub const fn indat1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Input data for channel 1 or channel 2."]
        #[inline(always)]
        pub fn set_indat1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ch1datinr {
        #[inline(always)]
        fn default() -> Ch1datinr {
            Ch1datinr(0)
        }
    }
    #[doc = "channel watchdog filter data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1wdatr(pub u32);
    impl Ch1wdatr {
        #[doc = "Enter channel 1 watchdog data."]
        #[inline(always)]
        pub const fn wdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Enter channel 1 watchdog data."]
        #[inline(always)]
        pub fn set_wdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch1wdatr {
        #[inline(always)]
        fn default() -> Ch1wdatr {
            Ch1wdatr(0)
        }
    }
    #[doc = "analog watchdog clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0awcfr(pub u32);
    impl DfsdmFlt0awcfr {
        #[doc = "Clear the analog watchdog low threshold flag."]
        #[inline(always)]
        pub const fn clrawltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Clear the analog watchdog low threshold flag."]
        #[inline(always)]
        pub fn set_clrawltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Clear the analog watchdog high threshold flag."]
        #[inline(always)]
        pub const fn clrawhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Clear the analog watchdog high threshold flag."]
        #[inline(always)]
        pub fn set_clrawhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for DfsdmFlt0awcfr {
        #[inline(always)]
        fn default() -> DfsdmFlt0awcfr {
            DfsdmFlt0awcfr(0)
        }
    }
    #[doc = "analog watchdog high threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0awhtr(pub u32);
    impl DfsdmFlt0awhtr {
        #[doc = "Break signal assignment to analog watchdog high threshold event."]
        #[inline(always)]
        pub const fn bkawh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog high threshold event."]
        #[inline(always)]
        pub fn set_bkawh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Analog watchdog high threshold."]
        #[inline(always)]
        pub const fn awht(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog high threshold."]
        #[inline(always)]
        pub fn set_awht(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0awhtr {
        #[inline(always)]
        fn default() -> DfsdmFlt0awhtr {
            DfsdmFlt0awhtr(0)
        }
    }
    #[doc = "analog watchdog low threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0awltr(pub u32);
    impl DfsdmFlt0awltr {
        #[doc = "Break signal assignment to analog watchdog low threshold event."]
        #[inline(always)]
        pub const fn bkawl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog low threshold event."]
        #[inline(always)]
        pub fn set_bkawl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Analog watchdog low threshold."]
        #[inline(always)]
        pub const fn awlt(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog low threshold."]
        #[inline(always)]
        pub fn set_awlt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0awltr {
        #[inline(always)]
        fn default() -> DfsdmFlt0awltr {
            DfsdmFlt0awltr(0)
        }
    }
    #[doc = "analog watchdog status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0awsr(pub u32);
    impl DfsdmFlt0awsr {
        #[doc = "Analog watchdog low threshold flag."]
        #[inline(always)]
        pub const fn awltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog low threshold flag."]
        #[inline(always)]
        pub fn set_awltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Analog watchdog high threshold flag."]
        #[inline(always)]
        pub const fn awhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog high threshold flag."]
        #[inline(always)]
        pub fn set_awhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0awsr {
        #[inline(always)]
        fn default() -> DfsdmFlt0awsr {
            DfsdmFlt0awsr(0)
        }
    }
    #[doc = "conversion timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0cnvtimr(pub u32);
    impl DfsdmFlt0cnvtimr {
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN."]
        #[inline(always)]
        pub const fn cnvcnt(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN."]
        #[inline(always)]
        pub fn set_cnvcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for DfsdmFlt0cnvtimr {
        #[inline(always)]
        fn default() -> DfsdmFlt0cnvtimr {
            DfsdmFlt0cnvtimr(0)
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0cr1(pub u32);
    impl DfsdmFlt0cr1 {
        #[doc = "DFSDM enable."]
        #[inline(always)]
        pub const fn dfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM enable."]
        #[inline(always)]
        pub fn set_dfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger."]
        #[inline(always)]
        pub const fn jsync(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger."]
        #[inline(always)]
        pub fn set_jsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scanning conversion mode for injected conversions."]
        #[inline(always)]
        pub const fn jscan(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Scanning conversion mode for injected conversions."]
        #[inline(always)]
        pub fn set_jscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub const fn jdmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub fn set_jdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub const fn rswstart(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub fn set_rswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub const fn rcont(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub fn set_rcont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Launch regular conversion synchronously with DFSDM0."]
        #[inline(always)]
        pub const fn rsync(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Launch regular conversion synchronously with DFSDM0."]
        #[inline(always)]
        pub fn set_rsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DMA channel enabled to read data for the regular conversion."]
        #[inline(always)]
        pub const fn rdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the regular conversion."]
        #[inline(always)]
        pub fn set_rdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub const fn rch(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub fn set_rch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Fast conversion mode selection for regular conversions."]
        #[inline(always)]
        pub const fn fast(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Fast conversion mode selection for regular conversions."]
        #[inline(always)]
        pub fn set_fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Analog watchdog fast mode select."]
        #[inline(always)]
        pub const fn awfsel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog fast mode select."]
        #[inline(always)]
        pub fn set_awfsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for DfsdmFlt0cr1 {
        #[inline(always)]
        fn default() -> DfsdmFlt0cr1 {
            DfsdmFlt0cr1(0)
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0cr2(pub u32);
    impl DfsdmFlt0cr2 {
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn reocie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_reocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub const fn jovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_jovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub const fn rovrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_rovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[inline(always)]
        pub fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Short-circuit detector interrupt enable."]
        #[inline(always)]
        pub const fn scdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Short-circuit detector interrupt enable."]
        #[inline(always)]
        pub fn set_scdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clock absence interrupt enable."]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence interrupt enable."]
        #[inline(always)]
        pub fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Extremes detector channel selection."]
        #[inline(always)]
        pub const fn exch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Extremes detector channel selection."]
        #[inline(always)]
        pub fn set_exch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Analog watchdog channel selection."]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog channel selection."]
        #[inline(always)]
        pub fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DfsdmFlt0cr2 {
        #[inline(always)]
        fn default() -> DfsdmFlt0cr2 {
            DfsdmFlt0cr2(0)
        }
    }
    #[doc = "Extremes detector maximum register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0exmax(pub u32);
    impl DfsdmFlt0exmax {
        #[doc = "Extremes detector maximum data channel."]
        #[inline(always)]
        pub const fn exmaxch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Extremes detector maximum data channel."]
        #[inline(always)]
        pub fn set_exmaxch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Extremes detector maximum value."]
        #[inline(always)]
        pub const fn exmax(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Extremes detector maximum value."]
        #[inline(always)]
        pub fn set_exmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0exmax {
        #[inline(always)]
        fn default() -> DfsdmFlt0exmax {
            DfsdmFlt0exmax(0)
        }
    }
    #[doc = "Extremes detector minimum register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0exmin(pub u32);
    impl DfsdmFlt0exmin {
        #[doc = "Extremes detector minimum data channel."]
        #[inline(always)]
        pub const fn exminch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Extremes detector minimum data channel."]
        #[inline(always)]
        pub fn set_exminch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXMIN."]
        #[inline(always)]
        pub const fn exmin(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "EXMIN."]
        #[inline(always)]
        pub fn set_exmin(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0exmin {
        #[inline(always)]
        fn default() -> DfsdmFlt0exmin {
            DfsdmFlt0exmin(0)
        }
    }
    #[doc = "control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0fcr3(pub u32);
    impl DfsdmFlt0fcr3 {
        #[doc = "The integrator oversampling rate is 2 to the power of IOSR."]
        #[inline(always)]
        pub const fn iosr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The integrator oversampling rate is 2 to the power of IOSR."]
        #[inline(always)]
        pub fn set_iosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Sinc filter oversampling rate."]
        #[inline(always)]
        pub const fn fosr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Sinc filter oversampling rate."]
        #[inline(always)]
        pub fn set_fosr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Sinc Filter Order."]
        #[inline(always)]
        pub const fn ford(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Sinc Filter Order."]
        #[inline(always)]
        pub fn set_ford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for DfsdmFlt0fcr3 {
        #[inline(always)]
        fn default() -> DfsdmFlt0fcr3 {
            DfsdmFlt0fcr3(0)
        }
    }
    #[doc = "interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0icr(pub u32);
    impl DfsdmFlt0icr {
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub const fn clrjovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrjovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub const fn clrrovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrrovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear the clock absence flag."]
        #[inline(always)]
        pub const fn clrckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the clock absence flag."]
        #[inline(always)]
        pub fn set_clrckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Clear the short-circuit detector flag."]
        #[inline(always)]
        pub const fn clrscdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the short-circuit detector flag."]
        #[inline(always)]
        pub fn set_clrscdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DfsdmFlt0icr {
        #[inline(always)]
        fn default() -> DfsdmFlt0icr {
            DfsdmFlt0icr(0)
        }
    }
    #[doc = "interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0isr(pub u32);
    impl DfsdmFlt0isr {
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub const fn jeocf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub fn set_jeocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub const fn reocf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub fn set_reocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub const fn jovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_jovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub const fn rovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_rovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog."]
        #[inline(always)]
        pub const fn awdf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog."]
        #[inline(always)]
        pub fn set_awdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub const fn jcip(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub fn set_jcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub const fn rcip(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub fn set_rcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Clock absence flag."]
        #[inline(always)]
        pub const fn ckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clock absence flag."]
        #[inline(always)]
        pub fn set_ckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "short-circuit detector flag."]
        #[inline(always)]
        pub const fn scdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "short-circuit detector flag."]
        #[inline(always)]
        pub fn set_scdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DfsdmFlt0isr {
        #[inline(always)]
        fn default() -> DfsdmFlt0isr {
            DfsdmFlt0isr(0)
        }
    }
    #[doc = "injected channel group selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0jchgr(pub u32);
    impl DfsdmFlt0jchgr {
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub const fn jchg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub fn set_jchg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for DfsdmFlt0jchgr {
        #[inline(always)]
        fn default() -> DfsdmFlt0jchgr {
            DfsdmFlt0jchgr(0)
        }
    }
    #[doc = "data register for injected group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0jdatar(pub u32);
    impl DfsdmFlt0jdatar {
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub const fn jdatach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub fn set_jdatach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub const fn jdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0jdatar {
        #[inline(always)]
        fn default() -> DfsdmFlt0jdatar {
            DfsdmFlt0jdatar(0)
        }
    }
    #[doc = "data register for the regular channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt0rdatar(pub u32);
    impl DfsdmFlt0rdatar {
        #[doc = "Regular channel most recently converted."]
        #[inline(always)]
        pub const fn rdatach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel most recently converted."]
        #[inline(always)]
        pub fn set_rdatach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular channel pending data."]
        #[inline(always)]
        pub const fn rpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel pending data."]
        #[inline(always)]
        pub fn set_rpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub const fn rdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub fn set_rdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt0rdatar {
        #[inline(always)]
        fn default() -> DfsdmFlt0rdatar {
            DfsdmFlt0rdatar(0)
        }
    }
    #[doc = "analog watchdog clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1awcfr(pub u32);
    impl DfsdmFlt1awcfr {
        #[doc = "Clear the analog watchdog low threshold flag."]
        #[inline(always)]
        pub const fn clrawltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Clear the analog watchdog low threshold flag."]
        #[inline(always)]
        pub fn set_clrawltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Clear the analog watchdog high threshold flag."]
        #[inline(always)]
        pub const fn clrawhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Clear the analog watchdog high threshold flag."]
        #[inline(always)]
        pub fn set_clrawhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for DfsdmFlt1awcfr {
        #[inline(always)]
        fn default() -> DfsdmFlt1awcfr {
            DfsdmFlt1awcfr(0)
        }
    }
    #[doc = "analog watchdog high threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1awhtr(pub u32);
    impl DfsdmFlt1awhtr {
        #[doc = "Break signal assignment to analog watchdog high threshold event."]
        #[inline(always)]
        pub const fn bkawh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog high threshold event."]
        #[inline(always)]
        pub fn set_bkawh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Analog watchdog high threshold."]
        #[inline(always)]
        pub const fn awht(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog high threshold."]
        #[inline(always)]
        pub fn set_awht(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1awhtr {
        #[inline(always)]
        fn default() -> DfsdmFlt1awhtr {
            DfsdmFlt1awhtr(0)
        }
    }
    #[doc = "analog watchdog low threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1awltr(pub u32);
    impl DfsdmFlt1awltr {
        #[doc = "Break signal assignment to analog watchdog low threshold event."]
        #[inline(always)]
        pub const fn bkawl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog low threshold event."]
        #[inline(always)]
        pub fn set_bkawl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Analog watchdog low threshold."]
        #[inline(always)]
        pub const fn awlt(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog low threshold."]
        #[inline(always)]
        pub fn set_awlt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1awltr {
        #[inline(always)]
        fn default() -> DfsdmFlt1awltr {
            DfsdmFlt1awltr(0)
        }
    }
    #[doc = "analog watchdog status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1awsr(pub u32);
    impl DfsdmFlt1awsr {
        #[doc = "Analog watchdog low threshold flag."]
        #[inline(always)]
        pub const fn awltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog low threshold flag."]
        #[inline(always)]
        pub fn set_awltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Analog watchdog high threshold flag."]
        #[inline(always)]
        pub const fn awhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog high threshold flag."]
        #[inline(always)]
        pub fn set_awhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1awsr {
        #[inline(always)]
        fn default() -> DfsdmFlt1awsr {
            DfsdmFlt1awsr(0)
        }
    }
    #[doc = "conversion timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1cnvtimr(pub u32);
    impl DfsdmFlt1cnvtimr {
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN."]
        #[inline(always)]
        pub const fn cnvcnt(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN."]
        #[inline(always)]
        pub fn set_cnvcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for DfsdmFlt1cnvtimr {
        #[inline(always)]
        fn default() -> DfsdmFlt1cnvtimr {
            DfsdmFlt1cnvtimr(0)
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1cr1(pub u32);
    impl DfsdmFlt1cr1 {
        #[doc = "DFSDM enable."]
        #[inline(always)]
        pub const fn dfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM enable."]
        #[inline(always)]
        pub fn set_dfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start a conversion of the injected group of channels."]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger."]
        #[inline(always)]
        pub const fn jsync(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger."]
        #[inline(always)]
        pub fn set_jsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scanning conversion mode for injected conversions."]
        #[inline(always)]
        pub const fn jscan(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Scanning conversion mode for injected conversions."]
        #[inline(always)]
        pub fn set_jscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub const fn jdmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the injected channel group."]
        #[inline(always)]
        pub fn set_jdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Trigger signal selection for launching injected conversions."]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions."]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub const fn rswstart(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Software start of a conversion on the regular channel."]
        #[inline(always)]
        pub fn set_rswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub const fn rcont(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode selection for regular conversions."]
        #[inline(always)]
        pub fn set_rcont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Launch regular conversion synchronously with DFSDM0."]
        #[inline(always)]
        pub const fn rsync(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Launch regular conversion synchronously with DFSDM0."]
        #[inline(always)]
        pub fn set_rsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DMA channel enabled to read data for the regular conversion."]
        #[inline(always)]
        pub const fn rdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the regular conversion."]
        #[inline(always)]
        pub fn set_rdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub const fn rch(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Regular channel selection."]
        #[inline(always)]
        pub fn set_rch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Fast conversion mode selection for regular conversions."]
        #[inline(always)]
        pub const fn fast(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Fast conversion mode selection for regular conversions."]
        #[inline(always)]
        pub fn set_fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Analog watchdog fast mode select."]
        #[inline(always)]
        pub const fn awfsel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog fast mode select."]
        #[inline(always)]
        pub fn set_awfsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for DfsdmFlt1cr1 {
        #[inline(always)]
        fn default() -> DfsdmFlt1cr1 {
            DfsdmFlt1cr1(0)
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1cr2(pub u32);
    impl DfsdmFlt1cr2 {
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Injected end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub const fn reocie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Regular end of conversion interrupt enable."]
        #[inline(always)]
        pub fn set_reocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub const fn jovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_jovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub const fn rovrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular data overrun interrupt enable."]
        #[inline(always)]
        pub fn set_rovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable."]
        #[inline(always)]
        pub fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Short-circuit detector interrupt enable."]
        #[inline(always)]
        pub const fn scdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Short-circuit detector interrupt enable."]
        #[inline(always)]
        pub fn set_scdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clock absence interrupt enable."]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence interrupt enable."]
        #[inline(always)]
        pub fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Extremes detector channel selection."]
        #[inline(always)]
        pub const fn exch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Extremes detector channel selection."]
        #[inline(always)]
        pub fn set_exch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Analog watchdog channel selection."]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog channel selection."]
        #[inline(always)]
        pub fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DfsdmFlt1cr2 {
        #[inline(always)]
        fn default() -> DfsdmFlt1cr2 {
            DfsdmFlt1cr2(0)
        }
    }
    #[doc = "Extremes detector maximum register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1exmax(pub u32);
    impl DfsdmFlt1exmax {
        #[doc = "Extremes detector maximum data channel."]
        #[inline(always)]
        pub const fn exmaxch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Extremes detector maximum data channel."]
        #[inline(always)]
        pub fn set_exmaxch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Extremes detector maximum value."]
        #[inline(always)]
        pub const fn exmax(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Extremes detector maximum value."]
        #[inline(always)]
        pub fn set_exmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1exmax {
        #[inline(always)]
        fn default() -> DfsdmFlt1exmax {
            DfsdmFlt1exmax(0)
        }
    }
    #[doc = "Extremes detector minimum register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1exmin(pub u32);
    impl DfsdmFlt1exmin {
        #[doc = "Extremes detector minimum data channel."]
        #[inline(always)]
        pub const fn exminch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Extremes detector minimum data channel."]
        #[inline(always)]
        pub fn set_exminch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXMIN."]
        #[inline(always)]
        pub const fn exmin(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "EXMIN."]
        #[inline(always)]
        pub fn set_exmin(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1exmin {
        #[inline(always)]
        fn default() -> DfsdmFlt1exmin {
            DfsdmFlt1exmin(0)
        }
    }
    #[doc = "control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1fcr3(pub u32);
    impl DfsdmFlt1fcr3 {
        #[doc = "The integrator oversampling rate is 2 to the power of IOSR."]
        #[inline(always)]
        pub const fn iosr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The integrator oversampling rate is 2 to the power of IOSR."]
        #[inline(always)]
        pub fn set_iosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Sinc filter oversampling rate."]
        #[inline(always)]
        pub const fn fosr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Sinc filter oversampling rate."]
        #[inline(always)]
        pub fn set_fosr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Sinc Filter Order."]
        #[inline(always)]
        pub const fn ford(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Sinc Filter Order."]
        #[inline(always)]
        pub fn set_ford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for DfsdmFlt1fcr3 {
        #[inline(always)]
        fn default() -> DfsdmFlt1fcr3 {
            DfsdmFlt1fcr3(0)
        }
    }
    #[doc = "interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1icr(pub u32);
    impl DfsdmFlt1icr {
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub const fn clrjovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrjovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub const fn clrrovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_clrrovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear the clock absence flag."]
        #[inline(always)]
        pub const fn clrckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the clock absence flag."]
        #[inline(always)]
        pub fn set_clrckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Clear the short-circuit detector flag."]
        #[inline(always)]
        pub const fn clrscdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the short-circuit detector flag."]
        #[inline(always)]
        pub fn set_clrscdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DfsdmFlt1icr {
        #[inline(always)]
        fn default() -> DfsdmFlt1icr {
            DfsdmFlt1icr(0)
        }
    }
    #[doc = "interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1isr(pub u32);
    impl DfsdmFlt1isr {
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub const fn jeocf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag."]
        #[inline(always)]
        pub fn set_jeocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub const fn reocf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion flag."]
        #[inline(always)]
        pub fn set_reocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub const fn jovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion overrun flag."]
        #[inline(always)]
        pub fn set_jovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub const fn rovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion overrun flag."]
        #[inline(always)]
        pub fn set_rovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog."]
        #[inline(always)]
        pub const fn awdf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog."]
        #[inline(always)]
        pub fn set_awdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub const fn jcip(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion in progress status."]
        #[inline(always)]
        pub fn set_jcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub const fn rcip(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion in progress status."]
        #[inline(always)]
        pub fn set_rcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Clock absence flag."]
        #[inline(always)]
        pub const fn ckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clock absence flag."]
        #[inline(always)]
        pub fn set_ckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "short-circuit detector flag."]
        #[inline(always)]
        pub const fn scdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "short-circuit detector flag."]
        #[inline(always)]
        pub fn set_scdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DfsdmFlt1isr {
        #[inline(always)]
        fn default() -> DfsdmFlt1isr {
            DfsdmFlt1isr(0)
        }
    }
    #[doc = "injected channel group selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1jchgr(pub u32);
    impl DfsdmFlt1jchgr {
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub const fn jchg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Injected channel group selection."]
        #[inline(always)]
        pub fn set_jchg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for DfsdmFlt1jchgr {
        #[inline(always)]
        fn default() -> DfsdmFlt1jchgr {
            DfsdmFlt1jchgr(0)
        }
    }
    #[doc = "data register for injected group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1jdatar(pub u32);
    impl DfsdmFlt1jdatar {
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub const fn jdatach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Injected channel most recently converted."]
        #[inline(always)]
        pub fn set_jdatach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub const fn jdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Injected group conversion data."]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1jdatar {
        #[inline(always)]
        fn default() -> DfsdmFlt1jdatar {
            DfsdmFlt1jdatar(0)
        }
    }
    #[doc = "data register for the regular channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfsdmFlt1rdatar(pub u32);
    impl DfsdmFlt1rdatar {
        #[doc = "Regular channel most recently converted."]
        #[inline(always)]
        pub const fn rdatach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel most recently converted."]
        #[inline(always)]
        pub fn set_rdatach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular channel pending data."]
        #[inline(always)]
        pub const fn rpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel pending data."]
        #[inline(always)]
        pub fn set_rpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub const fn rdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Regular channel conversion data."]
        #[inline(always)]
        pub fn set_rdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for DfsdmFlt1rdatar {
        #[inline(always)]
        fn default() -> DfsdmFlt1rdatar {
            DfsdmFlt1rdatar(0)
        }
    }
}
