#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CH32V407/CH32V467 external interrupt and event controller, lines 0..21."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt mask register (EXTI_INTENR)."]
    #[inline(always)]
    pub const fn intenr(self) -> crate::common::Reg<regs::Intenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event mask register (EXTI_EVENR)."]
    #[inline(always)]
    pub const fn evenr(self) -> crate::common::Reg<regs::Evenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Rising Trigger selection register (EXTI_RTENR)."]
    #[inline(always)]
    pub const fn rtenr(self) -> crate::common::Reg<regs::Rtenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Falling Trigger selection register (EXTI_FTENR)."]
    #[inline(always)]
    pub const fn ftenr(self) -> crate::common::Reg<regs::Ftenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEVR)."]
    #[inline(always)]
    pub const fn swievr(self) -> crate::common::Reg<regs::Swievr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt flag register (EXTI_INTFR)."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "Event mask register (EXTI_EVENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evenr(pub u32);
    impl Evenr {
        #[doc = "Event Mask on line 0."]
        #[must_use]
        #[inline(always)]
        pub const fn mr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Event Mask on line 0."]
        #[inline(always)]
        pub const fn set_mr(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Evenr {
        #[inline(always)]
        fn default() -> Evenr {
            Evenr(0)
        }
    }
    impl core::fmt::Debug for Evenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evenr")
                .field("mr[0]", &self.mr(0usize))
                .field("mr[1]", &self.mr(1usize))
                .field("mr[2]", &self.mr(2usize))
                .field("mr[3]", &self.mr(3usize))
                .field("mr[4]", &self.mr(4usize))
                .field("mr[5]", &self.mr(5usize))
                .field("mr[6]", &self.mr(6usize))
                .field("mr[7]", &self.mr(7usize))
                .field("mr[8]", &self.mr(8usize))
                .field("mr[9]", &self.mr(9usize))
                .field("mr[10]", &self.mr(10usize))
                .field("mr[11]", &self.mr(11usize))
                .field("mr[12]", &self.mr(12usize))
                .field("mr[13]", &self.mr(13usize))
                .field("mr[14]", &self.mr(14usize))
                .field("mr[15]", &self.mr(15usize))
                .field("mr[16]", &self.mr(16usize))
                .field("mr[17]", &self.mr(17usize))
                .field("mr[18]", &self.mr(18usize))
                .field("mr[19]", &self.mr(19usize))
                .field("mr[20]", &self.mr(20usize))
                .field("mr[21]", &self.mr(21usize))
                .finish()
        }
    }
    #[doc = "Falling Trigger selection register (EXTI_FTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ftenr(pub u32);
    impl Ftenr {
        #[doc = "Falling trigger event configuration of line 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Falling trigger event configuration of line 0."]
        #[inline(always)]
        pub const fn set_tr(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ftenr {
        #[inline(always)]
        fn default() -> Ftenr {
            Ftenr(0)
        }
    }
    impl core::fmt::Debug for Ftenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ftenr")
                .field("tr[0]", &self.tr(0usize))
                .field("tr[1]", &self.tr(1usize))
                .field("tr[2]", &self.tr(2usize))
                .field("tr[3]", &self.tr(3usize))
                .field("tr[4]", &self.tr(4usize))
                .field("tr[5]", &self.tr(5usize))
                .field("tr[6]", &self.tr(6usize))
                .field("tr[7]", &self.tr(7usize))
                .field("tr[8]", &self.tr(8usize))
                .field("tr[9]", &self.tr(9usize))
                .field("tr[10]", &self.tr(10usize))
                .field("tr[11]", &self.tr(11usize))
                .field("tr[12]", &self.tr(12usize))
                .field("tr[13]", &self.tr(13usize))
                .field("tr[14]", &self.tr(14usize))
                .field("tr[15]", &self.tr(15usize))
                .field("tr[16]", &self.tr(16usize))
                .field("tr[17]", &self.tr(17usize))
                .field("tr[18]", &self.tr(18usize))
                .field("tr[19]", &self.tr(19usize))
                .field("tr[20]", &self.tr(20usize))
                .field("tr[21]", &self.tr(21usize))
                .finish()
        }
    }
    #[doc = "Interrupt mask register (EXTI_INTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intenr(pub u32);
    impl Intenr {
        #[doc = "Interrupt Mask on line 0."]
        #[must_use]
        #[inline(always)]
        pub const fn mr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Mask on line 0."]
        #[inline(always)]
        pub const fn set_mr(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intenr {
        #[inline(always)]
        fn default() -> Intenr {
            Intenr(0)
        }
    }
    impl core::fmt::Debug for Intenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intenr")
                .field("mr[0]", &self.mr(0usize))
                .field("mr[1]", &self.mr(1usize))
                .field("mr[2]", &self.mr(2usize))
                .field("mr[3]", &self.mr(3usize))
                .field("mr[4]", &self.mr(4usize))
                .field("mr[5]", &self.mr(5usize))
                .field("mr[6]", &self.mr(6usize))
                .field("mr[7]", &self.mr(7usize))
                .field("mr[8]", &self.mr(8usize))
                .field("mr[9]", &self.mr(9usize))
                .field("mr[10]", &self.mr(10usize))
                .field("mr[11]", &self.mr(11usize))
                .field("mr[12]", &self.mr(12usize))
                .field("mr[13]", &self.mr(13usize))
                .field("mr[14]", &self.mr(14usize))
                .field("mr[15]", &self.mr(15usize))
                .field("mr[16]", &self.mr(16usize))
                .field("mr[17]", &self.mr(17usize))
                .field("mr[18]", &self.mr(18usize))
                .field("mr[19]", &self.mr(19usize))
                .field("mr[20]", &self.mr(20usize))
                .field("mr[21]", &self.mr(21usize))
                .finish()
        }
    }
    #[doc = "Interrupt flag register (EXTI_INTFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Pending flag for the selected line. Write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Pending flag for the selected line. Write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intfr {
        #[inline(always)]
        fn default() -> Intfr {
            Intfr(0)
        }
    }
    impl core::fmt::Debug for Intfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intfr")
                .field("if_[0]", &self.if_(0usize))
                .field("if_[1]", &self.if_(1usize))
                .field("if_[2]", &self.if_(2usize))
                .field("if_[3]", &self.if_(3usize))
                .field("if_[4]", &self.if_(4usize))
                .field("if_[5]", &self.if_(5usize))
                .field("if_[6]", &self.if_(6usize))
                .field("if_[7]", &self.if_(7usize))
                .field("if_[8]", &self.if_(8usize))
                .field("if_[9]", &self.if_(9usize))
                .field("if_[10]", &self.if_(10usize))
                .field("if_[11]", &self.if_(11usize))
                .field("if_[12]", &self.if_(12usize))
                .field("if_[13]", &self.if_(13usize))
                .field("if_[14]", &self.if_(14usize))
                .field("if_[15]", &self.if_(15usize))
                .field("if_[16]", &self.if_(16usize))
                .field("if_[17]", &self.if_(17usize))
                .field("if_[18]", &self.if_(18usize))
                .field("if_[19]", &self.if_(19usize))
                .field("if_[20]", &self.if_(20usize))
                .field("if_[21]", &self.if_(21usize))
                .finish()
        }
    }
    #[doc = "Rising Trigger selection register (EXTI_RTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtenr(pub u32);
    impl Rtenr {
        #[doc = "Rising trigger event configuration of line 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rising trigger event configuration of line 0."]
        #[inline(always)]
        pub const fn set_tr(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rtenr {
        #[inline(always)]
        fn default() -> Rtenr {
            Rtenr(0)
        }
    }
    impl core::fmt::Debug for Rtenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtenr")
                .field("tr[0]", &self.tr(0usize))
                .field("tr[1]", &self.tr(1usize))
                .field("tr[2]", &self.tr(2usize))
                .field("tr[3]", &self.tr(3usize))
                .field("tr[4]", &self.tr(4usize))
                .field("tr[5]", &self.tr(5usize))
                .field("tr[6]", &self.tr(6usize))
                .field("tr[7]", &self.tr(7usize))
                .field("tr[8]", &self.tr(8usize))
                .field("tr[9]", &self.tr(9usize))
                .field("tr[10]", &self.tr(10usize))
                .field("tr[11]", &self.tr(11usize))
                .field("tr[12]", &self.tr(12usize))
                .field("tr[13]", &self.tr(13usize))
                .field("tr[14]", &self.tr(14usize))
                .field("tr[15]", &self.tr(15usize))
                .field("tr[16]", &self.tr(16usize))
                .field("tr[17]", &self.tr(17usize))
                .field("tr[18]", &self.tr(18usize))
                .field("tr[19]", &self.tr(19usize))
                .field("tr[20]", &self.tr(20usize))
                .field("tr[21]", &self.tr(21usize))
                .finish()
        }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEVR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swievr(pub u32);
    impl Swievr {
        #[doc = "Software Interrupt on line 0."]
        #[must_use]
        #[inline(always)]
        pub const fn swie(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt on line 0."]
        #[inline(always)]
        pub const fn set_swie(&mut self, n: usize, val: bool) {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swievr {
        #[inline(always)]
        fn default() -> Swievr {
            Swievr(0)
        }
    }
    impl core::fmt::Debug for Swievr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swievr")
                .field("swie[0]", &self.swie(0usize))
                .field("swie[1]", &self.swie(1usize))
                .field("swie[2]", &self.swie(2usize))
                .field("swie[3]", &self.swie(3usize))
                .field("swie[4]", &self.swie(4usize))
                .field("swie[5]", &self.swie(5usize))
                .field("swie[6]", &self.swie(6usize))
                .field("swie[7]", &self.swie(7usize))
                .field("swie[8]", &self.swie(8usize))
                .field("swie[9]", &self.swie(9usize))
                .field("swie[10]", &self.swie(10usize))
                .field("swie[11]", &self.swie(11usize))
                .field("swie[12]", &self.swie(12usize))
                .field("swie[13]", &self.swie(13usize))
                .field("swie[14]", &self.swie(14usize))
                .field("swie[15]", &self.swie(15usize))
                .field("swie[16]", &self.swie(16usize))
                .field("swie[17]", &self.swie(17usize))
                .field("swie[18]", &self.swie(18usize))
                .field("swie[19]", &self.swie(19usize))
                .field("swie[20]", &self.swie(20usize))
                .field("swie[21]", &self.swie(21usize))
                .finish()
        }
    }
}
