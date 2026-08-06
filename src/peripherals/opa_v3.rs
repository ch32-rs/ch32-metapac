#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPA configuration register block. 4 independent operational amplifiers (OPA1..OPA4) on CH32F20x/CH32V20x/CH32V30x/CH32V31x."]
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
    #[doc = "OPA control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OPA control register 2 (Configuration Extended Control Register 2 / EXTEN_CTR2). Per-OPA high-speed mode enable."]
    #[inline(always)]
    pub const fn ctr2(self) -> crate::common::Reg<regs::Ctr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPA control register. Each OPA has 4 control bits (EN, MODE, NSEL, PSEL) at a 4-bit stride."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "OPA1 enable. 0=disable, 1=enable."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 enable. 0=disable, 1=enable."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA1 output channel selection. 0=OPA1_OUT0, 1=OPA1_OUT1."]
        #[inline(always)]
        pub const fn mode1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 output channel selection. 0=OPA1_OUT0, 1=OPA1_OUT1."]
        #[inline(always)]
        pub fn set_mode1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OPA1 negative input selection. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub const fn nsel1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 negative input selection. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub fn set_nsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPA1 positive input selection. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub const fn psel1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 positive input selection. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub fn set_psel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OPA2 enable. 0=disable, 1=enable."]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 enable. 0=disable, 1=enable."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OPA2 output channel selection. 0=OPA2_OUT0, 1=OPA2_OUT1."]
        #[inline(always)]
        pub const fn mode2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 output channel selection. 0=OPA2_OUT0, 1=OPA2_OUT1."]
        #[inline(always)]
        pub fn set_mode2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OPA2 negative input selection. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub const fn nsel2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 negative input selection. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub fn set_nsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "OPA2 positive input selection. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub const fn psel2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 positive input selection. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub fn set_psel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "OPA3 enable. Only present on D8/D8C variants. 0=disable, 1=enable."]
        #[inline(always)]
        pub const fn en3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 enable. Only present on D8/D8C variants. 0=disable, 1=enable."]
        #[inline(always)]
        pub fn set_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OPA3 output channel selection. Only present on D8/D8C variants. 0=OPA3_OUT0, 1=OPA3_OUT1."]
        #[inline(always)]
        pub const fn mode3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 output channel selection. Only present on D8/D8C variants. 0=OPA3_OUT0, 1=OPA3_OUT1."]
        #[inline(always)]
        pub fn set_mode3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OPA3 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub const fn nsel3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub fn set_nsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "OPA3 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub const fn psel3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub fn set_psel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPA4 enable. Only present on D8/D8C variants. 0=disable, 1=enable."]
        #[inline(always)]
        pub const fn en4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OPA4 enable. Only present on D8/D8C variants. 0=disable, 1=enable."]
        #[inline(always)]
        pub fn set_en4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OPA4 output channel selection. Only present on D8/D8C variants. 0=OPA4_OUT0, 1=OPA4_OUT1."]
        #[inline(always)]
        pub const fn mode4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPA4 output channel selection. Only present on D8/D8C variants. 0=OPA4_OUT0, 1=OPA4_OUT1."]
        #[inline(always)]
        pub fn set_mode4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPA4 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub const fn nsel4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPA4 negative input selection. Only present on D8/D8C variants. 0=CHN0, 1=CHN1."]
        #[inline(always)]
        pub fn set_nsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OPA4 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub const fn psel4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OPA4 positive input selection. Only present on D8/D8C variants. 0=CHP0, 1=CHP1."]
        #[inline(always)]
        pub fn set_psel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "OPA control register 2 (Configuration Extended Control Register 2 / EXTEN_CTR2). Each OPA has a 1-bit high-speed mode enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr2(pub u32);
    impl Ctr2 {
        #[doc = "OPA1 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub const fn hsmd1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPA1 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub fn set_hsmd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OPA2 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub const fn hsmd2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OPA2 high-speed mode enable. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub fn set_hsmd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OPA3 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub const fn hsmd3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OPA3 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub fn set_hsmd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPA4 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub const fn hsmd4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OPA4 high-speed mode enable. Only present on D8/D8C variants. 0=disable (low-power mode), 1=enable (high-speed mode, higher bandwidth/slew rate)."]
        #[inline(always)]
        pub fn set_hsmd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ctr2 {
        #[inline(always)]
        fn default() -> Ctr2 {
            Ctr2(0)
        }
    }
}
