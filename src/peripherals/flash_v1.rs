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
    #[doc = "Flash access control register."]
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
    #[doc = "Extension key register."]
    #[inline(always)]
    pub const fn modekeyr(self) -> crate::common::Reg<regs::Modekeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Actlr(pub u32);
    impl Actlr {
        #[doc = "Latency."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Latency."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Prefetch buffer enable."]
        #[inline(always)]
        pub const fn prftbe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer enable."]
        #[inline(always)]
        pub fn set_prftbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prefetch buffer status."]
        #[inline(always)]
        pub const fn prftbs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer status."]
        #[inline(always)]
        pub fn set_prftbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
        #[doc = "Programming."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
        #[doc = "FAST programming lock."]
        #[inline(always)]
        pub const fn flock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FAST programming lock."]
        #[inline(always)]
        pub fn set_flock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "execute fast programming."]
        #[inline(always)]
        pub const fn ftpg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "execute fast programming."]
        #[inline(always)]
        pub fn set_ftpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "execute fast 128byte erase."]
        #[inline(always)]
        pub const fn fter(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "execute fast 128byte erase."]
        #[inline(always)]
        pub fn set_fter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "execute data load inner buffer."]
        #[inline(always)]
        pub const fn bufload(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "execute data load inner buffer."]
        #[inline(always)]
        pub fn set_bufload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "execute inner buffer reset."]
        #[inline(always)]
        pub const fn bufrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "execute inner buffer reset."]
        #[inline(always)]
        pub fn set_bufrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
    #[doc = "Extension key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modekeyr(pub u32);
    impl Modekeyr {
        #[doc = "high speed write /erase mode ENABLE."]
        #[inline(always)]
        pub const fn modekeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "high speed write /erase mode ENABLE."]
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
        pub const fn obkeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option byte key."]
        #[inline(always)]
        pub fn set_obkeyr(&mut self, val: u32) {
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
        pub const fn opterr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte integrity error — a value/complement pair did not match when loading."]
        #[inline(always)]
        pub fn set_opterr(&mut self, val: bool) {
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
        pub const fn n_rst_stop(&self) -> super::vals::PowerModeReset {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::PowerModeReset::from_bits(val as u8)
        }
        #[doc = "Stop-mode reset behavior currently in effect."]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: super::vals::PowerModeReset) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> super::vals::PowerModeReset {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::PowerModeReset::from_bits(val as u8)
        }
        #[doc = "Standby-mode reset behavior currently in effect."]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: super::vals::PowerModeReset) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "USBD bus speed mode currently in effect."]
        #[inline(always)]
        pub const fn usbd_mode(&self) -> super::vals::UsbdSpeed {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::UsbdSpeed::from_bits(val as u8)
        }
        #[doc = "USBD bus speed mode currently in effect."]
        #[inline(always)]
        pub fn set_usbd_mode(&mut self, val: super::vals::UsbdSpeed) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "USBD built-in 1.5 kΩ pull-up resistor state currently in effect."]
        #[inline(always)]
        pub const fn usbd_pu(&self) -> super::vals::UsbdPullup {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::UsbdPullup::from_bits(val as u8)
        }
        #[doc = "USBD built-in 1.5 kΩ pull-up resistor state currently in effect."]
        #[inline(always)]
        pub fn set_usbd_pu(&mut self, val: super::vals::UsbdPullup) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Power-on reset timing window currently in effect."]
        #[inline(always)]
        pub const fn por_ctr(&self) -> super::vals::PowerOnResetTime {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::PowerOnResetTime::from_bits(val as u8)
        }
        #[doc = "Power-on reset timing window currently in effect."]
        #[inline(always)]
        pub fn set_por_ctr(&mut self, val: super::vals::PowerOnResetTime) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Free user data byte 0 (mirror of OB.DATA0)."]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 0 (mirror of OB.DATA0)."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[doc = "Free user data byte 1 (mirror of OB.DATA1)."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "Free user data byte 1 (mirror of OB.DATA1)."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
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
        #[doc = "Programming error."]
        #[inline(always)]
        pub const fn pgerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error."]
        #[inline(always)]
        pub fn set_pgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Length of the power-on reset window."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PowerOnResetTime {
        #[doc = "40.96 ms power-on reset window."]
        LONG = 0x0,
        #[doc = "16.384 ms power-on reset window."]
        SHORT = 0x01,
    }
    impl PowerOnResetTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PowerOnResetTime {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PowerOnResetTime {
        #[inline(always)]
        fn from(val: u8) -> PowerOnResetTime {
            PowerOnResetTime::from_bits(val)
        }
    }
    impl From<PowerOnResetTime> for u8 {
        #[inline(always)]
        fn from(val: PowerOnResetTime) -> u8 {
            PowerOnResetTime::to_bits(val)
        }
    }
    #[doc = "USBD built-in 1.5 kΩ pull-up resistor state loaded from Option Bytes at startup."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum UsbdPullup {
        #[doc = "Built-in pull-up disabled — an external pull-up is required on the USB data line."]
        DISABLED = 0x0,
        #[doc = "Built-in pull-up enabled — no external pull-up needed."]
        ENABLED = 0x01,
    }
    impl UsbdPullup {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbdPullup {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbdPullup {
        #[inline(always)]
        fn from(val: u8) -> UsbdPullup {
            UsbdPullup::from_bits(val)
        }
    }
    impl From<UsbdPullup> for u8 {
        #[inline(always)]
        fn from(val: UsbdPullup) -> u8 {
            UsbdPullup::to_bits(val)
        }
    }
    #[doc = "USBD bus speed mode loaded from Option Bytes at startup."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum UsbdSpeed {
        #[doc = "USB full-speed mode (12 Mbps)."]
        FULL_SPEED = 0x0,
        #[doc = "USB low-speed mode (1.5 Mbps)."]
        LOW_SPEED = 0x01,
    }
    impl UsbdSpeed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UsbdSpeed {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UsbdSpeed {
        #[inline(always)]
        fn from(val: u8) -> UsbdSpeed {
            UsbdSpeed::from_bits(val)
        }
    }
    impl From<UsbdSpeed> for u8 {
        #[inline(always)]
        fn from(val: UsbdSpeed) -> u8 {
            UsbdSpeed::to_bits(val)
        }
    }
}
