#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extend configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exten {
    ptr: *mut u8,
}
unsafe impl Send for Exten {}
unsafe impl Sync for Exten {}
impl Exten {
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
    pub const fn exten_ctlr0(self) -> crate::common::Reg<regs::ExtenCtlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configure unlock register."]
    #[inline(always)]
    pub const fn exten_keyr(self) -> crate::common::Reg<regs::ExtenKeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Configure the extended control register 1."]
    #[inline(always)]
    pub const fn exten_ctlr1(self) -> crate::common::Reg<regs::ExtenCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ISINK1 configuration register."]
    #[inline(always)]
    pub const fn isink1_cfgr(self) -> crate::common::Reg<regs::Isink1Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "ISINK2 configuration register."]
    #[inline(always)]
    pub const fn isink2_cfgr(self) -> crate::common::Reg<regs::Isink2Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "Configure the extended control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr0(pub u32);
    impl ExtenCtlr0 {
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
        #[doc = "Control register 0 and register 1 for configuration locking."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_lock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Control register 0 and register 1 for configuration locking."]
        #[inline(always)]
        pub const fn set_wr_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USBPD0 CC pin analog reference enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd0_cc_ref(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD0 CC pin analog reference enable."]
        #[inline(always)]
        pub const fn set_usbpd0_cc_ref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USBPD1 CC pin analog reference enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd1_cc_ref(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD1 CC pin analog reference enable."]
        #[inline(always)]
        pub const fn set_usbpd1_cc_ref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USBPD0 CC pin high threshold input mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd0_cc_hvt(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD0 CC pin high threshold input mode enabled."]
        #[inline(always)]
        pub const fn set_usbpd0_cc_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USBPD1 CC pin high threshold input mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd1_cc_hvt(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USBPD1 CC pin high threshold input mode enabled."]
        #[inline(always)]
        pub const fn set_usbpd1_cc_hvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "CC pin of USBPD0 1.12V output Tr,Tf difference adjustment."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd0_lve_t(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "CC pin of USBPD0 1.12V output Tr,Tf difference adjustment."]
        #[inline(always)]
        pub const fn set_usbpd0_lve_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "CC pin of USBPD1 1.12V output Tr,Tf difference adjustment."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd1_lve_t(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "CC pin of USBPD1 1.12V output Tr,Tf difference adjustment."]
        #[inline(always)]
        pub const fn set_usbpd1_lve_t(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Source current 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isrc1_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Source current 1 enable."]
        #[inline(always)]
        pub const fn set_isrc1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Source current 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isrc1_sel(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Source current 1 selection."]
        #[inline(always)]
        pub const fn set_isrc1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Source current 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn isrc2_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Source current 2 enable."]
        #[inline(always)]
        pub const fn set_isrc2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Source current 2 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn isrc2_sel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Source current 2 selection."]
        #[inline(always)]
        pub const fn set_isrc2_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for ExtenCtlr0 {
        #[inline(always)]
        fn default() -> ExtenCtlr0 {
            ExtenCtlr0(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr0")
                .field("lkupen", &self.lkupen())
                .field("lkuprst", &self.lkuprst())
                .field("wr_lock", &self.wr_lock())
                .field("usbpd0_cc_ref", &self.usbpd0_cc_ref())
                .field("usbpd1_cc_ref", &self.usbpd1_cc_ref())
                .field("usbpd0_cc_hvt", &self.usbpd0_cc_hvt())
                .field("usbpd1_cc_hvt", &self.usbpd1_cc_hvt())
                .field("usbpd0_lve_t", &self.usbpd0_lve_t())
                .field("usbpd1_lve_t", &self.usbpd1_lve_t())
                .field("isrc1_en", &self.isrc1_en())
                .field("isrc1_sel", &self.isrc1_sel())
                .field("isrc2_en", &self.isrc2_en())
                .field("isrc2_sel", &self.isrc2_sel())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr1(pub u32);
    impl ExtenCtlr1 {
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
        #[doc = "UDP pin speed selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_rat(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin speed selection."]
        #[inline(always)]
        pub const fn set_udp_rat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "UDP pin is input enabled when acting as a GPIO."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin is input enabled when acting as a GPIO."]
        #[inline(always)]
        pub const fn set_udp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
        #[doc = "UDM pin speed selection."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_rat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin speed selection."]
        #[inline(always)]
        pub const fn set_udm_rat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UDM pin is input enabled when acting as a GPIO."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin is input enabled when acting as a GPIO."]
        #[inline(always)]
        pub const fn set_udm_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UDP pin and UDM pin are shorted to enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udu_shrt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin and UDM pin are shorted to enable."]
        #[inline(always)]
        pub const fn set_udu_shrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ExtenCtlr1 {
        #[inline(always)]
        fn default() -> ExtenCtlr1 {
            ExtenCtlr1(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr1")
                .field("udp_bufoe", &self.udp_bufoe())
                .field("udp_pcs", &self.udp_pcs())
                .field("udp_pue", &self.udp_pue())
                .field("udp_pde", &self.udp_pde())
                .field("udp_dac", &self.udp_dac())
                .field("udp_ae", &self.udp_ae())
                .field("udp_ai", &self.udp_ai())
                .field("udp_rat", &self.udp_rat())
                .field("udp_ie", &self.udp_ie())
                .field("udm_bufoe", &self.udm_bufoe())
                .field("udm_pcs", &self.udm_pcs())
                .field("udm_pue", &self.udm_pue())
                .field("udm_pde", &self.udm_pde())
                .field("udm_dac", &self.udm_dac())
                .field("udm_ae", &self.udm_ae())
                .field("udm_ai", &self.udm_ai())
                .field("udm_rat", &self.udm_rat())
                .field("udm_ie", &self.udm_ie())
                .field("udu_shrt", &self.udu_shrt())
                .finish()
        }
    }
    #[doc = "Configure unlock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenKeyr(pub u32);
    impl ExtenKeyr {
        #[doc = "Control register 0 and register 1 for configuration unlocking."]
        #[must_use]
        #[inline(always)]
        pub const fn keyr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Control register 0 and register 1 for configuration unlocking."]
        #[inline(always)]
        pub const fn set_keyr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ExtenKeyr {
        #[inline(always)]
        fn default() -> ExtenKeyr {
            ExtenKeyr(0)
        }
    }
    impl core::fmt::Debug for ExtenKeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenKeyr")
                .field("keyr", &self.keyr())
                .finish()
        }
    }
    #[doc = "ISINK1 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isink1Cfgr(pub u16);
    impl Isink1Cfgr {
        #[doc = "Sink current 1 calibration value."]
        #[must_use]
        #[inline(always)]
        pub const fn isink1_adj(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Sink current 1 calibration value."]
        #[inline(always)]
        pub const fn set_isink1_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "Sink current 1 programmable data bit."]
        #[must_use]
        #[inline(always)]
        pub const fn isink1_dat(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x03ff;
            val as u16
        }
        #[doc = "Sink current 1 programmable data bit."]
        #[inline(always)]
        pub const fn set_isink1_dat(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
        }
    }
    impl Default for Isink1Cfgr {
        #[inline(always)]
        fn default() -> Isink1Cfgr {
            Isink1Cfgr(0)
        }
    }
    impl core::fmt::Debug for Isink1Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isink1Cfgr")
                .field("isink1_adj", &self.isink1_adj())
                .field("isink1_dat", &self.isink1_dat())
                .finish()
        }
    }
    #[doc = "ISINK2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isink2Cfgr(pub u16);
    impl Isink2Cfgr {
        #[doc = "Sink current 2 calibration value."]
        #[must_use]
        #[inline(always)]
        pub const fn isink2_adj(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Sink current 2 calibration value."]
        #[inline(always)]
        pub const fn set_isink2_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "Sink current 2 programmable data bit."]
        #[must_use]
        #[inline(always)]
        pub const fn isink2_dat(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x03ff;
            val as u16
        }
        #[doc = "Sink current 2 programmable data bit."]
        #[inline(always)]
        pub const fn set_isink2_dat(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u16) & 0x03ff) << 6usize);
        }
    }
    impl Default for Isink2Cfgr {
        #[inline(always)]
        fn default() -> Isink2Cfgr {
            Isink2Cfgr(0)
        }
    }
    impl core::fmt::Debug for Isink2Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isink2Cfgr")
                .field("isink2_adj", &self.isink2_adj())
                .field("isink2_dat", &self.isink2_dat())
                .finish()
        }
    }
}
