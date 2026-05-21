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
    #[doc = "Boot mode key register (RM 46.4.10). Write KEY1=0x45670123 then KEY2=0xCDEF89AB to unlock boot mode switch."]
    #[inline(always)]
    pub const fn boot_modekeyr(self) -> crate::common::Reg<regs::BootModekeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH configuration register (RM 46.4.11)."]
    #[inline(always)]
    pub const fn cfgr0(self) -> crate::common::Reg<regs::Cfgr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Actlr(pub u32);
    impl Actlr {
        #[doc = "Waiting Status."]
        #[inline(always)]
        pub const fn sck_cfg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Waiting Status."]
        #[inline(always)]
        pub fn set_sck_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Enhanced Status."]
        #[inline(always)]
        pub const fn enhance_status(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Status."]
        #[inline(always)]
        pub fn set_enhance_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "flash enhanced read mode control bit."]
        #[inline(always)]
        pub const fn ehmod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "flash enhanced read mode control bit."]
        #[inline(always)]
        pub fn set_ehmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FLASH Low Power Mode Control Bit."]
        #[inline(always)]
        pub const fn flash_lp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Low Power Mode Control Bit."]
        #[inline(always)]
        pub fn set_flash_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "flash ready indicator."]
        #[inline(always)]
        pub const fn flash_ready(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "flash ready indicator."]
        #[inline(always)]
        pub fn set_flash_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FLASH Low Power Status Indicator Bits."]
        #[inline(always)]
        pub const fn flash_st(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Low Power Status Indicator Bits."]
        #[inline(always)]
        pub fn set_flash_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
    #[doc = "Boot mode key register. Write KEY1=0x45670123, then KEY2=0xCDEF89AB to unlock the boot mode switch (RM 46.4.10)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootModekeyr(pub u32);
    impl BootModekeyr {
        #[doc = "BOOT unlock key."]
        #[inline(always)]
        pub const fn bootkeyr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BOOT unlock key."]
        #[inline(always)]
        pub fn set_bootkeyr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BootModekeyr {
        #[inline(always)]
        fn default() -> BootModekeyr {
            BootModekeyr(0)
        }
    }
    #[doc = "FLASH configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr0(pub u32);
    impl Cfgr0 {
        #[doc = "FLASH Mode status."]
        #[inline(always)]
        pub const fn dbmode(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Mode status."]
        #[inline(always)]
        pub fn set_dbmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Cfgr0 {
        #[inline(always)]
        fn default() -> Cfgr0 {
            Cfgr0(0)
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
        pub const fn ftpg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming."]
        #[inline(always)]
        pub fn set_ftpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Block Erase."]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Block Erase."]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Page Programming Start."]
        #[inline(always)]
        pub const fn pgstrt(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Page Programming Start."]
        #[inline(always)]
        pub fn set_pgstrt(&mut self, val: bool) {
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
    #[doc = "Option byte register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Option byte error."]
        #[inline(always)]
        pub const fn oberr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte error."]
        #[inline(always)]
        pub fn set_oberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read protection."]
        #[inline(always)]
        pub const fn rdprt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read protection."]
        #[inline(always)]
        pub fn set_rdprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IWDG_SW."]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG_SW."]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fixed to 11."]
        #[inline(always)]
        pub const fn fix_11(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Fixed to 11."]
        #[inline(always)]
        pub fn set_fix_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[doc = "Data byte 1."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 1."]
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
        #[doc = "Quick page programming."]
        #[inline(always)]
        pub const fn wrbsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Quick page programming."]
        #[inline(always)]
        pub fn set_wrbsy(&mut self, val: bool) {
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
        #[doc = "BOOT_AVA."]
        #[inline(always)]
        pub const fn boot_ava(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_AVA."]
        #[inline(always)]
        pub fn set_boot_ava(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BOOT_STATUS."]
        #[inline(always)]
        pub const fn boot_status(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_STATUS."]
        #[inline(always)]
        pub fn set_boot_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "BOOT_MODE."]
        #[inline(always)]
        pub const fn boot_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_MODE."]
        #[inline(always)]
        pub fn set_boot_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "BOOT_LOCK."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT_LOCK."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
