#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash ACTL register."]
    #[inline(always)]
    pub const fn actlr(self) -> crate::common::Reg<regs::Actlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash option key register."]
    #[inline(always)]
    pub const fn obkeyr(self) -> crate::common::Reg<regs::Obkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Flash address register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Option byte register."]
    #[inline(always)]
    pub const fn obr(self) -> crate::common::Reg<regs::Obr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Write protection register."]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Mode select register."]
    #[inline(always)]
    pub const fn modekeyr(self) -> crate::common::Reg<regs::Modekeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "BOOT Mode select register."]
    #[inline(always)]
    pub const fn boot_modekeyp(self) -> crate::common::Reg<regs::BootModekeyp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash ACTL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Actlr(pub u32);
    impl Actlr {
        #[doc = "LATENCY."]
        #[must_use]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "LATENCY."]
        #[inline(always)]
        pub const fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Actlr {
        #[inline(always)]
        fn default() -> Actlr {
            Actlr(0)
        }
    }
    impl core::fmt::Debug for Actlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Actlr")
                .field("latency", &self.latency())
                .finish()
        }
    }
    #[doc = "Flash address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "Flash Address."]
        #[must_use]
        #[inline(always)]
        pub const fn far(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash Address."]
        #[inline(always)]
        pub const fn set_far(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr").field("far", &self.far()).finish()
        }
    }
    #[doc = "BOOT Mode select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootModekeyp(pub u32);
    impl BootModekeyp {
        #[doc = "Mode select."]
        #[must_use]
        #[inline(always)]
        pub const fn modekeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mode select."]
        #[inline(always)]
        pub const fn set_modekeyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BootModekeyp {
        #[inline(always)]
        fn default() -> BootModekeyp {
            BootModekeyp(0)
        }
    }
    impl core::fmt::Debug for BootModekeyp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BootModekeyp")
                .field("modekeyr", &self.modekeyr())
                .finish()
        }
    }
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Page Erase."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page Erase."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass Erase."]
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase."]
        #[inline(always)]
        pub const fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Option byte erase."]
        #[must_use]
        #[inline(always)]
        pub const fn ober(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase."]
        #[inline(always)]
        pub const fn set_ober(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Start."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Start."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option bytes write enable."]
        #[must_use]
        #[inline(always)]
        pub const fn obwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Option bytes write enable."]
        #[inline(always)]
        pub const fn set_obwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wakeup enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fwakeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup enable."]
        #[inline(always)]
        pub const fn set_fwakeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Fast programmable lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programmable lock."]
        #[inline(always)]
        pub const fn set_flock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fast programming."]
        #[must_use]
        #[inline(always)]
        pub const fn ftpg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming."]
        #[inline(always)]
        pub const fn set_ftpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast erase."]
        #[must_use]
        #[inline(always)]
        pub const fn fter(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast erase."]
        #[inline(always)]
        pub const fn set_fter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BUFLOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn bufload(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BUFLOAD."]
        #[inline(always)]
        pub const fn set_bufload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUFRST."]
        #[must_use]
        #[inline(always)]
        pub const fn bufrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BUFRST."]
        #[inline(always)]
        pub const fn set_bufrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BER32."]
        #[must_use]
        #[inline(always)]
        pub const fn ber32(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "BER32."]
        #[inline(always)]
        pub const fn set_ber32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    impl core::fmt::Debug for Ctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr")
                .field("per", &self.per())
                .field("mer", &self.mer())
                .field("ober", &self.ober())
                .field("strt", &self.strt())
                .field("lock", &self.lock())
                .field("obwre", &self.obwre())
                .field("errie", &self.errie())
                .field("eopie", &self.eopie())
                .field("fwakeie", &self.fwakeie())
                .field("flock", &self.flock())
                .field("ftpg", &self.ftpg())
                .field("fter", &self.fter())
                .field("bufload", &self.bufload())
                .field("bufrst", &self.bufrst())
                .field("ber32", &self.ber32())
                .finish()
        }
    }
    #[doc = "Flash key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "FPEC key."]
        #[must_use]
        #[inline(always)]
        pub const fn keyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FPEC key."]
        #[inline(always)]
        pub const fn set_keyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("keyr", &self.keyr()).finish()
        }
    }
    #[doc = "Mode select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modekeyr(pub u32);
    impl Modekeyr {
        #[doc = "Mode select."]
        #[must_use]
        #[inline(always)]
        pub const fn modekeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mode select."]
        #[inline(always)]
        pub const fn set_modekeyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Modekeyr {
        #[inline(always)]
        fn default() -> Modekeyr {
            Modekeyr(0)
        }
    }
    impl core::fmt::Debug for Modekeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modekeyr")
                .field("modekeyr", &self.modekeyr())
                .finish()
        }
    }
    #[doc = "Flash option key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obkeyr(pub u32);
    impl Obkeyr {
        #[doc = "Option byte key."]
        #[must_use]
        #[inline(always)]
        pub const fn optkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option byte key."]
        #[inline(always)]
        pub const fn set_optkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Obkeyr {
        #[inline(always)]
        fn default() -> Obkeyr {
            Obkeyr(0)
        }
    }
    impl core::fmt::Debug for Obkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obkeyr")
                .field("optkey", &self.optkey())
                .finish()
        }
    }
    #[doc = "Option byte register — read-only mirror of the Option Bytes loaded from flash at reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Option byte integrity error — a value/complement pair did not match when loading."]
        #[must_use]
        #[inline(always)]
        pub const fn oberr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte integrity error — a value/complement pair did not match when loading."]
        #[inline(always)]
        pub const fn set_oberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read protection is currently active."]
        #[must_use]
        #[inline(always)]
        pub const fn rdprt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read protection is currently active."]
        #[inline(always)]
        pub const fn set_rdprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Independent watchdog start mode currently in effect."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::IwdgMode {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::IwdgMode::from_bits(val as u8)
        }
        #[doc = "Independent watchdog start mode currently in effect."]
        #[inline(always)]
        pub const fn set_iwdg_sw(&mut self, val: super::vals::IwdgMode) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[must_use]
        #[inline(always)]
        pub const fn standy_rst(&self) -> super::vals::PowerModeReset {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::PowerModeReset::from_bits(val as u8)
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[inline(always)]
        pub const fn set_standy_rst(&mut self, val: super::vals::PowerModeReset) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "NRST pin mode currently in effect."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_mode(&self) -> super::vals::RstPinMode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::RstPinMode::from_bits(val as u8)
        }
        #[doc = "NRST pin mode currently in effect."]
        #[inline(always)]
        pub const fn set_rst_mode(&mut self, val: super::vals::RstPinMode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Power-on boot source currently in effect — loaded from OB.USER.START_MODE; copied into STATR.MODE (bit 14) at reset."]
        #[must_use]
        #[inline(always)]
        pub const fn statr_mode(&self) -> super::vals::BootMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::BootMode::from_bits(val as u8)
        }
        #[doc = "Power-on boot source currently in effect — loaded from OB.USER.START_MODE; copied into STATR.MODE (bit 14) at reset."]
        #[inline(always)]
        pub const fn set_statr_mode(&mut self, val: super::vals::BootMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Free user data byte 0, mirrored from OB.DATA0."]
        #[must_use]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 0, mirrored from OB.DATA0."]
        #[inline(always)]
        pub const fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[doc = "Free user data byte 1, mirrored from OB.DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 1, mirrored from OB.DATA1."]
        #[inline(always)]
        pub const fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
        }
    }
    impl Default for Obr {
        #[inline(always)]
        fn default() -> Obr {
            Obr(0)
        }
    }
    impl core::fmt::Debug for Obr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obr")
                .field("oberr", &self.oberr())
                .field("rdprt", &self.rdprt())
                .field("iwdg_sw", &self.iwdg_sw())
                .field("standy_rst", &self.standy_rst())
                .field("rst_mode", &self.rst_mode())
                .field("statr_mode", &self.statr_mode())
                .field("data0", &self.data0())
                .field("data1", &self.data1())
                .finish()
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write protection error."]
        #[must_use]
        #[inline(always)]
        pub const fn wrprterr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error."]
        #[inline(always)]
        pub const fn set_wrprterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of operation."]
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation."]
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FWAKE_FLAG mode start."]
        #[must_use]
        #[inline(always)]
        pub const fn fwake_flag(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FWAKE_FLAG mode start."]
        #[inline(always)]
        pub const fn set_fwake_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TURBO mode start."]
        #[must_use]
        #[inline(always)]
        pub const fn turbo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TURBO mode start."]
        #[inline(always)]
        pub const fn set_turbo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BOOT_AVA."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_ava(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_AVA."]
        #[inline(always)]
        pub const fn set_boot_ava(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BOOT_STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_status(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_STATUS."]
        #[inline(always)]
        pub const fn set_boot_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "BOOT_MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_MODE."]
        #[inline(always)]
        pub const fn set_boot_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "BOOT_LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_LOCK."]
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    impl core::fmt::Debug for Statr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Statr")
                .field("bsy", &self.bsy())
                .field("wrprterr", &self.wrprterr())
                .field("eop", &self.eop())
                .field("fwake_flag", &self.fwake_flag())
                .field("turbo", &self.turbo())
                .field("boot_ava", &self.boot_ava())
                .field("boot_status", &self.boot_status())
                .field("boot_mode", &self.boot_mode())
                .field("boot_lock", &self.boot_lock())
                .finish()
        }
    }
    #[doc = "Write protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "Write protect."]
        #[must_use]
        #[inline(always)]
        pub const fn wrp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write protect."]
        #[inline(always)]
        pub const fn set_wrp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wpr {
        #[inline(always)]
        fn default() -> Wpr {
            Wpr(0)
        }
    }
    impl core::fmt::Debug for Wpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpr").field("wrp", &self.wrp()).finish()
        }
    }
}
pub mod vals {
    #[doc = "Which flash region the chip boots from at power-on."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BootMode {
        #[doc = "Boot from user application flash."]
        USER_FLASH = 0x0,
        #[doc = "Boot from the WCH ISP bootloader in system flash."]
        SYSTEM_FLASH = 0x01,
    }
    impl BootMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BootMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BootMode {
        #[inline(always)]
        fn from(val: u8) -> BootMode {
            BootMode::from_bits(val)
        }
    }
    impl From<BootMode> for u8 {
        #[inline(always)]
        fn from(val: BootMode) -> u8 {
            BootMode::to_bits(val)
        }
    }
    #[doc = "When the independent watchdog turns on after reset."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IwdgMode {
        #[doc = "Watchdog starts automatically at power-on."]
        HARDWARE = 0x0,
        #[doc = "Watchdog stays off until software enables it."]
        SOFTWARE = 0x01,
    }
    impl IwdgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IwdgMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IwdgMode {
        #[inline(always)]
        fn from(val: u8) -> IwdgMode {
            IwdgMode::from_bits(val)
        }
    }
    impl From<IwdgMode> for u8 {
        #[inline(always)]
        fn from(val: IwdgMode) -> u8 {
            IwdgMode::to_bits(val)
        }
    }
    #[doc = "Whether entering a low-power mode triggers a system reset."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PowerModeReset {
        #[doc = "Reset the chip on entering the mode."]
        RESET = 0x0,
        #[doc = "Keep running on entering the mode."]
        NO_RESET = 0x01,
    }
    impl PowerModeReset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PowerModeReset {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PowerModeReset {
        #[inline(always)]
        fn from(val: u8) -> PowerModeReset {
            PowerModeReset::from_bits(val)
        }
    }
    impl From<PowerModeReset> for u8 {
        #[inline(always)]
        fn from(val: PowerModeReset) -> u8 {
            PowerModeReset::to_bits(val)
        }
    }
    #[doc = "NRST pin behavior at power-on."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RstPinMode {
        #[doc = "Enable NRST with a 128us glitch-filter window at power-on."]
        NRST_128US = 0x0,
        #[doc = "Enable NRST with a 1ms glitch-filter window at power-on."]
        NRST_1MS = 0x01,
        #[doc = "Enable NRST with a 12ms glitch-filter window at power-on."]
        NRST_12MS = 0x02,
        #[doc = "Disable NRST and use PD7 as a general-purpose I/O."]
        GPIO = 0x03,
    }
    impl RstPinMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RstPinMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RstPinMode {
        #[inline(always)]
        fn from(val: u8) -> RstPinMode {
            RstPinMode::from_bits(val)
        }
    }
    impl From<RstPinMode> for u8 {
        #[inline(always)]
        fn from(val: RstPinMode) -> u8 {
            RstPinMode::to_bits(val)
        }
    }
}
