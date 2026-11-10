#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extend configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extend {
    ptr: *mut u8,
}
unsafe impl Send for Extend {}
unsafe impl Sync for Extend {}
impl Extend {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configure the extended control register 0."]
    #[inline(always)]
    pub const fn ctlr0(self) -> crate::common::Reg<regs::Ctlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configure the extended control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configure the extended control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Configure the extended control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr0(pub u32);
    impl Ctlr0 {
        #[doc = "LOCKUP_Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP_Enable."]
        #[inline(always)]
        pub const fn set_lkupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LOCKUP RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn lkuprst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LOCKUP RESET."]
        #[inline(always)]
        pub const fn set_lkuprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ctlr0 {
        #[inline(always)]
        fn default() -> Ctlr0 {
            Ctlr0(0)
        }
    }
    impl core::fmt::Debug for Ctlr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr0")
                .field("lkupen", &self.lkupen())
                .field("lkuprst", &self.lkuprst())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "UDP pin DAC_BUF output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bufoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin DAC_BUF output enable."]
        #[inline(always)]
        pub const fn set_udp_bufoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Select pull-up current or pull-down current. built into the UDP pin."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pcs(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Select pull-up current or pull-down current. built into the UDP pin."]
        #[inline(always)]
        pub const fn set_udp_pcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "UPD port DAC and programmable pull-down. and pull-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pue(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UPD port DAC and programmable pull-down. and pull-up mode selection."]
        #[inline(always)]
        pub const fn set_udp_pue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "UPD port DAC and programmable pull-down. and pull-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UPD port DAC and programmable pull-down. and pull-up mode selection."]
        #[inline(always)]
        pub const fn set_udp_pde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UPD port DAC data/programmable pull-down. resistor and pull-up resistor data."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_dac(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x3f;
            val as u8
        }
        #[doc = "UPD port DAC data/programmable pull-down. resistor and pull-up resistor data."]
        #[inline(always)]
        pub const fn set_udp_dac(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 5usize)) | (((val as u32) & 0x3f) << 5usize);
        }
        #[doc = "Enable of the UDP pin buffer/comparator DAC_BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_ae(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable of the UDP pin buffer/comparator DAC_BUF."]
        #[inline(always)]
        pub const fn set_udp_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output of the UDP pin comparator DAC_BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_ai(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output of the UDP pin comparator DAC_BUF."]
        #[inline(always)]
        pub const fn set_udp_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "UDM pin DAC_BUF output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bufoe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin DAC_BUF output enable."]
        #[inline(always)]
        pub const fn set_udm_bufoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Select pull-up current or pull-down current. built into the UDM pin."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_pcs(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Select pull-up current or pull-down current. built into the UDM pin."]
        #[inline(always)]
        pub const fn set_udm_pcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "UPM port DAC and programmable pull-down. and pull-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_pue(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UPM port DAC and programmable pull-down. and pull-up mode selection."]
        #[inline(always)]
        pub const fn set_udm_pue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UPM port DAC and programmable pull-down. and pull-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_pde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UPM port DAC and programmable pull-down. and pull-up mode selection."]
        #[inline(always)]
        pub const fn set_udm_pde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "UPM port DAC data/programmable pull-down. resistor and pull-up resistor data."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_dac(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x3f;
            val as u8
        }
        #[doc = "UPM port DAC data/programmable pull-down. resistor and pull-up resistor data."]
        #[inline(always)]
        pub const fn set_udm_dac(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 21usize)) | (((val as u32) & 0x3f) << 21usize);
        }
        #[doc = "Enable of the UDM pin buffer/comparator DAC_BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_ae(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable of the UDM pin buffer/comparator DAC_BUF."]
        #[inline(always)]
        pub const fn set_udm_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Output of the UDM pin comparator DAC_BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_ai(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Output of the UDM pin comparator DAC_BUF."]
        #[inline(always)]
        pub const fn set_udm_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    impl core::fmt::Debug for Ctlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr1")
                .field("udp_bufoe", &self.udp_bufoe())
                .field("udp_pcs", &self.udp_pcs())
                .field("udp_pue", &self.udp_pue())
                .field("udp_pde", &self.udp_pde())
                .field("udp_dac", &self.udp_dac())
                .field("udp_ae", &self.udp_ae())
                .field("udp_ai", &self.udp_ai())
                .field("udm_bufoe", &self.udm_bufoe())
                .field("udm_pcs", &self.udm_pcs())
                .field("udm_pue", &self.udm_pue())
                .field("udm_pde", &self.udm_pde())
                .field("udm_dac", &self.udm_dac())
                .field("udm_ae", &self.udm_ae())
                .field("udm_ai", &self.udm_ai())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "QII_OP and QII_CMP enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_ae(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "QII_OP and QII_CMP enable."]
        #[inline(always)]
        pub const fn set_qii_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "QII_OP channel selection at the input."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_ps(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "QII_OP channel selection at the input."]
        #[inline(always)]
        pub const fn set_qii_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "QII_OP gain select."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_av(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "QII_OP gain select."]
        #[inline(always)]
        pub const fn set_qii_av(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "QII_CMP hysteresis voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_hyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "QII_CMP hysteresis voltage selection."]
        #[inline(always)]
        pub const fn set_qii_hyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "QII digital filtering enable."]
        #[must_use]
        #[inline(always)]
        pub const fn qii_filt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "QII digital filtering enable."]
        #[inline(always)]
        pub const fn set_qii_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ISP_OP enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isp_ae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ISP_OP enable."]
        #[inline(always)]
        pub const fn set_isp_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ISP_OP channel selection at the positive input."]
        #[must_use]
        #[inline(always)]
        pub const fn isp_ps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ISP_OP channel selection at the positive input."]
        #[inline(always)]
        pub const fn set_isp_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ISP_OP bias enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isp_be(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ISP_OP bias enable."]
        #[inline(always)]
        pub const fn set_isp_be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ISP_OP channel selection at the negative input."]
        #[must_use]
        #[inline(always)]
        pub const fn isp_ns(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ISP_OP channel selection at the negative input."]
        #[inline(always)]
        pub const fn set_isp_ns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CC1 pin emulation reference enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1_ref(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin emulation reference enable."]
        #[inline(always)]
        pub const fn set_cc1_ref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CC2 pin emulation reference enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2_ref(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin emulation reference enable."]
        #[inline(always)]
        pub const fn set_cc2_ref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "CC3 pin emulation reference enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc3_ref(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "CC3 pin emulation reference enable."]
        #[inline(always)]
        pub const fn set_cc3_ref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CC pin high threshold input mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_hvt(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CC pin high threshold input mode enabled."]
        #[inline(always)]
        pub const fn set_cc_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Reference current state selection. for pins PD PHY and bc."]
        #[must_use]
        #[inline(always)]
        pub const fn iref_inc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reference current state selection. for pins PD PHY and bc."]
        #[inline(always)]
        pub const fn set_iref_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    impl core::fmt::Debug for Ctlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr2")
                .field("qii_ae", &self.qii_ae())
                .field("qii_ps", &self.qii_ps())
                .field("qii_av", &self.qii_av())
                .field("qii_hyp", &self.qii_hyp())
                .field("qii_filt", &self.qii_filt())
                .field("isp_ae", &self.isp_ae())
                .field("isp_ps", &self.isp_ps())
                .field("isp_be", &self.isp_be())
                .field("isp_ns", &self.isp_ns())
                .field("cc1_ref", &self.cc1_ref())
                .field("cc2_ref", &self.cc2_ref())
                .field("cc3_ref", &self.cc3_ref())
                .field("cc_hvt", &self.cc_hvt())
                .field("iref_inc", &self.iref_inc())
                .finish()
        }
    }
}
