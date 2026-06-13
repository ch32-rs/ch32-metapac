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
        pub const fn page_pg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming."]
        #[inline(always)]
        pub fn set_page_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast erase."]
        #[inline(always)]
        pub const fn page_er(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast erase."]
        #[inline(always)]
        pub fn set_page_er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Block Erase 32K."]
        #[inline(always)]
        pub const fn ber32(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Block Erase 32K."]
        #[inline(always)]
        pub fn set_ber32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Block Erase 64K."]
        #[inline(always)]
        pub const fn ber64(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Block Erase 64K."]
        #[inline(always)]
        pub fn set_ber64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Page Programming Start."]
        #[inline(always)]
        pub const fn pgstart(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page Programming Start."]
        #[inline(always)]
        pub fn set_pgstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Flash Enhance read mode."]
        #[inline(always)]
        pub const fn rsenact(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Flash Enhance read mode."]
        #[inline(always)]
        pub fn set_rsenact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash Enhance read mode."]
        #[inline(always)]
        pub const fn enhancemode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Enhance read mode."]
        #[inline(always)]
        pub fn set_enhancemode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flash SCK mode."]
        #[inline(always)]
        pub const fn sckmode(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Flash SCK mode."]
        #[inline(always)]
        pub fn set_sckmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Undocumented; OR with PAGE_PG and PGSTART to program the BOOT region (0x1FFF8000). Guessed name."]
        #[inline(always)]
        pub const fn btpg(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Undocumented; OR with PAGE_PG and PGSTART to program the BOOT region (0x1FFF8000). Guessed name."]
        #[inline(always)]
        pub fn set_btpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Undocumented; OR with PER (not PAGE_ER) for 4 KB erase of the BOOT region (0x1FFF8000). Guessed name."]
        #[inline(always)]
        pub const fn bter(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Undocumented; OR with PER (not PAGE_ER) for 4 KB erase of the BOOT region (0x1FFF8000). Guessed name."]
        #[inline(always)]
        pub fn set_bter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
        pub const fn optkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Option byte key."]
        #[inline(always)]
        pub fn set_optkey(&mut self, val: u32) {
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
        #[doc = "SRAM/CODE allocation currently in effect. Mirrors the upper two bits of OB.USER.RAM_CODE; the variant table is chip-specific (V2 D8/D8W vs V3 high-memory). On V3 parts the 110/111 OB encodings are not distinguishable here — read OB.USER.RAM_CODE directly to tell them apart."]
        #[inline(always)]
        pub const fn sram_code_mode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SRAM/CODE allocation currently in effect. Mirrors the upper two bits of OB.USER.RAM_CODE; the variant table is chip-specific (V2 D8/D8W vs V3 high-memory). On V3 parts the 110/111 OB encodings are not distinguishable here — read OB.USER.RAM_CODE directly to tell them apart."]
        #[inline(always)]
        pub fn set_sram_code_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
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
        #[doc = "Quick page programming."]
        #[inline(always)]
        pub const fn wr_bsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Quick page programming."]
        #[inline(always)]
        pub fn set_wr_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "Enhance mode start."]
        #[inline(always)]
        pub const fn enhance_mod_sta(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enhance mode start."]
        #[inline(always)]
        pub fn set_enhance_mod_sta(&mut self, val: bool) {
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
