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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Event mask register (EXTI_EVENR)."]
    #[inline(always)]
    pub const fn evenr(self) -> crate::common::Reg<regs::Evenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Rising Trigger selection register (EXTI_RTENR)."]
    #[inline(always)]
    pub const fn rtenr(self) -> crate::common::Reg<regs::Rtenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Falling Trigger selection register (EXTI_FTENR)."]
    #[inline(always)]
    pub const fn ftenr(self) -> crate::common::Reg<regs::Ftenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEVR)."]
    #[inline(always)]
    pub const fn swievr(self) -> crate::common::Reg<regs::Swievr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Interrupt flag register (EXTI_INTFR)."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "Event mask register (EXTI_EVENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evenr(pub u32);
    impl Evenr {
        #[doc = "Event Mask on line 0."]
        #[inline(always)]
        pub const fn mr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Event Mask on line 0."]
        #[inline(always)]
        pub fn set_mr(&mut self, n: usize, val: bool) {
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
    #[doc = "Falling Trigger selection register (EXTI_FTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ftenr(pub u32);
    impl Ftenr {
        #[doc = "Falling trigger event configuration of line 0."]
        #[inline(always)]
        pub const fn tr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Falling trigger event configuration of line 0."]
        #[inline(always)]
        pub fn set_tr(&mut self, n: usize, val: bool) {
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
    #[doc = "Interrupt mask register (EXTI_INTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intenr(pub u32);
    impl Intenr {
        #[doc = "Interrupt Mask on line 0."]
        #[inline(always)]
        pub const fn mr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Mask on line 0."]
        #[inline(always)]
        pub fn set_mr(&mut self, n: usize, val: bool) {
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
    #[doc = "Interrupt flag register (EXTI_INTFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Pending flag for the selected line. Write 1 to clear."]
        #[inline(always)]
        pub const fn if_(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Pending flag for the selected line. Write 1 to clear."]
        #[inline(always)]
        pub fn set_if_(&mut self, n: usize, val: bool) {
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
    #[doc = "Rising Trigger selection register (EXTI_RTENR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtenr(pub u32);
    impl Rtenr {
        #[doc = "Rising trigger event configuration of line 0."]
        #[inline(always)]
        pub const fn tr(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rising trigger event configuration of line 0."]
        #[inline(always)]
        pub fn set_tr(&mut self, n: usize, val: bool) {
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
    #[doc = "Software interrupt event register (EXTI_SWIEVR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swievr(pub u32);
    impl Swievr {
        #[doc = "Software Interrupt on line 0."]
        #[inline(always)]
        pub const fn swie(&self, n: usize) -> bool {
            assert!(n < 22usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt on line 0."]
        #[inline(always)]
        pub fn set_swie(&mut self, n: usize, val: bool) {
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
}
