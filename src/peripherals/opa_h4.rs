#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPA configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa {
    ptr: *mut u8,
}
unsafe impl Send for Opa {}
unsafe impl Sync for Opa {}
impl Opa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OPA Control Register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OPA Control Register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPA Control Register 3."]
    #[inline(always)]
    pub const fn ctlr3(self) -> crate::common::Reg<regs::Ctlr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CMP Control Registers."]
    #[inline(always)]
    pub const fn cmp_ctlr(self) -> crate::common::Reg<regs::CmpCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CMP Status Registers."]
    #[inline(always)]
    pub const fn cmp_statr(self) -> crate::common::Reg<regs::CmpStatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "CMP Control Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpCtlr(pub u32);
    impl CmpCtlr {
        #[doc = "CMP Positive Channel Selection Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn psel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CMP Positive Channel Selection Bits."]
        #[inline(always)]
        pub const fn set_psel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CMP Negative End Channel Selection Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "CMP Negative End Channel Selection Bits."]
        #[inline(always)]
        pub const fn set_nsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "CMP Output Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "CMP Output Channel Selection."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "CMP enables."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP enables."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CMP hysteresis voltage selector."]
        #[must_use]
        #[inline(always)]
        pub const fn hypsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "CMP hysteresis voltage selector."]
        #[inline(always)]
        pub const fn set_hypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "CMP Internal Bias Voltage Selector."]
        #[must_use]
        #[inline(always)]
        pub const fn vref(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "CMP Internal Bias Voltage Selector."]
        #[inline(always)]
        pub const fn set_vref(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "CMP Digital Filtering Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CMP Digital Filtering Enable."]
        #[inline(always)]
        pub const fn set_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CMP filter sampling interval configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_cfg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "CMP filter sampling interval configuration."]
        #[inline(always)]
        pub const fn set_filt_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "CMP filtering sampling time base configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_base(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "CMP filtering sampling time base configuration."]
        #[inline(always)]
        pub const fn set_filt_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for CmpCtlr {
        #[inline(always)]
        fn default() -> CmpCtlr {
            CmpCtlr(0)
        }
    }
    impl core::fmt::Debug for CmpCtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpCtlr")
                .field("psel", &self.psel())
                .field("nsel", &self.nsel())
                .field("mode", &self.mode())
                .field("en", &self.en())
                .field("hypsel", &self.hypsel())
                .field("vref", &self.vref())
                .field("filt_en", &self.filt_en())
                .field("filt_cfg", &self.filt_cfg())
                .field("filt_base", &self.filt_base())
                .finish()
        }
    }
    #[doc = "CMP Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpStatr(pub u32);
    impl CmpStatr {
        #[doc = "CMP Output."]
        #[must_use]
        #[inline(always)]
        pub const fn out_filt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP Output."]
        #[inline(always)]
        pub const fn set_out_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CmpStatr {
        #[inline(always)]
        fn default() -> CmpStatr {
            CmpStatr(0)
        }
    }
    impl core::fmt::Debug for CmpStatr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpStatr")
                .field("out_filt", &self.out_filt())
                .finish()
        }
    }
    #[doc = "OPA Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "OPA1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 enable."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA1 Output Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 Output Channel Selection."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA1 Positive Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 Positive Channel Selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OPA1 Negative Channel Selection vs. PGA Gain Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 Negative Channel Selection vs. PGA Gain Selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "OPA1's PGA mode feedback enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1's PGA mode feedback enable."]
        #[inline(always)]
        pub const fn set_fb_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA1 is used with NSEL1 as PGA and the N-terminal is connected to OPA1_CHN1 (PA7)."]
        #[must_use]
        #[inline(always)]
        pub const fn pgadif1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 is used with NSEL1 as PGA and the N-terminal is connected to OPA1_CHN1 (PA7)."]
        #[inline(always)]
        pub const fn set_pgadif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OPA1 high-speed mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn hs1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 high-speed mode enables."]
        #[inline(always)]
        pub const fn set_hs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    impl core::fmt::Debug for Ctlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr1")
                .field("en1", &self.en1())
                .field("mode1", &self.mode1())
                .field("psel1", &self.psel1())
                .field("nsel1", &self.nsel1())
                .field("fb_en1", &self.fb_en1())
                .field("pgadif1", &self.pgadif1())
                .field("hs1", &self.hs1())
                .finish()
        }
    }
    #[doc = "OPA Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "OPA2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 enable."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA2 Output Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA2 Output Channel Selection."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA2 Positive Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 Positive Channel Selection."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OPA2 Negative Channel Selection vs. PGA Gain Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "OPA2 Negative Channel Selection vs. PGA Gain Selection."]
        #[inline(always)]
        pub const fn set_nsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "OPA2's PGA mode feedback enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2's PGA mode feedback enable."]
        #[inline(always)]
        pub const fn set_fb_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA2 is used with NSEL2 as PGA and the N-terminal is connected to OPA2_CHN1 (PF12)."]
        #[must_use]
        #[inline(always)]
        pub const fn pgadif2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 is used with NSEL2 as PGA and the N-terminal is connected to OPA2_CHN1 (PF12)."]
        #[inline(always)]
        pub const fn set_pgadif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OPA2 high-speed mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn hs2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 high-speed mode enables."]
        #[inline(always)]
        pub const fn set_hs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    impl core::fmt::Debug for Ctlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr2")
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("psel2", &self.psel2())
                .field("nsel2", &self.nsel2())
                .field("fb_en2", &self.fb_en2())
                .field("pgadif2", &self.pgadif2())
                .field("hs2", &self.hs2())
                .finish()
        }
    }
    #[doc = "OPA Control Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr3(pub u32);
    impl Ctlr3 {
        #[doc = "OPA3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 enable."]
        #[inline(always)]
        pub const fn set_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA3 Output Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA3 Output Channel Selection."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA3 Positive Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 Positive Channel Selection."]
        #[inline(always)]
        pub const fn set_psel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OPA3 Negative Channel Selection vs. PGA Gain Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "OPA3 Negative Channel Selection vs. PGA Gain Selection."]
        #[inline(always)]
        pub const fn set_nsel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "OPA3's PGA mode feedback enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3's PGA mode feedback enable."]
        #[inline(always)]
        pub const fn set_fb_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA3 is used with NSEL3 as PGA and the N-terminal is connected to OPA3_CHN1 (PA3)."]
        #[must_use]
        #[inline(always)]
        pub const fn pgadif3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 is used with NSEL3 as PGA and the N-terminal is connected to OPA3_CHN1 (PA3)."]
        #[inline(always)]
        pub const fn set_pgadif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OPA3 high-speed mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn hs3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 high-speed mode enables."]
        #[inline(always)]
        pub const fn set_hs3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ctlr3 {
        #[inline(always)]
        fn default() -> Ctlr3 {
            Ctlr3(0)
        }
    }
    impl core::fmt::Debug for Ctlr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr3")
                .field("en3", &self.en3())
                .field("mode3", &self.mode3())
                .field("psel3", &self.psel3())
                .field("nsel3", &self.nsel3())
                .field("fb_en3", &self.fb_en3())
                .field("pgadif3", &self.pgadif3())
                .field("hs3", &self.hs3())
                .finish()
        }
    }
}
