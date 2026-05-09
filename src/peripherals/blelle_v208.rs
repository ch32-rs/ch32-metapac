#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BLE Link-Layer Engine — timing, state machine, IRQ, DMA buffer ptr."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleLle {
    ptr: *mut u8,
}
unsafe impl Send for BleLle {}
unsafe impl Sync for BleLle {}
impl BleLle {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Link-layer CTRL — channel field bits\\[5:0\\], whitening bit6, mode bits\\[8:7\\], rate bits\\[13:12\\], BLE GO bit23."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CRC seed (BLE adv default 0x555555)."]
    #[inline(always)]
    pub const fn crc_init(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Access address / IRQ status (W1C). Read=live status; write 1 to clear, or write ADV AA 0x8E89BED6 for advertising."]
    #[inline(always)]
    pub const fn access_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "IRQ mask (default 0xF00F = bits\\[15:12\\]
+ bits\\[3:0\\])."]
    #[inline(always)]
    pub const fn irq_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Timing slot 0 (default 140)."]
    #[inline(always)]
    pub const fn timing0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Link-layer state machine (108=Sleep, 93=ConnRxWait, 97=ConnTxPrep, 101=ConnAckWait, 105=ConnEventClosing, 107=SleepPrep)."]
    #[inline(always)]
    pub const fn state_machine(self) -> crate::common::Reg<regs::StateMachine, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Timing slot 2 (default 140)."]
    #[inline(always)]
    pub const fn timing2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Timing slot 3 (default 60)."]
    #[inline(always)]
    pub const fn timing3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Timing slot 4 (default 140)."]
    #[inline(always)]
    pub const fn timing4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Timing slot 5 (default 60)."]
    #[inline(always)]
    pub const fn timing5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Timing slot 6 (default 140)."]
    #[inline(always)]
    pub const fn timing6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Timing slot 7 (default 108)."]
    #[inline(always)]
    pub const fn timing7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Settle timer used during BLE_RegInit (93 during cal, 0 post)."]
    #[inline(always)]
    pub const fn settle(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Countdown timer for RFEND_WaitTune; written from gBleIPPara\\[16..19\\]
(=776 for ADV TX)."]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Active-scan offset (= gBleIPPara\\[20..23\\]
<< 1, set in `.L6` bit5 path)."]
    #[inline(always)]
    pub const fn scan_offset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "TX buffer base (BB+0x70 in adv.rs; written before ADV GO)."]
    #[inline(always)]
    pub const fn tx_buf_ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "DMA buffer base address (= gBleIPPara\\[36\\]
MEMAddr; required non-zero for LLE state machine to fire)."]
    #[inline(always)]
    pub const fn dma_buf(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "BLE_LLE CTRL — channel, whitening, mode, rate, GO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "BLE logical channel bits\\[5:0\\]
(37/38/39 for ADV, 0..36 for data)."]
        #[inline(always)]
        pub const fn channel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "BLE logical channel bits\\[5:0\\]
(37/38/39 for ADV, 0..36 for data)."]
        #[inline(always)]
        pub fn set_channel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Whitening enable bit6 (DTM uses 0; ADV may use 1)."]
        #[inline(always)]
        pub const fn whiten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Whitening enable bit6 (DTM uses 0; ADV may use 1)."]
        #[inline(always)]
        pub fn set_whiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TX/RX mode select bits\\[8:7\\]
(TX path sets bit8; RX sets bit8+other)."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "TX/RX mode select bits\\[8:7\\]
(TX path sets bit8; RX sets bit8+other)."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "PHY rate select bits\\[13:12\\]
(00=1Mbps; non-zero=2M/Coded)."]
        #[inline(always)]
        pub const fn rate(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "PHY rate select bits\\[13:12\\]
(00=1Mbps; non-zero=2M/Coded)."]
        #[inline(always)]
        pub fn set_rate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "BLE GO strobe bit23 (lui 0x800; set to fire TX/RX)."]
        #[inline(always)]
        pub const fn ble_go(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "BLE GO strobe bit23 (lui 0x800; set to fire TX/RX)."]
        #[inline(always)]
        pub fn set_ble_go(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "BLE_LLE state machine value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StateMachine(pub u32);
    impl StateMachine {
        #[doc = "8-bit state (108=Sleep default; see register description)."]
        #[inline(always)]
        pub const fn state(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit state (108=Sleep default; see register description)."]
        #[inline(always)]
        pub fn set_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for StateMachine {
        #[inline(always)]
        fn default() -> StateMachine {
            StateMachine(0)
        }
    }
}
