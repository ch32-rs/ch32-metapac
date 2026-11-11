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
    #[doc = "OPA configuration Register 1."]
    #[inline(always)]
    pub const fn opa_cfgr1(self) -> crate::common::Reg<regs::OpaCfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OPA control register 1."]
    #[inline(always)]
    pub const fn opa_ctlr1(self) -> crate::common::Reg<regs::OpaCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPA control register 2."]
    #[inline(always)]
    pub const fn opa_ctlr2(self) -> crate::common::Reg<regs::OpaCtlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CMP control register."]
    #[inline(always)]
    pub const fn cmp_ctlr(self) -> crate::common::Reg<regs::CmpCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Comparator Filter Configuration Register."]
    #[inline(always)]
    pub const fn cmp_filt(self) -> crate::common::Reg<regs::CmpFilt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OPA unlock key Register."]
    #[inline(always)]
    pub const fn opa_key(self) -> crate::common::Reg<regs::OpaKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CMP unlock key Register."]
    #[inline(always)]
    pub const fn cmp_key(self) -> crate::common::Reg<regs::CmpKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "OPA and CMP Status Registers."]
    #[inline(always)]
    pub const fn opcm_sr(self) -> crate::common::Reg<regs::OpcmSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "CMP control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpCtlr(pub u32);
    impl CmpCtlr {
        #[doc = "CMP1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 enable."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMP1 Output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 Output channel selection."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "CMP1 negative input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 negative input selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "CMP1 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "CMP1 forward input selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "CMP1 hysteresis comparator switch."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 hysteresis comparator switch."]
        #[inline(always)]
        pub const fn set_hyen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CMP2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 enable."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CMP2 Output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "CMP2 Output channel selection."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "CMP2 negative input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel2(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "CMP2 negative input selection."]
        #[inline(always)]
        pub const fn set_nsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "CMP2 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CMP2 forward input selection."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "CMP2 hysteresis comparator switch."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 hysteresis comparator switch."]
        #[inline(always)]
        pub const fn set_hyen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CMP1 reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 reset enable."]
        #[inline(always)]
        pub const fn set_rst_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CMP2 reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 reset enable."]
        #[inline(always)]
        pub const fn set_rst_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer brake source configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer brake source configuration."]
        #[inline(always)]
        pub const fn set_bkin_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CMP1 low-power switch."]
        #[must_use]
        #[inline(always)]
        pub const fn lp1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 low-power switch."]
        #[inline(always)]
        pub const fn set_lp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CMP2 low-power switch."]
        #[must_use]
        #[inline(always)]
        pub const fn lp2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 low-power switch."]
        #[inline(always)]
        pub const fn set_lp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CMP wake-up signal mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_md(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "CMP wake-up signal mode selection."]
        #[inline(always)]
        pub const fn set_wkup_md(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "DAC enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DAC enable bit."]
        #[inline(always)]
        pub const fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DAC data configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dacdat(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "DAC data configuration bit."]
        #[inline(always)]
        pub const fn set_dacdat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
        #[doc = "OPA LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPA LOCK."]
        #[inline(always)]
        pub const fn set_cmp_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en1", &self.en1())
                .field("mode1", &self.mode1())
                .field("nsel1", &self.nsel1())
                .field("psel1", &self.psel1())
                .field("hyen1", &self.hyen1())
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("nsel2", &self.nsel2())
                .field("psel2", &self.psel2())
                .field("hyen2", &self.hyen2())
                .field("rst_en1", &self.rst_en1())
                .field("rst_en2", &self.rst_en2())
                .field("bkin_sel", &self.bkin_sel())
                .field("lp1", &self.lp1())
                .field("lp2", &self.lp2())
                .field("wkup_md", &self.wkup_md())
                .field("dacen", &self.dacen())
                .field("dacdat", &self.dacdat())
                .field("cmp_lock", &self.cmp_lock())
                .finish()
        }
    }
    #[doc = "Comparator Filter Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpFilt(pub u32);
    impl CmpFilt {
        #[doc = "CMP1 Filter Enable Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 Filter Enable Bits."]
        #[inline(always)]
        pub const fn set_filt_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The CMP1 filter sampling interval configuration bit is 0.375us."]
        #[must_use]
        #[inline(always)]
        pub const fn filt1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "The CMP1 filter sampling interval configuration bit is 0.375us."]
        #[inline(always)]
        pub const fn set_filt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "CMP2 Filter Enable Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_en2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 Filter Enable Bits."]
        #[inline(always)]
        pub const fn set_filt_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "The CMP2 filter sampling interval configuration bit is 0.375us."]
        #[must_use]
        #[inline(always)]
        pub const fn filt2(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x1f;
            val as u8
        }
        #[doc = "The CMP2 filter sampling interval configuration bit is 0.375us."]
        #[inline(always)]
        pub const fn set_filt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
        }
    }
    impl Default for CmpFilt {
        #[inline(always)]
        fn default() -> CmpFilt {
            CmpFilt(0)
        }
    }
    impl core::fmt::Debug for CmpFilt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpFilt")
                .field("filt_en1", &self.filt_en1())
                .field("filt1", &self.filt1())
                .field("filt_en2", &self.filt_en2())
                .field("filt2", &self.filt2())
                .finish()
        }
    }
    #[doc = "CMP unlock key Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpKey(pub u32);
    impl CmpKey {
        #[doc = "CMP unlockkey."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CMP unlockkey."]
        #[inline(always)]
        pub const fn set_cmp_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmpKey {
        #[inline(always)]
        fn default() -> CmpKey {
            CmpKey(0)
        }
    }
    impl core::fmt::Debug for CmpKey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpKey")
                .field("cmp_key", &self.cmp_key())
                .finish()
        }
    }
    #[doc = "OPA configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaCfgr1(pub u32);
    impl OpaCfgr1 {
        #[doc = "Timer brake configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_cfg(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Timer brake configuration."]
        #[inline(always)]
        pub const fn set_bkin_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "OPA1 reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 reset enable."]
        #[inline(always)]
        pub const fn set_rst_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OPA1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_out1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA interrupt enable at the end of polling interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cnt1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OPA interrupt enable at the end of polling interval."]
        #[inline(always)]
        pub const fn set_ie_cnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OPA connection NMI interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nmi_en1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA connection NMI interrupt enable."]
        #[inline(always)]
        pub const fn set_nmi_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "OPA1 establishes a time configuration bit in system clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn stcfg1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "OPA1 establishes a time configuration bit in system clock cycles."]
        #[inline(always)]
        pub const fn set_stcfg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Sampling Source Selection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn smpsel1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Sampling Source Selection Bit."]
        #[inline(always)]
        pub const fn set_smpsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "OPA1 polling mode configuration register."]
        #[must_use]
        #[inline(always)]
        pub const fn pollmd1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 polling mode configuration register."]
        #[inline(always)]
        pub const fn set_pollmd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPA LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPA LOCK."]
        #[inline(always)]
        pub const fn set_opa_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OpaCfgr1 {
        #[inline(always)]
        fn default() -> OpaCfgr1 {
            OpaCfgr1(0)
        }
    }
    impl core::fmt::Debug for OpaCfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpaCfgr1")
                .field("bkin_cfg", &self.bkin_cfg())
                .field("rst_en1", &self.rst_en1())
                .field("ie_out1", &self.ie_out1())
                .field("ie_cnt1", &self.ie_cnt1())
                .field("nmi_en1", &self.nmi_en1())
                .field("stcfg1", &self.stcfg1())
                .field("smpsel1", &self.smpsel1())
                .field("pollmd1", &self.pollmd1())
                .field("opa_lock", &self.opa_lock())
                .finish()
        }
    }
    #[doc = "OPA control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaCtlr1(pub u32);
    impl OpaCtlr1 {
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
        #[doc = "OPA1 output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 output channel selection."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA1 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 forward input selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "OPA1 negative end channel selection with PGA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 negative end channel selection with PGA gain selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OPA1 Gain Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pga_sel1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 Gain Selection."]
        #[inline(always)]
        pub const fn set_pga_sel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "OPA1 internal feedback resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 internal feedback resistance enable."]
        #[inline(always)]
        pub const fn set_fb_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "OPA1 Output Bias Voltage Selection for Internal PGA."]
        #[must_use]
        #[inline(always)]
        pub const fn pgadif1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 Output Bias Voltage Selection for Internal PGA."]
        #[inline(always)]
        pub const fn set_pgadif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPA1 high-speed mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn hs1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 high-speed mode enables."]
        #[inline(always)]
        pub const fn set_hs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OPA1 Chopper Enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn chopen1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 Chopper Enablement."]
        #[inline(always)]
        pub const fn set_chopen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPA1 Chopper Frequency Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn chopsel1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 Chopper Frequency Selection."]
        #[inline(always)]
        pub const fn set_chopsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OPA1 enable positive polling."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_en1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 enable positive polling."]
        #[inline(always)]
        pub const fn set_poll_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPA1 polling the number of positive ends."]
        #[must_use]
        #[inline(always)]
        pub const fn poll1_num(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 polling the number of positive ends."]
        #[inline(always)]
        pub const fn set_poll1_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "OPA1 polling trigger source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_sel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 polling trigger source selection."]
        #[inline(always)]
        pub const fn set_poll_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
        #[doc = "To start OPA1 polling, you need to set the software trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_swstrt(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "To start OPA1 polling, you need to set the software trigger."]
        #[inline(always)]
        pub const fn set_poll_swstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OPA1 Polling Order."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_ch1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 Polling Order."]
        #[inline(always)]
        pub const fn set_poll_ch1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "OPA1 Polling Order."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_ch2(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 Polling Order."]
        #[inline(always)]
        pub const fn set_poll_ch2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "OPA1 Polling Order."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_ch3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "OPA1 Polling Order."]
        #[inline(always)]
        pub const fn set_poll_ch3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for OpaCtlr1 {
        #[inline(always)]
        fn default() -> OpaCtlr1 {
            OpaCtlr1(0)
        }
    }
    impl core::fmt::Debug for OpaCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpaCtlr1")
                .field("en1", &self.en1())
                .field("mode1", &self.mode1())
                .field("psel1", &self.psel1())
                .field("nsel1", &self.nsel1())
                .field("pga_sel1", &self.pga_sel1())
                .field("fb_en1", &self.fb_en1())
                .field("pgadif1", &self.pgadif1())
                .field("hs1", &self.hs1())
                .field("chopen1", &self.chopen1())
                .field("chopsel1", &self.chopsel1())
                .field("poll_en1", &self.poll_en1())
                .field("poll1_num", &self.poll1_num())
                .field("poll_sel", &self.poll_sel())
                .field("poll_swstrt", &self.poll_swstrt())
                .field("poll_ch1", &self.poll_ch1())
                .field("poll_ch2", &self.poll_ch2())
                .field("poll_ch3", &self.poll_ch3())
                .finish()
        }
    }
    #[doc = "OPA control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaCtlr2(pub u32);
    impl OpaCtlr2 {
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
        #[doc = "OPA2 output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "OPA2 output channel selection."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "OPA2 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 forward input selection."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OPA2 negative end channel selection with PGA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 negative end channel selection with PGA gain selection."]
        #[inline(always)]
        pub const fn set_nsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OPA2 Gain Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pga_sel2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "OPA2 Gain Selection."]
        #[inline(always)]
        pub const fn set_pga_sel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "OPA2 internal feedback resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 internal feedback resistance enable."]
        #[inline(always)]
        pub const fn set_fb_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "OPA2 for internal PGA output bias voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pgadif2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 for internal PGA output bias voltage selection."]
        #[inline(always)]
        pub const fn set_pgadif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPA2 high-speed mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn hs2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 high-speed mode enables."]
        #[inline(always)]
        pub const fn set_hs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for OpaCtlr2 {
        #[inline(always)]
        fn default() -> OpaCtlr2 {
            OpaCtlr2(0)
        }
    }
    impl core::fmt::Debug for OpaCtlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpaCtlr2")
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("psel2", &self.psel2())
                .field("nsel2", &self.nsel2())
                .field("pga_sel2", &self.pga_sel2())
                .field("fb_en2", &self.fb_en2())
                .field("pgadif2", &self.pgadif2())
                .field("hs2", &self.hs2())
                .finish()
        }
    }
    #[doc = "OPA unlock key Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaKey(pub u32);
    impl OpaKey {
        #[doc = "OPA unlockkey."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OPA unlockkey."]
        #[inline(always)]
        pub const fn set_opa_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpaKey {
        #[inline(always)]
        fn default() -> OpaKey {
            OpaKey(0)
        }
    }
    impl core::fmt::Debug for OpaKey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpaKey")
                .field("opa_key", &self.opa_key())
                .finish()
        }
    }
    #[doc = "OPA and CMP Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpcmSr(pub u32);
    impl OpcmSr {
        #[doc = "The 1st polling channel polls for a flag whose output is high."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out_poll_ch1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The 1st polling channel polls for a flag whose output is high."]
        #[inline(always)]
        pub const fn set_if_out_poll_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The second polling channel polls to the flag that the output is high."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out_poll_ch2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The second polling channel polls to the flag that the output is high."]
        #[inline(always)]
        pub const fn set_if_out_poll_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The 3rd polling channel polls to the flag that the output is high."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out_poll_ch3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The 3rd polling channel polls to the flag that the output is high."]
        #[inline(always)]
        pub const fn set_if_out_poll_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPA1 Polling Interval End Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cnt1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 Polling Interval End Flag."]
        #[inline(always)]
        pub const fn set_if_cnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for OpcmSr {
        #[inline(always)]
        fn default() -> OpcmSr {
            OpcmSr(0)
        }
    }
    impl core::fmt::Debug for OpcmSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpcmSr")
                .field("if_out_poll_ch1", &self.if_out_poll_ch1())
                .field("if_out_poll_ch2", &self.if_out_poll_ch2())
                .field("if_out_poll_ch3", &self.if_out_poll_ch3())
                .field("if_cnt1", &self.if_cnt1())
                .finish()
        }
    }
}
