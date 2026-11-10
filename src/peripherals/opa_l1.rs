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
    #[doc = "OPA configuration 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OPA configuration 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "OPA control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPA control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "OPA unlockkey."]
    #[inline(always)]
    pub const fn opa_key(self) -> crate::common::Reg<regs::OpaKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CMP unlockkey."]
    #[inline(always)]
    pub const fn cmp_key(self) -> crate::common::Reg<regs::CmpKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "polling unlockkey."]
    #[inline(always)]
    pub const fn poll_key(self) -> crate::common::Reg<regs::PollKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPA configuration 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u16);
    impl Cfgr1 {
        #[doc = "OPA1 enable positive polling."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 enable positive polling."]
        #[inline(always)]
        pub const fn set_poll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "OPA1 break function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 break function enable."]
        #[inline(always)]
        pub const fn set_bkin_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "OPA1 reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 reset enable."]
        #[inline(always)]
        pub const fn set_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "POLL LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "POLL LOCK."]
        #[inline(always)]
        pub const fn set_poll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "OPA1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_out(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "OPA interrupt enable at the end of polling interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cnt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA interrupt enable at the end of polling interval."]
        #[inline(always)]
        pub const fn set_ie_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "OPA connection NMI interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nmi_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OPA connection NMI interrupt enable."]
        #[inline(always)]
        pub const fn set_nmi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "OPA1 output interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 output interrupt."]
        #[inline(always)]
        pub const fn set_if_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "OPA interrupt flag at the end of polling interval."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cnt(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPA interrupt flag at the end of polling interval."]
        #[inline(always)]
        pub const fn set_if_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("poll_en", &self.poll_en())
                .field("bkin_en", &self.bkin_en())
                .field("rst_en", &self.rst_en())
                .field("poll_lock", &self.poll_lock())
                .field("ie_out", &self.ie_out())
                .field("ie_cnt", &self.ie_cnt())
                .field("nmi_en", &self.nmi_en())
                .field("if_out", &self.if_out())
                .field("if_cnt", &self.if_cnt())
                .finish()
        }
    }
    #[doc = "OPA configuration 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u16);
    impl Cfgr2 {
        #[doc = "OPA1 polling interval."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_vlu(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "OPA1 polling interval."]
        #[inline(always)]
        pub const fn set_poll_vlu(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
        #[doc = "OPA1 polling the number of positive ends."]
        #[must_use]
        #[inline(always)]
        pub const fn poll1_num(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 polling the number of positive ends."]
        #[inline(always)]
        pub const fn set_poll1_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u16) & 0x07) << 9usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("poll_vlu", &self.poll_vlu())
                .field("poll1_num", &self.poll1_num())
                .finish()
        }
    }
    #[doc = "CMP unlockkey."]
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
    #[doc = "OPA control register 1."]
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
        #[doc = "OPA1 output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 output channel selection."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "OPA1 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "OPA1 forward input selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "OPA1 internal feedback resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 internal feedback resistance enable."]
        #[inline(always)]
        pub const fn set_fb_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "OPA1 negative end channel selection with PGA gain selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "OPA1 negative end channel selection with PGA gain selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "OPA1 low-power mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lp1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 low-power mode selection."]
        #[inline(always)]
        pub const fn set_lp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OPA1 high-level offset voltage value polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn intrimp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 high-level offset voltage value polarity selection."]
        #[inline(always)]
        pub const fn set_intrimp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPA1 high-level offset voltage value selection."]
        #[must_use]
        #[inline(always)]
        pub const fn itrimp(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x1f;
            val as u8
        }
        #[doc = "OPA1 high-level offset voltage value selection."]
        #[inline(always)]
        pub const fn set_itrimp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
        }
        #[doc = "OPA1 low-level offset voltage value polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn intrimn(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 low-level offset voltage value polarity selection."]
        #[inline(always)]
        pub const fn set_intrimn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "OPA1 low-level offset voltage value selection."]
        #[must_use]
        #[inline(always)]
        pub const fn itrimn(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "OPA1 low-level offset voltage value selection."]
        #[inline(always)]
        pub const fn set_itrimn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[doc = "OPA lock."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPA lock."]
        #[inline(always)]
        pub const fn set_opa_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("fb_en1", &self.fb_en1())
                .field("nsel1", &self.nsel1())
                .field("lp1", &self.lp1())
                .field("intrimp", &self.intrimp())
                .field("itrimp", &self.itrimp())
                .field("intrimn", &self.intrimn())
                .field("itrimn", &self.itrimn())
                .field("opa_lock", &self.opa_lock())
                .finish()
        }
    }
    #[doc = "OPA control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
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
        pub const fn nsel1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 negative input selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CMP1 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 forward input selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CMP1 hysteresis function selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 hysteresis function selection."]
        #[inline(always)]
        pub const fn set_hyen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMP1 low-power switch."]
        #[must_use]
        #[inline(always)]
        pub const fn lp1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 low-power switch."]
        #[inline(always)]
        pub const fn set_lp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        pub const fn nse2l(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 negative input selection."]
        #[inline(always)]
        pub const fn set_nse2l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CMP2 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 forward input selection."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CMP2 hysteresis function selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 hysteresis function selection."]
        #[inline(always)]
        pub const fn set_hyen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CMP2 low-power switch."]
        #[must_use]
        #[inline(always)]
        pub const fn lp2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 low-power switch."]
        #[inline(always)]
        pub const fn set_lp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CMP3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en3(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 enable."]
        #[inline(always)]
        pub const fn set_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CMP3 Output channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "CMP3 Output channel selection."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "CMP3 negative input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 negative input selection."]
        #[inline(always)]
        pub const fn set_nsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "CMP3 forward input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel3(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 forward input selection."]
        #[inline(always)]
        pub const fn set_psel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CMP3 hysteresis function selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 hysteresis function selection."]
        #[inline(always)]
        pub const fn set_hyen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CMP3 low-power switch."]
        #[must_use]
        #[inline(always)]
        pub const fn lp3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 low-power switch."]
        #[inline(always)]
        pub const fn set_lp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CMP wake-up signal mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_md(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "CMP wake-up signal mode selection."]
        #[inline(always)]
        pub const fn set_wkup_md(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "CMP lock."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CMP lock."]
        #[inline(always)]
        pub const fn set_cmp_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en1", &self.en1())
                .field("mode1", &self.mode1())
                .field("nsel1", &self.nsel1())
                .field("psel1", &self.psel1())
                .field("hyen1", &self.hyen1())
                .field("lp1", &self.lp1())
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("nse2l", &self.nse2l())
                .field("psel2", &self.psel2())
                .field("hyen2", &self.hyen2())
                .field("lp2", &self.lp2())
                .field("en3", &self.en3())
                .field("mode3", &self.mode3())
                .field("nsel3", &self.nsel3())
                .field("psel3", &self.psel3())
                .field("hyen3", &self.hyen3())
                .field("lp3", &self.lp3())
                .field("wkup_md", &self.wkup_md())
                .field("cmp_lock", &self.cmp_lock())
                .finish()
        }
    }
    #[doc = "OPA unlockkey."]
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
    #[doc = "polling unlockkey."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PollKey(pub u32);
    impl PollKey {
        #[doc = "polling unlockkey."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "polling unlockkey."]
        #[inline(always)]
        pub const fn set_poll_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PollKey {
        #[inline(always)]
        fn default() -> PollKey {
            PollKey(0)
        }
    }
    impl core::fmt::Debug for PollKey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PollKey")
                .field("poll_key", &self.poll_key())
                .finish()
        }
    }
}
