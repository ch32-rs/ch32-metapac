#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USBPD configuration."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbpd {
    ptr: *mut u8,
}
unsafe impl Send for Usbpd {}
unsafe impl Sync for Usbpd {}
impl Usbpd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PD interrupt enable register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "BMC sampling clock counter."]
    #[inline(always)]
    pub const fn bmc_clk_cnt(self) -> crate::common::Reg<regs::BmcClkCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "PD Send and receive enable register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SOP port selection register."]
    #[inline(always)]
    pub const fn tx_sel(self) -> crate::common::Reg<regs::TxSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "PD send length register."]
    #[inline(always)]
    pub const fn bmc_tx_sz(self) -> crate::common::Reg<regs::BmcTxSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "DMA cache data register."]
    #[inline(always)]
    pub const fn data_buf(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PD interrupt flag register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "Byte counter."]
    #[inline(always)]
    pub const fn bmc_byte_cnt(self) -> crate::common::Reg<regs::BmcByteCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "CC port control register."]
    #[inline(always)]
    pub const fn port_cc(self, n: usize) -> crate::common::Reg<regs::PortCc, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 1usize) as _) }
    }
    #[doc = "PD buffer start address register."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Byte counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcByteCnt(pub u16);
    impl BmcByteCnt {
        #[doc = "BMC_BYTE_CNT value."]
        #[inline(always)]
        pub const fn bmc_byte_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "BMC_BYTE_CNT value."]
        #[inline(always)]
        pub fn set_bmc_byte_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcByteCnt {
        #[inline(always)]
        fn default() -> BmcByteCnt {
            BmcByteCnt(0)
        }
    }
    #[doc = "BMC sampling clock counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcClkCnt(pub u16);
    impl BmcClkCnt {
        #[doc = "R/T counter."]
        #[inline(always)]
        pub const fn bmc_clk_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "R/T counter."]
        #[inline(always)]
        pub fn set_bmc_clk_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcClkCnt {
        #[inline(always)]
        fn default() -> BmcClkCnt {
            BmcClkCnt(0)
        }
    }
    #[doc = "PD send length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcTxSz(pub u16);
    impl BmcTxSz {
        #[doc = "BMC_TX_SZ value."]
        #[inline(always)]
        pub const fn bmc_tx_sz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "BMC_TX_SZ value."]
        #[inline(always)]
        pub fn set_bmc_tx_sz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcTxSz {
        #[inline(always)]
        fn default() -> BmcTxSz {
            BmcTxSz(0)
        }
    }
    #[doc = "PD interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u16);
    impl Config {
        #[doc = "PIN filtering enabled."]
        #[inline(always)]
        pub const fn cc_filter(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PIN filtering enabled."]
        #[inline(always)]
        pub fn set_cc_filter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "PD ITClear."]
        #[inline(always)]
        pub const fn pd_all_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PD ITClear."]
        #[inline(always)]
        pub fn set_pd_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "PD Commutation port."]
        #[inline(always)]
        pub const fn cc_sel(&self) -> super::vals::CcSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::CcSel::from_bits(val as u8)
        }
        #[doc = "PD Commutation port."]
        #[inline(always)]
        pub fn set_cc_sel(&mut self, val: super::vals::CcSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
        }
        #[doc = "PD DMA Enable."]
        #[inline(always)]
        pub const fn pd_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PD DMA Enable."]
        #[inline(always)]
        pub fn set_pd_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "PD RST Enable."]
        #[inline(always)]
        pub const fn pd_rst_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PD RST Enable."]
        #[inline(always)]
        pub fn set_pd_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "wakeup polarity."]
        #[inline(always)]
        pub const fn wake_polar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup polarity."]
        #[inline(always)]
        pub fn set_wake_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "IO Enable."]
        #[inline(always)]
        pub const fn ie_pd_io(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IO Enable."]
        #[inline(always)]
        pub fn set_ie_pd_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "bit interrupt Enable."]
        #[inline(always)]
        pub const fn ie_rx_bit(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "bit interrupt Enable."]
        #[inline(always)]
        pub fn set_ie_rx_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Receive byte register."]
        #[inline(always)]
        pub const fn ie_rx_byte(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Receive byte register."]
        #[inline(always)]
        pub fn set_ie_rx_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Receive complete register."]
        #[inline(always)]
        pub const fn ie_rx_act(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Receive complete register."]
        #[inline(always)]
        pub fn set_ie_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Receive complete rst register."]
        #[inline(always)]
        pub const fn ie_rx_reset(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive complete rst register."]
        #[inline(always)]
        pub fn set_ie_rx_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "transfer complete register."]
        #[inline(always)]
        pub const fn ie_tx_end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "transfer complete register."]
        #[inline(always)]
        pub fn set_ie_tx_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    #[doc = "PD Send and receive enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u8);
    impl Control {
        #[doc = "PD_TX_EN value."]
        #[inline(always)]
        pub const fn pd_tx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PD_TX_EN value."]
        #[inline(always)]
        pub fn set_pd_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "BMC_START value."]
        #[inline(always)]
        pub const fn bmc_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BMC_START value."]
        #[inline(always)]
        pub fn set_bmc_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PD receive status identification."]
        #[inline(always)]
        pub const fn rx_start(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "PD receive status identification."]
        #[inline(always)]
        pub fn set_rx_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u8) & 0x07) << 2usize);
        }
        #[doc = "DATA_FLAG value."]
        #[inline(always)]
        pub const fn data_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DATA_FLAG value."]
        #[inline(always)]
        pub fn set_data_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "TX_BIT_BACK value."]
        #[inline(always)]
        pub const fn tx_bit_back(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX_BIT_BACK value."]
        #[inline(always)]
        pub fn set_tx_bit_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "BMC_BYTE_HI value."]
        #[inline(always)]
        pub const fn bmc_byte_hi(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BMC_BYTE_HI value."]
        #[inline(always)]
        pub fn set_bmc_byte_hi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    #[doc = "CC port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCc(pub u8);
    impl PortCc {
        #[doc = "CC1 output of the voltage comparator."]
        #[inline(always)]
        pub const fn pa_cc_ai(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 output of the voltage comparator."]
        #[inline(always)]
        pub fn set_pa_cc_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "CC1 port pull-down resistor enable."]
        #[inline(always)]
        pub const fn cc_pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 port pull-down resistor enable."]
        #[inline(always)]
        pub fn set_cc_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "CC1 port pull-up current selection."]
        #[inline(always)]
        pub const fn cc_pu(&self) -> super::vals::PortCcPu {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::PortCcPu::from_bits(val as u8)
        }
        #[doc = "CC1 port pull-up current selection."]
        #[inline(always)]
        pub fn set_cc_pu(&mut self, val: super::vals::PortCcPu) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u8) & 0x03) << 2usize);
        }
        #[doc = "CC1 port output of the low voltage."]
        #[inline(always)]
        pub const fn cc_lve(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 port output of the low voltage."]
        #[inline(always)]
        pub fn set_cc_lve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "CC1 voltage comparator enable and voltage comparator reference voltage. (CC_CE and CC_CVS)"]
        #[inline(always)]
        pub const fn cc_ce(&self) -> super::vals::PortCcCe {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::PortCcCe::from_bits(val as u8)
        }
        #[doc = "CC1 voltage comparator enable and voltage comparator reference voltage. (CC_CE and CC_CVS)"]
        #[inline(always)]
        pub fn set_cc_ce(&mut self, val: super::vals::PortCcCe) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u8) & 0x07) << 5usize);
        }
    }
    impl Default for PortCc {
        #[inline(always)]
        fn default() -> PortCc {
            PortCc(0)
        }
    }
    #[doc = "PD interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "BMC_AUX value."]
        #[inline(always)]
        pub const fn bmc_aux(&self) -> super::vals::BmcAux {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::BmcAux::from_bits(val as u8)
        }
        #[doc = "BMC_AUX value."]
        #[inline(always)]
        pub fn set_bmc_aux(&mut self, val: super::vals::BmcAux) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
        }
        #[doc = "BUF_ERR value."]
        #[inline(always)]
        pub const fn buf_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BUF_ERR value."]
        #[inline(always)]
        pub fn set_buf_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF_RX_BIT value."]
        #[inline(always)]
        pub const fn if_rx_bit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_BIT value."]
        #[inline(always)]
        pub fn set_if_rx_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "IF_RX_BYTE value."]
        #[inline(always)]
        pub const fn if_rx_byte(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_BYTE value."]
        #[inline(always)]
        pub fn set_if_rx_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "IF_RX_ACT value."]
        #[inline(always)]
        pub const fn if_rx_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_ACT value."]
        #[inline(always)]
        pub fn set_if_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "IF_RX_RESET value."]
        #[inline(always)]
        pub const fn if_rx_reset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_RESET value."]
        #[inline(always)]
        pub fn set_if_rx_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "IF_TX_END value."]
        #[inline(always)]
        pub const fn if_tx_end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IF_TX_END value."]
        #[inline(always)]
        pub fn set_if_tx_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "SOP port selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxSel(pub u8);
    impl TxSel {
        #[doc = "K-CODE1 type selection."]
        #[inline(always)]
        pub const fn tx_sel1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "K-CODE1 type selection."]
        #[inline(always)]
        pub fn set_tx_sel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "K-CODE2 type selection."]
        #[inline(always)]
        pub const fn tx_sel2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "K-CODE2 type selection."]
        #[inline(always)]
        pub fn set_tx_sel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "K-CODE3 type selection."]
        #[inline(always)]
        pub const fn tx_sel3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "K-CODE3 type selection."]
        #[inline(always)]
        pub fn set_tx_sel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "K-CODE4 type selection."]
        #[inline(always)]
        pub const fn tx_sel4(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "K-CODE4 type selection."]
        #[inline(always)]
        pub fn set_tx_sel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for TxSel {
        #[inline(always)]
        fn default() -> TxSel {
            TxSel(0)
        }
    }
}
pub mod vals {
    #[doc = "PD status after receive."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BmcAux {
        #[doc = "BMC_AUX0 value."]
        NONE = 0x0,
        #[doc = "SOP, aka SOP0"]
        SOP0 = 0x01,
        #[doc = "SOP', aka SOP1 or Hard Reset"]
        SOP1 = 0x02,
        #[doc = "SOP'', aka SOP2 or Cable Resed"]
        SOP2 = 0x03,
    }
    impl BmcAux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BmcAux {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BmcAux {
        #[inline(always)]
        fn from(val: u8) -> BmcAux {
            BmcAux::from_bits(val)
        }
    }
    impl From<BmcAux> for u8 {
        #[inline(always)]
        fn from(val: BmcAux) -> u8 {
            BmcAux::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcSel {
        #[doc = "Select CC1."]
        CC1 = 0x0,
        #[doc = "Select CC2."]
        CC2 = 0x01,
        #[doc = "Select CC3."]
        CC3 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcSel {
        #[inline(always)]
        fn from(val: u8) -> CcSel {
            CcSel::from_bits(val)
        }
    }
    impl From<CcSel> for u8 {
        #[inline(always)]
        fn from(val: CcSel) -> u8 {
            CcSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PortCcCe {
        #[doc = "No pull up current."]
        CLOSED = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "0.55V"]
        V0_55 = 0x04,
        #[doc = "0.22V"]
        V0_22 = 0x05,
        #[doc = "0.66V"]
        V0_66 = 0x06,
        #[doc = "1.23V"]
        V1_23 = 0x07,
    }
    impl PortCcCe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortCcCe {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortCcCe {
        #[inline(always)]
        fn from(val: u8) -> PortCcCe {
            PortCcCe::from_bits(val)
        }
    }
    impl From<PortCcCe> for u8 {
        #[inline(always)]
        fn from(val: PortCcCe) -> u8 {
            PortCcCe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PortCcPu {
        #[doc = "No pull up current."]
        CLOSED = 0x0,
        #[doc = "330uA."]
        UA330 = 0x01,
        #[doc = "180uA."]
        UA180 = 0x02,
        #[doc = "80uA."]
        UA80 = 0x03,
    }
    impl PortCcPu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortCcPu {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortCcPu {
        #[inline(always)]
        fn from(val: u8) -> PortCcPu {
            PortCcPu::from_bits(val)
        }
    }
    impl From<PortCcPu> for u8 {
        #[inline(always)]
        fn from(val: PortCcPu) -> u8 {
            PortCcPu::to_bits(val)
        }
    }
}
