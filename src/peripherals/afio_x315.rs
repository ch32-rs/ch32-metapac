#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Alternate function I/O."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afio {
    ptr: *mut u8,
}
unsafe impl Send for Afio {}
unsafe impl Sync for Afio {}
impl Afio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration register 1."]
    #[inline(always)]
    pub const fn exticr1(self) -> crate::common::Reg<regs::Exticr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "External interrupt configuration register 2."]
    #[inline(always)]
    pub const fn exticr2(self) -> crate::common::Reg<regs::Exticr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "External interrupt configuration register 3."]
    #[inline(always)]
    pub const fn exticr3(self) -> crate::common::Reg<regs::Exticr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "External interrupt configuration register 4."]
    #[inline(always)]
    pub const fn exticr4(self) -> crate::common::Reg<regs::Exticr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "PA port multiplexing function register."]
    #[inline(always)]
    pub const fn gpioa_aflr(self) -> crate::common::Reg<regs::GpioaAflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PA port multiplexing function register."]
    #[inline(always)]
    pub const fn gpioa_afhr(self) -> crate::common::Reg<regs::GpioaAfhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PB port multiplexing function register."]
    #[inline(always)]
    pub const fn gpiob_aflr(self) -> crate::common::Reg<regs::GpiobAflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PB port multiplexing function register."]
    #[inline(always)]
    pub const fn gpiob_afhr(self) -> crate::common::Reg<regs::GpiobAfhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "PC port multiplexing function register."]
    #[inline(always)]
    pub const fn gpioc_aflr(self) -> crate::common::Reg<regs::GpiocAflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PC port multiplexing function register."]
    #[inline(always)]
    pub const fn gpioc_afhr(self) -> crate::common::Reg<regs::GpiocAfhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "PD port multiplexing function register."]
    #[inline(always)]
    pub const fn gpiod_aflr(self) -> crate::common::Reg<regs::GpiodAflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "PD port multiplexing function register."]
    #[inline(always)]
    pub const fn gpiod_afhr(self) -> crate::common::Reg<regs::GpiodAfhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "CC pin intput channel threshold adjustment."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpdhvt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC pin intput channel threshold adjustment."]
        #[inline(always)]
        pub const fn set_usbpdhvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The Tr adjustment of the USBPD LV waveform is user-adjustment."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpdrise(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "The Tr adjustment of the USBPD LV waveform is user-adjustment."]
        #[inline(always)]
        pub const fn set_usbpdrise(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("usbpdhvt", &self.usbpdhvt())
                .field("usbpdrise", &self.usbpdrise())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr1(pub u32);
    impl Exticr1 {
        #[doc = "External interrupt 0 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 0 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External interrupt 1 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 1 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "External interrupt 2 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 2 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "External interrupt 3 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 3 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Exticr1 {
        #[inline(always)]
        fn default() -> Exticr1 {
            Exticr1(0)
        }
    }
    impl core::fmt::Debug for Exticr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr1")
                .field("exti0", &self.exti0())
                .field("exti1", &self.exti1())
                .field("exti2", &self.exti2())
                .field("exti3", &self.exti3())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr2(pub u32);
    impl Exticr2 {
        #[doc = "External interrupt 4 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 4 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External interrupt 5 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti5(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 5 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "External interrupt 6 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 6 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "External interrupt 7 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti7(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 7 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Exticr2 {
        #[inline(always)]
        fn default() -> Exticr2 {
            Exticr2(0)
        }
    }
    impl core::fmt::Debug for Exticr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr2")
                .field("exti4", &self.exti4())
                .field("exti5", &self.exti5())
                .field("exti6", &self.exti6())
                .field("exti7", &self.exti7())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr3(pub u32);
    impl Exticr3 {
        #[doc = "External interrupt 8 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 8 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External interrupt 9 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 9 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "External interrupt 10 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 10 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "External interrupt 11 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 11 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Exticr3 {
        #[inline(always)]
        fn default() -> Exticr3 {
            Exticr3(0)
        }
    }
    impl core::fmt::Debug for Exticr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr3")
                .field("exti8", &self.exti8())
                .field("exti9", &self.exti9())
                .field("exti10", &self.exti10())
                .field("exti11", &self.exti11())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr4(pub u32);
    impl Exticr4 {
        #[doc = "External interrupt 12 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti12(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 12 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External interrupt 13 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti13(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 13 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "External interrupt 14 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 14 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "External interrupt 15 input pin configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti15(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External interrupt 15 input pin configuration."]
        #[inline(always)]
        pub const fn set_exti15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Exticr4 {
        #[inline(always)]
        fn default() -> Exticr4 {
            Exticr4(0)
        }
    }
    impl core::fmt::Debug for Exticr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr4")
                .field("exti12", &self.exti12())
                .field("exti13", &self.exti13())
                .field("exti14", &self.exti14())
                .field("exti15", &self.exti15())
                .finish()
        }
    }
    #[doc = "PA port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpioaAfhr(pub u32);
    impl GpioaAfhr {
        #[doc = "selection of multiplexing function for pin 8 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 8 of PA port."]
        #[inline(always)]
        pub const fn set_afr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 9 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 9 of PA port."]
        #[inline(always)]
        pub const fn set_afr9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 10 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 10 of PA port."]
        #[inline(always)]
        pub const fn set_afr10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 11 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 11 of PA port."]
        #[inline(always)]
        pub const fn set_afr11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 12 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 12 of PA port."]
        #[inline(always)]
        pub const fn set_afr12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 13 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 13 of PA port."]
        #[inline(always)]
        pub const fn set_afr13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 14 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 14 of PA port."]
        #[inline(always)]
        pub const fn set_afr14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 15 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 15 of PA port."]
        #[inline(always)]
        pub const fn set_afr15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpioaAfhr {
        #[inline(always)]
        fn default() -> GpioaAfhr {
            GpioaAfhr(0)
        }
    }
    impl core::fmt::Debug for GpioaAfhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpioaAfhr")
                .field("afr8", &self.afr8())
                .field("afr9", &self.afr9())
                .field("afr10", &self.afr10())
                .field("afr11", &self.afr11())
                .field("afr12", &self.afr12())
                .field("afr13", &self.afr13())
                .field("afr14", &self.afr14())
                .field("afr15", &self.afr15())
                .finish()
        }
    }
    #[doc = "PA port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpioaAflr(pub u32);
    impl GpioaAflr {
        #[doc = "selection of multiplexing function for pin 0 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 0 of PA port."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 1 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 1 of PA port."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 2 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 2 of PA port."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 3 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 3 of PA port."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 4 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 4 of PA port."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 5 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 5 of PA port."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 6 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 6 of PA port."]
        #[inline(always)]
        pub const fn set_afr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 7 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 7 of PA port."]
        #[inline(always)]
        pub const fn set_afr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpioaAflr {
        #[inline(always)]
        fn default() -> GpioaAflr {
            GpioaAflr(0)
        }
    }
    impl core::fmt::Debug for GpioaAflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpioaAflr")
                .field("afr0", &self.afr0())
                .field("afr1", &self.afr1())
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
                .field("afr6", &self.afr6())
                .field("afr7", &self.afr7())
                .finish()
        }
    }
    #[doc = "PB port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiobAfhr(pub u32);
    impl GpiobAfhr {
        #[doc = "selection of multiplexing function for pin 8 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 8 of PA port."]
        #[inline(always)]
        pub const fn set_afr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 9 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 9 of PB port."]
        #[inline(always)]
        pub const fn set_afr9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 10 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 10 of PB port."]
        #[inline(always)]
        pub const fn set_afr10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 11 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 11 of PB port."]
        #[inline(always)]
        pub const fn set_afr11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 12 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 12 of PB port."]
        #[inline(always)]
        pub const fn set_afr12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 13 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 13 of PB port."]
        #[inline(always)]
        pub const fn set_afr13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 14 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 14 of PB port."]
        #[inline(always)]
        pub const fn set_afr14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 15 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 15 of PB port."]
        #[inline(always)]
        pub const fn set_afr15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiobAfhr {
        #[inline(always)]
        fn default() -> GpiobAfhr {
            GpiobAfhr(0)
        }
    }
    impl core::fmt::Debug for GpiobAfhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiobAfhr")
                .field("afr8", &self.afr8())
                .field("afr9", &self.afr9())
                .field("afr10", &self.afr10())
                .field("afr11", &self.afr11())
                .field("afr12", &self.afr12())
                .field("afr13", &self.afr13())
                .field("afr14", &self.afr14())
                .field("afr15", &self.afr15())
                .finish()
        }
    }
    #[doc = "PB port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiobAflr(pub u32);
    impl GpiobAflr {
        #[doc = "selection of multiplexing function for pin 0 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 0 of PB port."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 1 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 1 of PB port."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 2 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 2 of PB port."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 3 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 3 of PB port."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 4 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 4 of PB port."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 5 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 5 of PB port."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 6 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 6 of PB port."]
        #[inline(always)]
        pub const fn set_afr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 7 of PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 7 of PB port."]
        #[inline(always)]
        pub const fn set_afr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiobAflr {
        #[inline(always)]
        fn default() -> GpiobAflr {
            GpiobAflr(0)
        }
    }
    impl core::fmt::Debug for GpiobAflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiobAflr")
                .field("afr0", &self.afr0())
                .field("afr1", &self.afr1())
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
                .field("afr6", &self.afr6())
                .field("afr7", &self.afr7())
                .finish()
        }
    }
    #[doc = "PC port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiocAfhr(pub u32);
    impl GpiocAfhr {
        #[doc = "selection of multiplexing function for pin 8 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 8 of PC port."]
        #[inline(always)]
        pub const fn set_afr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 9 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 9 of PC port."]
        #[inline(always)]
        pub const fn set_afr9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 10 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 10 of PC port."]
        #[inline(always)]
        pub const fn set_afr10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 11 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 11 of PC port."]
        #[inline(always)]
        pub const fn set_afr11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 12 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 12 of PC port."]
        #[inline(always)]
        pub const fn set_afr12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 13 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 13 of PC port."]
        #[inline(always)]
        pub const fn set_afr13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 14 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 14 of PC port."]
        #[inline(always)]
        pub const fn set_afr14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 15 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 15 of PC port."]
        #[inline(always)]
        pub const fn set_afr15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiocAfhr {
        #[inline(always)]
        fn default() -> GpiocAfhr {
            GpiocAfhr(0)
        }
    }
    impl core::fmt::Debug for GpiocAfhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiocAfhr")
                .field("afr8", &self.afr8())
                .field("afr9", &self.afr9())
                .field("afr10", &self.afr10())
                .field("afr11", &self.afr11())
                .field("afr12", &self.afr12())
                .field("afr13", &self.afr13())
                .field("afr14", &self.afr14())
                .field("afr15", &self.afr15())
                .finish()
        }
    }
    #[doc = "PC port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiocAflr(pub u32);
    impl GpiocAflr {
        #[doc = "selection of multiplexing function for pin 0 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 0 of PC port."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 1 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 1 of PC port."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 2 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 2 of PC port."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 3 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 3 of PC port."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 4 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 4 of PC port."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 5 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 5 of PC port."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 6 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 6 of PC port."]
        #[inline(always)]
        pub const fn set_afr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 7 of PC port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 7 of PC port."]
        #[inline(always)]
        pub const fn set_afr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiocAflr {
        #[inline(always)]
        fn default() -> GpiocAflr {
            GpiocAflr(0)
        }
    }
    impl core::fmt::Debug for GpiocAflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiocAflr")
                .field("afr0", &self.afr0())
                .field("afr1", &self.afr1())
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
                .field("afr6", &self.afr6())
                .field("afr7", &self.afr7())
                .finish()
        }
    }
    #[doc = "PD port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiodAfhr(pub u32);
    impl GpiodAfhr {
        #[doc = "selection of multiplexing function for pin 8 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 8 of PD port."]
        #[inline(always)]
        pub const fn set_afr8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 9 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 9 of PD port."]
        #[inline(always)]
        pub const fn set_afr9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 10 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 10 of PD port."]
        #[inline(always)]
        pub const fn set_afr10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 11 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 11 of PD port."]
        #[inline(always)]
        pub const fn set_afr11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 12 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 12 of PD port."]
        #[inline(always)]
        pub const fn set_afr12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 13 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 13 of PD port."]
        #[inline(always)]
        pub const fn set_afr13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 14 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 14 of PD port."]
        #[inline(always)]
        pub const fn set_afr14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 15 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 15 of PD port."]
        #[inline(always)]
        pub const fn set_afr15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiodAfhr {
        #[inline(always)]
        fn default() -> GpiodAfhr {
            GpiodAfhr(0)
        }
    }
    impl core::fmt::Debug for GpiodAfhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiodAfhr")
                .field("afr8", &self.afr8())
                .field("afr9", &self.afr9())
                .field("afr10", &self.afr10())
                .field("afr11", &self.afr11())
                .field("afr12", &self.afr12())
                .field("afr13", &self.afr13())
                .field("afr14", &self.afr14())
                .field("afr15", &self.afr15())
                .finish()
        }
    }
    #[doc = "PD port multiplexing function register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiodAflr(pub u32);
    impl GpiodAflr {
        #[doc = "selection of multiplexing function for pin 0 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 0 of PD port."]
        #[inline(always)]
        pub const fn set_afr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "selection of multiplexing function for pin 1 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 1 of PD port."]
        #[inline(always)]
        pub const fn set_afr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "selection of multiplexing function for pin 2 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 2 of PD port."]
        #[inline(always)]
        pub const fn set_afr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "selection of multiplexing function for pin 3 of PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 3 of PA port."]
        #[inline(always)]
        pub const fn set_afr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "selection of multiplexing function for pin 4 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 4 of PD port."]
        #[inline(always)]
        pub const fn set_afr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "selection of multiplexing function for pin 5 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 5 of PD port."]
        #[inline(always)]
        pub const fn set_afr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "selection of multiplexing function for pin 6 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 6 of PD port."]
        #[inline(always)]
        pub const fn set_afr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "selection of multiplexing function for pin 7 of PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn afr7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "selection of multiplexing function for pin 7 of PD port."]
        #[inline(always)]
        pub const fn set_afr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GpiodAflr {
        #[inline(always)]
        fn default() -> GpiodAflr {
            GpiodAflr(0)
        }
    }
    impl core::fmt::Debug for GpiodAflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiodAflr")
                .field("afr0", &self.afr0())
                .field("afr1", &self.afr1())
                .field("afr2", &self.afr2())
                .field("afr3", &self.afr3())
                .field("afr4", &self.afr4())
                .field("afr5", &self.afr5())
                .field("afr6", &self.afr6())
                .field("afr7", &self.afr7())
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr1(pub u32);
    impl Pcfr1 {
        #[doc = "PD0PD1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn pd0pd1_rm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PD0PD1 remapping."]
        #[inline(always)]
        pub const fn set_pd0pd1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Serial wire JTAG configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub const fn set_sw_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Pcfr1 {
        #[inline(always)]
        fn default() -> Pcfr1 {
            Pcfr1(0)
        }
    }
    impl core::fmt::Debug for Pcfr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcfr1")
                .field("pd0pd1_rm", &self.pd0pd1_rm())
                .field("sw_cfg", &self.sw_cfg())
                .finish()
        }
    }
}
