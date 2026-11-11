#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Touch-key controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tkey {
    ptr: *mut u8,
}
unsafe impl Send for Tkey {}
unsafe impl Sync for Tkey {}
impl Tkey {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TKEY control register."]
    #[inline(always)]
    pub const fn tkey_ctlr(self) -> crate::common::Reg<regs::TkeyCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Charge Shift Configuration Register."]
    #[inline(always)]
    pub const fn tkey_trans_cfg(self) -> crate::common::Reg<regs::TkeyTransCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Capacitor Configuration Register."]
    #[inline(always)]
    pub const fn cap_cfg(self) -> crate::common::Reg<regs::CapCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Driver Shield Configuration Register."]
    #[inline(always)]
    pub const fn drv_cfg(self) -> crate::common::Reg<regs::DrvCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "TKEY Charge-Discharge Configuration Register."]
    #[inline(always)]
    pub const fn tkey_cdcc(self) -> crate::common::Reg<regs::TkeyCdcc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "TKEY Status Register."]
    #[inline(always)]
    pub const fn tkey_sr(self) -> crate::common::Reg<regs::TkeySr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Capacitor Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CapCfg(pub u32);
    impl CapCfg {
        #[doc = "In charge transfer mode, the channel with an external reference capacitor is configured,. and in current source charging mode, this bit is invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn selcx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "In charge transfer mode, the channel with an external reference capacitor is configured,. and in current source charging mode, this bit is invalid."]
        #[inline(always)]
        pub const fn set_selcx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "In charge transfer mode, a channel for charge transfer is configured."]
        #[must_use]
        #[inline(always)]
        pub const fn selcs(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0xffff;
            val as u16
        }
        #[doc = "In charge transfer mode, a channel for charge transfer is configured."]
        #[inline(always)]
        pub const fn set_selcs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 4usize)) | (((val as u32) & 0xffff) << 4usize);
        }
    }
    impl Default for CapCfg {
        #[inline(always)]
        fn default() -> CapCfg {
            CapCfg(0)
        }
    }
    impl core::fmt::Debug for CapCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CapCfg")
                .field("selcx", &self.selcx())
                .field("selcs", &self.selcs())
                .finish()
        }
    }
    #[doc = "Driver Shield Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DrvCfg(pub u32);
    impl DrvCfg {
        #[doc = "Drive Shielding Function Enable Position."]
        #[must_use]
        #[inline(always)]
        pub const fn drven(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Drive Shielding Function Enable Position."]
        #[inline(always)]
        pub const fn set_drven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Drive shield channel selection, active high."]
        #[must_use]
        #[inline(always)]
        pub const fn drvoutsel(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Drive shield channel selection, active high."]
        #[inline(always)]
        pub const fn set_drvoutsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DrvCfg {
        #[inline(always)]
        fn default() -> DrvCfg {
            DrvCfg(0)
        }
    }
    impl core::fmt::Debug for DrvCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DrvCfg")
                .field("drven", &self.drven())
                .field("drvoutsel", &self.drvoutsel())
                .finish()
        }
    }
    #[doc = "TKEY Charge-Discharge Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyCdcc(pub u32);
    impl TkeyCdcc {
        #[doc = "In current source charging mode, the discharge time is configured."]
        #[must_use]
        #[inline(always)]
        pub const fn tkcd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "In current source charging mode, the discharge time is configured."]
        #[inline(always)]
        pub const fn set_tkcd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "In the current source charging mode, the charging time is configured."]
        #[must_use]
        #[inline(always)]
        pub const fn tkcc(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "In the current source charging mode, the charging time is configured."]
        #[inline(always)]
        pub const fn set_tkcc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for TkeyCdcc {
        #[inline(always)]
        fn default() -> TkeyCdcc {
            TkeyCdcc(0)
        }
    }
    impl core::fmt::Debug for TkeyCdcc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TkeyCdcc")
                .field("tkcd", &self.tkcd())
                .field("tkcc", &self.tkcc())
                .finish()
        }
    }
    #[doc = "TKEY control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyCtlr(pub u32);
    impl TkeyCtlr {
        #[doc = "TKEY enabler."]
        #[must_use]
        #[inline(always)]
        pub const fn tken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY enabler."]
        #[inline(always)]
        pub const fn set_tken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Charging current configuration of the TKEY module."]
        #[must_use]
        #[inline(always)]
        pub const fn itune(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Charging current configuration of the TKEY module."]
        #[inline(always)]
        pub const fn set_itune(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TKEY mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY mode configuration."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn chsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel selection."]
        #[inline(always)]
        pub const fn set_chsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TKEY triggers the control."]
        #[must_use]
        #[inline(always)]
        pub const fn tkact(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY triggers the control."]
        #[inline(always)]
        pub const fn set_tkact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TKEY Channel Scan Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY Channel Scan Interrupt Enable."]
        #[inline(always)]
        pub const fn set_scanie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TKEY channel sweep function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TKEY channel sweep function enable."]
        #[inline(always)]
        pub const fn set_scanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Configure the number of TKEY scanning channels."]
        #[must_use]
        #[inline(always)]
        pub const fn scannum(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Configure the number of TKEY scanning channels."]
        #[inline(always)]
        pub const fn set_scannum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for TkeyCtlr {
        #[inline(always)]
        fn default() -> TkeyCtlr {
            TkeyCtlr(0)
        }
    }
    impl core::fmt::Debug for TkeyCtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TkeyCtlr")
                .field("tken", &self.tken())
                .field("itune", &self.itune())
                .field("mode", &self.mode())
                .field("chsel", &self.chsel())
                .field("tkact", &self.tkact())
                .field("scanie", &self.scanie())
                .field("scanen", &self.scanen())
                .field("scannum", &self.scannum())
                .finish()
        }
    }
    #[doc = "TKEY Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeySr(pub u32);
    impl TkeySr {
        #[doc = "The TKEY channel scan is interrupted at the end of the mark,. the software writes 0 clear, and the write 1 is invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn scanif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The TKEY channel scan is interrupted at the end of the mark,. the software writes 0 clear, and the write 1 is invalid."]
        #[inline(always)]
        pub const fn set_scanif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TkeySr {
        #[inline(always)]
        fn default() -> TkeySr {
            TkeySr(0)
        }
    }
    impl core::fmt::Debug for TkeySr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TkeySr")
                .field("scanif", &self.scanif())
                .finish()
        }
    }
    #[doc = "Charge Shift Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TkeyTransCfg(pub u32);
    impl TkeyTransCfg {
        #[doc = "In charge transfer mode, touch the button capacitor charge charge T2 and charge transfer time T4 configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn transcc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "In charge transfer mode, touch the button capacitor charge charge T2 and charge transfer time T4 configuration bits."]
        #[inline(always)]
        pub const fn set_transcc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "The actual number of transfer cycles is TRANSN+1, that is,. the maximum number of transfer cycles is 1024."]
        #[must_use]
        #[inline(always)]
        pub const fn transn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x03ff;
            val as u16
        }
        #[doc = "The actual number of transfer cycles is TRANSN+1, that is,. the maximum number of transfer cycles is 1024."]
        #[inline(always)]
        pub const fn set_transn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 3usize)) | (((val as u32) & 0x03ff) << 3usize);
        }
        #[doc = "TKEY switch switching time configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tksw(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "TKEY switch switching time configuration."]
        #[inline(always)]
        pub const fn set_tksw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
    }
    impl Default for TkeyTransCfg {
        #[inline(always)]
        fn default() -> TkeyTransCfg {
            TkeyTransCfg(0)
        }
    }
    impl core::fmt::Debug for TkeyTransCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TkeyTransCfg")
                .field("transcc", &self.transcc())
                .field("transn", &self.transn())
                .field("tksw", &self.tksw())
                .finish()
        }
    }
}
