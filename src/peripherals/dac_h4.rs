#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital to analog converter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register (DAC_CR)."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DAC software trigger register (DAC_SWTRIGR)."]
    #[inline(always)]
    pub const fn swtr(self) -> crate::common::Reg<regs::Swtr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)."]
    #[inline(always)]
    pub const fn r12bdhr1(self) -> crate::common::Reg<regs::R12bdhr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)."]
    #[inline(always)]
    pub const fn l12bdhr1(self) -> crate::common::Reg<regs::L12bdhr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)."]
    #[inline(always)]
    pub const fn r8bdhr1(self) -> crate::common::Reg<regs::R8bdhr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)."]
    #[inline(always)]
    pub const fn r12bdhr2(self) -> crate::common::Reg<regs::R12bdhr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)."]
    #[inline(always)]
    pub const fn l12bdhr2(self) -> crate::common::Reg<regs::L12bdhr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)."]
    #[inline(always)]
    pub const fn r8bdhr2(self) -> crate::common::Reg<regs::R8bdhr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved."]
    #[inline(always)]
    pub const fn rd12bdhr(self) -> crate::common::Reg<regs::Rd12bdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved."]
    #[inline(always)]
    pub const fn ld12bdhr(self) -> crate::common::Reg<regs::Ld12bdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved."]
    #[inline(always)]
    pub const fn rd8bdhr(self) -> crate::common::Reg<regs::Rd8bdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DAC channel1 data output register (DAC_DOR1)."]
    #[inline(always)]
    pub const fn dor1(self) -> crate::common::Reg<regs::Dor1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DAC channel2 data output register (DAC_DOR2)."]
    #[inline(always)]
    pub const fn dor2(self) -> crate::common::Reg<regs::Dor2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register (DAC_CR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "DAC channel1 enable."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel1 enable."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC channel1 output buffer disable."]
        #[inline(always)]
        pub const fn boff1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel1 output buffer disable."]
        #[inline(always)]
        pub fn set_boff1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DAC channel1 trigger enable."]
        #[inline(always)]
        pub const fn ten1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel1 trigger enable."]
        #[inline(always)]
        pub fn set_ten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DAC channel1 trigger selection."]
        #[inline(always)]
        pub const fn tsel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "DAC channel1 trigger selection."]
        #[inline(always)]
        pub fn set_tsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "DAC channel1 noise/triangle wave generation enable."]
        #[inline(always)]
        pub const fn wave1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "DAC channel1 noise/triangle wave generation enable."]
        #[inline(always)]
        pub fn set_wave1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "DAC channel1 mask/amplitude selector."]
        #[inline(always)]
        pub const fn mamp1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "DAC channel1 mask/amplitude selector."]
        #[inline(always)]
        pub fn set_mamp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DAC channel1 DMA enable."]
        #[inline(always)]
        pub const fn dmaen1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel1 DMA enable."]
        #[inline(always)]
        pub fn set_dmaen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC channel2 enable."]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel2 enable."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DAC channel2 output buffer disable."]
        #[inline(always)]
        pub const fn boff2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel2 output buffer disable."]
        #[inline(always)]
        pub fn set_boff2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DAC channel2 trigger enable."]
        #[inline(always)]
        pub const fn ten2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel2 trigger enable."]
        #[inline(always)]
        pub fn set_ten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DAC channel2 trigger selection."]
        #[inline(always)]
        pub const fn tsel2(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[doc = "DAC channel2 trigger selection."]
        #[inline(always)]
        pub fn set_tsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
        #[doc = "DAC channel2 noise/triangle wave generation enable."]
        #[inline(always)]
        pub const fn wave2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "DAC channel2 noise/triangle wave generation enable."]
        #[inline(always)]
        pub fn set_wave2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "DAC channel2 mask/amplitude selector."]
        #[inline(always)]
        pub const fn mamp2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "DAC channel2 mask/amplitude selector."]
        #[inline(always)]
        pub fn set_mamp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "DAC channel2 DMA enable."]
        #[inline(always)]
        pub const fn dmaen2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel2 DMA enable."]
        #[inline(always)]
        pub fn set_dmaen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "DAC channel1 data output register (DAC_DOR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor1(pub u32);
    impl Dor1 {
        #[doc = "DAC channel1 data output."]
        #[inline(always)]
        pub const fn dacc1dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel1 data output."]
        #[inline(always)]
        pub fn set_dacc1dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dor1 {
        #[inline(always)]
        fn default() -> Dor1 {
            Dor1(0)
        }
    }
    #[doc = "DAC channel2 data output register (DAC_DOR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor2(pub u32);
    impl Dor2 {
        #[doc = "DAC channel2 data output."]
        #[inline(always)]
        pub const fn dacc2dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel2 data output."]
        #[inline(always)]
        pub fn set_dacc2dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dor2 {
        #[inline(always)]
        fn default() -> Dor2 {
            Dor2(0)
        }
    }
    #[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L12bdhr1(pub u32);
    impl L12bdhr1 {
        #[doc = "DAC channel1 12-bit left-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel1 12-bit left-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for L12bdhr1 {
        #[inline(always)]
        fn default() -> L12bdhr1 {
            L12bdhr1(0)
        }
    }
    #[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L12bdhr2(pub u32);
    impl L12bdhr2 {
        #[doc = "DAC channel2 12-bit left-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel2 12-bit left-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for L12bdhr2 {
        #[inline(always)]
        fn default() -> L12bdhr2 {
            L12bdhr2(0)
        }
    }
    #[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ld12bdhr(pub u32);
    impl Ld12bdhr {
        #[doc = "DAC channel1 12-bit left-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel1 12-bit left-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Ld12bdhr {
        #[inline(always)]
        fn default() -> Ld12bdhr {
            Ld12bdhr(0)
        }
    }
    #[doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R12bdhr1(pub u32);
    impl R12bdhr1 {
        #[doc = "DAC channel1 12-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel1 12-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for R12bdhr1 {
        #[inline(always)]
        fn default() -> R12bdhr1 {
            R12bdhr1(0)
        }
    }
    #[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R12bdhr2(pub u32);
    impl R12bdhr2 {
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for R12bdhr2 {
        #[inline(always)]
        fn default() -> R12bdhr2 {
            R12bdhr2(0)
        }
    }
    #[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R8bdhr1(pub u32);
    impl R8bdhr1 {
        #[doc = "DAC channel1 8-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DAC channel1 8-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for R8bdhr1 {
        #[inline(always)]
        fn default() -> R8bdhr1 {
            R8bdhr1(0)
        }
    }
    #[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R8bdhr2(pub u32);
    impl R8bdhr2 {
        #[doc = "DAC channel2 8-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DAC channel2 8-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for R8bdhr2 {
        #[inline(always)]
        fn default() -> R8bdhr2 {
            R8bdhr2(0)
        }
    }
    #[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rd12bdhr(pub u32);
    impl Rd12bdhr {
        #[doc = "DAC channel1 12-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel1 12-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "DAC channel2 12-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Rd12bdhr {
        #[inline(always)]
        fn default() -> Rd12bdhr {
            Rd12bdhr(0)
        }
    }
    #[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rd8bdhr(pub u32);
    impl Rd8bdhr {
        #[doc = "DAC channel1 8-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc1dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DAC channel1 8-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc1dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DAC channel2 8-bit right-aligned data."]
        #[inline(always)]
        pub const fn dacc2dhr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "DAC channel2 8-bit right-aligned data."]
        #[inline(always)]
        pub fn set_dacc2dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Rd8bdhr {
        #[inline(always)]
        fn default() -> Rd8bdhr {
            Rd8bdhr(0)
        }
    }
    #[doc = "DAC software trigger register (DAC_SWTRIGR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swtr(pub u32);
    impl Swtr {
        #[doc = "DAC channel1 software trigger."]
        #[inline(always)]
        pub const fn swtrig1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel1 software trigger."]
        #[inline(always)]
        pub fn set_swtrig1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC channel2 software trigger."]
        #[inline(always)]
        pub const fn swtrig2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC channel2 software trigger."]
        #[inline(always)]
        pub fn set_swtrig2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Swtr {
        #[inline(always)]
        fn default() -> Swtr {
            Swtr(0)
        }
    }
}
