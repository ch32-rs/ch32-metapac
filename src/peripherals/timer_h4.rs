#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Advanced timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtm {
    ptr: *mut u8,
}
unsafe impl Send for Adtm {}
unsafe impl Sync for Adtm {}
impl Adtm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::Dmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::Swevgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[inline(always)]
    pub const fn chctlr_output(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register."]
    #[inline(always)]
    pub const fn rptcr(self) -> crate::common::Reg<regs::Rptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "capture/compare register 1."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<u16, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register."]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Auxiliary register (dead-time / brake select / dual-edge capture)."]
    #[inline(always)]
    pub const fn aux(self) -> crate::common::Reg<regs::Aux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
#[doc = "Basic timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bctm {
    ptr: *mut u8,
}
unsafe impl Send for Bctm {}
unsafe impl Sync for Bctm {}
impl Bctm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::Dmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::Swevgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "General purpose timer, 16bit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptm {
    ptr: *mut u8,
}
unsafe impl Send for Gptm {}
unsafe impl Sync for Gptm {}
impl Gptm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::Dmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::Swevgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)."]
    #[inline(always)]
    pub const fn chctlr_output(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register 1."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<u16, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
#[doc = "General purpose timer, 32bit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptm32 {
    ptr: *mut u8,
}
unsafe impl Send for Gptm32 {}
unsafe impl Sync for Gptm32 {}
impl Gptm32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::Dmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::Swevgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)."]
    #[inline(always)]
    pub const fn chctlr_output(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChctlrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter, 32bit."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register, 32bit."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register, 32bit."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Double-side edge capture register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aux(pub u16);
    impl Aux {
        #[doc = "Double-side edge capture is enable for channel2."]
        #[inline(always)]
        pub const fn cap_ed_ch2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Double-side edge capture is enable for channel2."]
        #[inline(always)]
        pub fn set_cap_ed_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Double-side edge capture is enable for channel3."]
        #[inline(always)]
        pub const fn cap_ed_ch3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Double-side edge capture is enable for channel3."]
        #[inline(always)]
        pub fn set_cap_ed_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Double-side edge capture is enable for channel4."]
        #[inline(always)]
        pub const fn cap_ed_ch4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Double-side edge capture is enable for channel4."]
        #[inline(always)]
        pub fn set_cap_ed_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Break source selection."]
        #[inline(always)]
        pub const fn bk_sel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Break source selection."]
        #[inline(always)]
        pub fn set_bk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u16) & 0x07) << 3usize);
        }
        #[doc = "The dead-time duration of the DT_VLU2 enabled setting occurs. on the rising edge of OCXREF."]
        #[inline(always)]
        pub const fn dtp_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The dead-time duration of the DT_VLU2 enabled setting occurs. on the rising edge of OCXREF."]
        #[inline(always)]
        pub fn set_dtp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "The dead-time duration of the DT_VLU2 enabled setting occurs. on the falling edge of OCXREF."]
        #[inline(always)]
        pub const fn dtn_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The dead-time duration of the DT_VLU2 enabled setting occurs. on the falling edge of OCXREF."]
        #[inline(always)]
        pub fn set_dtn_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "set the duration of the dead zone."]
        #[inline(always)]
        pub const fn dt_vlu2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "set the duration of the dead zone."]
        #[inline(always)]
        pub fn set_dt_vlu2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Aux {
        #[inline(always)]
        fn default() -> Aux {
            Aux(0)
        }
    }
    #[doc = "break and dead-time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr(pub u32);
    impl Bdtr {
        #[doc = "Dead-time generator setup."]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup."]
        #[inline(always)]
        pub fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration."]
        #[inline(always)]
        pub const fn lock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Lock configuration."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode."]
        #[inline(always)]
        pub const fn ossi(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Off-state selection for Idle mode."]
        #[inline(always)]
        pub fn set_ossi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode."]
        #[inline(always)]
        pub const fn ossr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Off-state selection for Run mode."]
        #[inline(always)]
        pub fn set_ossr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Break enable."]
        #[inline(always)]
        pub const fn bke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Break enable."]
        #[inline(always)]
        pub fn set_bke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Break polarity."]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Break polarity."]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Automatic output enable."]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable."]
        #[inline(always)]
        pub fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable."]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable."]
        #[inline(always)]
        pub fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Bdtr {
        #[inline(always)]
        fn default() -> Bdtr {
            Bdtr(0)
        }
    }
    #[doc = "capture/compare enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer(pub u32);
    impl Ccer {
        #[doc = "Capture/Compare 1 output enable."]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable."]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 complementary output enable."]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 complementary output enable."]
        #[inline(always)]
        pub fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer {
        #[inline(always)]
        fn default() -> Ccer {
            Ccer(0)
        }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChctlrInput(pub u32);
    impl ChctlrInput {
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for ChctlrInput {
        #[inline(always)]
        fn default() -> ChctlrInput {
            ChctlrInput(0)
        }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChctlrOutput(pub u32);
    impl ChctlrOutput {
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output Compare 1 fast enable."]
        #[inline(always)]
        pub const fn iocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 fast enable."]
        #[inline(always)]
        pub fn set_iocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Compare 1 preload enable."]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 preload enable."]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Compare 1 mode."]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output Compare 1 mode."]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Output Compare 1 clear enable."]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 clear enable."]
        #[inline(always)]
        pub fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ChctlrOutput {
        #[inline(always)]
        fn default() -> ChctlrOutput {
            ChctlrOutput(0)
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable."]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable."]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source."]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source."]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode."]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode."]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Direction."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection."]
        #[inline(always)]
        pub const fn cms(&self) -> super::vals::Cms {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cms::from_bits(val as u8)
        }
        #[doc = "Center-aligned mode selection."]
        #[inline(always)]
        pub fn set_cms(&mut self, val: super::vals::Cms) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division."]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division."]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Timer capture value configuration enable."]
        #[inline(always)]
        pub const fn capov(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer capture value configuration enable."]
        #[inline(always)]
        pub fn set_capov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timer capture level indication enable."]
        #[inline(always)]
        pub const fn caplvl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timer capture level indication enable."]
        #[inline(always)]
        pub fn set_caplvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "Capture/compare preloaded control."]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control."]
        #[inline(always)]
        pub fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection."]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection."]
        #[inline(always)]
        pub fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection."]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection."]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection."]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection."]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection."]
        #[inline(always)]
        pub const fn ti1s(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TI1 selection."]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    #[doc = "DMA address for full transfer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaadr(pub u32);
    impl Dmaadr {
        #[doc = "DMA register for burst accesses."]
        #[inline(always)]
        pub const fn dmaadr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA register for burst accesses."]
        #[inline(always)]
        pub fn set_dmaadr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dmaadr {
        #[inline(always)]
        fn default() -> Dmaadr {
            Dmaadr(0)
        }
    }
    #[doc = "DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfgr(pub u32);
    impl Dmacfgr {
        #[doc = "DMA base address."]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address."]
        #[inline(always)]
        pub fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Dmacfgr {
        #[inline(always)]
        fn default() -> Dmacfgr {
            Dmacfgr(0)
        }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaintenr(pub u32);
    impl Dmaintenr {
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable."]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable."]
        #[inline(always)]
        pub fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable."]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable."]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable."]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable."]
        #[inline(always)]
        pub fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable."]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable."]
        #[inline(always)]
        pub fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable."]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable."]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Dmaintenr {
        #[inline(always)]
        fn default() -> Dmaintenr {
            Dmaintenr(0)
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intfr(pub u32);
    impl Intfr {
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag."]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag."]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag."]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag."]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt flag."]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt flag."]
        #[inline(always)]
        pub fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intfr {
        #[inline(always)]
        fn default() -> Intfr {
            Intfr(0)
        }
    }
    #[doc = "repetition counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptcr(pub u32);
    impl Rptcr {
        #[doc = "Repetition counter value."]
        #[inline(always)]
        pub const fn rptcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value."]
        #[inline(always)]
        pub fn set_rptcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rptcr {
        #[inline(always)]
        fn default() -> Rptcr {
            Rptcr(0)
        }
    }
    #[doc = "slave mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcfgr(pub u32);
    impl Smcfgr {
        #[doc = "Slave mode selection."]
        #[inline(always)]
        pub const fn sms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Slave mode selection."]
        #[inline(always)]
        pub fn set_sms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection."]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger selection."]
        #[inline(always)]
        pub fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode."]
        #[inline(always)]
        pub const fn msm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master/Slave mode."]
        #[inline(always)]
        pub fn set_msm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter."]
        #[inline(always)]
        pub const fn etf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External trigger filter."]
        #[inline(always)]
        pub fn set_etf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler."]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler."]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock enable."]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock enable."]
        #[inline(always)]
        pub fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity."]
        #[inline(always)]
        pub const fn etp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger polarity."]
        #[inline(always)]
        pub fn set_etp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Smcfgr {
        #[inline(always)]
        fn default() -> Smcfgr {
            Smcfgr(0)
        }
    }
    #[doc = "event generation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swevgr(pub u32);
    impl Swevgr {
        #[doc = "Update generation."]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation."]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation."]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation."]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation."]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation."]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation."]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation."]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break generation."]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break generation."]
        #[inline(always)]
        pub fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Swevgr {
        #[inline(always)]
        fn default() -> Swevgr {
            Swevgr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        ONCOMPARE = 0x0,
        #[doc = "CCx DMA request sent when update event occurs"]
        ONUPDATE = 0x01,
    }
    impl Ccds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccds {
        #[inline(always)]
        fn from(val: u8) -> Ccds {
            Ccds::from_bits(val)
        }
    }
    impl From<Ccds> for u8 {
        #[inline(always)]
        fn from(val: Ccds) -> u8 {
            Ccds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrInputCcs {
        _RESERVED_0 = 0x0,
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        TI4 = 0x01,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        TI3 = 0x02,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        TRC = 0x03,
    }
    impl CcmrInputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrInputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrInputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrInputCcs {
            CcmrInputCcs::from_bits(val)
        }
    }
    impl From<CcmrInputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrInputCcs) -> u8 {
            CcmrInputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        OUTPUT = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcmrOutputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrOutputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrOutputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrOutputCcs {
            CcmrOutputCcs::from_bits(val)
        }
    }
    impl From<CcmrOutputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrOutputCcs) -> u8 {
            CcmrOutputCcs::to_bits(val)
        }
    }
    #[doc = "division ratio between the timer clock (CK_INT) frequency, the dead time and the sampling clock used by the dead time generator and the digital filter (ETR,TIx)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckd {
        #[doc = "Tdts=Tck_int"]
        DIV_1 = 0x0,
        #[doc = "Tdts=2*Tck_int"]
        DIV_2 = 0x01,
        #[doc = "Tdts=4*Tck_int"]
        DIV_4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckd {
        #[inline(always)]
        fn from(val: u8) -> Ckd {
            Ckd::from_bits(val)
        }
    }
    impl From<Ckd> for u8 {
        #[inline(always)]
        fn from(val: Ckd) -> u8 {
            Ckd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EDGEALIGNED = 0x0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CENTERALIGNED1 = 0x01,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CENTERALIGNED2 = 0x02,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CENTERALIGNED3 = 0x03,
    }
    impl Cms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cms {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cms {
        #[inline(always)]
        fn from(val: u8) -> Cms {
            Cms::from_bits(val)
        }
    }
    impl From<Cms> for u8 {
        #[inline(always)]
        fn from(val: Cms) -> u8 {
            Cms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        UP = 0x0,
        #[doc = "Counter used as downcounter"]
        DOWN = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[doc = "External trigger prescaler."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etps {
        #[doc = "ETRP frequency divided by 1"]
        DIV_1 = 0x0,
        #[doc = "ETRP frequency divided by 2"]
        DIV_2 = 0x01,
        #[doc = "ETRP frequency divided by 4"]
        DIV_4 = 0x02,
        #[doc = "ETRP frequency divided by 8"]
        DIV_8 = 0x03,
    }
    impl Etps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etps {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etps {
        #[inline(always)]
        fn from(val: u8) -> Etps {
            Etps::from_bits(val)
        }
    }
    impl From<Etps> for u8 {
        #[inline(always)]
        fn from(val: Etps) -> u8 {
            Etps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FilterValue {
        #[doc = "No filter, sampling is done at fDTS"]
        NOFILTER = 0x0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FCK_INT_N2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FCK_INT_N4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FCK_INT_N8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FDTS_DIV2_N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FDTS_DIV2_N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FDTS_DIV4_N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FDTS_DIV4_N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FDTS_DIV8_N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FDTS_DIV8_N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FDTS_DIV16_N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FDTS_DIV16_N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FDTS_DIV16_N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FDTS_DIV32_N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FDTS_DIV32_N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FDTS_DIV32_N8 = 0x0f,
    }
    impl FilterValue {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterValue {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterValue {
        #[inline(always)]
        fn from(val: u8) -> FilterValue {
            FilterValue::from_bits(val)
        }
    }
    impl From<FilterValue> for u8 {
        #[inline(always)]
        fn from(val: FilterValue) -> u8 {
            FilterValue::to_bits(val)
        }
    }
    #[doc = "Master mode selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mms {
        #[doc = "Reset"]
        RESET = 0x0,
        #[doc = "Enable"]
        ENABLE = 0x01,
        #[doc = "Update"]
        UPDATE = 0x02,
        #[doc = "Compare Pulse"]
        COMPARE_PULSE = 0x03,
        #[doc = "Compare OC1REF"]
        COMPARE_OC1REF = 0x04,
        #[doc = "Compare OC2REF"]
        COMPARE_OC2REF = 0x05,
        #[doc = "Compare OC3REF"]
        COMPARE_OC3REF = 0x06,
        #[doc = "Compare OC4REF"]
        COMPARE_OC4REF = 0x07,
    }
    impl Mms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms {
        #[inline(always)]
        fn from(val: u8) -> Mms {
            Mms::from_bits(val)
        }
    }
    impl From<Mms> for u8 {
        #[inline(always)]
        fn from(val: Mms) -> u8 {
            Mms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        FROZEN = 0x0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ACTIVEONMATCH = 0x01,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        INACTIVEONMATCH = 0x02,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        TOGGLE = 0x03,
        #[doc = "OCyREF is forced low"]
        FORCEINACTIVE = 0x04,
        #[doc = "OCyREF is forced high"]
        FORCEACTIVE = 0x05,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PWMMODE1 = 0x06,
        #[doc = "Inversely to PwmMode1"]
        PWMMODE2 = 0x07,
    }
    impl Ocm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocm {
        #[inline(always)]
        fn from(val: u8) -> Ocm {
            Ocm::from_bits(val)
        }
    }
    impl From<Ocm> for u8 {
        #[inline(always)]
        fn from(val: Ocm) -> u8 {
            Ocm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        ANYEVENT = 0x0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        COUNTERONLY = 0x01,
    }
    impl Urs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Urs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Urs {
        #[inline(always)]
        fn from(val: u8) -> Urs {
            Urs::from_bits(val)
        }
    }
    impl From<Urs> for u8 {
        #[inline(always)]
        fn from(val: Urs) -> u8 {
            Urs::to_bits(val)
        }
    }
}
