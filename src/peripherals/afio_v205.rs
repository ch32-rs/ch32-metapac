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
    #[doc = "Event Control Register (AFIO_ECR)."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register 1 (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn pcfr1(self) -> crate::common::Reg<regs::Pcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration registers."]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "AFIO control register (AFIO_CR)."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Port AF low registers."]
    #[inline(always)]
    pub const fn gpio_aflr(self, n: usize) -> crate::common::Reg<regs::Aflr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "Port AF high registers."]
    #[inline(always)]
    pub const fn gpio_afhr(self, n: usize) -> crate::common::Reg<regs::Afhr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Port AF high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afhr(pub u32);
    impl Afhr {
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
    impl Default for Afhr {
        #[inline(always)]
        fn default() -> Afhr {
            Afhr(0)
        }
    }
    impl core::fmt::Debug for Afhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afhr")
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
    #[doc = "Port AF low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aflr(pub u32);
    impl Aflr {
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
    impl Default for Aflr {
        #[inline(always)]
        fn default() -> Aflr {
            Aflr(0)
        }
    }
    impl core::fmt::Debug for Aflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aflr")
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
    #[doc = "AFIO control register (AFIO_CR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "PD pin PB6/PD7 High threshold input mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_in_hvt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PD pin PB6/PD7 High threshold input mode."]
        #[inline(always)]
        pub const fn set_usbpd_in_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA12/UDP pin BC source voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC source voltage enable."]
        #[inline(always)]
        pub const fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PA11/UDM pin BC source voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC source voltage enable."]
        #[inline(always)]
        pub const fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PA12/UDP pin BC protocol comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC protocol comparator enable."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PA11/UDM pin BC protocol comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC protocol comparator enable."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PA12/UDP pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PA12/UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PA11/UDM pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PA11/UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("usbpd_in_hvt", &self.usbpd_in_hvt())
                .field("udp_bc_vsrc", &self.udp_bc_vsrc())
                .field("udm_bc_vsrc", &self.udm_bc_vsrc())
                .field("udp_bc_cmpe", &self.udp_bc_cmpe())
                .field("udm_bc_cmpe", &self.udm_bc_cmpe())
                .field("udp_bc_cmpo", &self.udp_bc_cmpo())
                .field("udm_bc_cmpo", &self.udm_bc_cmpo())
                .finish()
        }
    }
    #[doc = "Event Control Register (AFIO_ECR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Pin selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pin selection."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Port selection."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Port selection."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event Output Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn evoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event Output Enable."]
        #[inline(always)]
        pub const fn set_evoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    impl core::fmt::Debug for Ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecr")
                .field("pin", &self.pin())
                .field("port", &self.port())
                .field("evoe", &self.evoe())
                .finish()
        }
    }
    #[doc = "External interrupt configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI line configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI line configuration."]
        #[inline(always)]
        pub const fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register 1 (AFIO_PCFR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfr1(pub u32);
    impl Pcfr1 {
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn pd0pd1_rm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT."]
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
        #[doc = "TIM1 channel capture mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_cap_rm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 channel capture mapping."]
        #[inline(always)]
        pub const fn set_tim1_cap_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim1_cap_rm", &self.tim1_cap_rm())
                .finish()
        }
    }
}
