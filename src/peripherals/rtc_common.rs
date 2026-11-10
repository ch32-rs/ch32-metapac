#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Real time clock."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC Control Register High."]
    #[inline(always)]
    pub const fn ctlrh(self) -> crate::common::Reg<regs::Ctlrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC Control Register Low."]
    #[inline(always)]
    pub const fn ctlrl(self) -> crate::common::Reg<regs::Ctlrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC Prescaler Load Register High."]
    #[inline(always)]
    pub const fn pscrh(self) -> crate::common::Reg<regs::Pscrh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RTC Prescaler Load Register Low."]
    #[inline(always)]
    pub const fn pscrl(self) -> crate::common::Reg<regs::Pscrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RTC Prescaler Divider Register High."]
    #[inline(always)]
    pub const fn divh(self) -> crate::common::Reg<regs::Divh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RTC Prescaler Divider Register Low."]
    #[inline(always)]
    pub const fn divl(self) -> crate::common::Reg<regs::Divl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RTC Counter Register High."]
    #[inline(always)]
    pub const fn cnth(self) -> crate::common::Reg<regs::Cnth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RTC Counter Register Low."]
    #[inline(always)]
    pub const fn cntl(self) -> crate::common::Reg<regs::Cntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RTC Alarm Register High."]
    #[inline(always)]
    pub const fn alrmh(self) -> crate::common::Reg<regs::Alrmh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RTC Alarm Register Low."]
    #[inline(always)]
    pub const fn alrml(self) -> crate::common::Reg<regs::Alrml, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "RTC Alarm Register High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmh(pub u32);
    impl Alrmh {
        #[doc = "RTC alarm register high."]
        #[must_use]
        #[inline(always)]
        pub const fn alrh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC alarm register high."]
        #[inline(always)]
        pub const fn set_alrh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Alrmh {
        #[inline(always)]
        fn default() -> Alrmh {
            Alrmh(0)
        }
    }
    impl core::fmt::Debug for Alrmh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrmh").field("alrh", &self.alrh()).finish()
        }
    }
    #[doc = "RTC Alarm Register Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrml(pub u32);
    impl Alrml {
        #[doc = "RTC alarm register low."]
        #[must_use]
        #[inline(always)]
        pub const fn alrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC alarm register low."]
        #[inline(always)]
        pub const fn set_alrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Alrml {
        #[inline(always)]
        fn default() -> Alrml {
            Alrml(0)
        }
    }
    impl core::fmt::Debug for Alrml {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrml").field("alrl", &self.alrl()).finish()
        }
    }
    #[doc = "RTC Counter Register High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnth(pub u32);
    impl Cnth {
        #[doc = "RTC counter register high."]
        #[must_use]
        #[inline(always)]
        pub const fn cnth(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC counter register high."]
        #[inline(always)]
        pub const fn set_cnth(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnth {
        #[inline(always)]
        fn default() -> Cnth {
            Cnth(0)
        }
    }
    impl core::fmt::Debug for Cnth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnth").field("cnth", &self.cnth()).finish()
        }
    }
    #[doc = "RTC Counter Register Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntl(pub u32);
    impl Cntl {
        #[doc = "RTC counter register Low."]
        #[must_use]
        #[inline(always)]
        pub const fn cntl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC counter register Low."]
        #[inline(always)]
        pub const fn set_cntl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntl {
        #[inline(always)]
        fn default() -> Cntl {
            Cntl(0)
        }
    }
    impl core::fmt::Debug for Cntl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntl").field("cntl", &self.cntl()).finish()
        }
    }
    #[doc = "RTC Control Register High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlrh(pub u32);
    impl Ctlrh {
        #[doc = "Second interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn secie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Second interrupt Enable."]
        #[inline(always)]
        pub const fn set_secie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alarm interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn alrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt Enable."]
        #[inline(always)]
        pub const fn set_alrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Overflow interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn owie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow interrupt Enable."]
        #[inline(always)]
        pub const fn set_owie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Ctlrh {
        #[inline(always)]
        fn default() -> Ctlrh {
            Ctlrh(0)
        }
    }
    impl core::fmt::Debug for Ctlrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlrh")
                .field("secie", &self.secie())
                .field("alrie", &self.alrie())
                .field("owie", &self.owie())
                .finish()
        }
    }
    #[doc = "RTC Control Register Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlrl(pub u32);
    impl Ctlrl {
        #[doc = "Second Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn secf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Second Flag."]
        #[inline(always)]
        pub const fn set_secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alarm Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn alrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm Flag."]
        #[inline(always)]
        pub const fn set_alrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Overflow Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn owf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Flag."]
        #[inline(always)]
        pub const fn set_owf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Registers Synchronized Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Registers Synchronized Flag."]
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Configuration Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cnf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Flag."]
        #[inline(always)]
        pub const fn set_cnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RTC operation OFF."]
        #[must_use]
        #[inline(always)]
        pub const fn rtoff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RTC operation OFF."]
        #[inline(always)]
        pub const fn set_rtoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ctlrl {
        #[inline(always)]
        fn default() -> Ctlrl {
            Ctlrl(0)
        }
    }
    impl core::fmt::Debug for Ctlrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlrl")
                .field("secf", &self.secf())
                .field("alrf", &self.alrf())
                .field("owf", &self.owf())
                .field("rsf", &self.rsf())
                .field("cnf", &self.cnf())
                .field("rtoff", &self.rtoff())
                .finish()
        }
    }
    #[doc = "RTC Prescaler Divider Register High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divh(pub u32);
    impl Divh {
        #[doc = "RTC prescaler divider register high."]
        #[must_use]
        #[inline(always)]
        pub const fn divh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RTC prescaler divider register high."]
        #[inline(always)]
        pub const fn set_divh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Divh {
        #[inline(always)]
        fn default() -> Divh {
            Divh(0)
        }
    }
    impl core::fmt::Debug for Divh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divh").field("divh", &self.divh()).finish()
        }
    }
    #[doc = "RTC Prescaler Divider Register Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divl(pub u32);
    impl Divl {
        #[doc = "RTC prescaler divider register Low."]
        #[must_use]
        #[inline(always)]
        pub const fn divl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC prescaler divider register Low."]
        #[inline(always)]
        pub const fn set_divl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Divl {
        #[inline(always)]
        fn default() -> Divl {
            Divl(0)
        }
    }
    impl core::fmt::Debug for Divl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divl").field("divl", &self.divl()).finish()
        }
    }
    #[doc = "RTC Prescaler Load Register High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pscrh(pub u32);
    impl Pscrh {
        #[doc = "RTC Prescaler Load Register High."]
        #[must_use]
        #[inline(always)]
        pub const fn prlh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RTC Prescaler Load Register High."]
        #[inline(always)]
        pub const fn set_prlh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Pscrh {
        #[inline(always)]
        fn default() -> Pscrh {
            Pscrh(0)
        }
    }
    impl core::fmt::Debug for Pscrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pscrh").field("prlh", &self.prlh()).finish()
        }
    }
    #[doc = "RTC Prescaler Load Register Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pscrl(pub u32);
    impl Pscrl {
        #[doc = "RTC Prescaler Divider Register Low."]
        #[must_use]
        #[inline(always)]
        pub const fn prll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RTC Prescaler Divider Register Low."]
        #[inline(always)]
        pub const fn set_prll(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pscrl {
        #[inline(always)]
        fn default() -> Pscrl {
            Pscrl(0)
        }
    }
    impl core::fmt::Debug for Pscrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pscrl").field("prll", &self.prll()).finish()
        }
    }
}
