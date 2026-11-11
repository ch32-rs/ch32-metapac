#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Receive (OUT) registers of one device endpoint, 1 to 7. USBSS_EP_RX_TypeDef in the EVT header."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpRx {
    ptr: *mut u8,
}
unsafe impl Send for EpRx {}
unsafe impl Sync for EpRx {}
impl EpRx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Endpoint n configuration register (R8_UEPn_RX_CFG)."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::EpRxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Endpoint n control register (R8_UEPn_RX_CR)."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::EpRxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Endpoint n sequence number register (R8_UEPn_RX_SEQ)."]
    #[inline(always)]
    pub const fn seq(self) -> crate::common::Reg<regs::EpRxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Endpoint n status register (R8_UEPn_RX_ST)."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::EpRxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Endpoint n CHAIN control register (R8_UEPn_RX_CHAIN_CR)."]
    #[inline(always)]
    pub const fn chain_cr(self) -> crate::common::Reg<regs::EpRxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Endpoint n CHAIN status register (R8_UEPn_RX_CHAIN_ST)."]
    #[inline(always)]
    pub const fn chain_st(self) -> crate::common::Reg<regs::EpRxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Length of the last packet the CHAIN received (R16_UEPn_RX_CHAIN_LEN)."]
    #[inline(always)]
    pub const fn chain_len(self) -> crate::common::Reg<regs::EpRxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Number of DPPs the CHAIN can receive (R8_UEPn_RX_CHAIN_MAX_NUMP)."]
    #[inline(always)]
    pub const fn chain_max_nump(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Number of DPPs the CHAIN has received (R8_UEPn_RX_CHAIN_NUMP)."]
    #[inline(always)]
    pub const fn chain_nump(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Offset of a DPP in the CHAIN, the distance between two packets in the buffer (R16_UEPn_RX_DMA_OFS). Resets to 0x0400."]
    #[inline(always)]
    pub const fn dma_ofs(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Normal mode: DMA start address of the CHAIN. FIFO mode: FIFO start and end addresses (R32_UEPn_RX_DMA)."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Transmit (IN) registers of one device endpoint, 1 to 7. USBSS_EP_TX_TypeDef in the EVT header."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpTx {
    ptr: *mut u8,
}
unsafe impl Send for EpTx {}
unsafe impl Sync for EpTx {}
impl EpTx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Endpoint n configuration register (R8_UEPn_TX_CFG)."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::EpTxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Endpoint n control register (R8_UEPn_TX_CR)."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::EpTxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Endpoint n sequence number register (R8_UEPn_TX_SEQ)."]
    #[inline(always)]
    pub const fn seq(self) -> crate::common::Reg<regs::EpTxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Endpoint n status register (R8_UEPn_TX_ST)."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::EpTxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Endpoint n CHAIN control register (R8_UEPn_TX_CHAIN_CR)."]
    #[inline(always)]
    pub const fn chain_cr(self) -> crate::common::Reg<regs::EpTxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Endpoint n CHAIN status register (R8_UEPn_TX_CHAIN_ST)."]
    #[inline(always)]
    pub const fn chain_st(self) -> crate::common::Reg<regs::EpTxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Length of the last packet the CHAIN sends (R16_UEPn_TX_CHAIN_LEN)."]
    #[inline(always)]
    pub const fn chain_len(self) -> crate::common::Reg<regs::EpTxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Number of DPPs the CHAIN is to send (R8_UEPn_TX_CHAIN_EXP_NUMP)."]
    #[inline(always)]
    pub const fn chain_exp_nump(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Number of DPPs the CHAIN has sent (R8_UEPn_TX_CHAIN_NUMP)."]
    #[inline(always)]
    pub const fn chain_nump(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Offset of a DPP in the CHAIN, the distance between two packets in the buffer (R16_UEPn_TX_DMA_OFS). Resets to 0x0400."]
    #[inline(always)]
    pub const fn dma_ofs(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Normal mode: DMA start address of the CHAIN. FIFO mode: FIFO start and end addresses (R32_UEPn_TX_DMA)."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "USB3.0 ultra-high speed host /device controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbss {
    ptr: *mut u8,
}
unsafe impl Send for Usbss {}
unsafe impl Sync for Usbss {}
impl Usbss {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LINK Configuration Register."]
    #[inline(always)]
    pub const fn link_cfg(self) -> crate::common::Reg<regs::LinkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LINK control registers."]
    #[inline(always)]
    pub const fn link_ctrl(self) -> crate::common::Reg<regs::LinkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LINK interrupt enable register."]
    #[inline(always)]
    pub const fn link_int_ctrl(self) -> crate::common::Reg<regs::LinkIntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LINK Interrupt Flag Register."]
    #[inline(always)]
    pub const fn link_int_flag(self) -> crate::common::Reg<regs::LinkIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LINK Status Register."]
    #[inline(always)]
    pub const fn link_status(self) -> crate::common::Reg<regs::LinkStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LINK ITP Timeout Mode Register."]
    #[inline(always)]
    pub const fn link_itp_pre(self) -> crate::common::Reg<regs::LinkItpPre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "LINK U2 Inactivity Timeout Counter Threshold Register."]
    #[inline(always)]
    pub const fn link_u2_inact_timer(
        self,
    ) -> crate::common::Reg<regs::LinkU2InactTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "U1 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u1_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120."]
    #[inline(always)]
    pub const fn link_u1_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "U2 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u2_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU2WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "U3 wakes up the LFPS validity duration register."]
    #[inline(always)]
    pub const fn link_u3_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn link_u3_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training."]
    #[inline(always)]
    pub const fn link_u1_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU1ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u2_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU2ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u3_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU3ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "LINK Synchronous Delay Register."]
    #[inline(always)]
    pub const fn link_iso_dly(self) -> crate::common::Reg<regs::LinkIsoDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port."]
    #[inline(always)]
    pub const fn link_tx_route(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Link Power Management Registers."]
    #[inline(always)]
    pub const fn link_lpm_cr(self) -> crate::common::Reg<regs::LinkLpmCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PORT_CAP Registers."]
    #[inline(always)]
    pub const fn link_lmp_port_cap(
        self,
    ) -> crate::common::Reg<regs::LinkLmpPortCap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Header packet of the last LMP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize + n * 4usize) as _) }
    }
    #[doc = "User-defined header packet to send, such as an LMP; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_tx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "USBSS Control Register."]
    #[inline(always)]
    pub const fn usbss_ctrl(self) -> crate::common::Reg<regs::UsbssCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "USBSS Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Interval Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp(self) -> crate::common::Reg<regs::Itp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interval Adaptive Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp_adj(self) -> crate::common::Reg<regs::ItpAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Header packet of the last DEV_NOTIF-TP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn tp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize + n * 4usize) as _) }
    }
    #[doc = "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`."]
    #[inline(always)]
    pub const fn phy_cfg_cr(self) -> crate::common::Reg<regs::PhyCfgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it."]
    #[inline(always)]
    pub const fn phy_cfg_dat(self) -> crate::common::Reg<regs::PhyCfgDat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
#[doc = "USBSS in device mode. Endpoint configuration / TX / RX registers (UEP* family). Offsets overlap with USBSS_HOST host-mode registers; chiptool's overlap check is intra-block only so the two views coexist via the extends mechanism."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbssDevice {
    ptr: *mut u8,
}
unsafe impl Send for UsbssDevice {}
unsafe impl Sync for UsbssDevice {}
impl UsbssDevice {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LINK Configuration Register."]
    #[inline(always)]
    pub const fn link_cfg(self) -> crate::common::Reg<regs::LinkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LINK control registers."]
    #[inline(always)]
    pub const fn link_ctrl(self) -> crate::common::Reg<regs::LinkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LINK interrupt enable register."]
    #[inline(always)]
    pub const fn link_int_ctrl(self) -> crate::common::Reg<regs::LinkIntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LINK Interrupt Flag Register."]
    #[inline(always)]
    pub const fn link_int_flag(self) -> crate::common::Reg<regs::LinkIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LINK Status Register."]
    #[inline(always)]
    pub const fn link_status(self) -> crate::common::Reg<regs::LinkStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LINK ITP Timeout Mode Register."]
    #[inline(always)]
    pub const fn link_itp_pre(self) -> crate::common::Reg<regs::LinkItpPre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "LINK U2 Inactivity Timeout Counter Threshold Register."]
    #[inline(always)]
    pub const fn link_u2_inact_timer(
        self,
    ) -> crate::common::Reg<regs::LinkU2InactTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "U1 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u1_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120."]
    #[inline(always)]
    pub const fn link_u1_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "U2 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u2_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU2WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "U3 wakes up the LFPS validity duration register."]
    #[inline(always)]
    pub const fn link_u3_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn link_u3_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training."]
    #[inline(always)]
    pub const fn link_u1_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU1ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u2_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU2ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u3_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU3ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "LINK Synchronous Delay Register."]
    #[inline(always)]
    pub const fn link_iso_dly(self) -> crate::common::Reg<regs::LinkIsoDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port."]
    #[inline(always)]
    pub const fn link_tx_route(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Link Power Management Registers."]
    #[inline(always)]
    pub const fn link_lpm_cr(self) -> crate::common::Reg<regs::LinkLpmCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PORT_CAP Registers."]
    #[inline(always)]
    pub const fn link_lmp_port_cap(
        self,
    ) -> crate::common::Reg<regs::LinkLmpPortCap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Header packet of the last LMP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize + n * 4usize) as _) }
    }
    #[doc = "User-defined header packet to send, such as an LMP; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_tx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "USBSS Control Register."]
    #[inline(always)]
    pub const fn usbss_ctrl(self) -> crate::common::Reg<regs::UsbssCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "USBSS Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Interval Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp(self) -> crate::common::Reg<regs::Itp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interval Adaptive Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp_adj(self) -> crate::common::Reg<regs::ItpAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Endpoint Sends Enable Register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Endpoint Receive Enable Register."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Endpoint 0 sends control registers."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Endpoint 0 receives control registers."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Start address of the endpoint 0 transmit buffer."]
    #[inline(always)]
    pub const fn uep0_tx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Start address of the endpoint 0 receive buffer."]
    #[inline(always)]
    pub const fn uep0_rx_dma(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Endpoint 0 transmit DMA offset. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn uep0_tx_dma_ofs(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Endpoint 0 receive DMA offset. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn uep0_rx_dma_ofs(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Header packet of the last DEV_NOTIF-TP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn tp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize + n * 4usize) as _) }
    }
    #[doc = "Transmit (IN) registers of endpoints 1 to 7. Index 0 is endpoint 1."]
    #[inline(always)]
    pub const fn ep_tx(self, n: usize) -> EpTx {
        assert!(n < 7usize);
        unsafe { EpTx::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 32usize) as _) }
    }
    #[doc = "Receive (OUT) registers of endpoints 1 to 7. Index 0 is endpoint 1."]
    #[inline(always)]
    pub const fn ep_rx(self, n: usize) -> EpRx {
        assert!(n < 7usize);
        unsafe { EpRx::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 32usize) as _) }
    }
    #[doc = "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`."]
    #[inline(always)]
    pub const fn phy_cfg_cr(self) -> crate::common::Reg<regs::PhyCfgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it."]
    #[inline(always)]
    pub const fn phy_cfg_dat(self) -> crate::common::Reg<regs::PhyCfgDat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
#[doc = "USBSS in host mode. UH_* / HOST_* registers. Offsets overlap with USBSS_DEVICE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbssHost {
    ptr: *mut u8,
}
unsafe impl Send for UsbssHost {}
unsafe impl Sync for UsbssHost {}
impl UsbssHost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LINK Configuration Register."]
    #[inline(always)]
    pub const fn link_cfg(self) -> crate::common::Reg<regs::LinkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LINK control registers."]
    #[inline(always)]
    pub const fn link_ctrl(self) -> crate::common::Reg<regs::LinkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LINK interrupt enable register."]
    #[inline(always)]
    pub const fn link_int_ctrl(self) -> crate::common::Reg<regs::LinkIntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LINK Interrupt Flag Register."]
    #[inline(always)]
    pub const fn link_int_flag(self) -> crate::common::Reg<regs::LinkIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LINK Status Register."]
    #[inline(always)]
    pub const fn link_status(self) -> crate::common::Reg<regs::LinkStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LINK ITP Timeout Mode Register."]
    #[inline(always)]
    pub const fn link_itp_pre(self) -> crate::common::Reg<regs::LinkItpPre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "LINK U2 Inactivity Timeout Counter Threshold Register."]
    #[inline(always)]
    pub const fn link_u2_inact_timer(
        self,
    ) -> crate::common::Reg<regs::LinkU2InactTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "U1 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u1_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "U1 wake-up timer. The RM register table omits this register; the EVT device header exposes it and the device SDK writes 120."]
    #[inline(always)]
    pub const fn link_u1_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "U2 wakes up the LFPS Duration Register."]
    #[inline(always)]
    pub const fn link_u2_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU2WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "U3 wakes up the LFPS validity duration register."]
    #[inline(always)]
    pub const fn link_u3_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "U3 wake-up timer. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn link_u3_wkup_tmr(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupTmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "U1 exit configuration. The RM register table omits this register; the EVT device header exposes it and the device SDK sets bit 9 before link training."]
    #[inline(always)]
    pub const fn link_u1_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU1ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "U2 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u2_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU2ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "U3 exit configuration. The RM register table omits this register and the EVT header gives no bit definitions."]
    #[inline(always)]
    pub const fn link_u3_exit_cfg(
        self,
    ) -> crate::common::Reg<regs::LinkU3ExitCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "LINK Synchronous Delay Register."]
    #[inline(always)]
    pub const fn link_iso_dly(self) -> crate::common::Reg<regs::LinkIsoDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Hub routing for host transactions. The RM register table omits this register; the EVT host header exposes it and the hub host SDK writes the downstream hub port number to reach a device behind a hub, or 0 for the root hub port."]
    #[inline(always)]
    pub const fn link_tx_route(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Link Power Management Registers."]
    #[inline(always)]
    pub const fn link_lpm_cr(self) -> crate::common::Reg<regs::LinkLpmCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PORT_CAP Registers."]
    #[inline(always)]
    pub const fn link_lmp_port_cap(
        self,
    ) -> crate::common::Reg<regs::LinkLmpPortCap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Header packet of the last LMP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize + n * 4usize) as _) }
    }
    #[doc = "User-defined header packet to send, such as an LMP; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn link_lmp_tx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "USBSS Control Register."]
    #[inline(always)]
    pub const fn usbss_ctrl(self) -> crate::common::Reg<regs::UsbssCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "USBSS Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Interval Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp(self) -> crate::common::Reg<regs::Itp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interval Adaptive Registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp_adj(self) -> crate::common::Reg<regs::ItpAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Endpoint Transmit Enable Register. The RM lists it only among the device registers; the EVT host header exposes it and the host SDK sets bit 1."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Endpoint Receive Enable Register. The RM lists it only among the device registers; the EVT host header exposes it and the host SDK sets bit 1."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Host Transmit Control Registers."]
    #[inline(always)]
    pub const fn uh_tx_ctrl(self) -> crate::common::Reg<regs::UhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Host receives control registers."]
    #[inline(always)]
    pub const fn uh_rx_ctrl(self) -> crate::common::Reg<regs::UhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Send buffer address registers."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<regs::UhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Receive buffer address registers."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<regs::UhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Host Transmit Address Offset Register."]
    #[inline(always)]
    pub const fn uh_tx_dma_ofs(self) -> crate::common::Reg<regs::UhTxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Host Receive Address Offset Register."]
    #[inline(always)]
    pub const fn uh_rx_dma_ofs(self) -> crate::common::Reg<regs::UhRxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "The host transmits the NUMP register."]
    #[inline(always)]
    pub const fn host_tx_nump(self) -> crate::common::Reg<regs::HostTxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "The host receives the NUMP register."]
    #[inline(always)]
    pub const fn host_rx_nump(self) -> crate::common::Reg<regs::HostRxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Host Status Registers."]
    #[inline(always)]
    pub const fn host_status(self) -> crate::common::Reg<regs::HostStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "The host endpoint sends flow control register."]
    #[inline(always)]
    pub const fn host_tx_fc_status(
        self,
    ) -> crate::common::Reg<regs::HostTxFcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "The host endpoint receives the flow control register."]
    #[inline(always)]
    pub const fn host_rx_fc_status(
        self,
    ) -> crate::common::Reg<regs::HostRxFcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Header packet of the last DEV_NOTIF-TP received; word n holds bits \\[32n+31:32n\\]."]
    #[inline(always)]
    pub const fn tp_rx_data(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize + n * 4usize) as _) }
    }
    #[doc = "Indirect PHY configuration command. The RM does not document this register; the device SDK writes `(1 << 23) | (addr << 16) | data`."]
    #[inline(always)]
    pub const fn phy_cfg_cr(self) -> crate::common::Reg<regs::PhyCfgCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Indirect PHY configuration data. The RM does not document this register; the device SDK reads the PHY response from it."]
    #[inline(always)]
    pub const fn phy_cfg_dat(self) -> crate::common::Reg<regs::PhyCfgDat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Endpoint n receive configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxCfg(pub u8);
    impl EpRxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM."]
        #[inline(always)]
        pub const fn set_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "After a short packet or a DPH with PP=0, clear every CHAIN_EN, so that later DPHs get NRDY until software configures a CHAIN again. RB_EP_RX_NRDY_MODE in the RM; the EVT header still calls it USBSS_EP_RX_EOB_MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn nrdy_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "After a short packet or a DPH with PP=0, clear every CHAIN_EN, so that later DPHs get NRDY until software configures a CHAIN again. RB_EP_RX_NRDY_MODE in the RM; the EVT header still calls it USBSS_EP_RX_EOB_MODE."]
        #[inline(always)]
        pub const fn set_nrdy_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "After a burst packet with PP=1 that is not a short packet, raise the TOUT_IF interrupt when no DPH follows within the timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "After a burst packet with PP=1 that is not a short packet, raise the TOUT_IF interrupt when no DPH follows within the timeout."]
        #[inline(always)]
        pub const fn set_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpRxCfg {
        #[inline(always)]
        fn default() -> EpRxCfg {
            EpRxCfg(0)
        }
    }
    impl core::fmt::Debug for EpRxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxCfg")
                .field("iso_mode", &self.iso_mode())
                .field("seq_auto", &self.seq_auto())
                .field("erdy_auto", &self.erdy_auto())
                .field("nrdy_mode", &self.nrdy_mode())
                .field("tout_mode", &self.tout_mode())
                .field("fifo_cfg", &self.fifo_cfg())
                .field("fifo_mode", &self.fifo_mode())
                .field("chain_auto", &self.chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint n receive CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxChainCr(pub u8);
    impl EpRxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for EpRxChainCr {
        #[inline(always)]
        fn default() -> EpRxChainCr {
            EpRxChainCr(0)
        }
    }
    impl core::fmt::Debug for EpRxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxChainCr")
                .field("ret_sel", &self.ret_sel())
                .field("force_ret", &self.force_ret())
                .field("cur_cfg", &self.cur_cfg())
                .field("cur_use", &self.cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint n CHAIN receive packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxChainLen(pub u16);
    impl EpRxChainLen {
        #[doc = "The length of the last packet received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet received by the CHAIN."]
        #[inline(always)]
        pub const fn set_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for EpRxChainLen {
        #[inline(always)]
        fn default() -> EpRxChainLen {
            EpRxChainLen(0)
        }
    }
    impl core::fmt::Debug for EpRxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxChainLen")
                .field("len", &self.len())
                .finish()
        }
    }
    #[doc = "Endpoint n receive CHAIN status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxChainSt(pub u8);
    impl EpRxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Synchronous OUT endpoints only. The DPP just received has a CRC32 error."]
        #[must_use]
        #[inline(always)]
        pub const fn iso_pkt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous OUT endpoints only. The DPP just received has a CRC32 error."]
        #[inline(always)]
        pub const fn set_iso_pkt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Synchronous OUT endpoints only. LPF bit of the DPH just received."]
        #[must_use]
        #[inline(always)]
        pub const fn lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous OUT endpoints only. LPF bit of the DPH just received."]
        #[inline(always)]
        pub const fn set_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpRxChainSt {
        #[inline(always)]
        fn default() -> EpRxChainSt {
            EpRxChainSt(0)
        }
    }
    impl core::fmt::Debug for EpRxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxChainSt")
                .field("chain_no", &self.chain_no())
                .field("dph_pp", &self.dph_pp())
                .field("nump_empty", &self.nump_empty())
                .field("iso_pkt_err", &self.iso_pkt_err())
                .field("lpf_flag", &self.lpf_flag())
                .field("chain_if", &self.chain_if())
                .field("chain_en", &self.chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint n receive control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxCr(pub u8);
    impl EpRxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpRxCr {
        #[inline(always)]
        fn default() -> EpRxCr {
            EpRxCr(0)
        }
    }
    impl core::fmt::Debug for EpRxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxCr")
                .field("erdy_nump", &self.erdy_nump())
                .field("chain_clr", &self.chain_clr())
                .field("clr", &self.clr())
                .field("halt", &self.halt())
                .finish()
        }
    }
    #[doc = "Endpoint n receive sequence number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxSeq(pub u8);
    impl EpRxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for EpRxSeq {
        #[inline(always)]
        fn default() -> EpRxSeq {
            EpRxSeq(0)
        }
    }
    impl core::fmt::Debug for EpRxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxSeq")
                .field("seq_num", &self.seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint n receive status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpRxSt(pub u8);
    impl EpRxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpRxSt {
        #[inline(always)]
        fn default() -> EpRxSt {
            EpRxSt(0)
        }
    }
    impl core::fmt::Debug for EpRxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpRxSt")
                .field("chain_en", &self.chain_en())
                .field("chain_res", &self.chain_res())
                .field("erdy_req", &self.erdy_req())
                .field("fc_st", &self.fc_st())
                .field("int_flag", &self.int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxCfg(pub u8);
    impl EpTxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: software cannot write SEQ_NUM, hardware maintains it. 0: software may write SEQ_NUM."]
        #[inline(always)]
        pub const fn set_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware sends ERDY without software control. The RM recommends clearing this bit on a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpTxCfg {
        #[inline(always)]
        fn default() -> EpTxCfg {
            EpTxCfg(0)
        }
    }
    impl core::fmt::Debug for EpTxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxCfg")
                .field("iso_mode", &self.iso_mode())
                .field("seq_auto", &self.seq_auto())
                .field("erdy_auto", &self.erdy_auto())
                .field("eob_mode", &self.eob_mode())
                .field("fifo_cfg", &self.fifo_cfg())
                .field("fifo_mode", &self.fifo_mode())
                .field("chain_auto", &self.chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxChainCr(pub u8);
    impl EpTxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for EpTxChainCr {
        #[inline(always)]
        fn default() -> EpTxChainCr {
            EpTxChainCr(0)
        }
    }
    impl core::fmt::Debug for EpTxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxChainCr")
                .field("ret_sel", &self.ret_sel())
                .field("force_ret", &self.force_ret())
                .field("cur_cfg", &self.cur_cfg())
                .field("cur_use", &self.cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint n CHAIN transmit packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxChainLen(pub u16);
    impl EpTxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for EpTxChainLen {
        #[inline(always)]
        fn default() -> EpTxChainLen {
            EpTxChainLen(0)
        }
    }
    impl core::fmt::Debug for EpTxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxChainLen")
                .field("len", &self.len())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit CHAIN status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxChainSt(pub u8);
    impl EpTxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpTxChainSt {
        #[inline(always)]
        fn default() -> EpTxChainSt {
            EpTxChainSt(0)
        }
    }
    impl core::fmt::Debug for EpTxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxChainSt")
                .field("chain_no", &self.chain_no())
                .field("dph_pp", &self.dph_pp())
                .field("nump_empty", &self.nump_empty())
                .field("eob_lpf", &self.eob_lpf())
                .field("chain_if", &self.chain_if())
                .field("chain_en", &self.chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxCr(pub u8);
    impl EpTxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpTxCr {
        #[inline(always)]
        fn default() -> EpTxCr {
            EpTxCr(0)
        }
    }
    impl core::fmt::Debug for EpTxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxCr")
                .field("erdy_nump", &self.erdy_nump())
                .field("chain_clr", &self.chain_clr())
                .field("clr", &self.clr())
                .field("halt", &self.halt())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit sequence number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxSeq(pub u8);
    impl EpTxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for EpTxSeq {
        #[inline(always)]
        fn default() -> EpTxSeq {
            EpTxSeq(0)
        }
    }
    impl core::fmt::Debug for EpTxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxSeq")
                .field("seq_num", &self.seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint n transmit status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpTxSt(pub u8);
    impl EpTxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EpTxSt {
        #[inline(always)]
        fn default() -> EpTxSt {
            EpTxSt(0)
        }
    }
    impl core::fmt::Debug for EpTxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpTxSt")
                .field("chain_en", &self.chain_en())
                .field("chain_res", &self.chain_res())
                .field("erdy_req", &self.erdy_req())
                .field("fc_st", &self.fc_st())
                .field("int_flag", &self.int_flag())
                .finish()
        }
    }
    #[doc = "The host endpoint receives the flow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HostRxFcStatus(pub u16);
    impl HostRxFcStatus {
        #[doc = "The flow control status of the host to receiving endpoints 1-15."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The flow control status of the host to receiving endpoints 1-15."]
        #[inline(always)]
        pub const fn set_ep_rx_fc(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for HostRxFcStatus {
        #[inline(always)]
        fn default() -> HostRxFcStatus {
            HostRxFcStatus(0)
        }
    }
    impl core::fmt::Debug for HostRxFcStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HostRxFcStatus")
                .field("ep_rx_fc[0]", &self.ep_rx_fc(0usize))
                .field("ep_rx_fc[1]", &self.ep_rx_fc(1usize))
                .field("ep_rx_fc[2]", &self.ep_rx_fc(2usize))
                .field("ep_rx_fc[3]", &self.ep_rx_fc(3usize))
                .field("ep_rx_fc[4]", &self.ep_rx_fc(4usize))
                .field("ep_rx_fc[5]", &self.ep_rx_fc(5usize))
                .field("ep_rx_fc[6]", &self.ep_rx_fc(6usize))
                .field("ep_rx_fc[7]", &self.ep_rx_fc(7usize))
                .field("ep_rx_fc[8]", &self.ep_rx_fc(8usize))
                .field("ep_rx_fc[9]", &self.ep_rx_fc(9usize))
                .field("ep_rx_fc[10]", &self.ep_rx_fc(10usize))
                .field("ep_rx_fc[11]", &self.ep_rx_fc(11usize))
                .field("ep_rx_fc[12]", &self.ep_rx_fc(12usize))
                .field("ep_rx_fc[13]", &self.ep_rx_fc(13usize))
                .field("ep_rx_fc[14]", &self.ep_rx_fc(14usize))
                .finish()
        }
    }
    #[doc = "The host receives the NUMP register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HostRxNump(pub u16);
    impl HostRxNump {
        #[doc = "The number of packets that the host expects to receive,. if it is a synchronous transmission, will automatically subtract 1 from the hardware for each packet it receives."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of packets that the host expects to receive,. if it is a synchronous transmission, will automatically subtract 1 from the hardware for each packet it receives."]
        #[inline(always)]
        pub const fn set_uh_rx_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "The number of DPPs that have been accepted by the host."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dpp_num(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPPs that have been accepted by the host."]
        #[inline(always)]
        pub const fn set_uh_rx_dpp_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for HostRxNump {
        #[inline(always)]
        fn default() -> HostRxNump {
            HostRxNump(0)
        }
    }
    impl core::fmt::Debug for HostRxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HostRxNump")
                .field("uh_rx_nump", &self.uh_rx_nump())
                .field("uh_rx_dpp_num", &self.uh_rx_dpp_num())
                .finish()
        }
    }
    #[doc = "Host Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HostStatus(pub u32);
    impl HostStatus {
        #[doc = "ERDY is received from the device, and the segment represents the endpoint number of the device ERDY."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_erdy_ep(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "ERDY is received from the device, and the segment represents the endpoint number of the device ERDY."]
        #[inline(always)]
        pub const fn set_uh_rx_erdy_ep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "ERDY received from the device,. which indicates the number of packets that can be sent/received by the device's corresponding endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "ERDY received from the device,. which indicates the number of packets that can be sent/received by the device's corresponding endpoint."]
        #[inline(always)]
        pub const fn set_uh_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "ERDY is received from the device, and the segment represents the direction of the endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_erdy_dir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY is received from the device, and the segment represents the direction of the endpoint."]
        #[inline(always)]
        pub const fn set_uh_rx_erdy_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "This bit represents the EOB/LPF status in the received packet."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "This bit represents the EOB/LPF status in the received packet."]
        #[inline(always)]
        pub const fn set_uh_rx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "A CRC error was received for the packet (DPP) during synchronous transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_iso_pkt_err(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "A CRC error was received for the packet (DPP) during synchronous transmission."]
        #[inline(always)]
        pub const fn set_uh_rx_iso_pkt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "In host mode, this bit indicates the time when the ITP packet was sent."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_itp_presage(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "In host mode, this bit indicates the time when the ITP packet was sent."]
        #[inline(always)]
        pub const fn set_uh_itp_presage(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for HostStatus {
        #[inline(always)]
        fn default() -> HostStatus {
            HostStatus(0)
        }
    }
    impl core::fmt::Debug for HostStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HostStatus")
                .field("uh_rx_erdy_ep", &self.uh_rx_erdy_ep())
                .field("uh_rx_erdy_nump", &self.uh_rx_erdy_nump())
                .field("uh_rx_erdy_dir", &self.uh_rx_erdy_dir())
                .field("uh_rx_eob_lpf", &self.uh_rx_eob_lpf())
                .field("uh_rx_iso_pkt_err", &self.uh_rx_iso_pkt_err())
                .field("uh_itp_presage", &self.uh_itp_presage())
                .finish()
        }
    }
    #[doc = "The host endpoint sends flow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HostTxFcStatus(pub u16);
    impl HostTxFcStatus {
        #[doc = "The flow control status of the host to send endpoints 1-15."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fc(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The flow control status of the host to send endpoints 1-15."]
        #[inline(always)]
        pub const fn set_ep_tx_fc(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for HostTxFcStatus {
        #[inline(always)]
        fn default() -> HostTxFcStatus {
            HostTxFcStatus(0)
        }
    }
    impl core::fmt::Debug for HostTxFcStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HostTxFcStatus")
                .field("ep_tx_fc[0]", &self.ep_tx_fc(0usize))
                .field("ep_tx_fc[1]", &self.ep_tx_fc(1usize))
                .field("ep_tx_fc[2]", &self.ep_tx_fc(2usize))
                .field("ep_tx_fc[3]", &self.ep_tx_fc(3usize))
                .field("ep_tx_fc[4]", &self.ep_tx_fc(4usize))
                .field("ep_tx_fc[5]", &self.ep_tx_fc(5usize))
                .field("ep_tx_fc[6]", &self.ep_tx_fc(6usize))
                .field("ep_tx_fc[7]", &self.ep_tx_fc(7usize))
                .field("ep_tx_fc[8]", &self.ep_tx_fc(8usize))
                .field("ep_tx_fc[9]", &self.ep_tx_fc(9usize))
                .field("ep_tx_fc[10]", &self.ep_tx_fc(10usize))
                .field("ep_tx_fc[11]", &self.ep_tx_fc(11usize))
                .field("ep_tx_fc[12]", &self.ep_tx_fc(12usize))
                .field("ep_tx_fc[13]", &self.ep_tx_fc(13usize))
                .field("ep_tx_fc[14]", &self.ep_tx_fc(14usize))
                .finish()
        }
    }
    #[doc = "The host transmits the NUMP register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HostTxNump(pub u16);
    impl HostTxNump {
        #[doc = "The number of packets that the host expects to send; the hardware subtracts 1 for each packet it sends."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of packets that the host expects to send; the hardware subtracts 1 for each packet it sends."]
        #[inline(always)]
        pub const fn set_uh_tx_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for HostTxNump {
        #[inline(always)]
        fn default() -> HostTxNump {
            HostTxNump(0)
        }
    }
    impl core::fmt::Debug for HostTxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HostTxNump")
                .field("uh_tx_nump", &self.uh_tx_nump())
                .finish()
        }
    }
    #[doc = "Interval Registers for ITP packets in USB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itp(pub u32);
    impl Itp {
        #[doc = "Bus interval Counter field in the received ITP."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_itp_interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Bus interval Counter field in the received ITP."]
        #[inline(always)]
        pub const fn set_reg_itp_interval(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Itp {
        #[inline(always)]
        fn default() -> Itp {
            Itp(0)
        }
    }
    impl core::fmt::Debug for Itp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itp")
                .field("reg_itp_interval", &self.reg_itp_interval())
                .finish()
        }
    }
    #[doc = "Interval Adaptive Registers for ITP packets in USB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ItpAdj(pub u32);
    impl ItpAdj {
        #[doc = "In device mode, the Bus Interval Adjustment Control field in the received ITP should be 0 after power-on reset or device disconnection."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_adj_cr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "In device mode, the Bus Interval Adjustment Control field in the received ITP should be 0 after power-on reset or device disconnection."]
        #[inline(always)]
        pub const fn set_itp_adj_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "In device mode, the Delayed bit of Link Control Word in the received ITP is 1 when the ITP is delayed."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_delayed(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "In device mode, the Delayed bit of Link Control Word in the received ITP is 1 when the ITP is delayed."]
        #[inline(always)]
        pub const fn set_itp_delayed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The higher 13-bit delta of the ITS in. the ITP received in device mode indicates the time difference from the start of the current ITP packet to the previous bus interval boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_delta(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x1fff;
            val as u16
        }
        #[doc = "The higher 13-bit delta of the ITS in. the ITP received in device mode indicates the time difference from the start of the current ITP packet to the previous bus interval boundary."]
        #[inline(always)]
        pub const fn set_itp_delta(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 8usize)) | (((val as u32) & 0x1fff) << 8usize);
        }
    }
    impl Default for ItpAdj {
        #[inline(always)]
        fn default() -> ItpAdj {
            ItpAdj(0)
        }
    }
    impl core::fmt::Debug for ItpAdj {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ItpAdj")
                .field("itp_adj_cr", &self.itp_adj_cr())
                .field("itp_delayed", &self.itp_delayed())
                .field("itp_delta", &self.itp_delta())
                .finish()
        }
    }
    #[doc = "LINK Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkCfg(pub u32);
    impl LinkCfg {
        #[doc = "Peripheral Type."]
        #[must_use]
        #[inline(always)]
        pub const fn link_down_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral Type."]
        #[inline(always)]
        pub const fn set_link_down_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver Termination Resistance Control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_term_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Termination Resistance Control."]
        #[inline(always)]
        pub const fn set_link_rx_term_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Exchange SSTX and SSRX polarities as follows."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ss_plr_swap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Exchange SSTX and SSRX polarities as follows."]
        #[inline(always)]
        pub const fn set_link_ss_plr_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The PIPE interface is reset."]
        #[must_use]
        #[inline(always)]
        pub const fn link_phy_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The PIPE interface is reset."]
        #[inline(always)]
        pub const fn set_link_phy_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "POLLING_LFPS timeout is in COMPLIANCE mode."]
        #[must_use]
        #[inline(always)]
        pub const fn link_compliance_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "POLLING_LFPS timeout is in COMPLIANCE mode."]
        #[inline(always)]
        pub const fn set_link_compliance_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LFPS receive control, this bit is 1 to disable LFPS reception."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lfps_rx_pd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LFPS receive control, this bit is 1 to disable LFPS reception."]
        #[inline(always)]
        pub const fn set_link_lfps_rx_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receiver equalization enable control, optional protocol specifications."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_eq_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver equalization enable control, optional protocol specifications."]
        #[inline(always)]
        pub const fn set_link_rx_eq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The transmitter signal swing control, low swing power consumption, but affect the transmission distance."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_swing(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The transmitter signal swing control, low swing power consumption, but affect the transmission distance."]
        #[inline(always)]
        pub const fn set_link_tx_swing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmitter de-emphasis control, -3.5 dB after reset. The RM allows only -6.0 dB and -3.5 dB in normal operation; the device SDK writes the reserved value 3."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_deemph(&self) -> super::vals::TxDeemph {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::TxDeemph::from_bits(val as u8)
        }
        #[doc = "Transmitter de-emphasis control, -3.5 dB after reset. The RM allows only -6.0 dB and -3.5 dB in normal operation; the device SDK writes the reserved value 3."]
        #[inline(always)]
        pub const fn set_link_tx_deemph(&mut self, val: super::vals::TxDeemph) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "In Compliance Pattern 7/8, send a length of consecutive 0 or consecutive 1."]
        #[must_use]
        #[inline(always)]
        pub const fn link_cp78_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "In Compliance Pattern 7/8, send a length of consecutive 0 or consecutive 1."]
        #[inline(always)]
        pub const fn set_link_cp78_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Detection mode of connected devices in U2 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_det_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Detection mode of connected devices in U2 state."]
        #[inline(always)]
        pub const fn set_link_u2_det_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The LOOPBACK enable bit is allowed, which is highly active, and can be used with the LOOKBACK enable in TX/RX_TS_CFG \\[3\\],. both of which are valid before LINK can enter the LOOKBACK mode (used to enable the data loopback and error counting of the LOOPBACK slave, and the LOOPBACK master remains 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_loopback_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The LOOPBACK enable bit is allowed, which is highly active, and can be used with the LOOKBACK enable in TX/RX_TS_CFG \\[3\\],. both of which are valid before LINK can enter the LOOKBACK mode (used to enable the data loopback and error counting of the LOOPBACK slave, and the LOOPBACK master remains 0)."]
        #[inline(always)]
        pub const fn set_link_loopback_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "It is used in LOOPBACK mode for the LOOPBACK master to control the pattern transmission (the LOOPBACK slave keeps 0),. and the ACT is a high-level signal that lasts for a period of time."]
        #[must_use]
        #[inline(always)]
        pub const fn link_loopback_act(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "It is used in LOOPBACK mode for the LOOPBACK master to control the pattern transmission (the LOOPBACK slave keeps 0),. and the ACT is a high-level signal that lasts for a period of time."]
        #[inline(always)]
        pub const fn set_link_loopback_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "The link state machine enters DISABLE mode."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ltssm_mode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "The link state machine enters DISABLE mode."]
        #[inline(always)]
        pub const fn set_link_ltssm_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "High validity, after receiving the LGO_U1, the response LAU allows to enter the U1 state,. otherwise, after receiving the LGO_U1, the response LXU refuses to enter the U1 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u1_allow(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, after receiving the LGO_U1, the response LAU allows to enter the U1 state,. otherwise, after receiving the LGO_U1, the response LXU refuses to enter the U1 state."]
        #[inline(always)]
        pub const fn set_link_u1_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "High validity, after receiving the LGO_U2, the response LAU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_allow(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, after receiving the LGO_U2, the response LAU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[inline(always)]
        pub const fn set_link_u2_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SKP discard control for descrambling. 0: a byte pair is not descrambled when its low byte is SKP; 1 or 3: when both of its bytes are SKP; 2: when either of its bytes is SKP."]
        #[must_use]
        #[inline(always)]
        pub const fn link_skp_mode(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "SKP discard control for descrambling. 0: a byte pair is not descrambled when its low byte is SKP; 1 or 3: when both of its bytes are SKP; 2: when either of its bytes is SKP."]
        #[inline(always)]
        pub const fn set_link_skp_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Send PING_FPFS under U1 to enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u1_ping_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Send PING_FPFS under U1 to enable."]
        #[inline(always)]
        pub const fn set_link_u1_ping_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Link timer values. 1: USB 3.2 (PM_LC_TIMER 4 us, PM_ENTRY_TIMER 8 us, PENDING_HP_TIMER 10 us); 0: USB 3.0 (3 us, 6 us, 3 us)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tout_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Link timer values. 1: USB 3.2 (PM_LC_TIMER 4 us, PM_ENTRY_TIMER 8 us, PENDING_HP_TIMER 10 us); 0: USB 3.0 (3 us, 6 us, 3 us)."]
        #[inline(always)]
        pub const fn set_link_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[must_use]
        #[inline(always)]
        pub const fn link_force_polling(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[inline(always)]
        pub const fn set_link_force_polling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[must_use]
        #[inline(always)]
        pub const fn link_force_rxterm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Not in the RM, which marks bits 30:22 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[inline(always)]
        pub const fn set_link_force_rxterm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "LINK reset, including the reset state machine and all interrupt flags, is highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LINK reset, including the reset state machine and all interrupt flags, is highly effective."]
        #[inline(always)]
        pub const fn set_link_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LinkCfg {
        #[inline(always)]
        fn default() -> LinkCfg {
            LinkCfg(0)
        }
    }
    impl core::fmt::Debug for LinkCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkCfg")
                .field("link_down_mode", &self.link_down_mode())
                .field("link_rx_term_en", &self.link_rx_term_en())
                .field("link_ss_plr_swap", &self.link_ss_plr_swap())
                .field("link_phy_reset", &self.link_phy_reset())
                .field("link_compliance_en", &self.link_compliance_en())
                .field("link_lfps_rx_pd", &self.link_lfps_rx_pd())
                .field("link_rx_eq_en", &self.link_rx_eq_en())
                .field("link_tx_swing", &self.link_tx_swing())
                .field("link_tx_deemph", &self.link_tx_deemph())
                .field("link_cp78_sel", &self.link_cp78_sel())
                .field("link_u2_det_en", &self.link_u2_det_en())
                .field("link_loopback_en", &self.link_loopback_en())
                .field("link_loopback_act", &self.link_loopback_act())
                .field("link_ltssm_mode", &self.link_ltssm_mode())
                .field("link_u1_allow", &self.link_u1_allow())
                .field("link_u2_allow", &self.link_u2_allow())
                .field("link_skp_mode", &self.link_skp_mode())
                .field("link_u1_ping_en", &self.link_u1_ping_en())
                .field("link_tout_mode", &self.link_tout_mode())
                .field("link_force_polling", &self.link_force_polling())
                .field("link_force_rxterm", &self.link_force_rxterm())
                .field("link_reset", &self.link_reset())
                .finish()
        }
    }
    #[doc = "LINK control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkCtrl(pub u32);
    impl LinkCtrl {
        #[doc = "Configure the current power mode of the PHY, corresponding to P0/P1/P2/P3 in the PIPE. P3 after reset."]
        #[must_use]
        #[inline(always)]
        pub const fn link_pd_mode(&self) -> super::vals::PdMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::PdMode::from_bits(val as u8)
        }
        #[doc = "Configure the current power mode of the PHY, corresponding to P0/P1/P2/P3 in the PIPE. P3 after reset."]
        #[inline(always)]
        pub const fn set_link_pd_mode(&mut self, val: super::vals::PdMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SET LINK TO ENTER SS.DISABLED, WHICH IS HIGHLY VALID,. AND REQUIRES SOFTWARE TO CLEAR."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_disabled(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SET LINK TO ENTER SS.DISABLED, WHICH IS HIGHLY VALID,. AND REQUIRES SOFTWARE TO CLEAR."]
        #[inline(always)]
        pub const fn set_link_go_disabled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SET THE LINK TO ENTER SS.INACTIVE,. WHICH IS HIGHLY VALID, AND THE HARDWARE IS AUTOMATICALLY CLEARED."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_inactive(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SET THE LINK TO ENTER SS.INACTIVE,. WHICH IS HIGHLY VALID, AND THE HARDWARE IS AUTOMATICALLY CLEARED."]
        #[inline(always)]
        pub const fn set_link_go_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SET THE LINK TO ENTER SS.RECOVERY,. WHICH IS HIGHLY EFFECTIVE, AND THE HARDWARE IS AUTOMATICALLY CLEARED."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_recovery(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SET THE LINK TO ENTER SS.RECOVERY,. WHICH IS HIGHLY EFFECTIVE, AND THE HARDWARE IS AUTOMATICALLY CLEARED."]
        #[inline(always)]
        pub const fn set_link_go_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Before setting this bit, the PD_MODE should be set to P2 mode,. and the LINK should be set to enter the SS.RX_DETECT, and the software will query TERM_PRESENT to know whether there is a connection or whether the connection is disconnected. High effectiveness, automatic clearing."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_rx_det(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Before setting this bit, the PD_MODE should be set to P2 mode,. and the LINK should be set to enter the SS.RX_DETECT, and the software will query TERM_PRESENT to know whether there is a connection or whether the connection is disconnected. High effectiveness, automatic clearing."]
        #[inline(always)]
        pub const fn set_link_go_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High validity, cleared by software. A valid bit will send a warm-reset."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_warm_rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, cleared by software. A valid bit will send a warm-reset."]
        #[inline(always)]
        pub const fn set_link_tx_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "High validity, hardware automatic zeroing."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_ux_exit(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, hardware automatic zeroing."]
        #[inline(always)]
        pub const fn set_link_tx_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "In the U0 state, if there is no data, whether to send LUP and LDN packets every 10us."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lup_ldn_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In the U0 state, if there is no data, whether to send LUP and LDN packets every 10us."]
        #[inline(always)]
        pub const fn set_link_lup_ldn_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable the routing function of the HUB, which is highly effective, with registers and no interfaces."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reg_rout_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the routing function of the HUB, which is highly effective, with registers and no interfaces."]
        #[inline(always)]
        pub const fn set_link_reg_rout_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "If the TERM is detected by the SS.RX_DETECT,. a POLLING handshake will be performed after the software sets the bit to be valid."]
        #[must_use]
        #[inline(always)]
        pub const fn link_polling_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "If the TERM is detected by the SS.RX_DETECT,. a POLLING handshake will be performed after the software sets the bit to be valid."]
        #[inline(always)]
        pub const fn set_link_polling_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "After the bit is valid, the LGO_U1 is sent, the high is active, and the hardware is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, the LGO_U1 is sent, the high is active, and the hardware is automatically cleared."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "After the bit is valid, the LGO_U2 is sent, the high is active, and the hardware is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, the LGO_U2 is sent, the high is active, and the hardware is automatically cleared."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "After the bit is valid, the LGO_U3 is transmitted, and the hardware is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, the LGO_U3 is transmitted, and the hardware is automatically cleared."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Send the link configuration of the TS1/TS2 training sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_ts_cfg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Send the link configuration of the TS1/TS2 training sequence."]
        #[inline(always)]
        pub const fn set_link_tx_ts_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Received Link Control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_ts_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Received Link Control."]
        #[inline(always)]
        pub const fn set_link_rx_ts_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for LinkCtrl {
        #[inline(always)]
        fn default() -> LinkCtrl {
            LinkCtrl(0)
        }
    }
    impl core::fmt::Debug for LinkCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkCtrl")
                .field("link_pd_mode", &self.link_pd_mode())
                .field("link_go_disabled", &self.link_go_disabled())
                .field("link_go_inactive", &self.link_go_inactive())
                .field("link_go_recovery", &self.link_go_recovery())
                .field("link_go_rx_det", &self.link_go_rx_det())
                .field("link_tx_warm_rst", &self.link_tx_warm_rst())
                .field("link_tx_ux_exit", &self.link_tx_ux_exit())
                .field("link_lup_ldn_en", &self.link_lup_ldn_en())
                .field("link_reg_rout_en", &self.link_reg_rout_en())
                .field("link_polling_en", &self.link_polling_en())
                .field("link_tx_lgo_u1", &self.link_tx_lgo_u1())
                .field("link_tx_lgo_u2", &self.link_tx_lgo_u2())
                .field("link_tx_lgo_u3", &self.link_tx_lgo_u3())
                .field("link_tx_ts_cfg", &self.link_tx_ts_cfg())
                .field("link_rx_ts_cfg", &self.link_rx_ts_cfg())
                .finish()
        }
    }
    #[doc = "LINK interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkIntCtrl(pub u32);
    impl LinkIntCtrl {
        #[doc = "LINK is initialized, including two ports before (Header Sequence Number Advertisement). and (RX Headr Buffer Credit Advertisement) interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is initialized, including two ports before (Header Sequence Number Advertisement). and (RX Headr Buffer Credit Advertisement) interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DUE TO ERROR LINK IN RECOVERY INTERRUPTED ENABLED."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_recovery(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DUE TO ERROR LINK IN RECOVERY INTERRUPTED ENABLED."]
        #[inline(always)]
        pub const fn set_link_ie_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LINK is enabled on INACTIVE interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_inactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is enabled on INACTIVE interrupt."]
        #[inline(always)]
        pub const fn set_link_ie_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LINK is disabled, interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is disabled, interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LINK is enabled when the U3 interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is enabled when the U3 interrupt is enabled."]
        #[inline(always)]
        pub const fn set_link_ie_go_u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LINK is interrupted at U2 and enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is interrupted at U2 and enabled."]
        #[inline(always)]
        pub const fn set_link_ie_go_u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LINK is interrupted at U1 to enable the following."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is interrupted at U1 to enable the following."]
        #[inline(always)]
        pub const fn set_link_ie_go_u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LINK is interrupted at U0 to enable the following."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is interrupted at U0 to enable the following."]
        #[inline(always)]
        pub const fn set_link_ie_go_u0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Interrupt the U3 command when requesting to exit U3 command timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u3_wk_tout(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt the U3 command when requesting to exit U3 command timeout."]
        #[inline(always)]
        pub const fn set_link_ie_u3_wk_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmit LGO_ Ux, receive LXU interrupt enabled that refuses to enter the Ux."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_rej(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LGO_ Ux, receive LXU interrupt enabled that refuses to enter the Ux."]
        #[inline(always)]
        pub const fn set_link_ie_ux_rej(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LINK enters the RX_DETECT to detect the impedance of the remote receiver trace segment and detect the interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_term_pres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the RX_DETECT to detect the impedance of the remote receiver trace segment and detect the interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LINK enters the POLLING_RXEQ for receiver equalization training interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_txeq(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the POLLING_RXEQ for receiver equalization training interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_txeq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Request to exit UX is received Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_exit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Request to exit UX is received Interrupt Enable."]
        #[inline(always)]
        pub const fn set_link_ie_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Warm reset (not connected) interrupt enabled with LPFS."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_warm_rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Warm reset (not connected) interrupt enabled with LPFS."]
        #[inline(always)]
        pub const fn set_link_ie_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "In the U3 state, the Low Frequency Periodic Signal (LFPS) is received to wake up interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u3_wakeup(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "In the U3 state, the Low Frequency Periodic Signal (LFPS) is received to wake up interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_u3_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "hot reset, which uses the reset interrupt enable of the TS1/TS2 ordered set."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hot_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "hot reset, which uses the reset interrupt enable of the TS1/TS2 ordered set."]
        #[inline(always)]
        pub const fn set_link_ie_hot_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "BUF sends FIFO null interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUF sends FIFO null interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_hpbuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BUF sends FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hpbuf_full(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BUF sends FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_hpbuf_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EVT header name for bit 17, which the RM documents as LINK_IE_HPBUF_FULL. The device SDK enables it on chip revision 3 and later only."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_set_fc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EVT header name for bit 17, which the RM documents as LINK_IE_HPBUF_FULL. The device SDK enables it on chip revision 3 and later only."]
        #[inline(always)]
        pub const fn set_link_ie_rx_set_fc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The link enters the compliance test,. and the interrupt is enabled by the compatibility test or the physical layer conformance test."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_compliance(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "The link enters the compliance test,. and the interrupt is enabled by the compatibility test or the physical layer conformance test."]
        #[inline(always)]
        pub const fn set_link_ie_compliance(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "The link goes into loopback mode for testing and error isolation interrupt enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_loopback(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "The link goes into loopback mode for testing and error isolation interrupt enablement."]
        #[inline(always)]
        pub const fn set_link_ie_loopback(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "The link enters the Rx.Detect state and is interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_det(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "The link enters the Rx.Detect state and is interrupted."]
        #[inline(always)]
        pub const fn set_link_ie_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Received Link Command Flag Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_lmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Received Link Command Flag Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_rx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "If you successfully enter U0, you can send an HP packet to interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_tx_lmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "If you successfully enter U0, you can send an HP packet to interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_tx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Receive LMP Timeout Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_lmp_tout(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LMP Timeout Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_rx_lmp_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Exit UX failed to interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_exit_fail(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Exit UX failed to interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_ux_exit_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Send warm_reset end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_tx_warmrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Send warm_reset end interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_tx_warmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "UX Conversion Failure Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_fail(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "UX Conversion Failure Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_ux_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "U2 Timeout Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u2_tout(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "U2 Timeout Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_u2_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "U1 Timeout Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u1_tout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "U1 Timeout Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_u1_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Link State Machine Change Flag Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_state_chg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Link State Machine Change Flag Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_link_ie_state_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LinkIntCtrl {
        #[inline(always)]
        fn default() -> LinkIntCtrl {
            LinkIntCtrl(0)
        }
    }
    impl core::fmt::Debug for LinkIntCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkIntCtrl")
                .field("link_ie_ready", &self.link_ie_ready())
                .field("link_ie_recovery", &self.link_ie_recovery())
                .field("link_ie_inactive", &self.link_ie_inactive())
                .field("link_ie_disable", &self.link_ie_disable())
                .field("link_ie_go_u3", &self.link_ie_go_u3())
                .field("link_ie_go_u2", &self.link_ie_go_u2())
                .field("link_ie_go_u1", &self.link_ie_go_u1())
                .field("link_ie_go_u0", &self.link_ie_go_u0())
                .field("link_ie_u3_wk_tout", &self.link_ie_u3_wk_tout())
                .field("link_ie_ux_rej", &self.link_ie_ux_rej())
                .field("link_ie_term_pres", &self.link_ie_term_pres())
                .field("link_ie_txeq", &self.link_ie_txeq())
                .field("link_ie_ux_exit", &self.link_ie_ux_exit())
                .field("link_ie_warm_rst", &self.link_ie_warm_rst())
                .field("link_ie_u3_wakeup", &self.link_ie_u3_wakeup())
                .field("link_ie_hot_rst", &self.link_ie_hot_rst())
                .field("link_ie_hpbuf_empty", &self.link_ie_hpbuf_empty())
                .field("link_ie_hpbuf_full", &self.link_ie_hpbuf_full())
                .field("link_ie_rx_set_fc", &self.link_ie_rx_set_fc())
                .field("link_ie_compliance", &self.link_ie_compliance())
                .field("link_ie_loopback", &self.link_ie_loopback())
                .field("link_ie_rx_det", &self.link_ie_rx_det())
                .field("link_ie_rx_lmp", &self.link_ie_rx_lmp())
                .field("link_ie_tx_lmp", &self.link_ie_tx_lmp())
                .field("link_ie_rx_lmp_tout", &self.link_ie_rx_lmp_tout())
                .field("link_ie_ux_exit_fail", &self.link_ie_ux_exit_fail())
                .field("link_ie_tx_warmrst", &self.link_ie_tx_warmrst())
                .field("link_ie_ux_fail", &self.link_ie_ux_fail())
                .field("link_ie_u2_tout", &self.link_ie_u2_tout())
                .field("link_ie_u1_tout", &self.link_ie_u1_tout())
                .field("link_ie_state_chg", &self.link_ie_state_chg())
                .finish()
        }
    }
    #[doc = "LINK Interrupt Flag Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkIntFlag(pub u32);
    impl LinkIntFlag {
        #[doc = "LINK enters the U0 state and completes LINK initialization of the interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U0 state and completes LINK initialization of the interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LINK INTO THE SS.RECOVERY STATE INTERRUPT FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_recovery(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LINK INTO THE SS.RECOVERY STATE INTERRUPT FLAG."]
        #[inline(always)]
        pub const fn set_link_if_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LINK ENTERS THE SS.INACTIVE STATE INTERRUPT FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_inactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK ENTERS THE SS.INACTIVE STATE INTERRUPT FLAG."]
        #[inline(always)]
        pub const fn set_link_if_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LINK INTO SS.DISABLE STATE INTERRUPT FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LINK INTO SS.DISABLE STATE INTERRUPT FLAG."]
        #[inline(always)]
        pub const fn set_link_if_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LINK enters the U3 state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_go_u3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U3 state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_go_u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LINK enters the U2 state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_go_u2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U2 state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_go_u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LINK enters the U1 state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_go_u1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U1 state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_go_u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LINK enters the U0 state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_go_u0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U0 state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_go_u0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Wake up from U3 to timeout interrupt flag (10ms)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u3_wk_tout(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up from U3 to timeout interrupt flag (10ms)."]
        #[inline(always)]
        pub const fn set_link_if_u3_wk_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LINK refuses to enter the low-power mode (U1/U2) interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_rej(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LINK refuses to enter the low-power mode (U1/U2) interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_ux_rej(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The TERM disconnected or disconnected flag was detected."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_term_pres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "The TERM disconnected or disconnected flag was detected."]
        #[inline(always)]
        pub const fn set_link_if_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LINK enters TXEQ state interrupt flag:. INDICATE THAT THE POLLING HANDSHAKE IS COMPLETE, AND WAIT FOR THE SOFTWARE TO SET THE PWR_MODE TO P0 IN ORDER TO ENTER THE TXEQ PHASE."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_txeq(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters TXEQ state interrupt flag:. INDICATE THAT THE POLLING HANDSHAKE IS COMPLETE, AND WAIT FOR THE SOFTWARE TO SET THE PWR_MODE TO P0 IN ORDER TO ENTER THE TXEQ PHASE."]
        #[inline(always)]
        pub const fn set_link_if_txeq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_exit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The WARM RESET status change (active-> inactive,. or invalid->active) interrupt flag received by the device."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_warm_rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The WARM RESET status change (active-> inactive,. or invalid->active) interrupt flag received by the device."]
        #[inline(always)]
        pub const fn set_link_if_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "The power supply is in P3 mode and the LFPS signal interrupt flag is detected."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_wakeup(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "The power supply is in P3 mode and the LFPS signal interrupt flag is detected."]
        #[inline(always)]
        pub const fn set_link_if_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "The device receives the HOT RESET interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_hot_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "The device receives the HOT RESET interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_hot_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Header Packet buffer empty interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Header Packet buffer empty interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_hpbuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Header Packet buffer full interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_hpbuf_full(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Header Packet buffer full interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_hpbuf_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EVT header name for bit 17, which the RM documents as LINK_IF_HPBUF_FULL. On chip revision 3 and later the device SDK takes it as a received Set Link Function LMP and then reads FORCE_PM in LINK_LMP_PORT_CAP."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_set_fc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EVT header name for bit 17, which the RM documents as LINK_IF_HPBUF_FULL. On chip revision 3 and later the device SDK takes it as a received Set Link Function LMP and then reads FORCE_PM in LINK_LMP_PORT_CAP."]
        #[inline(always)]
        pub const fn set_link_if_rx_set_fc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LINK enters the COMPLIANCE state with an outage flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_compliance(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the COMPLIANCE state with an outage flag."]
        #[inline(always)]
        pub const fn set_link_if_compliance(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LINK enters the LOOPBACK state and the LINK is interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_loopback(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the LOOPBACK state and the LINK is interrupted."]
        #[inline(always)]
        pub const fn set_link_if_loopback(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "After entering the RX_DETECT state interrupt flag, this position 1,. the software sets the PWR_MODE to P2, ready for RX_DETECT."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_det(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "After entering the RX_DETECT state interrupt flag, this position 1,. the software sets the PWR_MODE to P2, ready for RX_DETECT."]
        #[inline(always)]
        pub const fn set_link_if_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Receive LMP Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_lmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LMP Interrupt Flag."]
        #[inline(always)]
        pub const fn set_link_if_rx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "After the LINK initialization is completed,. the software is configured to send Port Capabilities LMP for port capability switching."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_tx_lmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "After the LINK initialization is completed,. the software is configured to send Port Capabilities LMP for port capability switching."]
        #[inline(always)]
        pub const fn set_link_if_tx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "After the LINK initialization is complete,. the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_lmp_tout(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "After the LINK initialization is complete,. the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged."]
        #[inline(always)]
        pub const fn set_link_if_rx_lmp_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Exit UX fails to interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_exit_fail(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Exit UX fails to interrupt."]
        #[inline(always)]
        pub const fn set_link_if_ux_exit_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "send warm_reset to end interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_tx_warmrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "send warm_reset to end interrupt."]
        #[inline(always)]
        pub const fn set_link_if_tx_warmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Enter the Ux failed to interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_fail(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Enter the Ux failed to interrupt."]
        #[inline(always)]
        pub const fn set_link_if_ux_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "U2 timeout interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u2_tout(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "U2 timeout interrupted."]
        #[inline(always)]
        pub const fn set_link_if_u2_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "U1 timeout interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u1_tout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "U1 timeout interrupted."]
        #[inline(always)]
        pub const fn set_link_if_u1_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Link state machine change flag. Write 1 to clear; LINK_RESET also clears it."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_state_chg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Link state machine change flag. Write 1 to clear; LINK_RESET also clears it."]
        #[inline(always)]
        pub const fn set_link_if_state_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LinkIntFlag {
        #[inline(always)]
        fn default() -> LinkIntFlag {
            LinkIntFlag(0)
        }
    }
    impl core::fmt::Debug for LinkIntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkIntFlag")
                .field("link_if_ready", &self.link_if_ready())
                .field("link_if_recovery", &self.link_if_recovery())
                .field("link_if_inactive", &self.link_if_inactive())
                .field("link_if_disable", &self.link_if_disable())
                .field("link_if_go_u3", &self.link_if_go_u3())
                .field("link_if_go_u2", &self.link_if_go_u2())
                .field("link_if_go_u1", &self.link_if_go_u1())
                .field("link_if_go_u0", &self.link_if_go_u0())
                .field("link_if_u3_wk_tout", &self.link_if_u3_wk_tout())
                .field("link_if_ux_rej", &self.link_if_ux_rej())
                .field("link_if_term_pres", &self.link_if_term_pres())
                .field("link_if_txeq", &self.link_if_txeq())
                .field("link_if_ux_exit", &self.link_if_ux_exit())
                .field("link_if_warm_rst", &self.link_if_warm_rst())
                .field("link_if_wakeup", &self.link_if_wakeup())
                .field("link_if_hot_rst", &self.link_if_hot_rst())
                .field("link_if_hpbuf_empty", &self.link_if_hpbuf_empty())
                .field("link_if_hpbuf_full", &self.link_if_hpbuf_full())
                .field("link_if_rx_set_fc", &self.link_if_rx_set_fc())
                .field("link_if_compliance", &self.link_if_compliance())
                .field("link_if_loopback", &self.link_if_loopback())
                .field("link_if_rx_det", &self.link_if_rx_det())
                .field("link_if_rx_lmp", &self.link_if_rx_lmp())
                .field("link_if_tx_lmp", &self.link_if_tx_lmp())
                .field("link_if_rx_lmp_tout", &self.link_if_rx_lmp_tout())
                .field("link_if_ux_exit_fail", &self.link_if_ux_exit_fail())
                .field("link_if_tx_warmrst", &self.link_if_tx_warmrst())
                .field("link_if_ux_fail", &self.link_if_ux_fail())
                .field("link_if_u2_tout", &self.link_if_u2_tout())
                .field("link_if_u1_tout", &self.link_if_u1_tout())
                .field("link_if_state_chg", &self.link_if_state_chg())
                .finish()
        }
    }
    #[doc = "LINK Synchronous Delay Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkIsoDly(pub u16);
    impl LinkIsoDly {
        #[doc = "The delay time for the serial bitstream to parse to the parallel data is 40ns by default,. and the SET ISOCH DELAY host sends this information to the device when enumerated, and writes the delay information to the register as the device, and the lower 3 bits of the register are invalid (because the clock frequency is 125MHz, the delay time needs to be set to 8*n)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_isoch_dly(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The delay time for the serial bitstream to parse to the parallel data is 40ns by default,. and the SET ISOCH DELAY host sends this information to the device when enumerated, and writes the delay information to the register as the device, and the lower 3 bits of the register are invalid (because the clock frequency is 125MHz, the delay time needs to be set to 8*n)."]
        #[inline(always)]
        pub const fn set_link_isoch_dly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for LinkIsoDly {
        #[inline(always)]
        fn default() -> LinkIsoDly {
            LinkIsoDly(0)
        }
    }
    impl core::fmt::Debug for LinkIsoDly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkIsoDly")
                .field("link_isoch_dly", &self.link_isoch_dly())
                .finish()
        }
    }
    #[doc = "LINK ITP Timeout Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkItpPre(pub u8);
    impl LinkItpPre {
        #[doc = "ITP Timeout Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_pre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ITP Timeout Mode."]
        #[inline(always)]
        pub const fn set_itp_pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LinkItpPre {
        #[inline(always)]
        fn default() -> LinkItpPre {
            LinkItpPre(0)
        }
    }
    impl core::fmt::Debug for LinkItpPre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkItpPre")
                .field("itp_pre", &self.itp_pre())
                .finish()
        }
    }
    #[doc = "PORT_CAP Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpPortCap(pub u32);
    impl LinkLmpPortCap {
        #[doc = "link port configure."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reg_port_cap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "link port configure."]
        #[inline(always)]
        pub const fn set_link_reg_port_cap(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "24\\]
Position 1, indicating that the supported device supports USB3.2 Gen1 (5Gbps)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_speed(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "24\\]
Position 1, indicating that the supported device supports USB3.2 Gen1 (5Gbps)."]
        #[inline(always)]
        pub const fn set_link_speed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Force_LinkPM_Accept of the received Set Link Function LMP. Not in the RM, where bit 29 is part of LINK_SPEED. Defined by the EVT header; the device SDK reads it after LINK_IF_RX_SET_FC on chip revision 3 and later."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Force_LinkPM_Accept of the received Set Link Function LMP. Not in the RM, where bit 29 is part of LINK_SPEED. Defined by the EVT header; the device SDK reads it after LINK_IF_RX_SET_FC on chip revision 3 and later."]
        #[inline(always)]
        pub const fn set_force_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PORT Capability configuration completion flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_cap_vld(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PORT Capability configuration completion flag."]
        #[inline(always)]
        pub const fn set_link_lmp_tx_cap_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 microseconds after entering the U0 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_cap_vld(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 microseconds after entering the U0 state."]
        #[inline(always)]
        pub const fn set_link_lmp_rx_cap_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LinkLmpPortCap {
        #[inline(always)]
        fn default() -> LinkLmpPortCap {
            LinkLmpPortCap(0)
        }
    }
    impl core::fmt::Debug for LinkLmpPortCap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpPortCap")
                .field("link_reg_port_cap", &self.link_reg_port_cap())
                .field("link_speed", &self.link_speed())
                .field("force_pm", &self.force_pm())
                .field("link_lmp_tx_cap_vld", &self.link_lmp_tx_cap_vld())
                .field("link_lmp_rx_cap_vld", &self.link_lmp_rx_cap_vld())
                .finish()
        }
    }
    #[doc = "Link Power Management Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLpmCr(pub u16);
    impl LinkLpmCr {
        #[doc = "Detects external device counter registers."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdet_exp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Detects external device counter registers."]
        #[inline(always)]
        pub const fn set_rxdet_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "Reset signal for LPM related register."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signal for LPM related register."]
        #[inline(always)]
        pub const fn set_lpm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "LPM Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPM Enable."]
        #[inline(always)]
        pub const fn set_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "The connected device changes, plugs in the device or unplugs the device."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_term_chg(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "The connected device changes, plugs in the device or unplugs the device."]
        #[inline(always)]
        pub const fn set_lpm_term_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "The PHY layer detects a device connection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_term_present(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The PHY layer detects a device connection."]
        #[inline(always)]
        pub const fn set_lpm_term_present(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "When the lpm count reaches the expected value RXDET_EXP it is confirmed that. a device is connected to the PHY, this bit is raised."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_rxdet_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When the lpm count reaches the expected value RXDET_EXP it is confirmed that. a device is connected to the PHY, this bit is raised."]
        #[inline(always)]
        pub const fn set_lpm_rxdet_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Automatic selection of the PHY layer transceiver channel type."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_chsel_auto(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic selection of the PHY layer transceiver channel type."]
        #[inline(always)]
        pub const fn set_phy_chsel_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for LinkLpmCr {
        #[inline(always)]
        fn default() -> LinkLpmCr {
            LinkLpmCr(0)
        }
    }
    impl core::fmt::Debug for LinkLpmCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLpmCr")
                .field("rxdet_exp", &self.rxdet_exp())
                .field("lpm_rst", &self.lpm_rst())
                .field("lpm_en", &self.lpm_en())
                .field("lpm_term_chg", &self.lpm_term_chg())
                .field("lpm_term_present", &self.lpm_term_present())
                .field("lpm_rxdet_en", &self.lpm_rxdet_en())
                .field("phy_chsel_auto", &self.phy_chsel_auto())
                .finish()
        }
    }
    #[doc = "LINK Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkStatus(pub u32);
    impl LinkStatus {
        #[doc = "After RX_DETECT, if a receive termination resistor is present, the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_term_pres(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "After RX_DETECT, if a receive termination resistor is present, the bit is 1."]
        #[inline(always)]
        pub const fn set_link_rx_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "A valid warm-reset signal is received from the host. (the hardware automatically pulls up after receiving the host's warm_reset 18ms)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_warm_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "A valid warm-reset signal is received from the host. (the hardware automatically pulls up after receiving the host's warm_reset 18ms)."]
        #[inline(always)]
        pub const fn set_link_rx_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "When the LINK is busy, the bit is 1 when the switchover is PD_MODE,. and the hardware will automatically clear the link after the switchover is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn link_busy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "When the LINK is busy, the bit is 1 when the switchover is PD_MODE,. and the hardware will automatically clear the link after the switchover is completed."]
        #[inline(always)]
        pub const fn set_link_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When the LINK enters the U0 state, the position 1 exits the U0 state after the initialization. (broadcast) is completed, and the bit is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ready(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When the LINK enters the U0 state, the position 1 exits the U0 state after the initialization. (broadcast) is completed, and the bit is automatically cleared."]
        #[inline(always)]
        pub const fn set_link_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Current power mode of the PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn link_pd_mode_mask(&self) -> super::vals::PdMode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::PdMode::from_bits(val as u8)
        }
        #[doc = "Current power mode of the PHY."]
        #[inline(always)]
        pub const fn set_link_pd_mode_mask(&mut self, val: super::vals::PdMode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "The link is in POLLING_RXEQ."]
        #[must_use]
        #[inline(always)]
        pub const fn link_txeq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in POLLING_RXEQ."]
        #[inline(always)]
        pub const fn set_link_txeq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Link state. DISABLED after reset."]
        #[must_use]
        #[inline(always)]
        pub const fn link_state(&self) -> super::vals::LinkState {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::LinkState::from_bits(val as u8)
        }
        #[doc = "Link state. DISABLED after reset."]
        #[inline(always)]
        pub const fn set_link_state(&mut self, val: super::vals::LinkState) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Received a request to exit Ux."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_ux_exit_req(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Received a request to exit Ux."]
        #[inline(always)]
        pub const fn set_link_rx_ux_exit_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The link is at P2 and is in rx_detect."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_detect(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "The link is at P2 and is in rx_detect."]
        #[inline(always)]
        pub const fn set_link_rx_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The link receives an LFPS signal."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_lfps(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "The link receives an LFPS signal."]
        #[inline(always)]
        pub const fn set_link_rx_lfps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "A link wake-up signal is received."]
        #[must_use]
        #[inline(always)]
        pub const fn link_wakup(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "A link wake-up signal is received."]
        #[inline(always)]
        pub const fn set_link_wakup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "The link is in the RXDET state, allowing sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rxdet_sleep_allow(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in the RXDET state, allowing sleep."]
        #[inline(always)]
        pub const fn set_link_rxdet_sleep_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "The link is in the U2 state and sleep is allowed."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_sleep_allow(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in the U2 state and sleep is allowed."]
        #[inline(always)]
        pub const fn set_link_u2_sleep_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "The link is in the U3 state and sleep is allowed."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u3_sleep_allow(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in the U3 state and sleep is allowed."]
        #[inline(always)]
        pub const fn set_link_u3_sleep_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "BUF sends the status of the FIFO IDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_idle(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "BUF sends the status of the FIFO IDLE."]
        #[inline(always)]
        pub const fn set_link_hpbuf_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "The HP buffer is full."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_full(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "The HP buffer is full."]
        #[inline(always)]
        pub const fn set_link_hpbuf_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "The HP buffer is empty."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The HP buffer is empty."]
        #[inline(always)]
        pub const fn set_link_hpbuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LinkStatus {
        #[inline(always)]
        fn default() -> LinkStatus {
            LinkStatus(0)
        }
    }
    impl core::fmt::Debug for LinkStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkStatus")
                .field("link_rx_term_pres", &self.link_rx_term_pres())
                .field("link_rx_warm_rst", &self.link_rx_warm_rst())
                .field("link_busy", &self.link_busy())
                .field("link_ready", &self.link_ready())
                .field("link_pd_mode_mask", &self.link_pd_mode_mask())
                .field("link_txeq", &self.link_txeq())
                .field("link_state", &self.link_state())
                .field("link_rx_ux_exit_req", &self.link_rx_ux_exit_req())
                .field("link_rx_detect", &self.link_rx_detect())
                .field("link_rx_lfps", &self.link_rx_lfps())
                .field("link_wakup", &self.link_wakup())
                .field("link_rxdet_sleep_allow", &self.link_rxdet_sleep_allow())
                .field("link_u2_sleep_allow", &self.link_u2_sleep_allow())
                .field("link_u3_sleep_allow", &self.link_u3_sleep_allow())
                .field("link_hpbuf_idle", &self.link_hpbuf_idle())
                .field("link_hpbuf_full", &self.link_hpbuf_full())
                .field("link_hpbuf_empty", &self.link_hpbuf_empty())
                .finish()
        }
    }
    #[doc = "U1 exit configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU1ExitCfg(pub u32);
    impl LinkU1ExitCfg {
        #[doc = "U1 exit time enable. The device SDK sets this bit before link training."]
        #[must_use]
        #[inline(always)]
        pub const fn u1_exit_time(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "U1 exit time enable. The device SDK sets this bit before link training."]
        #[inline(always)]
        pub const fn set_u1_exit_time(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for LinkU1ExitCfg {
        #[inline(always)]
        fn default() -> LinkU1ExitCfg {
            LinkU1ExitCfg(0)
        }
    }
    impl core::fmt::Debug for LinkU1ExitCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU1ExitCfg")
                .field("u1_exit_time", &self.u1_exit_time())
                .finish()
        }
    }
    #[doc = "U1 wakes up the LFPS Duration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU1WkupFilter(pub u8);
    impl LinkU1WkupFilter {
        #[doc = "The duration of the LFPS received by U1 when exiting.. When the receiving LFPS reaches this time, the received U1 EXIT is considered valid, and the handshake is successful when the transmitting LFPS_last is raised, which is 600ns by default U1_WKUP_FILTER \\[7\\]: the unit of time that controls the U1_WKUP_FILTER \\[6:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn u1_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The duration of the LFPS received by U1 when exiting.. When the receiving LFPS reaches this time, the received U1 EXIT is considered valid, and the handshake is successful when the transmitting LFPS_last is raised, which is 600ns by default U1_WKUP_FILTER \\[7\\]: the unit of time that controls the U1_WKUP_FILTER \\[6:0\\]."]
        #[inline(always)]
        pub const fn set_u1_wkup_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LinkU1WkupFilter {
        #[inline(always)]
        fn default() -> LinkU1WkupFilter {
            LinkU1WkupFilter(0)
        }
    }
    impl core::fmt::Debug for LinkU1WkupFilter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU1WkupFilter")
                .field("u1_wkup_filter", &self.u1_wkup_filter())
                .finish()
        }
    }
    #[doc = "U1 wake-up timer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU1WkupTmr(pub u16);
    impl LinkU1WkupTmr {
        #[doc = "U1 wake-up timer value. The device SDK writes 120."]
        #[must_use]
        #[inline(always)]
        pub const fn u1_wkup_tmr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "U1 wake-up timer value. The device SDK writes 120."]
        #[inline(always)]
        pub const fn set_u1_wkup_tmr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for LinkU1WkupTmr {
        #[inline(always)]
        fn default() -> LinkU1WkupTmr {
            LinkU1WkupTmr(0)
        }
    }
    impl core::fmt::Debug for LinkU1WkupTmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU1WkupTmr")
                .field("u1_wkup_tmr", &self.u1_wkup_tmr())
                .finish()
        }
    }
    #[doc = "U2 exit configuration. The EVT header gives no bit definitions."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU2ExitCfg(pub u32);
    impl LinkU2ExitCfg {
        #[doc = "Undocumented configuration value."]
        #[must_use]
        #[inline(always)]
        pub const fn exit_cfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Undocumented configuration value."]
        #[inline(always)]
        pub const fn set_exit_cfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkU2ExitCfg {
        #[inline(always)]
        fn default() -> LinkU2ExitCfg {
            LinkU2ExitCfg(0)
        }
    }
    impl core::fmt::Debug for LinkU2ExitCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU2ExitCfg")
                .field("exit_cfg", &self.exit_cfg())
                .finish()
        }
    }
    #[doc = "LINK U2 Inactivity Timeout Counter Threshold Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU2InactTimer(pub u8);
    impl LinkU2InactTimer {
        #[doc = "The value of the inactivity timeout counter threshold for U2."]
        #[must_use]
        #[inline(always)]
        pub const fn u2_inactive_timer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The value of the inactivity timeout counter threshold for U2."]
        #[inline(always)]
        pub const fn set_u2_inactive_timer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LinkU2InactTimer {
        #[inline(always)]
        fn default() -> LinkU2InactTimer {
            LinkU2InactTimer(0)
        }
    }
    impl core::fmt::Debug for LinkU2InactTimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU2InactTimer")
                .field("u2_inactive_timer", &self.u2_inactive_timer())
                .finish()
        }
    }
    #[doc = "U2 wakes up the LFPS Duration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU2WkupFilter(pub u8);
    impl LinkU2WkupFilter {
        #[doc = "When the receiving LFPS reaches this time, the sending LFPS_last is pulled up and the handshake is successful,. and the received U2 EXIT can be considered valid with the duration of (n)us, the default is 2us."]
        #[must_use]
        #[inline(always)]
        pub const fn u2_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "When the receiving LFPS reaches this time, the sending LFPS_last is pulled up and the handshake is successful,. and the received U2 EXIT can be considered valid with the duration of (n)us, the default is 2us."]
        #[inline(always)]
        pub const fn set_u2_wkup_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LinkU2WkupFilter {
        #[inline(always)]
        fn default() -> LinkU2WkupFilter {
            LinkU2WkupFilter(0)
        }
    }
    impl core::fmt::Debug for LinkU2WkupFilter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU2WkupFilter")
                .field("u2_wkup_filter", &self.u2_wkup_filter())
                .finish()
        }
    }
    #[doc = "U3 exit configuration. The EVT header gives no bit definitions."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU3ExitCfg(pub u32);
    impl LinkU3ExitCfg {
        #[doc = "Undocumented configuration value."]
        #[must_use]
        #[inline(always)]
        pub const fn exit_cfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Undocumented configuration value."]
        #[inline(always)]
        pub const fn set_exit_cfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkU3ExitCfg {
        #[inline(always)]
        fn default() -> LinkU3ExitCfg {
            LinkU3ExitCfg(0)
        }
    }
    impl core::fmt::Debug for LinkU3ExitCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU3ExitCfg")
                .field("exit_cfg", &self.exit_cfg())
                .finish()
        }
    }
    #[doc = "U3 wakes up the LFPS validity duration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU3WkupFilter(pub u8);
    impl LinkU3WkupFilter {
        #[doc = "When the receiving LFPS reaches this time, the LFPS_last is raised, and the handshake is successful,. and the received U3 EXIT is considered valid for (n)us, with a default of 100us."]
        #[must_use]
        #[inline(always)]
        pub const fn u3_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "When the receiving LFPS reaches this time, the LFPS_last is raised, and the handshake is successful,. and the received U3 EXIT is considered valid for (n)us, with a default of 100us."]
        #[inline(always)]
        pub const fn set_u3_wkup_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LinkU3WkupFilter {
        #[inline(always)]
        fn default() -> LinkU3WkupFilter {
            LinkU3WkupFilter(0)
        }
    }
    impl core::fmt::Debug for LinkU3WkupFilter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU3WkupFilter")
                .field("u3_wkup_filter", &self.u3_wkup_filter())
                .finish()
        }
    }
    #[doc = "U3 wake-up timer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU3WkupTmr(pub u16);
    impl LinkU3WkupTmr {
        #[doc = "U3 wake-up timer value."]
        #[must_use]
        #[inline(always)]
        pub const fn u3_wkup_tmr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "U3 wake-up timer value."]
        #[inline(always)]
        pub const fn set_u3_wkup_tmr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for LinkU3WkupTmr {
        #[inline(always)]
        fn default() -> LinkU3WkupTmr {
            LinkU3WkupTmr(0)
        }
    }
    impl core::fmt::Debug for LinkU3WkupTmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkU3WkupTmr")
                .field("u3_wkup_tmr", &self.u3_wkup_tmr())
                .finish()
        }
    }
    #[doc = "Indirect PHY configuration command."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCfgCr(pub u32);
    impl PhyCfgCr {
        #[doc = "Data written to the addressed PHY register."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_cfg_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data written to the addressed PHY register."]
        #[inline(always)]
        pub const fn set_phy_cfg_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "PHY register address."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_cfg_addr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY register address."]
        #[inline(always)]
        pub const fn set_phy_cfg_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Command strobe. The device SDK always sets it together with the address and data."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_cfg_write(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Command strobe. The device SDK always sets it together with the address and data."]
        #[inline(always)]
        pub const fn set_phy_cfg_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for PhyCfgCr {
        #[inline(always)]
        fn default() -> PhyCfgCr {
            PhyCfgCr(0)
        }
    }
    impl core::fmt::Debug for PhyCfgCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCfgCr")
                .field("phy_cfg_data", &self.phy_cfg_data())
                .field("phy_cfg_addr", &self.phy_cfg_addr())
                .field("phy_cfg_write", &self.phy_cfg_write())
                .finish()
        }
    }
    #[doc = "Indirect PHY configuration data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCfgDat(pub u32);
    impl PhyCfgDat {
        #[doc = "Data read back from the PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_cfg_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data read back from the PHY."]
        #[inline(always)]
        pub const fn set_phy_cfg_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PhyCfgDat {
        #[inline(always)]
        fn default() -> PhyCfgDat {
            PhyCfgDat(0)
        }
    }
    impl core::fmt::Debug for PhyCfgDat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCfgDat")
                .field("phy_cfg_data", &self.phy_cfg_data())
                .finish()
        }
    }
    #[doc = "USBSS Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "USB Transaction Completion Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_transfer(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB Transaction Completion Interrupt Flag."]
        #[inline(always)]
        pub const fn set_uif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receive SETUP Transaction Completion Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_setup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SETUP Transaction Completion Interrupt Flag."]
        #[inline(always)]
        pub const fn set_udif_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive ERDY-TP Complete Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_erdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receive ERDY-TP Complete Interrupt Flag."]
        #[inline(always)]
        pub const fn set_uhif_erdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive STATUS Transaction Completion Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_status(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive STATUS Transaction Completion Interrupt Flag."]
        #[inline(always)]
        pub const fn set_udif_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive DEV_NOTIF-TP Complete Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_notif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive DEV_NOTIF-TP Complete Interrupt Flag."]
        #[inline(always)]
        pub const fn set_uhif_notif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive PING-TP Complete Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rx_ping(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive PING-TP Complete Interrupt Flag."]
        #[inline(always)]
        pub const fn set_uif_rx_ping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Send the ITP Complete Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_itp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Send the ITP Complete Interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_itp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Send FIFO overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_txov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Send FIFO overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_fifo_txov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive FIFO overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_rxov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_fifo_rxov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "If multiple endpoints have interrupt flags at the same time, the order of endpoint priority is as follows."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_id(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "If multiple endpoints have interrupt flags at the same time, the order of endpoint priority is as follows."]
        #[inline(always)]
        pub const fn set_ep_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "The direction of the endpoint that currently has an interrupt flag for transmission completion,."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_dir(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "The direction of the endpoint that currently has an interrupt flag for transmission completion,."]
        #[inline(always)]
        pub const fn set_ep_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bit is invalid for synchronous transmission, and is defined as follows in Control Transfer,. Block Transfer, and Interrupt Transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn host_ack_nump(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "This bit is invalid for synchronous transmission, and is defined as follows in Control Transfer,. Block Transfer, and Interrupt Transfer."]
        #[inline(always)]
        pub const fn set_host_ack_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Received a reply TP from the device returning."]
        #[must_use]
        #[inline(always)]
        pub const fn htx_res(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Received a reply TP from the device returning."]
        #[inline(always)]
        pub const fn set_htx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "A reply TP is received from the device back."]
        #[must_use]
        #[inline(always)]
        pub const fn hrx_res(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "A reply TP is received from the device back."]
        #[inline(always)]
        pub const fn set_hrx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
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
                .field("uif_transfer", &self.uif_transfer())
                .field("udif_setup", &self.udif_setup())
                .field("uhif_erdy", &self.uhif_erdy())
                .field("udif_status", &self.udif_status())
                .field("uhif_notif", &self.uhif_notif())
                .field("uif_rx_ping", &self.uif_rx_ping())
                .field("uif_itp", &self.uif_itp())
                .field("uif_fifo_txov", &self.uif_fifo_txov())
                .field("uif_fifo_rxov", &self.uif_fifo_rxov())
                .field("ep_id", &self.ep_id())
                .field("ep_dir", &self.ep_dir())
                .field("host_ack_nump", &self.host_ack_nump())
                .field("htx_res", &self.htx_res())
                .field("hrx_res", &self.hrx_res())
                .finish()
        }
    }
    #[doc = "Endpoint 0 receives control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u32);
    impl Uep0RxCtrl {
        #[doc = "The endpoint receives the length register."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The endpoint receives the length register."]
        #[inline(always)]
        pub const fn set_ep0_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "The sequence number of the endpoint expects to accept."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The sequence number of the endpoint expects to accept."]
        #[inline(always)]
        pub const fn set_ep0_rx_seq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Response to DPH. Cleared by hardware when the OUT transaction completes. A SETUP packet is always answered with ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_res(&self) -> super::vals::Ep0RxRes {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Ep0RxRes::from_bits(val as u8)
        }
        #[doc = "Response to DPH. Cleared by hardware when the OUT transaction completes. A SETUP packet is always answered with ACK-TP."]
        #[inline(always)]
        pub const fn set_ep0_rx_res(&mut self, val: super::vals::Ep0RxRes) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "ERDY received."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_erdy(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY received."]
        #[inline(always)]
        pub const fn set_ep0_rx_erdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PP bits in the received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_pp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PP bits in the received DPH."]
        #[inline(always)]
        pub const fn set_ep0_rx_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Upload the transaction completion interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_ep0_rx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Upload the transaction completion interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[inline(always)]
        pub const fn set_uif_ep0_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Uep0RxCtrl {
        #[inline(always)]
        fn default() -> Uep0RxCtrl {
            Uep0RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep0RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxCtrl")
                .field("ep0_rx_len", &self.ep0_rx_len())
                .field("ep0_rx_seq", &self.ep0_rx_seq())
                .field("ep0_rx_res", &self.ep0_rx_res())
                .field("ep0_rx_erdy", &self.ep0_rx_erdy())
                .field("ep0_rx_pp", &self.ep0_rx_pp())
                .field("uif_ep0_rx_act", &self.uif_ep0_rx_act())
                .finish()
        }
    }
    #[doc = "Endpoint 0 sends control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u32);
    impl Uep0TxCtrl {
        #[doc = "Endpoint transmits length registers with a maximum value of 512B."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Endpoint transmits length registers with a maximum value of 512B."]
        #[inline(always)]
        pub const fn set_ep0_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "The current sequence number of the endpoint,. which is aotomatically reset after receiving the SETUP transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The current sequence number of the endpoint,. which is aotomatically reset after receiving the SETUP transaction."]
        #[inline(always)]
        pub const fn set_ep0_tx_seq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Response to an ACK-TP with a non-zero NumP. Cleared by hardware when the IN transaction completes."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_res(&self) -> super::vals::Ep0TxRes {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Ep0TxRes::from_bits(val as u8)
        }
        #[doc = "Response to an ACK-TP with a non-zero NumP. Cleared by hardware when the IN transaction completes."]
        #[inline(always)]
        pub const fn set_ep0_tx_res(&mut self, val: super::vals::Ep0TxRes) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "ERDY received."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_erdy(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY received."]
        #[inline(always)]
        pub const fn set_ep0_tx_erdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The PP bit in the received ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_pp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The PP bit in the received ACK-TP."]
        #[inline(always)]
        pub const fn set_ep0_tx_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "A sign that completes the NRDY-TP send (answer)."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_flow(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "A sign that completes the NRDY-TP send (answer)."]
        #[inline(always)]
        pub const fn set_ep0_tx_flow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "The upload transaction is interrupted, the software writes 0 to zero, and the hardware sets 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_ep0_tx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The upload transaction is interrupted, the software writes 0 to zero, and the hardware sets 1."]
        #[inline(always)]
        pub const fn set_uif_ep0_tx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Uep0TxCtrl {
        #[inline(always)]
        fn default() -> Uep0TxCtrl {
            Uep0TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep0TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TxCtrl")
                .field("ep0_tx_len", &self.ep0_tx_len())
                .field("ep0_tx_seq", &self.ep0_tx_seq())
                .field("ep0_tx_res", &self.ep0_tx_res())
                .field("ep0_tx_erdy", &self.ep0_tx_erdy())
                .field("ep0_tx_pp", &self.ep0_tx_pp())
                .field("ep0_tx_flow", &self.ep0_tx_flow())
                .field("uif_ep0_tx_act", &self.uif_ep0_tx_act())
                .finish()
        }
    }
    #[doc = "Endpoint Receive Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoints 1~15 downpass enabled. Index 0 is endpoint 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_en(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoints 1~15 downpass enabled. Index 0 is endpoint 1."]
        #[inline(always)]
        pub const fn set_ep_rx_en(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for UepRxEn {
        #[inline(always)]
        fn default() -> UepRxEn {
            UepRxEn(0)
        }
    }
    impl core::fmt::Debug for UepRxEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxEn")
                .field("ep_rx_en[0]", &self.ep_rx_en(0usize))
                .field("ep_rx_en[1]", &self.ep_rx_en(1usize))
                .field("ep_rx_en[2]", &self.ep_rx_en(2usize))
                .field("ep_rx_en[3]", &self.ep_rx_en(3usize))
                .field("ep_rx_en[4]", &self.ep_rx_en(4usize))
                .field("ep_rx_en[5]", &self.ep_rx_en(5usize))
                .field("ep_rx_en[6]", &self.ep_rx_en(6usize))
                .field("ep_rx_en[7]", &self.ep_rx_en(7usize))
                .field("ep_rx_en[8]", &self.ep_rx_en(8usize))
                .field("ep_rx_en[9]", &self.ep_rx_en(9usize))
                .field("ep_rx_en[10]", &self.ep_rx_en(10usize))
                .field("ep_rx_en[11]", &self.ep_rx_en(11usize))
                .field("ep_rx_en[12]", &self.ep_rx_en(12usize))
                .field("ep_rx_en[13]", &self.ep_rx_en(13usize))
                .field("ep_rx_en[14]", &self.ep_rx_en(14usize))
                .finish()
        }
    }
    #[doc = "Endpoint Sends Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoints 1~15 upload enabled. Index 0 is endpoint 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_en(&self, n: usize) -> bool {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoints 1~15 upload enabled. Index 0 is endpoint 1."]
        #[inline(always)]
        pub const fn set_ep_tx_en(&mut self, n: usize, val: bool) {
            assert!(n < 15usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for UepTxEn {
        #[inline(always)]
        fn default() -> UepTxEn {
            UepTxEn(0)
        }
    }
    impl core::fmt::Debug for UepTxEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxEn")
                .field("ep_tx_en[0]", &self.ep_tx_en(0usize))
                .field("ep_tx_en[1]", &self.ep_tx_en(1usize))
                .field("ep_tx_en[2]", &self.ep_tx_en(2usize))
                .field("ep_tx_en[3]", &self.ep_tx_en(3usize))
                .field("ep_tx_en[4]", &self.ep_tx_en(4usize))
                .field("ep_tx_en[5]", &self.ep_tx_en(5usize))
                .field("ep_tx_en[6]", &self.ep_tx_en(6usize))
                .field("ep_tx_en[7]", &self.ep_tx_en(7usize))
                .field("ep_tx_en[8]", &self.ep_tx_en(8usize))
                .field("ep_tx_en[9]", &self.ep_tx_en(9usize))
                .field("ep_tx_en[10]", &self.ep_tx_en(10usize))
                .field("ep_tx_en[11]", &self.ep_tx_en(11usize))
                .field("ep_tx_en[12]", &self.ep_tx_en(12usize))
                .field("ep_tx_en[13]", &self.ep_tx_en(13usize))
                .field("ep_tx_en[14]", &self.ep_tx_en(14usize))
                .finish()
        }
    }
    #[doc = "Host receives control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxCtrl(pub u32);
    impl UhRxCtrl {
        #[doc = "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission, and the other packets must be 1024B as specified in the protocol."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission, and the other packets must be 1024B as specified in the protocol."]
        #[inline(always)]
        pub const fn set_uh_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Indicates the source (device endpoint number) from which the packet was received in host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_ep(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Indicates the source (device endpoint number) from which the packet was received in host mode."]
        #[inline(always)]
        pub const fn set_uh_rx_ep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "The SEQ _NUM that the endpoint expects to receive, the hardware automatically adds 1, except for endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The SEQ _NUM that the endpoint expects to receive, the hardware automatically adds 1, except for endpoint 0."]
        #[inline(always)]
        pub const fn set_uh_rx_seq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Response to DPH+DPP or STATUS-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_res(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Response to DPH+DPP or STATUS-TP."]
        #[inline(always)]
        pub const fn set_uh_rx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "The number of packets (DPP) that the endpoint is capable of receiving (burst transmission)."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_nump(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "The number of packets (DPP) that the endpoint is capable of receiving (burst transmission)."]
        #[inline(always)]
        pub const fn set_uh_rx_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Received Packets (DPP) are transmitted synchronously."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_iso(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Received Packets (DPP) are transmitted synchronously."]
        #[inline(always)]
        pub const fn set_uh_rx_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "The OUT transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The OUT transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[inline(always)]
        pub const fn set_uh_rx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for UhRxCtrl {
        #[inline(always)]
        fn default() -> UhRxCtrl {
            UhRxCtrl(0)
        }
    }
    impl core::fmt::Debug for UhRxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxCtrl")
                .field("uh_rx_len", &self.uh_rx_len())
                .field("uh_rx_ep", &self.uh_rx_ep())
                .field("uh_rx_seq", &self.uh_rx_seq())
                .field("uh_rx_res", &self.uh_rx_res())
                .field("uh_rx_nump", &self.uh_rx_nump())
                .field("uh_rx_iso", &self.uh_rx_iso())
                .field("uh_rx_act", &self.uh_rx_act())
                .finish()
        }
    }
    #[doc = "Receive buffer address registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDma(pub u32);
    impl UhRxDma {
        #[doc = "The start address of the host receiving buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the host receiving buffer."]
        #[inline(always)]
        pub const fn set_uh_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UhRxDma {
        #[inline(always)]
        fn default() -> UhRxDma {
            UhRxDma(0)
        }
    }
    impl core::fmt::Debug for UhRxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxDma")
                .field("uh_rx_dma", &self.uh_rx_dma())
                .finish()
        }
    }
    #[doc = "Host Receive Address Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDmaOfs(pub u32);
    impl UhRxDmaOfs {
        #[doc = "After the host receives it, the DMA's address is offset by a large amount."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dma_ofs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After the host receives it, the DMA's address is offset by a large amount."]
        #[inline(always)]
        pub const fn set_uh_rx_dma_ofs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UhRxDmaOfs {
        #[inline(always)]
        fn default() -> UhRxDmaOfs {
            UhRxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for UhRxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxDmaOfs")
                .field("uh_rx_dma_ofs", &self.uh_rx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Host Transmit Control Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxCtrl(pub u32);
    impl UhTxCtrl {
        #[doc = "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission, and the other packets must be 1024B as specified in the protocol."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The endpoint receives the length register,. which for burst transmissions indicates the packet length of the last packet of the burst transmission, and the other packets must be 1024B as specified in the protocol."]
        #[inline(always)]
        pub const fn set_uh_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Indicates the destination of the packet sent in host mode (the target endpoint number of the device)."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_ep(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Indicates the destination of the packet sent in host mode (the target endpoint number of the device)."]
        #[inline(always)]
        pub const fn set_uh_tx_ep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "The SEQ _NUM for which the endpoint receives the packet, the hardware automatically adds 1, except for endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The SEQ _NUM for which the endpoint receives the packet, the hardware automatically adds 1, except for endpoint 0."]
        #[inline(always)]
        pub const fn set_uh_tx_seq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Response to DPH+DPP."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_res(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Response to DPH+DPP."]
        #[inline(always)]
        pub const fn set_uh_tx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "For burst transmissions, this bit simply represents the LPF/EOB of the last packet,. and the LPF/EOB of the preceding packet uses a fixed value of 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_lpf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "For burst transmissions, this bit simply represents the LPF/EOB of the last packet,. and the LPF/EOB of the preceding packet uses a fixed value of 0."]
        #[inline(always)]
        pub const fn set_uh_tx_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Indicates that the packet sent by the host is STATUS TP."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_status(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the packet sent by the host is STATUS TP."]
        #[inline(always)]
        pub const fn set_uh_tx_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Indicates that the packet sent by the host is a Setup packet, and the setup flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_setup(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the packet sent by the host is a Setup packet, and the setup flag is set."]
        #[inline(always)]
        pub const fn set_uh_tx_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "The host prepares to send ISO packets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_iso(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "The host prepares to send ISO packets."]
        #[inline(always)]
        pub const fn set_uh_tx_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "The IN transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The IN transaction completes the interrupt flag, the software writes 0 to zero, and the hardware sets 1."]
        #[inline(always)]
        pub const fn set_uh_tx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for UhTxCtrl {
        #[inline(always)]
        fn default() -> UhTxCtrl {
            UhTxCtrl(0)
        }
    }
    impl core::fmt::Debug for UhTxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxCtrl")
                .field("uh_tx_len", &self.uh_tx_len())
                .field("uh_tx_ep", &self.uh_tx_ep())
                .field("uh_tx_seq", &self.uh_tx_seq())
                .field("uh_tx_res", &self.uh_tx_res())
                .field("uh_tx_lpf", &self.uh_tx_lpf())
                .field("uh_tx_status", &self.uh_tx_status())
                .field("uh_tx_setup", &self.uh_tx_setup())
                .field("uh_tx_iso", &self.uh_tx_iso())
                .field("uh_tx_act", &self.uh_tx_act())
                .finish()
        }
    }
    #[doc = "Send buffer address registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDma(pub u32);
    impl UhTxDma {
        #[doc = "The start address of the host sending buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the host sending buffer."]
        #[inline(always)]
        pub const fn set_uh_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UhTxDma {
        #[inline(always)]
        fn default() -> UhTxDma {
            UhTxDma(0)
        }
    }
    impl core::fmt::Debug for UhTxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxDma")
                .field("uh_tx_dma", &self.uh_tx_dma())
                .finish()
        }
    }
    #[doc = "Host Transmit Address Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDmaOfs(pub u32);
    impl UhTxDmaOfs {
        #[doc = "After the host sends the DMA, the size of the DMA address offset."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_dma_ofs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After the host sends the DMA, the size of the DMA address offset."]
        #[inline(always)]
        pub const fn set_uh_tx_dma_ofs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UhTxDmaOfs {
        #[inline(always)]
        fn default() -> UhTxDmaOfs {
            UhTxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for UhTxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxDmaOfs")
                .field("uh_tx_dma_ofs", &self.uh_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "USBSS Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbssCtrl(pub u32);
    impl UsbssCtrl {
        #[doc = "Enable DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA."]
        #[inline(always)]
        pub const fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reset all software configuration registers, high validity, software clearance required."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reset all software configuration registers, high validity, software clearance required."]
        #[inline(always)]
        pub const fn set_usb_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The protocol layer and FIFO module are reset and need to be cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn force_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The protocol layer and FIFO module are reset and need to be cleared by software."]
        #[inline(always)]
        pub const fn set_force_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1: do not prefetch the next packet of a burst into the FIFO; 0: prefetch it (reset value), which shortens the gap between the DPHs of a burst."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: do not prefetch the next packet of a burst into the FIFO; 0: prefetch it (reset value), which shortens the gap between the DPHs of a burst."]
        #[inline(always)]
        pub const fn set_dma_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Not in the RM, which marks bit 4 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[must_use]
        #[inline(always)]
        pub const fn dir_abort(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Not in the RM, which marks bit 4 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[inline(always)]
        pub const fn set_dir_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SETUP flow control. 1: the control endpoint enters flow control when a SETUP transaction arrives; 0: it does not."]
        #[must_use]
        #[inline(always)]
        pub const fn setup_flow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP flow control. 1: the control endpoint enters flow control when a SETUP transaction arrives; 0: it does not."]
        #[inline(always)]
        pub const fn set_setup_flow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "In host mode, send ITP enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "In host mode, send ITP enabled."]
        #[inline(always)]
        pub const fn set_itp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB Operating Mode Selection Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn host_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Operating Mode Selection Bits."]
        #[inline(always)]
        pub const fn set_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Packet Pending bit of the TPs and DPs sent. In a burst, bit 0 is the PP of the last packet and bit 1 the PP of the others; otherwise only bit 0 is used."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_hp_pend(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Packet Pending bit of the TPs and DPs sent. In a burst, bit 0 is the PP of the last packet and bit 1 the PP of the others; otherwise only bit 0 is used."]
        #[inline(always)]
        pub const fn set_reg_hp_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Device mode. 1: no ERDY-TP is inserted between two DPHs; 0: an ERDY-TP may be inserted between two DPHs."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_erdy_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Device mode. 1: no ERDY-TP is inserted between two DPHs; 0: an ERDY-TP may be inserted between two DPHs."]
        #[inline(always)]
        pub const fn set_tx_erdy_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Not in the RM, which marks bit 15 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_conflict(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Not in the RM, which marks bit 15 reserved. Defined by the EVT header; the SDK examples do not use it."]
        #[inline(always)]
        pub const fn set_chain_conflict(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB Transaction Completion Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_transfer(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "USB Transaction Completion Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_uie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive SETUP Transaction Completion Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_setup(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SETUP Transaction Completion Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_udie_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Receive ERDY-TP Complete Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_erdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive ERDY-TP Complete Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_uhie_erdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Receiving STATUS transaction completion interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_status(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Receiving STATUS transaction completion interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Interrupt enabled for receiving DEV_NOTIF-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_notif(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enabled for receiving DEV_NOTIF-TP."]
        #[inline(always)]
        pub const fn set_uhie_notif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Interrupt Enabled for Receiving PING-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_rx_ping(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enabled for Receiving PING-TP."]
        #[inline(always)]
        pub const fn set_uie_rx_ping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Send ITP to complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_itp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Send ITP to complete interrupt enable."]
        #[inline(always)]
        pub const fn set_uie_itp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Sending FIFO overflow interrupt enables."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_fifo_txov(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Sending FIFO overflow interrupt enables."]
        #[inline(always)]
        pub const fn set_uie_fifo_txov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Receive FIFO overflow interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_fifo_rxov(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO overflow interrupt enabled."]
        #[inline(always)]
        pub const fn set_uie_fifo_rxov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Host mode: address of the device being accessed. Device mode: address of this device."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_addr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Host mode: address of the device being accessed. Device mode: address of this device."]
        #[inline(always)]
        pub const fn set_dev_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for UsbssCtrl {
        #[inline(always)]
        fn default() -> UsbssCtrl {
            UsbssCtrl(0)
        }
    }
    impl core::fmt::Debug for UsbssCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbssCtrl")
                .field("dma_en", &self.dma_en())
                .field("usb_clr_all", &self.usb_clr_all())
                .field("force_rst", &self.force_rst())
                .field("dma_mode", &self.dma_mode())
                .field("dir_abort", &self.dir_abort())
                .field("setup_flow", &self.setup_flow())
                .field("itp_en", &self.itp_en())
                .field("host_mode", &self.host_mode())
                .field("reg_hp_pend", &self.reg_hp_pend())
                .field("tx_erdy_mode", &self.tx_erdy_mode())
                .field("chain_conflict", &self.chain_conflict())
                .field("uie_transfer", &self.uie_transfer())
                .field("udie_setup", &self.udie_setup())
                .field("uhie_erdy", &self.uhie_erdy())
                .field("udie_status", &self.udie_status())
                .field("uhie_notif", &self.uhie_notif())
                .field("uie_rx_ping", &self.uie_rx_ping())
                .field("uie_itp", &self.uie_itp())
                .field("uie_fifo_txov", &self.uie_fifo_txov())
                .field("uie_fifo_rxov", &self.uie_fifo_rxov())
                .field("dev_addr", &self.dev_addr())
                .finish()
        }
    }
}
pub mod vals {
    #[doc = "Endpoint 0 response to a DPH. The value 3 is invalid (no response)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ep0RxRes {
        #[doc = "NRDY-TP."]
        NRDY = 0x0,
        #[doc = "ACK-TP."]
        ACK = 0x01,
        #[doc = "STALL-TP."]
        STALL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ep0RxRes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ep0RxRes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ep0RxRes {
        #[inline(always)]
        fn from(val: u8) -> Ep0RxRes {
            Ep0RxRes::from_bits(val)
        }
    }
    impl From<Ep0RxRes> for u8 {
        #[inline(always)]
        fn from(val: Ep0RxRes) -> u8 {
            Ep0RxRes::to_bits(val)
        }
    }
    #[doc = "Endpoint 0 response to an ACK-TP. The value 3 is invalid (no response)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ep0TxRes {
        #[doc = "NRDY-TP."]
        NRDY = 0x0,
        #[doc = "Data packet."]
        DPH = 0x01,
        #[doc = "STALL-TP."]
        STALL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ep0TxRes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ep0TxRes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ep0TxRes {
        #[inline(always)]
        fn from(val: u8) -> Ep0TxRes {
            Ep0TxRes::from_bits(val)
        }
    }
    impl From<Ep0TxRes> for u8 {
        #[inline(always)]
        fn from(val: Ep0TxRes) -> u8 {
            Ep0TxRes::to_bits(val)
        }
    }
    #[doc = "Link state (LTSSM). Other values are reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum LinkState {
        #[doc = "U0."]
        U0 = 0x0,
        #[doc = "U1."]
        U1 = 0x01,
        #[doc = "U2."]
        U2 = 0x02,
        #[doc = "U3."]
        U3 = 0x03,
        #[doc = "SS.Disabled."]
        DISABLED = 0x04,
        #[doc = "Rx.Detect."]
        RXDET = 0x05,
        #[doc = "SS.Inactive."]
        INACTIVE = 0x06,
        #[doc = "Polling."]
        POLLING = 0x07,
        #[doc = "Recovery."]
        RECOVERY = 0x08,
        #[doc = "Hot Reset."]
        HOTRST = 0x09,
        #[doc = "Compliance Mode."]
        COMPLIANCE = 0x0a,
        #[doc = "Loopback."]
        LOOPBACK = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl LinkState {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LinkState {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LinkState {
        #[inline(always)]
        fn from(val: u8) -> LinkState {
            LinkState::from_bits(val)
        }
    }
    impl From<LinkState> for u8 {
        #[inline(always)]
        fn from(val: LinkState) -> u8 {
            LinkState::to_bits(val)
        }
    }
    #[doc = "PHY power mode, P0 to P3 of the PIPE."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PdMode {
        #[doc = "Normal operation."]
        P0 = 0x0,
        #[doc = "Low power. Both ends of the port send PING.LFPS periodically to keep the connection."]
        P1 = 0x01,
        #[doc = "Lower power than P1, with a longer exit time."]
        P2 = 0x02,
        #[doc = "Lowest power, the 125 MHz PHY clock is off."]
        P3 = 0x03,
    }
    impl PdMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PdMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PdMode {
        #[inline(always)]
        fn from(val: u8) -> PdMode {
            PdMode::from_bits(val)
        }
    }
    impl From<PdMode> for u8 {
        #[inline(always)]
        fn from(val: PdMode) -> u8 {
            PdMode::to_bits(val)
        }
    }
    #[doc = "Transmitter de-emphasis, TxDeemph of the PIPE. The value 3 is reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TxDeemph {
        #[doc = "-6.0 dB."]
        DB6 = 0x0,
        #[doc = "-3.5 dB."]
        DB3_5 = 0x01,
        #[doc = "No de-emphasis."]
        NONE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl TxDeemph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TxDeemph {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TxDeemph {
        #[inline(always)]
        fn from(val: u8) -> TxDeemph {
            TxDeemph::from_bits(val)
        }
    }
    impl From<TxDeemph> for u8 {
        #[inline(always)]
        fn from(val: TxDeemph) -> u8 {
            TxDeemph::to_bits(val)
        }
    }
}
