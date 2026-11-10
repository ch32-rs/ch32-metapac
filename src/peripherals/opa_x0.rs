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
    #[doc = "OPA Configuration register."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OPA Configuration register."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "OPA Control1 register."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPA Control2 register."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "OPA unlock key register."]
    #[inline(always)]
    pub const fn opa_key(self) -> crate::common::Reg<regs::OpaKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CMP unlock key register."]
    #[inline(always)]
    pub const fn cmp_key(self) -> crate::common::Reg<regs::CmpKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "POLL unlock key register."]
    #[inline(always)]
    pub const fn poll_key(self) -> crate::common::Reg<regs::PollKey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPA Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u16);
    impl Cfgr1 {
        #[doc = "POLL_EN1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "POLL_EN1 Enable."]
        #[inline(always)]
        pub const fn set_poll_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "POLL_EN2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_en2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "POLL_EN2 Enable."]
        #[inline(always)]
        pub const fn set_poll_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "BKIN_EN1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_en1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BKIN_EN1 Enable."]
        #[inline(always)]
        pub const fn set_bkin_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "BKIN_EN2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_en2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BKIN_EN2 Enable."]
        #[inline(always)]
        pub const fn set_bkin_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "RST_EN1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RST_EN1 Enable."]
        #[inline(always)]
        pub const fn set_rst_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RST_EN2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_en2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RST_EN2 Enable."]
        #[inline(always)]
        pub const fn set_rst_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "BKIN_SEL select."]
        #[must_use]
        #[inline(always)]
        pub const fn bkin_sel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "BKIN_SEL select."]
        #[inline(always)]
        pub const fn set_bkin_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "POLL_LOCK select."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "POLL_LOCK select."]
        #[inline(always)]
        pub const fn set_poll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "IE_OUT1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_out1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IE_OUT1 Enable."]
        #[inline(always)]
        pub const fn set_ie_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "IE_OUT2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_out2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "IE_OUT2 Enable."]
        #[inline(always)]
        pub const fn set_ie_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "IE_CNT Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cnt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IE_CNT Enable."]
        #[inline(always)]
        pub const fn set_ie_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "NMI_EN Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nmi_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "NMI_EN Enable."]
        #[inline(always)]
        pub const fn set_nmi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "IF_OUT1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IF_OUT1 Enable."]
        #[inline(always)]
        pub const fn set_if_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "IF_OUT2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn if_out2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IF_OUT2 Enable."]
        #[inline(always)]
        pub const fn set_if_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "IF_CNT Configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cnt(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IF_CNT Configuration."]
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
                .field("poll_en1", &self.poll_en1())
                .field("poll_en2", &self.poll_en2())
                .field("bkin_en1", &self.bkin_en1())
                .field("bkin_en2", &self.bkin_en2())
                .field("rst_en1", &self.rst_en1())
                .field("rst_en2", &self.rst_en2())
                .field("bkin_sel", &self.bkin_sel())
                .field("poll_lock", &self.poll_lock())
                .field("ie_out1", &self.ie_out1())
                .field("ie_out2", &self.ie_out2())
                .field("ie_cnt", &self.ie_cnt())
                .field("nmi_en", &self.nmi_en())
                .field("if_out1", &self.if_out1())
                .field("if_out2", &self.if_out2())
                .field("if_cnt", &self.if_cnt())
                .finish()
        }
    }
    #[doc = "OPA Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u16);
    impl Cfgr2 {
        #[doc = "POLL_VLU Configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_vlu(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "POLL_VLU Configuration."]
        #[inline(always)]
        pub const fn set_poll_vlu(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
        #[doc = "POLL1_NUM Configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn poll1_num(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "POLL1_NUM Configuration."]
        #[inline(always)]
        pub const fn set_poll1_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u16) & 0x03) << 9usize);
        }
        #[doc = "POLL2_NUM Configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn poll2_num(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "POLL2_NUM Configuration."]
        #[inline(always)]
        pub const fn set_poll2_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u16) & 0x03) << 11usize);
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
                .field("poll2_num", &self.poll2_num())
                .finish()
        }
    }
    #[doc = "CMP unlock key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpKey(pub u32);
    impl CmpKey {
        #[doc = "CMP_KEY value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CMP_KEY value."]
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
    #[doc = "OPA Control1 register."]
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
        #[doc = "OPA1 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 mode."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Input select."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Input select."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "res inside enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "res inside enable."]
        #[inline(always)]
        pub const fn set_fb_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "output select."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "output select."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "OPA2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 enable."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPA2 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 mode."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Input select."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Input select."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "res inside enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_en2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "res inside enable."]
        #[inline(always)]
        pub const fn set_fb_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "output select."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "output select."]
        #[inline(always)]
        pub const fn set_nsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[doc = "OPA_LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OPA_LOCK."]
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
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("psel2", &self.psel2())
                .field("fb_en2", &self.fb_en2())
                .field("nsel2", &self.nsel2())
                .field("opa_lock", &self.opa_lock())
                .finish()
        }
    }
    #[doc = "OPA Control2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "CMP1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 Enable."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMP1 OUT Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 OUT Selection."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CMP1 IN_N Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 IN_N Selection."]
        #[inline(always)]
        pub const fn set_nsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CMP1 IN_P Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 IN_P Selection."]
        #[inline(always)]
        pub const fn set_psel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CMP1 HYEN Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CMP1 HYEN Enable."]
        #[inline(always)]
        pub const fn set_hyen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CMP2 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 Enable."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMP2 OUT Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 OUT Selection."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMP2 IN_N Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 IN_N Selection."]
        #[inline(always)]
        pub const fn set_nsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CMP2 IN_P Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 IN_P Selection."]
        #[inline(always)]
        pub const fn set_psel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CMP2 HYEN Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CMP2 HYEN Enable."]
        #[inline(always)]
        pub const fn set_hyen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CMP3 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 Enable."]
        #[inline(always)]
        pub const fn set_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CMP3 OUT Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 OUT Selection."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CMP3 IN_N Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel3(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 IN_N Selection."]
        #[inline(always)]
        pub const fn set_nsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CMP3 IN_P Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn psel3(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 IN_P Selection."]
        #[inline(always)]
        pub const fn set_psel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CMP3 HYEN Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hyen3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CMP3 HYEN Enable."]
        #[inline(always)]
        pub const fn set_hyen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CMP_LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CMP_LOCK."]
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
                .field("en2", &self.en2())
                .field("mode2", &self.mode2())
                .field("nsel2", &self.nsel2())
                .field("psel2", &self.psel2())
                .field("hyen2", &self.hyen2())
                .field("en3", &self.en3())
                .field("mode3", &self.mode3())
                .field("nsel3", &self.nsel3())
                .field("psel3", &self.psel3())
                .field("hyen3", &self.hyen3())
                .field("cmp_lock", &self.cmp_lock())
                .finish()
        }
    }
    #[doc = "OPA unlock key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpaKey(pub u32);
    impl OpaKey {
        #[doc = "OPA_KEY value."]
        #[must_use]
        #[inline(always)]
        pub const fn opa_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OPA_KEY value."]
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
    #[doc = "POLL unlock key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PollKey(pub u32);
    impl PollKey {
        #[doc = "POLL_KEY value."]
        #[must_use]
        #[inline(always)]
        pub const fn poll_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "POLL_KEY value."]
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
