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
    #[doc = "Access control register."]
    #[inline(always)]
    pub const fn actlr(self) -> crate::common::Reg<regs::Actlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Flash key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flash option key register."]
    #[inline(always)]
    pub const fn obkeyr(self) -> crate::common::Reg<regs::Obkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn ctlr(self) -> crate::common::Reg<regs::Ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash address register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Option byte register."]
    #[inline(always)]
    pub const fn obr(self) -> crate::common::Reg<regs::Obr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write protection register."]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Mode select register."]
    #[inline(always)]
    pub const fn modekeyr(self) -> crate::common::Reg<regs::Modekeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Actlr(pub u32);
    impl Actlr {
        #[doc = "FLASH standby condition."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FLASH standby condition."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Actlr {
        #[inline(always)]
        fn default() -> Actlr {
            Actlr(0)
        }
    }
    #[doc = "Flash address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "Flash Address."]
        #[inline(always)]
        pub const fn far(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash Address."]
        #[inline(always)]
        pub fn set_far(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr(pub u32);
    impl Ctlr {
        #[doc = "Page Erase."]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page Erase."]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass Erase."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Option byte programming."]
        #[inline(always)]
        pub const fn obpg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte programming."]
        #[inline(always)]
        pub fn set_obpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Option byte erase."]
        #[inline(always)]
        pub const fn ober(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase."]
        #[inline(always)]
        pub fn set_ober(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Start."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Start."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option bytes write enable."]
        #[inline(always)]
        pub const fn obwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Option bytes write enable."]
        #[inline(always)]
        pub fn set_obwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "wake-up interrupt enable."]
        #[inline(always)]
        pub const fn fwakeie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up interrupt enable."]
        #[inline(always)]
        pub fn set_fwakeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Fast programmable lock."]
        #[inline(always)]
        pub const fn flock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programmable lock."]
        #[inline(always)]
        pub fn set_flock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fast programming."]
        #[inline(always)]
        pub const fn ptpg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming."]
        #[inline(always)]
        pub fn set_ptpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast erase."]
        #[inline(always)]
        pub const fn pter(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast erase."]
        #[inline(always)]
        pub fn set_pter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Data buffer."]
        #[inline(always)]
        pub const fn bufload(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer."]
        #[inline(always)]
        pub fn set_bufload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUF reset."]
        #[inline(always)]
        pub const fn bufrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BUF reset."]
        #[inline(always)]
        pub fn set_bufrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Block Erase 32K."]
        #[inline(always)]
        pub const fn ber32(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Block Erase 32K."]
        #[inline(always)]
        pub fn set_ber32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctlr {
        #[inline(always)]
        fn default() -> Ctlr {
            Ctlr(0)
        }
    }
    #[doc = "Flash key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "FPEC key."]
        #[inline(always)]
        pub const fn keyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FPEC key."]
        #[inline(always)]
        pub fn set_keyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    #[doc = "Mode select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modekeyr(pub u32);
    impl Modekeyr {
        #[doc = "Mode select."]
        #[inline(always)]
        pub const fn modekeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mode select."]
        #[inline(always)]
        pub fn set_modekeyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Modekeyr {
        #[inline(always)]
        fn default() -> Modekeyr {
            Modekeyr(0)
        }
    }
    #[doc = "Flash option key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obkeyr(pub u32);
    impl Obkeyr {
        #[doc = "Option byte key."]
        #[inline(always)]
        pub const fn obtkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option byte key."]
        #[inline(always)]
        pub fn set_obtkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Obkeyr {
        #[inline(always)]
        fn default() -> Obkeyr {
            Obkeyr(0)
        }
    }
    #[doc = "Option byte register — read-only mirror of the Option Bytes loaded from flash at reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Option byte integrity error — a value/complement pair did not match when loading."]
        #[inline(always)]
        pub const fn oberr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte integrity error — a value/complement pair did not match when loading."]
        #[inline(always)]
        pub fn set_oberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read protection is currently active."]
        #[inline(always)]
        pub const fn rdprt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read protection is currently active."]
        #[inline(always)]
        pub fn set_rdprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Independent watchdog start mode currently in effect."]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::IwdgMode {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::IwdgMode::from_bits(val as u8)
        }
        #[doc = "Independent watchdog start mode currently in effect."]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: super::vals::IwdgMode) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Stop-mode reset behavior currently in effect."]
        #[inline(always)]
        pub const fn stop_rst(&self) -> super::vals::PowerModeReset {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::PowerModeReset::from_bits(val as u8)
        }
        #[doc = "Stop-mode reset behavior currently in effect."]
        #[inline(always)]
        pub fn set_stop_rst(&mut self, val: super::vals::PowerModeReset) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[inline(always)]
        pub const fn standy_rst(&self) -> super::vals::PowerModeReset {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::PowerModeReset::from_bits(val as u8)
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[inline(always)]
        pub fn set_standy_rst(&mut self, val: super::vals::PowerModeReset) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "CAN offline-to-normal recovery timing currently in effect (mirror of OB.USER.CFGCANM)."]
        #[inline(always)]
        pub const fn cfgcanm(&self) -> super::vals::CanRecoveryMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::CanRecoveryMode::from_bits(val as u8)
        }
        #[doc = "CAN offline-to-normal recovery timing currently in effect (mirror of OB.USER.CFGCANM)."]
        #[inline(always)]
        pub fn set_cfgcanm(&mut self, val: super::vals::CanRecoveryMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Hard-wired to 11b."]
        #[inline(always)]
        pub const fn fix_11(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Hard-wired to 11b."]
        #[inline(always)]
        pub fn set_fix_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Free user data byte 0 (mirror of OB.DATA0)."]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 0 (mirror of OB.DATA0)."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Free user data byte 1 (mirror of OB.DATA1)."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 1 (mirror of OB.DATA1)."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
    }
    impl Default for Obr {
        #[inline(always)]
        fn default() -> Obr {
            Obr(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Busy."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write protection error."]
        #[inline(always)]
        pub const fn wrprterr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error."]
        #[inline(always)]
        pub fn set_wrprterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of operation."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wake-Up flag."]
        #[inline(always)]
        pub const fn fwake_flag(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-Up flag."]
        #[inline(always)]
        pub fn set_fwake_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TURBO."]
        #[inline(always)]
        pub const fn turbo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TURBO."]
        #[inline(always)]
        pub fn set_turbo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    #[doc = "Write protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "Write protect."]
        #[inline(always)]
        pub const fn wrp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write protect."]
        #[inline(always)]
        pub fn set_wrp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wpr {
        #[inline(always)]
        fn default() -> Wpr {
            Wpr(0)
        }
    }
}
pub mod vals {
    #[doc = "How long the CAN controller waits before returning from bus-off to normal operation."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CanRecoveryMode {
        #[doc = "Recover from bus-off in accordance with the CAN protocol."]
        CAN_PROTOCOL = 0x0,
        #[doc = "Recover from bus-off slightly faster than the CAN protocol specifies."]
        FAST = 0x01,
    }
    impl CanRecoveryMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CanRecoveryMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CanRecoveryMode {
        #[inline(always)]
        fn from(val: u8) -> CanRecoveryMode {
            CanRecoveryMode::from_bits(val)
        }
    }
    impl From<CanRecoveryMode> for u8 {
        #[inline(always)]
        fn from(val: CanRecoveryMode) -> u8 {
            CanRecoveryMode::to_bits(val)
        }
    }
    #[doc = "When the independent watchdog turns on after reset."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
}
