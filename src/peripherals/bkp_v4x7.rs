#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Backup registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp {
    ptr: *mut u8,
}
unsafe impl Send for Bkp {}
unsafe impl Sync for Bkp {}
impl Bkp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Backup data register 1. Index 0 is DATAR1."]
    #[inline(always)]
    pub const fn datar(self, n: usize) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        assert!(n < 10usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "RTC clock calibration register."]
    #[inline(always)]
    pub const fn octlr(self) -> crate::common::Reg<regs::Octlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Tamper control register."]
    #[inline(always)]
    pub const fn tpctlr(self) -> crate::common::Reg<regs::Tpctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Tamper control/status register."]
    #[inline(always)]
    pub const fn tpcsr(self) -> crate::common::Reg<regs::Tpcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Backup data register 11. Index 0 is DATAR11."]
    #[inline(always)]
    pub const fn datar_h(self, n: usize) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Backup data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "Backup data."]
        #[inline(always)]
        pub const fn d(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Backup data."]
        #[inline(always)]
        pub fn set_d(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "RTC clock calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Octlr(pub u32);
    impl Octlr {
        #[doc = "Calibration value, number of clock pulses skipped out of every 2^20."]
        #[inline(always)]
        pub const fn cal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Calibration value, number of clock pulses skipped out of every 2^20."]
        #[inline(always)]
        pub fn set_cal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Calibration clock output, RTC clock divided by 64 on the TAMPER pin."]
        #[inline(always)]
        pub const fn cco(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration clock output, RTC clock divided by 64 on the TAMPER pin."]
        #[inline(always)]
        pub fn set_cco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alarm or second pulse output enable."]
        #[inline(always)]
        pub const fn asoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm or second pulse output enable."]
        #[inline(always)]
        pub fn set_asoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Alarm or second pulse output selection."]
        #[inline(always)]
        pub const fn asos(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm or second pulse output selection."]
        #[inline(always)]
        pub fn set_asos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Octlr {
        #[inline(always)]
        fn default() -> Octlr {
            Octlr(0)
        }
    }
    #[doc = "Tamper control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpcsr(pub u32);
    impl Tpcsr {
        #[doc = "Clear tamper event."]
        #[inline(always)]
        pub const fn cte(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear tamper event."]
        #[inline(always)]
        pub fn set_cte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear tamper interrupt."]
        #[inline(always)]
        pub const fn cti(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear tamper interrupt."]
        #[inline(always)]
        pub fn set_cti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Tamper pin interrupt enable."]
        #[inline(always)]
        pub const fn tpie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper pin interrupt enable."]
        #[inline(always)]
        pub fn set_tpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tamper event flag."]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper event flag."]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tamper interrupt flag."]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper interrupt flag."]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Tpcsr {
        #[inline(always)]
        fn default() -> Tpcsr {
            Tpcsr(0)
        }
    }
    #[doc = "Tamper control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpctlr(pub u32);
    impl Tpctlr {
        #[doc = "Tamper pin enable."]
        #[inline(always)]
        pub const fn tpe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper pin enable."]
        #[inline(always)]
        pub fn set_tpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tamper pin active level."]
        #[inline(always)]
        pub const fn tpal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper pin active level."]
        #[inline(always)]
        pub fn set_tpal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Tpctlr {
        #[inline(always)]
        fn default() -> Tpctlr {
            Tpctlr(0)
        }
    }
}
