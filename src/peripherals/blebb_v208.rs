#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BLE Baseband — link-layer CTRL/GO/MODE/CFG, IRQ STATUS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBb {
    ptr: *mut u8,
}
unsafe impl Send for BleBb {}
unsafe impl Sync for BleBb {}
impl BleBb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Baseband control + GO strobes (bit23/bit28 enables, bits\\[8:7\\]
analog gate)."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Baseband mode register (default 0x00090083)."]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Baseband CFG (bits\\[30:25\\]
PHY mode = rf_flag, base 0x80010EC8)."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Baseband timing (default 0x000001D0 = 464)."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "IRQ status register (W1C). Read=live status; write 1 to clear."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "BLE_BB CFG — PHY mode + base configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Base config bits\\[24:0\\]
(default 0x010EC8)."]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Base config bits\\[24:0\\]
(default 0x010EC8)."]
        #[inline(always)]
        pub fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
        #[doc = "PHY mode flag (BB_RF_FLAG_1M = 0x09; bit0=1M PHY enable)."]
        #[inline(always)]
        pub const fn rf_flag(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[doc = "PHY mode flag (BB_RF_FLAG_1M = 0x09; bit0=1M PHY enable)."]
        #[inline(always)]
        pub fn set_rf_flag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
        #[doc = "Hardware reserved bit (always set, base=0x80010EC8)."]
        #[inline(always)]
        pub const fn hw_reserved(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware reserved bit (always set, base=0x80010EC8)."]
        #[inline(always)]
        pub fn set_hw_reserved(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    #[doc = "BLE_BB CTRL — baseband enable + analog gate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Analog clear bits\\[8:7\\]
cleared pre-init, bit7 set post-init."]
        #[inline(always)]
        pub const fn analog_clear(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Analog clear bits\\[8:7\\]
cleared pre-init, bit7 set post-init."]
        #[inline(always)]
        pub fn set_analog_clear(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Analog gate (BLE_RegInit pre-init clears bit13 sets bit12)."]
        #[inline(always)]
        pub const fn analog_gate(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Analog gate (BLE_RegInit pre-init clears bit13 sets bit12)."]
        #[inline(always)]
        pub fn set_analog_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Baseband enable (permanent, set in bb_dev_init)."]
        #[inline(always)]
        pub const fn bb_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Baseband enable (permanent, set in bb_dev_init)."]
        #[inline(always)]
        pub fn set_bb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Hardware enable (permanent, set in bb_dev_init)."]
        #[inline(always)]
        pub const fn hw_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware enable (permanent, set in bb_dev_init)."]
        #[inline(always)]
        pub fn set_hw_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "BLE_BB IRQ status (W1C — write 1 to clear)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "IRQ status bit4 — `.L4` cleanup; sets gBleIPPara\\[4\\]=1 (re-arm)."]
        #[inline(always)]
        pub const fn bit4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ status bit4 — `.L4` cleanup; sets gBleIPPara\\[4\\]=1 (re-arm)."]
        #[inline(always)]
        pub fn set_bit4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IRQ status bit5 — paired with bit6 in W1C mask 0x60 (PLL-related)."]
        #[inline(always)]
        pub const fn bit5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ status bit5 — paired with bit6 in W1C mask 0x60 (PLL-related)."]
        #[inline(always)]
        pub fn set_bit5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL-ready (triggers `.L6` TX advance path; W1C with bit5 via 0x60)."]
        #[inline(always)]
        pub const fn pll_ready(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL-ready (triggers `.L6` TX advance path; W1C with bit5 via 0x60)."]
        #[inline(always)]
        pub fn set_pll_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IRQ status bit7 — `.L8` cleanup; sets gBleIPPara\\[4\\]=1 (re-arm)."]
        #[inline(always)]
        pub const fn bit7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ status bit7 — `.L8` cleanup; sets gBleIPPara\\[4\\]=1 (re-arm)."]
        #[inline(always)]
        pub fn set_bit7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
}
