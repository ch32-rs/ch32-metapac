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
    pub const fn ctlr1(self) -> crate::common::Reg<regs::AdtmCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::AdtmCtlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::AdtmDmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::AdtmIntfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::AdtmSwevgr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr1_input(
        self,
    ) -> crate::common::Reg<regs::AdtmChctlr1Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[inline(always)]
    pub const fn chctlr1_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr1Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 2 (input mode)."]
    #[inline(always)]
    pub const fn chctlr2_input(self) -> crate::common::Reg<regs::Chctlr2Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[inline(always)]
    pub const fn chctlr2_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr2Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::AdtmCcer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::AdtmCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<regs::AdtmAtrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register."]
    #[inline(always)]
    pub const fn rptcr(self) -> crate::common::Reg<regs::Rptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture/compare register."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<regs::Chcvr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register."]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
#[doc = "General purpose timer."]
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
    pub const fn ctlr1(self) -> crate::common::Reg<regs::GptmCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::GptmCtlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::GptmDmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::GptmIntfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::GptmSwevgr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr1_input(
        self,
    ) -> crate::common::Reg<regs::GptmChctlr1Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)."]
    #[inline(always)]
    pub const fn chctlr1_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr1Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 2 (input mode)."]
    #[inline(always)]
    pub const fn chctlr2_input(self) -> crate::common::Reg<regs::Chctlr2Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare mode register 2 (output mode)."]
    #[inline(always)]
    pub const fn chctlr2_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr2Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::GptmCcer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::GptmCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<regs::GptmAtrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<regs::Chcvr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
#[doc = "Timer4."]
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
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Gptm32Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Gptm32Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register."]
    #[inline(always)]
    pub const fn smcfgr(self) -> crate::common::Reg<regs::Smcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[inline(always)]
    pub const fn dmaintenr(self) -> crate::common::Reg<regs::Gptm32Dmaintenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn intfr(self) -> crate::common::Reg<regs::Gptm32Intfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register."]
    #[inline(always)]
    pub const fn swevgr(self) -> crate::common::Reg<regs::Gptm32Swevgr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[inline(always)]
    pub const fn chctlr1_input(
        self,
    ) -> crate::common::Reg<regs::Gptm32Chctlr1Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)."]
    #[inline(always)]
    pub const fn chctlr1_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr1Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 2 (input mode)."]
    #[inline(always)]
    pub const fn chctlr2_input(self) -> crate::common::Reg<regs::Chctlr2Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare mode register 2 (output mode)."]
    #[inline(always)]
    pub const fn chctlr2_output(
        self,
    ) -> crate::common::Reg<regs::Chctlr2Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare enable register."]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Gptm32Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Gptm32Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler."]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "auto-reload register."]
    #[inline(always)]
    pub const fn atrlr(self) -> crate::common::Reg<regs::Gptm32Atrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register."]
    #[inline(always)]
    pub const fn chcvr(self, n: usize) -> crate::common::Reg<regs::Gptm32Chcvr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacfgr(self) -> crate::common::Reg<regs::Dmacfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer."]
    #[inline(always)]
    pub const fn dmaadr(self) -> crate::common::Reg<regs::Dmaadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "auto-reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmAtrlr(pub u16);
    impl AdtmAtrlr {
        #[doc = "Auto-reload value."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for AdtmAtrlr {
        #[inline(always)]
        fn default() -> AdtmAtrlr {
            AdtmAtrlr(0)
        }
    }
    impl core::fmt::Debug for AdtmAtrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmAtrlr")
                .field("arr", &self.arr())
                .finish()
        }
    }
    #[doc = "capture/compare enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmCcer(pub u16);
    impl AdtmCcer {
        #[doc = "Capture/Compare 1 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable."]
        #[inline(always)]
        pub const fn set_cc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn set_cc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 1 complementary output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 complementary output enable."]
        #[inline(always)]
        pub const fn set_cc1ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1np(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn set_cc1np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[inline(always)]
        pub const fn set_cc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2p(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[inline(always)]
        pub const fn set_cc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Capture/Compare 2 complementary output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 complementary output enable."]
        #[inline(always)]
        pub const fn set_cc2ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2np(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[inline(always)]
        pub const fn set_cc2np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[inline(always)]
        pub const fn set_cc3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc3p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 3 complementary output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 complementary output enable."]
        #[inline(always)]
        pub const fn set_cc3ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3np(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc3np(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[inline(always)]
        pub const fn set_cc4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4p(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc4p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for AdtmCcer {
        #[inline(always)]
        fn default() -> AdtmCcer {
            AdtmCcer(0)
        }
    }
    impl core::fmt::Debug for AdtmCcer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmCcer")
                .field("cc1e", &self.cc1e())
                .field("cc1p", &self.cc1p())
                .field("cc1ne", &self.cc1ne())
                .field("cc1np", &self.cc1np())
                .field("cc2e", &self.cc2e())
                .field("cc2p", &self.cc2p())
                .field("cc2ne", &self.cc2ne())
                .field("cc2np", &self.cc2np())
                .field("cc3e", &self.cc3e())
                .field("cc3p", &self.cc3p())
                .field("cc3ne", &self.cc3ne())
                .field("cc3np", &self.cc3np())
                .field("cc4e", &self.cc4e())
                .field("cc4p", &self.cc4p())
                .finish()
        }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmChctlr1Input(pub u32);
    impl AdtmChctlr1Input {
        #[doc = "Capture/Compare 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Input capture 1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub const fn set_ic1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Input capture 1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub const fn set_ic1f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Capture/Compare 2 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 2 selection."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Input capture 2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2pcs(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 2 prescaler."]
        #[inline(always)]
        pub const fn set_ic2pcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Input capture 2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 2 filter."]
        #[inline(always)]
        pub const fn set_ic2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for AdtmChctlr1Input {
        #[inline(always)]
        fn default() -> AdtmChctlr1Input {
            AdtmChctlr1Input(0)
        }
    }
    impl core::fmt::Debug for AdtmChctlr1Input {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmChctlr1Input")
                .field("cc1s", &self.cc1s())
                .field("ic1psc", &self.ic1psc())
                .field("ic1f", &self.ic1f())
                .field("cc2s", &self.cc2s())
                .field("ic2pcs", &self.ic2pcs())
                .field("ic2f", &self.ic2f())
                .finish()
        }
    }
    #[doc = "counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmCnt(pub u16);
    impl AdtmCnt {
        #[doc = "counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for AdtmCnt {
        #[inline(always)]
        fn default() -> AdtmCnt {
            AdtmCnt(0)
        }
    }
    impl core::fmt::Debug for AdtmCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmCnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmCtlr1(pub u16);
    impl AdtmCtlr1 {
        #[doc = "Counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable."]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Update request source."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Update request source."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode."]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cms(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Center-aligned mode selection."]
        #[inline(always)]
        pub const fn set_cms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Clock division."]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "TIM1 brake input source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn bksel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 brake input source selection."]
        #[inline(always)]
        pub const fn set_bksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Capture the value mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_ov_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Capture the value mode configuration."]
        #[inline(always)]
        pub const fn set_tmr_cap_ov_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Directive function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_lvl_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Directive function enable."]
        #[inline(always)]
        pub const fn set_tmr_cap_lvl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for AdtmCtlr1 {
        #[inline(always)]
        fn default() -> AdtmCtlr1 {
            AdtmCtlr1(0)
        }
    }
    impl core::fmt::Debug for AdtmCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmCtlr1")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("bksel", &self.bksel())
                .field("tmr_cap_ov_en", &self.tmr_cap_ov_en())
                .field("tmr_cap_lvl_en", &self.tmr_cap_lvl_en())
                .finish()
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmCtlr2(pub u16);
    impl AdtmCtlr2 {
        #[doc = "Capture/compare preloaded control."]
        #[must_use]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control."]
        #[inline(always)]
        pub const fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection."]
        #[inline(always)]
        pub const fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare DMA selection."]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Master mode selection."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TI1 selection."]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ois1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub const fn set_ois1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Output Idle state 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ois1n(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1."]
        #[inline(always)]
        pub const fn set_ois1n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Output Idle state 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ois2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 2."]
        #[inline(always)]
        pub const fn set_ois2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Output Idle state 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ois2n(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 2."]
        #[inline(always)]
        pub const fn set_ois2n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Output Idle state 3."]
        #[must_use]
        #[inline(always)]
        pub const fn ois3(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 3."]
        #[inline(always)]
        pub const fn set_ois3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Output Idle state 3."]
        #[must_use]
        #[inline(always)]
        pub const fn ois3n(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 3."]
        #[inline(always)]
        pub const fn set_ois3n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Output Idle state 4."]
        #[must_use]
        #[inline(always)]
        pub const fn ois4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 4."]
        #[inline(always)]
        pub const fn set_ois4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for AdtmCtlr2 {
        #[inline(always)]
        fn default() -> AdtmCtlr2 {
            AdtmCtlr2(0)
        }
    }
    impl core::fmt::Debug for AdtmCtlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmCtlr2")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois1", &self.ois1())
                .field("ois1n", &self.ois1n())
                .field("ois2", &self.ois2())
                .field("ois2n", &self.ois2n())
                .field("ois3", &self.ois3())
                .field("ois3n", &self.ois3n())
                .field("ois4", &self.ois4())
                .finish()
        }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmDmaintenr(pub u16);
    impl AdtmDmaintenr {
        #[doc = "Update interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "COM interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable."]
        #[inline(always)]
        pub const fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable."]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1de(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2de(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3de(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc3de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4de(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc4de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "COM DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable."]
        #[inline(always)]
        pub const fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable."]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for AdtmDmaintenr {
        #[inline(always)]
        fn default() -> AdtmDmaintenr {
            AdtmDmaintenr(0)
        }
    }
    impl core::fmt::Debug for AdtmDmaintenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmDmaintenr")
                .field("uie", &self.uie())
                .field("cc1ie", &self.cc1ie())
                .field("cc2ie", &self.cc2ie())
                .field("cc3ie", &self.cc3ie())
                .field("cc4ie", &self.cc4ie())
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("cc1de", &self.cc1de())
                .field("cc2de", &self.cc2de())
                .field("cc3de", &self.cc3de())
                .field("cc4de", &self.cc4de())
                .field("comde", &self.comde())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmIntfr(pub u16);
    impl AdtmIntfr {
        #[doc = "Update interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc4if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "COM interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag."]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt flag."]
        #[inline(always)]
        pub const fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1of(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2of(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3of(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4of(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc4of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
    }
    impl Default for AdtmIntfr {
        #[inline(always)]
        fn default() -> AdtmIntfr {
            AdtmIntfr(0)
        }
    }
    impl core::fmt::Debug for AdtmIntfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmIntfr")
                .field("uif", &self.uif())
                .field("cc1if", &self.cc1if())
                .field("cc2if", &self.cc2if())
                .field("cc3if", &self.cc3if())
                .field("cc4if", &self.cc4if())
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif", &self.bif())
                .field("cc1of", &self.cc1of())
                .field("cc2of", &self.cc2of())
                .field("cc3of", &self.cc3of())
                .field("cc4of", &self.cc4of())
                .finish()
        }
    }
    #[doc = "event generation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdtmSwevgr(pub u16);
    impl AdtmSwevgr {
        #[doc = "Update generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1g(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation."]
        #[inline(always)]
        pub const fn set_cc1g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/compare 2 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2g(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 generation."]
        #[inline(always)]
        pub const fn set_cc2g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare 3 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3g(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 3 generation."]
        #[inline(always)]
        pub const fn set_cc3g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/compare 4 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4g(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 4 generation."]
        #[inline(always)]
        pub const fn set_cc4g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Capture/Compare control update generation."]
        #[must_use]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation."]
        #[inline(always)]
        pub const fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Break generation."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break generation."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for AdtmSwevgr {
        #[inline(always)]
        fn default() -> AdtmSwevgr {
            AdtmSwevgr(0)
        }
    }
    impl core::fmt::Debug for AdtmSwevgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdtmSwevgr")
                .field("ug", &self.ug())
                .field("cc1g", &self.cc1g())
                .field("cc2g", &self.cc2g())
                .field("cc3g", &self.cc3g())
                .field("cc4g", &self.cc4g())
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg", &self.bg())
                .finish()
        }
    }
    #[doc = "break and dead-time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr(pub u16);
    impl Bdtr {
        #[doc = "Dead-time generator setup."]
        #[must_use]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup."]
        #[inline(always)]
        pub const fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Lock configuration."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ossi(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Off-state selection for Idle mode."]
        #[inline(always)]
        pub const fn set_ossi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ossr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Off-state selection for Run mode."]
        #[inline(always)]
        pub const fn set_ossr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Break enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Break enable."]
        #[inline(always)]
        pub const fn set_bke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Break polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Break polarity."]
        #[inline(always)]
        pub const fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Automatic output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable."]
        #[inline(always)]
        pub const fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Main output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable."]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Bdtr {
        #[inline(always)]
        fn default() -> Bdtr {
            Bdtr(0)
        }
    }
    impl core::fmt::Debug for Bdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdtr")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke", &self.bke())
                .field("bkp", &self.bkp())
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .finish()
        }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctlr1Output(pub u16);
    impl Chctlr1Output {
        #[doc = "Capture/Compare 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "Output Compare 1 fast enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 fast enable."]
        #[inline(always)]
        pub const fn set_oc1fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Output Compare 1 preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1pe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 preload enable."]
        #[inline(always)]
        pub const fn set_oc1pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Output Compare 1 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1m(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Output Compare 1 mode."]
        #[inline(always)]
        pub const fn set_oc1m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "Output Compare 1 clear enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc1ce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 1 clear enable."]
        #[inline(always)]
        pub const fn set_oc1ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 2 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 2 selection."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Output Compare 2 fast enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2fe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 2 fast enable."]
        #[inline(always)]
        pub const fn set_oc2fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Output Compare 2 preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2pe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 2 preload enable."]
        #[inline(always)]
        pub const fn set_oc2pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Output Compare 2 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2m(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Output Compare 2 mode."]
        #[inline(always)]
        pub const fn set_oc2m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
        }
        #[doc = "Output Compare 2 clear enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc2ce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Output Compare 2 clear enable."]
        #[inline(always)]
        pub const fn set_oc2ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Chctlr1Output {
        #[inline(always)]
        fn default() -> Chctlr1Output {
            Chctlr1Output(0)
        }
    }
    impl core::fmt::Debug for Chctlr1Output {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctlr1Output")
                .field("cc1s", &self.cc1s())
                .field("oc1fe", &self.oc1fe())
                .field("oc1pe", &self.oc1pe())
                .field("oc1m", &self.oc1m())
                .field("oc1ce", &self.oc1ce())
                .field("cc2s", &self.cc2s())
                .field("oc2fe", &self.oc2fe())
                .field("oc2pe", &self.oc2pe())
                .field("oc2m", &self.oc2m())
                .field("oc2ce", &self.oc2ce())
                .finish()
        }
    }
    #[doc = "capture/compare mode register 2 (input mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctlr2Input(pub u16);
    impl Chctlr2Input {
        #[doc = "Capture/compare 3 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/compare 3 selection."]
        #[inline(always)]
        pub const fn set_cc3s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "Input capture 3 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic3psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 3 prescaler."]
        #[inline(always)]
        pub const fn set_ic3psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[doc = "Input capture 3 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic3f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 3 filter."]
        #[inline(always)]
        pub const fn set_ic3f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "Capture/Compare 4 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 4 selection."]
        #[inline(always)]
        pub const fn set_cc4s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Input capture 4 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic4psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 4 prescaler."]
        #[inline(always)]
        pub const fn set_ic4psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[doc = "Input capture 4 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic4f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 4 filter."]
        #[inline(always)]
        pub const fn set_ic4f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for Chctlr2Input {
        #[inline(always)]
        fn default() -> Chctlr2Input {
            Chctlr2Input(0)
        }
    }
    impl core::fmt::Debug for Chctlr2Input {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctlr2Input")
                .field("cc3s", &self.cc3s())
                .field("ic3psc", &self.ic3psc())
                .field("ic3f", &self.ic3f())
                .field("cc4s", &self.cc4s())
                .field("ic4psc", &self.ic4psc())
                .field("ic4f", &self.ic4f())
                .finish()
        }
    }
    #[doc = "capture/compare mode register (output mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctlr2Output(pub u16);
    impl Chctlr2Output {
        #[doc = "Capture/Compare 3 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 3 selection."]
        #[inline(always)]
        pub const fn set_cc3s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "Output compare 3 fast enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 3 fast enable."]
        #[inline(always)]
        pub const fn set_oc3fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Output compare 3 preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3pe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 3 preload enable."]
        #[inline(always)]
        pub const fn set_oc3pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Output compare 3 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3m(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Output compare 3 mode."]
        #[inline(always)]
        pub const fn set_oc3m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "Output compare 3 clear enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc3ce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 3 clear enable."]
        #[inline(always)]
        pub const fn set_oc3ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 4 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 4 selection."]
        #[inline(always)]
        pub const fn set_cc4s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Output compare 4 fast enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4fe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 4 fast enable."]
        #[inline(always)]
        pub const fn set_oc4fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Output compare 4 preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4pe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 4 preload enable."]
        #[inline(always)]
        pub const fn set_oc4pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Output compare 4 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4m(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Output compare 4 mode."]
        #[inline(always)]
        pub const fn set_oc4m(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
        }
        #[doc = "Output compare 4 clear enable."]
        #[must_use]
        #[inline(always)]
        pub const fn oc4ce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare 4 clear enable."]
        #[inline(always)]
        pub const fn set_oc4ce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Chctlr2Output {
        #[inline(always)]
        fn default() -> Chctlr2Output {
            Chctlr2Output(0)
        }
    }
    impl core::fmt::Debug for Chctlr2Output {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctlr2Output")
                .field("cc3s", &self.cc3s())
                .field("oc3fe", &self.oc3fe())
                .field("oc3pe", &self.oc3pe())
                .field("oc3m", &self.oc3m())
                .field("oc3ce", &self.oc3ce())
                .field("cc4s", &self.cc4s())
                .field("oc4fe", &self.oc4fe())
                .field("oc4pe", &self.oc4pe())
                .field("oc4m", &self.oc4m())
                .field("oc4ce", &self.oc4ce())
                .finish()
        }
    }
    #[doc = "capture/compare register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chcvr(pub u32);
    impl Chcvr {
        #[doc = "Capture/Compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Capture/Compare 1 value."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Level indicating bit."]
        #[must_use]
        #[inline(always)]
        pub const fn level(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Level indicating bit."]
        #[inline(always)]
        pub const fn set_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Chcvr {
        #[inline(always)]
        fn default() -> Chcvr {
            Chcvr(0)
        }
    }
    impl core::fmt::Debug for Chcvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chcvr")
                .field("ccr", &self.ccr())
                .field("level", &self.level())
                .finish()
        }
    }
    #[doc = "DMA address for full transfer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaadr(pub u32);
    impl Dmaadr {
        #[doc = "DMA register for burst accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn dmab(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DMA register for burst accesses."]
        #[inline(always)]
        pub const fn set_dmab(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmaadr {
        #[inline(always)]
        fn default() -> Dmaadr {
            Dmaadr(0)
        }
    }
    impl core::fmt::Debug for Dmaadr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaadr")
                .field("dmab", &self.dmab())
                .finish()
        }
    }
    #[doc = "DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfgr(pub u16);
    impl Dmacfgr {
        #[doc = "DMA base address."]
        #[must_use]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address."]
        #[inline(always)]
        pub const fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length."]
        #[must_use]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length."]
        #[inline(always)]
        pub const fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
    }
    impl Default for Dmacfgr {
        #[inline(always)]
        fn default() -> Dmacfgr {
            Dmacfgr(0)
        }
    }
    impl core::fmt::Debug for Dmacfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacfgr")
                .field("dba", &self.dba())
                .field("dbl", &self.dbl())
                .finish()
        }
    }
    #[doc = "auto-reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Atrlr(pub u32);
    impl Gptm32Atrlr {
        #[doc = "Auto-reload value."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Auto-reload value."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gptm32Atrlr {
        #[inline(always)]
        fn default() -> Gptm32Atrlr {
            Gptm32Atrlr(0)
        }
    }
    impl core::fmt::Debug for Gptm32Atrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Atrlr")
                .field("arr", &self.arr())
                .finish()
        }
    }
    #[doc = "capture/compare enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Ccer(pub u16);
    impl Gptm32Ccer {
        #[doc = "Capture/Compare 1 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable."]
        #[inline(always)]
        pub const fn set_cc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn set_cc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[inline(always)]
        pub const fn set_cc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2p(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[inline(always)]
        pub const fn set_cc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[inline(always)]
        pub const fn set_cc3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc3p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[inline(always)]
        pub const fn set_cc4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4p(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc4p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for Gptm32Ccer {
        #[inline(always)]
        fn default() -> Gptm32Ccer {
            Gptm32Ccer(0)
        }
    }
    impl core::fmt::Debug for Gptm32Ccer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Ccer")
                .field("cc1e", &self.cc1e())
                .field("cc1p", &self.cc1p())
                .field("cc2e", &self.cc2e())
                .field("cc2p", &self.cc2p())
                .field("cc3e", &self.cc3e())
                .field("cc3p", &self.cc3p())
                .field("cc4e", &self.cc4e())
                .field("cc4p", &self.cc4p())
                .finish()
        }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Chctlr1Input(pub u16);
    impl Gptm32Chctlr1Input {
        #[doc = "Capture/Compare 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "Input capture 1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub const fn set_ic1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[doc = "Input capture 1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub const fn set_ic1f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "Capture/compare 2 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/compare 2 selection."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Input capture 2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 2 prescaler."]
        #[inline(always)]
        pub const fn set_ic2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[doc = "Input capture 2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 2 filter."]
        #[inline(always)]
        pub const fn set_ic2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for Gptm32Chctlr1Input {
        #[inline(always)]
        fn default() -> Gptm32Chctlr1Input {
            Gptm32Chctlr1Input(0)
        }
    }
    impl core::fmt::Debug for Gptm32Chctlr1Input {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Chctlr1Input")
                .field("cc1s", &self.cc1s())
                .field("ic1psc", &self.ic1psc())
                .field("ic1f", &self.ic1f())
                .field("cc2s", &self.cc2s())
                .field("ic2psc", &self.ic2psc())
                .field("ic2f", &self.ic2f())
                .finish()
        }
    }
    #[doc = "capture/compare register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Chcvr(pub u32);
    impl Gptm32Chcvr {
        #[doc = "Capture/Compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Capture/Compare 1 value."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gptm32Chcvr {
        #[inline(always)]
        fn default() -> Gptm32Chcvr {
            Gptm32Chcvr(0)
        }
    }
    impl core::fmt::Debug for Gptm32Chcvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Chcvr")
                .field("ccr", &self.ccr())
                .finish()
        }
    }
    #[doc = "counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Cnt(pub u32);
    impl Gptm32Cnt {
        #[doc = "counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gptm32Cnt {
        #[inline(always)]
        fn default() -> Gptm32Cnt {
            Gptm32Cnt(0)
        }
    }
    impl core::fmt::Debug for Gptm32Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Cnt")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Ctlr1(pub u16);
    impl Gptm32Ctlr1 {
        #[doc = "Counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable."]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Update request source."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Update request source."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode."]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cms(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Center-aligned mode selection."]
        #[inline(always)]
        pub const fn set_cms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Clock division."]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_ov_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn set_tmr_cap_ov_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Directive function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_lvl_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Directive function enable."]
        #[inline(always)]
        pub const fn set_tmr_cap_lvl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Gptm32Ctlr1 {
        #[inline(always)]
        fn default() -> Gptm32Ctlr1 {
            Gptm32Ctlr1(0)
        }
    }
    impl core::fmt::Debug for Gptm32Ctlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Ctlr1")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("tmr_cap_ov_en", &self.tmr_cap_ov_en())
                .field("tmr_cap_lvl_en", &self.tmr_cap_lvl_en())
                .finish()
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Ctlr2(pub u16);
    impl Gptm32Ctlr2 {
        #[doc = "Capture/compare DMA selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare DMA selection."]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Master mode selection."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TI1 selection."]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for Gptm32Ctlr2 {
        #[inline(always)]
        fn default() -> Gptm32Ctlr2 {
            Gptm32Ctlr2(0)
        }
    }
    impl core::fmt::Debug for Gptm32Ctlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Ctlr2")
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Dmaintenr(pub u16);
    impl Gptm32Dmaintenr {
        #[doc = "Update interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1de(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2de(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3de(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc3de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4de(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc4de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Trigger DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable."]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for Gptm32Dmaintenr {
        #[inline(always)]
        fn default() -> Gptm32Dmaintenr {
            Gptm32Dmaintenr(0)
        }
    }
    impl core::fmt::Debug for Gptm32Dmaintenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Dmaintenr")
                .field("uie", &self.uie())
                .field("cc1ie", &self.cc1ie())
                .field("cc2ie", &self.cc2ie())
                .field("cc3ie", &self.cc3ie())
                .field("cc4ie", &self.cc4ie())
                .field("tie", &self.tie())
                .field("ude", &self.ude())
                .field("cc1de", &self.cc1de())
                .field("cc2de", &self.cc2de())
                .field("cc3de", &self.cc3de())
                .field("cc4de", &self.cc4de())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Intfr(pub u16);
    impl Gptm32Intfr {
        #[doc = "Update interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc4if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1of(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2of(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3of(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4of(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc4of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
    }
    impl Default for Gptm32Intfr {
        #[inline(always)]
        fn default() -> Gptm32Intfr {
            Gptm32Intfr(0)
        }
    }
    impl core::fmt::Debug for Gptm32Intfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Intfr")
                .field("uif", &self.uif())
                .field("cc1if", &self.cc1if())
                .field("cc2if", &self.cc2if())
                .field("cc3if", &self.cc3if())
                .field("cc4if", &self.cc4if())
                .field("tif", &self.tif())
                .field("cc1of", &self.cc1of())
                .field("cc2of", &self.cc2of())
                .field("cc3of", &self.cc3of())
                .field("cc4of", &self.cc4of())
                .finish()
        }
    }
    #[doc = "event generation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gptm32Swevgr(pub u16);
    impl Gptm32Swevgr {
        #[doc = "Update generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1g(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation."]
        #[inline(always)]
        pub const fn set_cc1g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/compare 2 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2g(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 generation."]
        #[inline(always)]
        pub const fn set_cc2g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare 3 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3g(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 3 generation."]
        #[inline(always)]
        pub const fn set_cc3g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/compare 4 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4g(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 4 generation."]
        #[inline(always)]
        pub const fn set_cc4g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger generation."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
    }
    impl Default for Gptm32Swevgr {
        #[inline(always)]
        fn default() -> Gptm32Swevgr {
            Gptm32Swevgr(0)
        }
    }
    impl core::fmt::Debug for Gptm32Swevgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gptm32Swevgr")
                .field("ug", &self.ug())
                .field("cc1g", &self.cc1g())
                .field("cc2g", &self.cc2g())
                .field("cc3g", &self.cc3g())
                .field("cc4g", &self.cc4g())
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[doc = "auto-reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmAtrlr(pub u16);
    impl GptmAtrlr {
        #[doc = "Auto-reload value."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for GptmAtrlr {
        #[inline(always)]
        fn default() -> GptmAtrlr {
            GptmAtrlr(0)
        }
    }
    impl core::fmt::Debug for GptmAtrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmAtrlr")
                .field("arr", &self.arr())
                .finish()
        }
    }
    #[doc = "capture/compare enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmCcer(pub u16);
    impl GptmCcer {
        #[doc = "Capture/Compare 1 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable."]
        #[inline(always)]
        pub const fn set_cc1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity."]
        #[inline(always)]
        pub const fn set_cc1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output enable."]
        #[inline(always)]
        pub const fn set_cc2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2p(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 output Polarity."]
        #[inline(always)]
        pub const fn set_cc2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output enable."]
        #[inline(always)]
        pub const fn set_cc3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc3p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 output enable."]
        #[inline(always)]
        pub const fn set_cc4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4p(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 output Polarity."]
        #[inline(always)]
        pub const fn set_cc4p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for GptmCcer {
        #[inline(always)]
        fn default() -> GptmCcer {
            GptmCcer(0)
        }
    }
    impl core::fmt::Debug for GptmCcer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmCcer")
                .field("cc1e", &self.cc1e())
                .field("cc1p", &self.cc1p())
                .field("cc2e", &self.cc2e())
                .field("cc2p", &self.cc2p())
                .field("cc3e", &self.cc3e())
                .field("cc3p", &self.cc3p())
                .field("cc4e", &self.cc4e())
                .field("cc4p", &self.cc4p())
                .finish()
        }
    }
    #[doc = "capture/compare mode register 1 (input mode)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmChctlr1Input(pub u16);
    impl GptmChctlr1Input {
        #[doc = "Capture/Compare 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/Compare 1 selection."]
        #[inline(always)]
        pub const fn set_cc1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "Input capture 1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1psc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub const fn set_ic1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[doc = "Input capture 1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic1f(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub const fn set_ic1f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "Capture/compare 2 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Capture/compare 2 selection."]
        #[inline(always)]
        pub const fn set_cc2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Input capture 2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2psc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Input capture 2 prescaler."]
        #[inline(always)]
        pub const fn set_ic2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[doc = "Input capture 2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ic2f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Input capture 2 filter."]
        #[inline(always)]
        pub const fn set_ic2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for GptmChctlr1Input {
        #[inline(always)]
        fn default() -> GptmChctlr1Input {
            GptmChctlr1Input(0)
        }
    }
    impl core::fmt::Debug for GptmChctlr1Input {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmChctlr1Input")
                .field("cc1s", &self.cc1s())
                .field("ic1psc", &self.ic1psc())
                .field("ic1f", &self.ic1f())
                .field("cc2s", &self.cc2s())
                .field("ic2psc", &self.ic2psc())
                .field("ic2f", &self.ic2f())
                .finish()
        }
    }
    #[doc = "counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmCnt(pub u16);
    impl GptmCnt {
        #[doc = "counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for GptmCnt {
        #[inline(always)]
        fn default() -> GptmCnt {
            GptmCnt(0)
        }
    }
    impl core::fmt::Debug for GptmCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmCnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmCtlr1(pub u16);
    impl GptmCtlr1 {
        #[doc = "Counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable."]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Update request source."]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Update request source."]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode."]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn cms(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Center-aligned mode selection."]
        #[inline(always)]
        pub const fn set_cms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Clock division."]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Auto-reload preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_ov_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable."]
        #[inline(always)]
        pub const fn set_tmr_cap_ov_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Directive function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_cap_lvl_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Directive function enable."]
        #[inline(always)]
        pub const fn set_tmr_cap_lvl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for GptmCtlr1 {
        #[inline(always)]
        fn default() -> GptmCtlr1 {
            GptmCtlr1(0)
        }
    }
    impl core::fmt::Debug for GptmCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmCtlr1")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .field("tmr_cap_ov_en", &self.tmr_cap_ov_en())
                .field("tmr_cap_lvl_en", &self.tmr_cap_lvl_en())
                .finish()
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmCtlr2(pub u16);
    impl GptmCtlr2 {
        #[doc = "Capture/compare DMA selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare DMA selection."]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Master mode selection."]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TI1 selection."]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for GptmCtlr2 {
        #[inline(always)]
        fn default() -> GptmCtlr2 {
            GptmCtlr2(0)
        }
    }
    impl core::fmt::Debug for GptmCtlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmCtlr2")
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[doc = "DMA/Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmDmaintenr(pub u16);
    impl GptmDmaintenr {
        #[doc = "Update interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_cc4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1de(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2de(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3de(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc3de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4de(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 DMA request enable."]
        #[inline(always)]
        pub const fn set_cc4de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Trigger DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable."]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for GptmDmaintenr {
        #[inline(always)]
        fn default() -> GptmDmaintenr {
            GptmDmaintenr(0)
        }
    }
    impl core::fmt::Debug for GptmDmaintenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmDmaintenr")
                .field("uie", &self.uie())
                .field("cc1ie", &self.cc1ie())
                .field("cc2ie", &self.cc2ie())
                .field("cc3ie", &self.cc3ie())
                .field("cc4ie", &self.cc4ie())
                .field("tie", &self.tie())
                .field("ude", &self.ude())
                .field("cc1de", &self.cc1de())
                .field("cc2de", &self.cc2de())
                .field("cc3de", &self.cc3de())
                .field("cc4de", &self.cc4de())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmIntfr(pub u16);
    impl GptmIntfr {
        #[doc = "Update interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc1if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc2if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc3if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_cc4if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag."]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1of(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2of(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3of(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 3 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4of(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 4 overcapture flag."]
        #[inline(always)]
        pub const fn set_cc4of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
    }
    impl Default for GptmIntfr {
        #[inline(always)]
        fn default() -> GptmIntfr {
            GptmIntfr(0)
        }
    }
    impl core::fmt::Debug for GptmIntfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmIntfr")
                .field("uif", &self.uif())
                .field("cc1if", &self.cc1if())
                .field("cc2if", &self.cc2if())
                .field("cc3if", &self.cc3if())
                .field("cc4if", &self.cc4if())
                .field("tif", &self.tif())
                .field("cc1of", &self.cc1of())
                .field("cc2of", &self.cc2of())
                .field("cc3of", &self.cc3of())
                .field("cc4of", &self.cc4of())
                .finish()
        }
    }
    #[doc = "event generation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GptmSwevgr(pub u16);
    impl GptmSwevgr {
        #[doc = "Update generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation."]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1g(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation."]
        #[inline(always)]
        pub const fn set_cc1g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Capture/compare 2 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2g(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 2 generation."]
        #[inline(always)]
        pub const fn set_cc2g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare 3 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3g(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 3 generation."]
        #[inline(always)]
        pub const fn set_cc3g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Capture/compare 4 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn cc4g(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 4 generation."]
        #[inline(always)]
        pub const fn set_cc4g(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Trigger generation."]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation."]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
    }
    impl Default for GptmSwevgr {
        #[inline(always)]
        fn default() -> GptmSwevgr {
            GptmSwevgr(0)
        }
    }
    impl core::fmt::Debug for GptmSwevgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GptmSwevgr")
                .field("ug", &self.ug())
                .field("cc1g", &self.cc1g())
                .field("cc2g", &self.cc2g())
                .field("cc3g", &self.cc3g())
                .field("cc4g", &self.cc4g())
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[doc = "prescaler."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psc(pub u16);
    impl Psc {
        #[doc = "Prescaler value."]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Prescaler value."]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Psc {
        #[inline(always)]
        fn default() -> Psc {
            Psc(0)
        }
    }
    impl core::fmt::Debug for Psc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psc").field("psc", &self.psc()).finish()
        }
    }
    #[doc = "repetition counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptcr(pub u16);
    impl Rptcr {
        #[doc = "Repetition counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for Rptcr {
        #[inline(always)]
        fn default() -> Rptcr {
            Rptcr(0)
        }
    }
    impl core::fmt::Debug for Rptcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptcr").field("rep", &self.rep()).finish()
        }
    }
    #[doc = "slave mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcfgr(pub u16);
    impl Smcfgr {
        #[doc = "Slave mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Slave mode selection."]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger selection."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master/Slave mode."]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter."]
        #[must_use]
        #[inline(always)]
        pub const fn etf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External trigger filter."]
        #[inline(always)]
        pub const fn set_etf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn etps(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External trigger prescaler."]
        #[inline(always)]
        pub const fn set_etps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[doc = "External clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock enable."]
        #[inline(always)]
        pub const fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn etp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger polarity."]
        #[inline(always)]
        pub const fn set_etp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Smcfgr {
        #[inline(always)]
        fn default() -> Smcfgr {
            Smcfgr(0)
        }
    }
    impl core::fmt::Debug for Smcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcfgr")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("etf", &self.etf())
                .field("etps", &self.etps())
                .field("ece", &self.ece())
                .field("etp", &self.etp())
                .finish()
        }
    }
}
