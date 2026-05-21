#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ESIG configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esig {
    ptr: *mut u8,
}
unsafe impl Send for Esig {}
unsafe impl Sync for Esig {}
impl Esig {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash capacity register."]
    #[inline(always)]
    pub const fn flacap(self) -> crate::common::Reg<regs::Flacap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "UID register."]
    #[inline(always)]
    pub const fn uniid1(self) -> crate::common::Reg<regs::Uniid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "UID register."]
    #[inline(always)]
    pub const fn uniid2(self) -> crate::common::Reg<regs::Uniid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Div register."]
    #[inline(always)]
    pub const fn uniid3(self) -> crate::common::Reg<regs::Uniid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash capacity register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flacap(pub u16);
    impl Flacap {
        #[doc = "F_SIZE/kByte."]
        #[inline(always)]
        pub const fn f_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "F_SIZE/kByte."]
        #[inline(always)]
        pub fn set_f_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Flacap {
        #[inline(always)]
        fn default() -> Flacap {
            Flacap(0)
        }
    }
    #[doc = "UID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uniid1(pub u32);
    impl Uniid1 {
        #[doc = "U_ID value."]
        #[inline(always)]
        pub const fn u_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "U_ID value."]
        #[inline(always)]
        pub fn set_u_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uniid1 {
        #[inline(always)]
        fn default() -> Uniid1 {
            Uniid1(0)
        }
    }
    #[doc = "UID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uniid2(pub u32);
    impl Uniid2 {
        #[doc = "U_ID value."]
        #[inline(always)]
        pub const fn u_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "U_ID value."]
        #[inline(always)]
        pub fn set_u_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uniid2 {
        #[inline(always)]
        fn default() -> Uniid2 {
            Uniid2(0)
        }
    }
    #[doc = "Div register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uniid3(pub u32);
    impl Uniid3 {
        #[doc = "U_ID value."]
        #[inline(always)]
        pub const fn u_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "U_ID value."]
        #[inline(always)]
        pub fn set_u_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uniid3 {
        #[inline(always)]
        fn default() -> Uniid3 {
            Uniid3(0)
        }
    }
}
