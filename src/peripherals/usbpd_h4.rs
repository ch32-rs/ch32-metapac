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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "BMC sampling clock counter."]
    #[inline(always)]
    pub const fn bmc_clk_cnt(self) -> crate::common::Reg<regs::BmcClkCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "PD Send and receive enable register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SOP port selection register."]
    #[inline(always)]
    pub const fn tx_sel(self) -> crate::common::Reg<regs::TxSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "PD send length register."]
    #[inline(always)]
    pub const fn bmc_tx_sz(self) -> crate::common::Reg<regs::BmcTxSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "DMA cache data register."]
    #[inline(always)]
    pub const fn data_buf(self) -> crate::common::Reg<regs::DataBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PD interrupt flag register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Byte counter."]
    #[inline(always)]
    pub const fn bmc_byte_cnt(self) -> crate::common::Reg<regs::BmcByteCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "CC1 port control register."]
    #[inline(always)]
    pub const fn port_cc1(self) -> crate::common::Reg<regs::PortCc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CC2 port control register."]
    #[inline(always)]
    pub const fn port_cc2(self) -> crate::common::Reg<regs::PortCc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "PD buffer start address register."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<regs::Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Byte counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcByteCnt(pub u16);
    impl BmcByteCnt {
        #[doc = "BMC_BYTE_CNT value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_byte_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "BMC_BYTE_CNT value."]
        #[inline(always)]
        pub const fn set_bmc_byte_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcByteCnt {
        #[inline(always)]
        fn default() -> BmcByteCnt {
            BmcByteCnt(0)
        }
    }
    impl core::fmt::Debug for BmcByteCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BmcByteCnt")
                .field("bmc_byte_cnt", &self.bmc_byte_cnt())
                .finish()
        }
    }
    #[doc = "BMC sampling clock counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcClkCnt(pub u16);
    impl BmcClkCnt {
        #[doc = "R/T counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_clk_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "R/T counter."]
        #[inline(always)]
        pub const fn set_bmc_clk_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcClkCnt {
        #[inline(always)]
        fn default() -> BmcClkCnt {
            BmcClkCnt(0)
        }
    }
    impl core::fmt::Debug for BmcClkCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BmcClkCnt")
                .field("bmc_clk_cnt", &self.bmc_clk_cnt())
                .finish()
        }
    }
    #[doc = "PD send length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BmcTxSz(pub u16);
    impl BmcTxSz {
        #[doc = "BMC_TX_SZ value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_tx_sz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "BMC_TX_SZ value."]
        #[inline(always)]
        pub const fn set_bmc_tx_sz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for BmcTxSz {
        #[inline(always)]
        fn default() -> BmcTxSz {
            BmcTxSz(0)
        }
    }
    impl core::fmt::Debug for BmcTxSz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BmcTxSz")
                .field("bmc_tx_sz", &self.bmc_tx_sz())
                .finish()
        }
    }
    #[doc = "PD interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u16);
    impl Config {
        #[doc = "control PD pin input filter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_filt_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "control PD pin input filter enable."]
        #[inline(always)]
        pub const fn set_pd_filt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "PD ITClear."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_all_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PD ITClear."]
        #[inline(always)]
        pub const fn set_pd_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "PD Commutation port."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PD Commutation port."]
        #[inline(always)]
        pub const fn set_cc_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "PD DMA Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_dma_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PD DMA Enable."]
        #[inline(always)]
        pub const fn set_pd_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "PD RST Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_rst_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PD RST Enable."]
        #[inline(always)]
        pub const fn set_pd_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "wakeup polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_polar(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup polarity."]
        #[inline(always)]
        pub const fn set_wake_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "TR conditioning of the CC LV waveform."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_tr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TR conditioning of the CC LV waveform."]
        #[inline(always)]
        pub const fn set_cc_tr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "CC bias current adjustment bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_inc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CC bias current adjustment bit."]
        #[inline(always)]
        pub const fn set_cc_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Continuously receive several bits of 0 indicator flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_multi_0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Continuously receive several bits of 0 indicator flag."]
        #[inline(always)]
        pub const fn set_rx_multi_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "IO Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_pd_io(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IO Enable."]
        #[inline(always)]
        pub const fn set_ie_pd_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "bit interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_rx_bit(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "bit interrupt Enable."]
        #[inline(always)]
        pub const fn set_ie_rx_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Receive byte register."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_rx_byte(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Receive byte register."]
        #[inline(always)]
        pub const fn set_ie_rx_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Receive complete register."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_rx_act(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Receive complete register."]
        #[inline(always)]
        pub const fn set_ie_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Receive complete rst register."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_rx_reset(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive complete rst register."]
        #[inline(always)]
        pub const fn set_ie_rx_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "transfer complete register."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_tx_end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "transfer complete register."]
        #[inline(always)]
        pub const fn set_ie_tx_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("pd_filt_en", &self.pd_filt_en())
                .field("pd_all_clr", &self.pd_all_clr())
                .field("cc_sel", &self.cc_sel())
                .field("pd_dma_en", &self.pd_dma_en())
                .field("pd_rst_en", &self.pd_rst_en())
                .field("wake_polar", &self.wake_polar())
                .field("cc_tr", &self.cc_tr())
                .field("cc_inc", &self.cc_inc())
                .field("rx_multi_0", &self.rx_multi_0())
                .field("ie_pd_io", &self.ie_pd_io())
                .field("ie_rx_bit", &self.ie_rx_bit())
                .field("ie_rx_byte", &self.ie_rx_byte())
                .field("ie_rx_act", &self.ie_rx_act())
                .field("ie_rx_reset", &self.ie_rx_reset())
                .field("ie_tx_end", &self.ie_tx_end())
                .finish()
        }
    }
    #[doc = "PD Send and receive enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u8);
    impl Control {
        #[doc = "PD_TX_EN value."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_tx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PD_TX_EN value."]
        #[inline(always)]
        pub const fn set_pd_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "BMC_START value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BMC_START value."]
        #[inline(always)]
        pub const fn set_bmc_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PD receive status identification."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_state(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "PD receive status identification."]
        #[inline(always)]
        pub const fn set_rx_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u8) & 0x07) << 2usize);
        }
        #[doc = "DATA_FLAG value."]
        #[must_use]
        #[inline(always)]
        pub const fn data_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DATA_FLAG value."]
        #[inline(always)]
        pub const fn set_data_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "TX_BIT_BACK value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_bit_back(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX_BIT_BACK value."]
        #[inline(always)]
        pub const fn set_tx_bit_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "BMC_BYTE_HI value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_byte_hi(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BMC_BYTE_HI value."]
        #[inline(always)]
        pub const fn set_bmc_byte_hi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    impl core::fmt::Debug for Control {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Control")
                .field("pd_tx_en", &self.pd_tx_en())
                .field("bmc_start", &self.bmc_start())
                .field("rx_state", &self.rx_state())
                .field("data_flag", &self.data_flag())
                .field("tx_bit_back", &self.tx_bit_back())
                .field("bmc_byte_hi", &self.bmc_byte_hi())
                .finish()
        }
    }
    #[doc = "DMA cache data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataBuf(pub u8);
    impl DataBuf {
        #[doc = "DATA_BUF value."]
        #[must_use]
        #[inline(always)]
        pub const fn data_buf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DATA_BUF value."]
        #[inline(always)]
        pub const fn set_data_buf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DataBuf {
        #[inline(always)]
        fn default() -> DataBuf {
            DataBuf(0)
        }
    }
    impl core::fmt::Debug for DataBuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DataBuf")
                .field("data_buf", &self.data_buf())
                .finish()
        }
    }
    #[doc = "PD buffer start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dma(pub u32);
    impl Dma {
        #[doc = "USBPD_DMA_ADDR value."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_dma_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "USBPD_DMA_ADDR value."]
        #[inline(always)]
        pub const fn set_usbpd_dma_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
    }
    impl Default for Dma {
        #[inline(always)]
        fn default() -> Dma {
            Dma(0)
        }
    }
    impl core::fmt::Debug for Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dma")
                .field("usbpd_dma_addr", &self.usbpd_dma_addr())
                .finish()
        }
    }
    #[doc = "CCn port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCc(pub u16);
    impl PortCc {
        #[doc = "CCn port comparator analog input."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_cc_ai(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCn port comparator analog input."]
        #[inline(always)]
        pub const fn set_pa_cc_ai(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "CCn port pull-down resistor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CCn port pull-down resistor enable."]
        #[inline(always)]
        pub const fn set_cc_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "CCn port pull-up current."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_pu(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "CCn port pull-up current."]
        #[inline(always)]
        pub const fn set_cc_pu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[doc = "CCn port output of the low voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_lve(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CCn port output of the low voltage."]
        #[inline(always)]
        pub const fn set_cc_lve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "CCn voltage comparator enable and reference voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn cc_ce(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "CCn voltage comparator enable and reference voltage."]
        #[inline(always)]
        pub const fn set_cc_ce(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u16) & 0x07) << 5usize);
        }
    }
    impl Default for PortCc {
        #[inline(always)]
        fn default() -> PortCc {
            PortCc(0)
        }
    }
    impl core::fmt::Debug for PortCc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortCc")
                .field("pa_cc_ai", &self.pa_cc_ai())
                .field("cc_pd", &self.cc_pd())
                .field("cc_pu", &self.cc_pu())
                .field("cc_lve", &self.cc_lve())
                .field("cc_ce", &self.cc_ce())
                .finish()
        }
    }
    #[doc = "PD interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "BMC_AUX value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmc_aux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "BMC_AUX value."]
        #[inline(always)]
        pub const fn set_bmc_aux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "BUF_ERR value."]
        #[must_use]
        #[inline(always)]
        pub const fn buf_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BUF_ERR value."]
        #[inline(always)]
        pub const fn set_buf_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF_RX_BIT value."]
        #[must_use]
        #[inline(always)]
        pub const fn if_rx_bit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_BIT value."]
        #[inline(always)]
        pub const fn set_if_rx_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "IF_RX_BYTE value."]
        #[must_use]
        #[inline(always)]
        pub const fn if_rx_byte(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_BYTE value."]
        #[inline(always)]
        pub const fn set_if_rx_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "IF_RX_ACT value."]
        #[must_use]
        #[inline(always)]
        pub const fn if_rx_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_ACT value."]
        #[inline(always)]
        pub const fn set_if_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "IF_RX_RESET value."]
        #[must_use]
        #[inline(always)]
        pub const fn if_rx_reset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IF_RX_RESET value."]
        #[inline(always)]
        pub const fn set_if_rx_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "IF_TX_END value."]
        #[must_use]
        #[inline(always)]
        pub const fn if_tx_end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IF_TX_END value."]
        #[inline(always)]
        pub const fn set_if_tx_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("bmc_aux", &self.bmc_aux())
                .field("buf_err", &self.buf_err())
                .field("if_rx_bit", &self.if_rx_bit())
                .field("if_rx_byte", &self.if_rx_byte())
                .field("if_rx_act", &self.if_rx_act())
                .field("if_rx_reset", &self.if_rx_reset())
                .field("if_tx_end", &self.if_tx_end())
                .finish()
        }
    }
    #[doc = "SOP port selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxSel(pub u8);
    impl TxSel {
        #[doc = "TX_SEL1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_sel1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_SEL1 value."]
        #[inline(always)]
        pub const fn set_tx_sel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TX_SEL2 value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_sel2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "TX_SEL2 value."]
        #[inline(always)]
        pub const fn set_tx_sel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "TX_SEL3 value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_sel3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "TX_SEL3 value."]
        #[inline(always)]
        pub const fn set_tx_sel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "TX_SEL4 value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_sel4(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TX_SEL4 value."]
        #[inline(always)]
        pub const fn set_tx_sel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for TxSel {
        #[inline(always)]
        fn default() -> TxSel {
            TxSel(0)
        }
    }
    impl core::fmt::Debug for TxSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxSel")
                .field("tx_sel1", &self.tx_sel1())
                .field("tx_sel2", &self.tx_sel2())
                .field("tx_sel3", &self.tx_sel3())
                .field("tx_sel4", &self.tx_sel4())
                .finish()
        }
    }
}
