#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

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
    #[doc = "LMP receives data 0 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "LMP receives data 1 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "LMP receives data 2 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "USB Custom HP Data 0 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "USB Custom HP Data 1 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "USB Custom HP Data 2 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
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
    #[doc = "DEV_NOTIF-TP Data 0 Register."]
    #[inline(always)]
    pub const fn tp_rx_data0(self) -> crate::common::Reg<regs::TpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 1 Register."]
    #[inline(always)]
    pub const fn tp_rx_data1(self) -> crate::common::Reg<regs::TpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 2 Register."]
    #[inline(always)]
    pub const fn tp_rx_data2(self) -> crate::common::Reg<regs::TpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
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
    #[doc = "LMP receives data 0 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "LMP receives data 1 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "LMP receives data 2 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "USB Custom HP Data 0 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "USB Custom HP Data 1 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "USB Custom HP Data 2 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
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
    #[doc = "DMA start address for Endpoint 0 sending."]
    #[inline(always)]
    pub const fn u3ep0_tx_dma(self) -> crate::common::Reg<regs::U3ep0TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "DMA start address for Endpoint 0 receiving."]
    #[inline(always)]
    pub const fn u3ep0_rx_dma(self) -> crate::common::Reg<regs::U3ep0RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Endpoint 0 transmit DMA offset. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn uep0_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep0TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Endpoint 0 receive DMA offset. The RM register table omits this register; the EVT device header exposes it."]
    #[inline(always)]
    pub const fn uep0_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep0RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 0 Register."]
    #[inline(always)]
    pub const fn tp_rx_data0(self) -> crate::common::Reg<regs::TpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 1 Register."]
    #[inline(always)]
    pub const fn tp_rx_data1(self) -> crate::common::Reg<regs::TpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 2 Register."]
    #[inline(always)]
    pub const fn tp_rx_data2(self) -> crate::common::Reg<regs::TpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[inline(always)]
    pub const fn uep1_tx_cfg(self) -> crate::common::Reg<regs::Uep1TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Endpoint 1 control register."]
    #[inline(always)]
    pub const fn uep1_tx_cr(self) -> crate::common::Reg<regs::Uep1TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc1usize) as _) }
    }
    #[doc = "Endpoint 1 Serial Number Register."]
    #[inline(always)]
    pub const fn uep1_tx_seq(self) -> crate::common::Reg<regs::Uep1TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Endpoint 1 status register."]
    #[inline(always)]
    pub const fn uep1_tx_st(self) -> crate::common::Reg<regs::Uep1TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc3usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[inline(always)]
    pub const fn uep1_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep1TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN state register."]
    #[inline(always)]
    pub const fn uep1_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep1TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc5usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep1_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep1TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 1."]
    #[inline(always)]
    pub const fn uep1_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep1TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by endpoint n."]
    #[inline(always)]
    pub const fn uep1_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep1TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc9usize) as _) }
    }
    #[doc = "DMA offset length for endpoint 1."]
    #[inline(always)]
    pub const fn uep1_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep1TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize) as _) }
    }
    #[doc = "DMA start address for endpoint 1."]
    #[inline(always)]
    pub const fn uep1_tx_dma(self) -> crate::common::Reg<regs::Uep1TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[inline(always)]
    pub const fn uep1_rx_cfg(self) -> crate::common::Reg<regs::Uep1RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Endpoint 1 control register."]
    #[inline(always)]
    pub const fn uep1_rx_cr(self) -> crate::common::Reg<regs::Uep1RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd1usize) as _) }
    }
    #[doc = "Endpoint 1 Serial Number Register."]
    #[inline(always)]
    pub const fn uep1_rx_seq(self) -> crate::common::Reg<regs::Uep1RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd2usize) as _) }
    }
    #[doc = "Endpoint 1 status register."]
    #[inline(always)]
    pub const fn uep1_rx_st(self) -> crate::common::Reg<regs::Uep1RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd3usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[inline(always)]
    pub const fn uep1_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep1RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN state register."]
    #[inline(always)]
    pub const fn uep1_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep1RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd5usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep1_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep1RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd6usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 1 can receive."]
    #[inline(always)]
    pub const fn uep1_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep1RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by endpoint 1."]
    #[inline(always)]
    pub const fn uep1_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep1RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd9usize) as _) }
    }
    #[doc = "DMA offset length for endpoint 1."]
    #[inline(always)]
    pub const fn uep1_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep1RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdausize) as _) }
    }
    #[doc = "DMA start address for endpoint 1."]
    #[inline(always)]
    pub const fn uep1_rx_dma(self) -> crate::common::Reg<regs::Uep1RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Endpoint 2 Configuration Register."]
    #[inline(always)]
    pub const fn uep2_tx_cfg(self) -> crate::common::Reg<regs::Uep2TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Endpoint 2 control register."]
    #[inline(always)]
    pub const fn uep2_tx_cr(self) -> crate::common::Reg<regs::Uep2TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe1usize) as _) }
    }
    #[doc = "Endpoint 2 Serial Number Register."]
    #[inline(always)]
    pub const fn uep2_tx_seq(self) -> crate::common::Reg<regs::Uep2TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Endpoint 2 status register."]
    #[inline(always)]
    pub const fn uep2_tx_st(self) -> crate::common::Reg<regs::Uep2TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe3usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN control register."]
    #[inline(always)]
    pub const fn uep2_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep2TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN state register."]
    #[inline(always)]
    pub const fn uep2_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep2TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe5usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep2_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep2TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 2."]
    #[inline(always)]
    pub const fn uep2_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep2TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by endpoint 2."]
    #[inline(always)]
    pub const fn uep2_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep2TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe9usize) as _) }
    }
    #[doc = "DMA offset length for endpoint 2."]
    #[inline(always)]
    pub const fn uep2_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep2TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "DMA start address for endpoint 2."]
    #[inline(always)]
    pub const fn uep2_tx_dma(self) -> crate::common::Reg<regs::Uep2TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Endpoint 2 Configuration Register."]
    #[inline(always)]
    pub const fn uep2_rx_cfg(self) -> crate::common::Reg<regs::Uep2RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Endpoint 2 control register."]
    #[inline(always)]
    pub const fn uep2_rx_cr(self) -> crate::common::Reg<regs::Uep2RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf1usize) as _) }
    }
    #[doc = "Endpoint 2 Serial Number Register."]
    #[inline(always)]
    pub const fn uep2_rx_seq(self) -> crate::common::Reg<regs::Uep2RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Endpoint 2 status register."]
    #[inline(always)]
    pub const fn uep2_rx_st(self) -> crate::common::Reg<regs::Uep2RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf3usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN control register."]
    #[inline(always)]
    pub const fn uep2_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep2RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN state register."]
    #[inline(always)]
    pub const fn uep2_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep2RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf5usize) as _) }
    }
    #[doc = "Endpoint 2 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep2_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep2RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf6usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 2 can receive."]
    #[inline(always)]
    pub const fn uep2_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep2RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by endpoint 2."]
    #[inline(always)]
    pub const fn uep2_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep2RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf9usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 2."]
    #[inline(always)]
    pub const fn uep2_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep2RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 2."]
    #[inline(always)]
    pub const fn uep2_rx_dma(self) -> crate::common::Reg<regs::Uep2RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Endpoint 3 Configuration Register."]
    #[inline(always)]
    pub const fn uep3_tx_cfg(self) -> crate::common::Reg<regs::Uep3TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Endpoint 3 control register."]
    #[inline(always)]
    pub const fn uep3_tx_cr(self) -> crate::common::Reg<regs::Uep3TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0101usize) as _) }
    }
    #[doc = "Endpoint 3 Serial Number Register."]
    #[inline(always)]
    pub const fn uep3_tx_seq(self) -> crate::common::Reg<regs::Uep3TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Endpoint 3 status register."]
    #[inline(always)]
    pub const fn uep3_tx_st(self) -> crate::common::Reg<regs::Uep3TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN control register."]
    #[inline(always)]
    pub const fn uep3_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep3TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN state register."]
    #[inline(always)]
    pub const fn uep3_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep3TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0105usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep3_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep3TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 3."]
    #[inline(always)]
    pub const fn uep3_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep3TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by endpoint 3."]
    #[inline(always)]
    pub const fn uep3_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep3TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "DMA offset length for endpoint 3."]
    #[inline(always)]
    pub const fn uep3_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep3TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "DMA start address for endpoint 3."]
    #[inline(always)]
    pub const fn uep3_tx_dma(self) -> crate::common::Reg<regs::Uep3TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Endpoint 3 Configuration Register."]
    #[inline(always)]
    pub const fn uep3_rx_cfg(self) -> crate::common::Reg<regs::Uep3RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Endpoint 3 control register."]
    #[inline(always)]
    pub const fn uep3_rx_cr(self) -> crate::common::Reg<regs::Uep3RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0111usize) as _) }
    }
    #[doc = "Endpoint 3 Serial Number Register."]
    #[inline(always)]
    pub const fn uep3_rx_seq(self) -> crate::common::Reg<regs::Uep3RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0112usize) as _) }
    }
    #[doc = "Endpoint 3 status register."]
    #[inline(always)]
    pub const fn uep3_rx_st(self) -> crate::common::Reg<regs::Uep3RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0113usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN control register."]
    #[inline(always)]
    pub const fn uep3_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep3RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN state register."]
    #[inline(always)]
    pub const fn uep3_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep3RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0115usize) as _) }
    }
    #[doc = "Endpoint 3 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep3_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep3RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0116usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 3 can receive."]
    #[inline(always)]
    pub const fn uep3_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep3RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 3."]
    #[inline(always)]
    pub const fn uep3_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep3RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0119usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 3."]
    #[inline(always)]
    pub const fn uep3_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep3RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 3."]
    #[inline(always)]
    pub const fn uep3_rx_dma(self) -> crate::common::Reg<regs::Uep3RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Endpoint 4 Configuration Register."]
    #[inline(always)]
    pub const fn uep4_tx_cfg(self) -> crate::common::Reg<regs::Uep4TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Endpoint 4 control register."]
    #[inline(always)]
    pub const fn uep4_tx_cr(self) -> crate::common::Reg<regs::Uep4TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0121usize) as _) }
    }
    #[doc = "Endpoint 4 Serial Number Register."]
    #[inline(always)]
    pub const fn uep4_tx_seq(self) -> crate::common::Reg<regs::Uep4TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Endpoint 4 status register."]
    #[inline(always)]
    pub const fn uep4_tx_st(self) -> crate::common::Reg<regs::Uep4TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0123usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN control register."]
    #[inline(always)]
    pub const fn uep4_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep4TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN state register."]
    #[inline(always)]
    pub const fn uep4_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep4TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0125usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep4_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep4TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep4TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep4TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0129usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep4TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_tx_dma(self) -> crate::common::Reg<regs::Uep4TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Endpoint 4 Configuration Register."]
    #[inline(always)]
    pub const fn uep4_rx_cfg(self) -> crate::common::Reg<regs::Uep4RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Endpoint 4 control register."]
    #[inline(always)]
    pub const fn uep4_rx_cr(self) -> crate::common::Reg<regs::Uep4RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0131usize) as _) }
    }
    #[doc = "Endpoint 4 Serial Number Register."]
    #[inline(always)]
    pub const fn uep4_rx_seq(self) -> crate::common::Reg<regs::Uep4RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0132usize) as _) }
    }
    #[doc = "Endpoint 4 status register."]
    #[inline(always)]
    pub const fn uep4_rx_st(self) -> crate::common::Reg<regs::Uep4RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0133usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN control register."]
    #[inline(always)]
    pub const fn uep4_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep4RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN state register."]
    #[inline(always)]
    pub const fn uep4_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep4RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0135usize) as _) }
    }
    #[doc = "Endpoint 4 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep4_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep4RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0136usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 4 can receive."]
    #[inline(always)]
    pub const fn uep4_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep4RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep4RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0139usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep4RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 4."]
    #[inline(always)]
    pub const fn uep4_rx_dma(self) -> crate::common::Reg<regs::Uep4RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Endpoint 5 Configuration Register."]
    #[inline(always)]
    pub const fn uep5_tx_cfg(self) -> crate::common::Reg<regs::Uep5TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Endpoint 5 control register."]
    #[inline(always)]
    pub const fn uep5_tx_cr(self) -> crate::common::Reg<regs::Uep5TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0141usize) as _) }
    }
    #[doc = "Endpoint 5 Serial Number Register."]
    #[inline(always)]
    pub const fn uep5_tx_seq(self) -> crate::common::Reg<regs::Uep5TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "Endpoint 5 status register."]
    #[inline(always)]
    pub const fn uep5_tx_st(self) -> crate::common::Reg<regs::Uep5TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0143usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN control register."]
    #[inline(always)]
    pub const fn uep5_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep5TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN state register."]
    #[inline(always)]
    pub const fn uep5_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep5TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0145usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep5_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep5TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep5TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep5TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0149usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep5TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_tx_dma(self) -> crate::common::Reg<regs::Uep5TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Endpoint 5 Configuration Register."]
    #[inline(always)]
    pub const fn uep5_rx_cfg(self) -> crate::common::Reg<regs::Uep5RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Endpoint 5 control register."]
    #[inline(always)]
    pub const fn uep5_rx_cr(self) -> crate::common::Reg<regs::Uep5RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0151usize) as _) }
    }
    #[doc = "Endpoint 5 Serial Number Register."]
    #[inline(always)]
    pub const fn uep5_rx_seq(self) -> crate::common::Reg<regs::Uep5RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "Endpoint 5 status register."]
    #[inline(always)]
    pub const fn uep5_rx_st(self) -> crate::common::Reg<regs::Uep5RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0153usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN control register."]
    #[inline(always)]
    pub const fn uep5_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep5RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN state register."]
    #[inline(always)]
    pub const fn uep5_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep5RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0155usize) as _) }
    }
    #[doc = "Endpoint 5 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep5_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep5RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0156usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 5 can receive."]
    #[inline(always)]
    pub const fn uep5_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep5RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep5RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0159usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep5RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 5."]
    #[inline(always)]
    pub const fn uep5_rx_dma(self) -> crate::common::Reg<regs::Uep5RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Endpoint 6 Configuration Register."]
    #[inline(always)]
    pub const fn uep6_tx_cfg(self) -> crate::common::Reg<regs::Uep6TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Endpoint 6 control register."]
    #[inline(always)]
    pub const fn uep6_tx_cr(self) -> crate::common::Reg<regs::Uep6TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0161usize) as _) }
    }
    #[doc = "Endpoint 6 Serial Number Register."]
    #[inline(always)]
    pub const fn uep6_tx_seq(self) -> crate::common::Reg<regs::Uep6TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Endpoint 6 status register."]
    #[inline(always)]
    pub const fn uep6_tx_st(self) -> crate::common::Reg<regs::Uep6TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0163usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN control register."]
    #[inline(always)]
    pub const fn uep6_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep6TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN state register."]
    #[inline(always)]
    pub const fn uep6_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep6TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0165usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep6_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep6TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep6TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep6TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0169usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep6TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_tx_dma(self) -> crate::common::Reg<regs::Uep6TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Endpoint 6 Configuration Register."]
    #[inline(always)]
    pub const fn uep6_rx_cfg(self) -> crate::common::Reg<regs::Uep6RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Endpoint 6 control register."]
    #[inline(always)]
    pub const fn uep6_rx_cr(self) -> crate::common::Reg<regs::Uep6RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0171usize) as _) }
    }
    #[doc = "Endpoint 6 Serial Number Register."]
    #[inline(always)]
    pub const fn uep6_rx_seq(self) -> crate::common::Reg<regs::Uep6RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0172usize) as _) }
    }
    #[doc = "Endpoint 6 status register."]
    #[inline(always)]
    pub const fn uep6_rx_st(self) -> crate::common::Reg<regs::Uep6RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0173usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN control register."]
    #[inline(always)]
    pub const fn uep6_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep6RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN state register."]
    #[inline(always)]
    pub const fn uep6_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep6RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0175usize) as _) }
    }
    #[doc = "Endpoint 6 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep6_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep6RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0176usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 6 can receive."]
    #[inline(always)]
    pub const fn uep6_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep6RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep6RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0179usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep6RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 6."]
    #[inline(always)]
    pub const fn uep6_rx_dma(self) -> crate::common::Reg<regs::Uep6RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Endpoint 7 Configuration Register."]
    #[inline(always)]
    pub const fn uep7_tx_cfg(self) -> crate::common::Reg<regs::Uep7TxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Endpoint 7 control register."]
    #[inline(always)]
    pub const fn uep7_tx_cr(self) -> crate::common::Reg<regs::Uep7TxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0181usize) as _) }
    }
    #[doc = "Endpoint 7 Serial Number Register."]
    #[inline(always)]
    pub const fn uep7_tx_seq(self) -> crate::common::Reg<regs::Uep7TxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "Endpoint 7 status register."]
    #[inline(always)]
    pub const fn uep7_tx_st(self) -> crate::common::Reg<regs::Uep7TxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0183usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN control register."]
    #[inline(always)]
    pub const fn uep7_tx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep7TxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN state register."]
    #[inline(always)]
    pub const fn uep7_tx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep7TxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0185usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep7_tx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep7TxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0186usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_tx_chain_exp_nump(
        self,
    ) -> crate::common::Reg<regs::Uep7TxChainExpNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_tx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep7TxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0189usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_tx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep7TxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_tx_dma(self) -> crate::common::Reg<regs::Uep7TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Endpoint 7 Configuration Register."]
    #[inline(always)]
    pub const fn uep7_rx_cfg(self) -> crate::common::Reg<regs::Uep7RxCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Endpoint 7 control register."]
    #[inline(always)]
    pub const fn uep7_rx_cr(self) -> crate::common::Reg<regs::Uep7RxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0191usize) as _) }
    }
    #[doc = "Endpoint 7 Serial Number Register."]
    #[inline(always)]
    pub const fn uep7_rx_seq(self) -> crate::common::Reg<regs::Uep7RxSeq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0192usize) as _) }
    }
    #[doc = "Endpoint 7 status register."]
    #[inline(always)]
    pub const fn uep7_rx_st(self) -> crate::common::Reg<regs::Uep7RxSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0193usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN control register."]
    #[inline(always)]
    pub const fn uep7_rx_chain_cr(
        self,
    ) -> crate::common::Reg<regs::Uep7RxChainCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN state register."]
    #[inline(always)]
    pub const fn uep7_rx_chain_st(
        self,
    ) -> crate::common::Reg<regs::Uep7RxChainSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0195usize) as _) }
    }
    #[doc = "Endpoint 7 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep7_rx_chain_len(
        self,
    ) -> crate::common::Reg<regs::Uep7RxChainLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0196usize) as _) }
    }
    #[doc = "Number of NUMPs that Endpoint 7 can receive."]
    #[inline(always)]
    pub const fn uep7_rx_chain_max_nump(
        self,
    ) -> crate::common::Reg<regs::Uep7RxChainMaxNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_rx_chain_nump(
        self,
    ) -> crate::common::Reg<regs::Uep7RxChainNump, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0199usize) as _) }
    }
    #[doc = "DMA offset length for Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_rx_dma_ofs(
        self,
    ) -> crate::common::Reg<regs::Uep7RxDmaOfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019ausize) as _) }
    }
    #[doc = "DMA start address for Endpoint 7."]
    #[inline(always)]
    pub const fn uep7_rx_dma(self) -> crate::common::Reg<regs::Uep7RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
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
    #[doc = "LMP receives data 0 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "LMP receives data 1 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "LMP receives data 2 register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "USB Custom HP Data 0 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "USB Custom HP Data 1 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "USB Custom HP Data 2 Register."]
    #[inline(always)]
    pub const fn link_lmp_tx_data2(
        self,
    ) -> crate::common::Reg<regs::LinkLmpTxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
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
    #[doc = "DEV_NOTIF-TP Data 0 Register."]
    #[inline(always)]
    pub const fn tp_rx_data0(self) -> crate::common::Reg<regs::TpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 1 Register."]
    #[inline(always)]
    pub const fn tp_rx_data1(self) -> crate::common::Reg<regs::TpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "DEV_NOTIF-TP Data 2 Register."]
    #[inline(always)]
    pub const fn tp_rx_data2(self) -> crate::common::Reg<regs::TpRxData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
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
        #[doc = "Transmitter de-emphasis control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_deemph(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Transmitter de-emphasis control."]
        #[inline(always)]
        pub const fn set_link_tx_deemph(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
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
        pub const fn link_lookback_act(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "It is used in LOOPBACK mode for the LOOPBACK master to control the pattern transmission (the LOOPBACK slave keeps 0),. and the ACT is a high-level signal that lasts for a period of time."]
        #[inline(always)]
        pub const fn set_link_lookback_act(&mut self, val: bool) {
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
        #[doc = "High validity, after receiving the LGO_U2, the response LXU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_allow(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, after receiving the LGO_U2, the response LXU allows to enter the U2 state,. otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[inline(always)]
        pub const fn set_link_u2_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        #[doc = "SPEC configure."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tout_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SPEC configure."]
        #[inline(always)]
        pub const fn set_link_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("link_lookback_act", &self.link_lookback_act())
                .field("link_ltssm_mode", &self.link_ltssm_mode())
                .field("link_u1_allow", &self.link_u1_allow())
                .field("link_u2_allow", &self.link_u2_allow())
                .field("link_u1_ping_en", &self.link_u1_ping_en())
                .field("link_tout_mode", &self.link_tout_mode())
                .field("link_reset", &self.link_reset())
                .finish()
        }
    }
    #[doc = "LINK control registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkCtrl(pub u32);
    impl LinkCtrl {
        #[doc = "Configure the current power mode of the PHY, corresponding to PO/P1/P2/P3 in the PIPE."]
        #[must_use]
        #[inline(always)]
        pub const fn link_pd_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Configure the current power mode of the PHY, corresponding to PO/P1/P2/P3 in the PIPE."]
        #[inline(always)]
        pub const fn set_link_pd_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
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
        #[doc = "LINK ENTERS THE SS.ACTIVE STATE INTERRUPT FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_inactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK ENTERS THE SS.ACTIVE STATE INTERRUPT FLAG."]
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
        #[doc = "he device receives the HOT RESET interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "he device receives the HOT RESET interrupt flag."]
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
        #[doc = "link_reset reset to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_state_chg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "link_reset reset to 0."]
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
        #[doc = "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 years after entering the U0 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_cap_vld(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "A valid PORT_CAP-LMP is received, and the protocol stipulates that. the two LINK parties exchange PORT_CAP-LMP within 20 years after entering the U0 state."]
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
                .field("link_lmp_tx_cap_vld", &self.link_lmp_tx_cap_vld())
                .field("link_lmp_rx_cap_vld", &self.link_lmp_rx_cap_vld())
                .finish()
        }
    }
    #[doc = "LMP receives data 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData0(pub u32);
    impl LinkLmpRxData0 {
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_rx_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpRxData0 {
        #[inline(always)]
        fn default() -> LinkLmpRxData0 {
            LinkLmpRxData0(0)
        }
    }
    impl core::fmt::Debug for LinkLmpRxData0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpRxData0")
                .field("link_lmp_rx_data0", &self.link_lmp_rx_data0())
                .finish()
        }
    }
    #[doc = "LMP receives data 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData1(pub u32);
    impl LinkLmpRxData1 {
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[63:32\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_rx_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpRxData1 {
        #[inline(always)]
        fn default() -> LinkLmpRxData1 {
            LinkLmpRxData1(0)
        }
    }
    impl core::fmt::Debug for LinkLmpRxData1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpRxData1")
                .field("link_lmp_rx_data1", &self.link_lmp_rx_data1())
                .finish()
        }
    }
    #[doc = "LMP receives data 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData2(pub u32);
    impl LinkLmpRxData2 {
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[95:64\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Once the LMP is received, the HP data is stored in this register \\[95:64\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_rx_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpRxData2 {
        #[inline(always)]
        fn default() -> LinkLmpRxData2 {
            LinkLmpRxData2(0)
        }
    }
    impl core::fmt::Debug for LinkLmpRxData2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpRxData2")
                .field("link_lmp_rx_data1", &self.link_lmp_rx_data1())
                .finish()
        }
    }
    #[doc = "USB Custom HP Data 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpTxData0(pub u32);
    impl LinkLmpTxData0 {
        #[doc = "The data of the user-defined HP is sent \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data of the user-defined HP is sent \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_tx_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpTxData0 {
        #[inline(always)]
        fn default() -> LinkLmpTxData0 {
            LinkLmpTxData0(0)
        }
    }
    impl core::fmt::Debug for LinkLmpTxData0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpTxData0")
                .field("link_lmp_tx_data0", &self.link_lmp_tx_data0())
                .finish()
        }
    }
    #[doc = "USB Custom HP Data 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpTxData1(pub u32);
    impl LinkLmpTxData1 {
        #[doc = "The data of the user-defined HP is sent \\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data of the user-defined HP is sent \\[63:32\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_tx_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpTxData1 {
        #[inline(always)]
        fn default() -> LinkLmpTxData1 {
            LinkLmpTxData1(0)
        }
    }
    impl core::fmt::Debug for LinkLmpTxData1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpTxData1")
                .field("link_lmp_tx_data1", &self.link_lmp_tx_data1())
                .finish()
        }
    }
    #[doc = "USB Custom HP Data 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpTxData2(pub u32);
    impl LinkLmpTxData2 {
        #[doc = "The data of the user-defined HP is sent \\[95:64\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data of the user-defined HP is sent \\[95:64\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_tx_data2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LinkLmpTxData2 {
        #[inline(always)]
        fn default() -> LinkLmpTxData2 {
            LinkLmpTxData2(0)
        }
    }
    impl core::fmt::Debug for LinkLmpTxData2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LinkLmpTxData2")
                .field("link_lmp_tx_data2", &self.link_lmp_tx_data2())
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
        #[doc = "Link Power Status."]
        #[must_use]
        #[inline(always)]
        pub const fn link_pd_mode_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Link Power Status."]
        #[inline(always)]
        pub const fn set_link_pd_mode_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
        #[doc = "Link Status."]
        #[must_use]
        #[inline(always)]
        pub const fn link_state(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Link Status."]
        #[inline(always)]
        pub const fn set_link_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
    #[doc = "DEV_NOTIF-TP Data 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpRxData0(pub u32);
    impl TpRxData0 {
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn usb3_notif_data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[inline(always)]
        pub const fn set_usb3_notif_data0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpRxData0 {
        #[inline(always)]
        fn default() -> TpRxData0 {
            TpRxData0(0)
        }
    }
    impl core::fmt::Debug for TpRxData0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TpRxData0")
                .field("usb3_notif_data0", &self.usb3_notif_data0())
                .finish()
        }
    }
    #[doc = "DEV_NOTIF-TP Data 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpRxData1(pub u32);
    impl TpRxData1 {
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn usb3_notif_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[inline(always)]
        pub const fn set_usb3_notif_data1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpRxData1 {
        #[inline(always)]
        fn default() -> TpRxData1 {
            TpRxData1(0)
        }
    }
    impl core::fmt::Debug for TpRxData1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TpRxData1")
                .field("usb3_notif_data1", &self.usb3_notif_data1())
                .finish()
        }
    }
    #[doc = "DEV_NOTIF-TP Data 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpRxData2(pub u32);
    impl TpRxData2 {
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn usb3_notif_data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After the DEV_NOTIF-TP is received, the HP data is stored in this register."]
        #[inline(always)]
        pub const fn set_usb3_notif_data2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpRxData2 {
        #[inline(always)]
        fn default() -> TpRxData2 {
            TpRxData2(0)
        }
    }
    impl core::fmt::Debug for TpRxData2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TpRxData2")
                .field("usb3_notif_data2", &self.usb3_notif_data2())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 0 receiving."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct U3ep0RxDma(pub u32);
    impl U3ep0RxDma {
        #[doc = "The start address of the endpoint 0 receiving buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn u3ep0_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the endpoint 0 receiving buffer."]
        #[inline(always)]
        pub const fn set_u3ep0_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for U3ep0RxDma {
        #[inline(always)]
        fn default() -> U3ep0RxDma {
            U3ep0RxDma(0)
        }
    }
    impl core::fmt::Debug for U3ep0RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("U3ep0RxDma")
                .field("u3ep0_rx_dma", &self.u3ep0_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 0 sending."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct U3ep0TxDma(pub u32);
    impl U3ep0TxDma {
        #[doc = "The start address of the endpoint 0 sending buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn u3ep0_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The start address of the endpoint 0 sending buffer."]
        #[inline(always)]
        pub const fn set_u3ep0_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for U3ep0TxDma {
        #[inline(always)]
        fn default() -> U3ep0TxDma {
            U3ep0TxDma(0)
        }
    }
    impl core::fmt::Debug for U3ep0TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("U3ep0TxDma")
                .field("u3ep0_tx_dma", &self.u3ep0_tx_dma())
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
        #[doc = "Response to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_res(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Response to DPH."]
        #[inline(always)]
        pub const fn set_ep0_rx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
    #[doc = "Endpoint 0 receive DMA offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxDmaOfs(pub u32);
    impl Uep0RxDmaOfs {
        #[doc = "DMA offset value."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_ofs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DMA offset value."]
        #[inline(always)]
        pub const fn set_dma_ofs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep0RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep0RxDmaOfs {
            Uep0RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep0RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxDmaOfs")
                .field("dma_ofs", &self.dma_ofs())
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
        #[doc = "Response to ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_res(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Response to ACK-TP."]
        #[inline(always)]
        pub const fn set_ep0_tx_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
    #[doc = "Endpoint 0 transmit DMA offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxDmaOfs(pub u32);
    impl Uep0TxDmaOfs {
        #[doc = "DMA offset value."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_ofs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DMA offset value."]
        #[inline(always)]
        pub const fn set_dma_ofs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep0TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep0TxDmaOfs {
            Uep0TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep0TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TxDmaOfs")
                .field("dma_ofs", &self.dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxCfg(pub u8);
    impl Uep1RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1RxCfg {
        #[inline(always)]
        fn default() -> Uep1RxCfg {
            Uep1RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep1RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxChainCr(pub u8);
    impl Uep1RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep1RxChainCr {
        #[inline(always)]
        fn default() -> Uep1RxChainCr {
            Uep1RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep1RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxChainLen(pub u16);
    impl Uep1RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep1RxChainLen {
        #[inline(always)]
        fn default() -> Uep1RxChainLen {
            Uep1RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep1RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 1 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxChainMaxNump(pub u8);
    impl Uep1RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep1RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep1RxChainMaxNump {
            Uep1RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep1RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxChainNump(pub u8);
    impl Uep1RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep1RxChainNump {
        #[inline(always)]
        fn default() -> Uep1RxChainNump {
            Uep1RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep1RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxChainSt(pub u8);
    impl Uep1RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1RxChainSt {
        #[inline(always)]
        fn default() -> Uep1RxChainSt {
            Uep1RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep1RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxCr(pub u8);
    impl Uep1RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1RxCr {
        #[inline(always)]
        fn default() -> Uep1RxCr {
            Uep1RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep1RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxDma(pub u32);
    impl Uep1RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep1RxDma {
        #[inline(always)]
        fn default() -> Uep1RxDma {
            Uep1RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep1RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxDmaOfs(pub u16);
    impl Uep1RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep1RxDmaOfs {
            Uep1RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep1RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxSeq(pub u8);
    impl Uep1RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep1RxSeq {
        #[inline(always)]
        fn default() -> Uep1RxSeq {
            Uep1RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep1RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 1 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxSt(pub u8);
    impl Uep1RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1RxSt {
        #[inline(always)]
        fn default() -> Uep1RxSt {
            Uep1RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep1RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxCfg(pub u8);
    impl Uep1TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1TxCfg {
        #[inline(always)]
        fn default() -> Uep1TxCfg {
            Uep1TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep1TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxChainCr(pub u8);
    impl Uep1TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep1TxChainCr {
        #[inline(always)]
        fn default() -> Uep1TxChainCr {
            Uep1TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep1TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxChainExpNump(pub u8);
    impl Uep1TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep1TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep1TxChainExpNump {
            Uep1TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep1TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxChainLen(pub u16);
    impl Uep1TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep1TxChainLen {
        #[inline(always)]
        fn default() -> Uep1TxChainLen {
            Uep1TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep1TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by endpoint n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxChainNump(pub u8);
    impl Uep1TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep1TxChainNump {
        #[inline(always)]
        fn default() -> Uep1TxChainNump {
            Uep1TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep1TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxChainSt(pub u8);
    impl Uep1TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1TxChainSt {
        #[inline(always)]
        fn default() -> Uep1TxChainSt {
            Uep1TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep1TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxCr(pub u8);
    impl Uep1TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1TxCr {
        #[inline(always)]
        fn default() -> Uep1TxCr {
            Uep1TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep1TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxDma(pub u32);
    impl Uep1TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep1TxDma {
        #[inline(always)]
        fn default() -> Uep1TxDma {
            Uep1TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep1TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxDmaOfs(pub u16);
    impl Uep1TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep1TxDmaOfs {
            Uep1TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep1TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxSeq(pub u8);
    impl Uep1TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep1TxSeq {
        #[inline(always)]
        fn default() -> Uep1TxSeq {
            Uep1TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep1TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 1 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxSt(pub u8);
    impl Uep1TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1TxSt {
        #[inline(always)]
        fn default() -> Uep1TxSt {
            Uep1TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep1TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 2 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxCfg(pub u8);
    impl Uep2RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2RxCfg {
        #[inline(always)]
        fn default() -> Uep2RxCfg {
            Uep2RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep2RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxChainCr(pub u8);
    impl Uep2RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep2RxChainCr {
        #[inline(always)]
        fn default() -> Uep2RxChainCr {
            Uep2RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep2RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxChainLen(pub u16);
    impl Uep2RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep2RxChainLen {
        #[inline(always)]
        fn default() -> Uep2RxChainLen {
            Uep2RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep2RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 2 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxChainMaxNump(pub u8);
    impl Uep2RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep2RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep2RxChainMaxNump {
            Uep2RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep2RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxChainNump(pub u8);
    impl Uep2RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep2RxChainNump {
        #[inline(always)]
        fn default() -> Uep2RxChainNump {
            Uep2RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep2RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxChainSt(pub u8);
    impl Uep2RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2RxChainSt {
        #[inline(always)]
        fn default() -> Uep2RxChainSt {
            Uep2RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep2RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxCr(pub u8);
    impl Uep2RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2RxCr {
        #[inline(always)]
        fn default() -> Uep2RxCr {
            Uep2RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep2RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxDma(pub u32);
    impl Uep2RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep2RxDma {
        #[inline(always)]
        fn default() -> Uep2RxDma {
            Uep2RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep2RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxDmaOfs(pub u16);
    impl Uep2RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep2RxDmaOfs {
            Uep2RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep2RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 2 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxSeq(pub u8);
    impl Uep2RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep2RxSeq {
        #[inline(always)]
        fn default() -> Uep2RxSeq {
            Uep2RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep2RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 2 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxSt(pub u8);
    impl Uep2RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2RxSt {
        #[inline(always)]
        fn default() -> Uep2RxSt {
            Uep2RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep2RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 2 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxCfg(pub u8);
    impl Uep2TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2TxCfg {
        #[inline(always)]
        fn default() -> Uep2TxCfg {
            Uep2TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep2TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxChainCr(pub u8);
    impl Uep2TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep2TxChainCr {
        #[inline(always)]
        fn default() -> Uep2TxChainCr {
            Uep2TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep2TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxChainExpNump(pub u8);
    impl Uep2TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep2TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep2TxChainExpNump {
            Uep2TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep2TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxChainLen(pub u16);
    impl Uep2TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep2TxChainLen {
        #[inline(always)]
        fn default() -> Uep2TxChainLen {
            Uep2TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep2TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxChainNump(pub u8);
    impl Uep2TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep2TxChainNump {
        #[inline(always)]
        fn default() -> Uep2TxChainNump {
            Uep2TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep2TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 2 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxChainSt(pub u8);
    impl Uep2TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2TxChainSt {
        #[inline(always)]
        fn default() -> Uep2TxChainSt {
            Uep2TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep2TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxCr(pub u8);
    impl Uep2TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2TxCr {
        #[inline(always)]
        fn default() -> Uep2TxCr {
            Uep2TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep2TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxDma(pub u32);
    impl Uep2TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep2TxDma {
        #[inline(always)]
        fn default() -> Uep2TxDma {
            Uep2TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep2TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxDmaOfs(pub u16);
    impl Uep2TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep2TxDmaOfs {
            Uep2TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep2TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 2 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxSeq(pub u8);
    impl Uep2TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep2TxSeq {
        #[inline(always)]
        fn default() -> Uep2TxSeq {
            Uep2TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep2TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 2 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxSt(pub u8);
    impl Uep2TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2TxSt {
        #[inline(always)]
        fn default() -> Uep2TxSt {
            Uep2TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep2TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 3 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxCfg(pub u8);
    impl Uep3RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3RxCfg {
        #[inline(always)]
        fn default() -> Uep3RxCfg {
            Uep3RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep3RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxChainCr(pub u8);
    impl Uep3RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep3RxChainCr {
        #[inline(always)]
        fn default() -> Uep3RxChainCr {
            Uep3RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep3RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxChainLen(pub u16);
    impl Uep3RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep3RxChainLen {
        #[inline(always)]
        fn default() -> Uep3RxChainLen {
            Uep3RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep3RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 3 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxChainMaxNump(pub u8);
    impl Uep3RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep3RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep3RxChainMaxNump {
            Uep3RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep3RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxChainNump(pub u8);
    impl Uep3RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep3RxChainNump {
        #[inline(always)]
        fn default() -> Uep3RxChainNump {
            Uep3RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep3RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxChainSt(pub u8);
    impl Uep3RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3RxChainSt {
        #[inline(always)]
        fn default() -> Uep3RxChainSt {
            Uep3RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep3RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxCr(pub u8);
    impl Uep3RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3RxCr {
        #[inline(always)]
        fn default() -> Uep3RxCr {
            Uep3RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep3RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxDma(pub u32);
    impl Uep3RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep3RxDma {
        #[inline(always)]
        fn default() -> Uep3RxDma {
            Uep3RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep3RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxDmaOfs(pub u16);
    impl Uep3RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep3RxDmaOfs {
            Uep3RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep3RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 3 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxSeq(pub u8);
    impl Uep3RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep3RxSeq {
        #[inline(always)]
        fn default() -> Uep3RxSeq {
            Uep3RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep3RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 3 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxSt(pub u8);
    impl Uep3RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3RxSt {
        #[inline(always)]
        fn default() -> Uep3RxSt {
            Uep3RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep3RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 3 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxCfg(pub u8);
    impl Uep3TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3TxCfg {
        #[inline(always)]
        fn default() -> Uep3TxCfg {
            Uep3TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep3TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxChainCr(pub u8);
    impl Uep3TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep3TxChainCr {
        #[inline(always)]
        fn default() -> Uep3TxChainCr {
            Uep3TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep3TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxChainExpNump(pub u8);
    impl Uep3TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep3TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep3TxChainExpNump {
            Uep3TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep3TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxChainLen(pub u16);
    impl Uep3TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep3TxChainLen {
        #[inline(always)]
        fn default() -> Uep3TxChainLen {
            Uep3TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep3TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxChainNump(pub u8);
    impl Uep3TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep3TxChainNump {
        #[inline(always)]
        fn default() -> Uep3TxChainNump {
            Uep3TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep3TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 3 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxChainSt(pub u8);
    impl Uep3TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3TxChainSt {
        #[inline(always)]
        fn default() -> Uep3TxChainSt {
            Uep3TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep3TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxCr(pub u8);
    impl Uep3TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3TxCr {
        #[inline(always)]
        fn default() -> Uep3TxCr {
            Uep3TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep3TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxDma(pub u32);
    impl Uep3TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep3TxDma {
        #[inline(always)]
        fn default() -> Uep3TxDma {
            Uep3TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep3TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxDmaOfs(pub u16);
    impl Uep3TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep3TxDmaOfs {
            Uep3TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep3TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 3 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxSeq(pub u8);
    impl Uep3TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep3TxSeq {
        #[inline(always)]
        fn default() -> Uep3TxSeq {
            Uep3TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep3TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 3 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxSt(pub u8);
    impl Uep3TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3TxSt {
        #[inline(always)]
        fn default() -> Uep3TxSt {
            Uep3TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep3TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 4 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxCfg(pub u8);
    impl Uep4RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4RxCfg {
        #[inline(always)]
        fn default() -> Uep4RxCfg {
            Uep4RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep4RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxChainCr(pub u8);
    impl Uep4RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep4RxChainCr {
        #[inline(always)]
        fn default() -> Uep4RxChainCr {
            Uep4RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep4RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxChainLen(pub u16);
    impl Uep4RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep4RxChainLen {
        #[inline(always)]
        fn default() -> Uep4RxChainLen {
            Uep4RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep4RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 4 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxChainMaxNump(pub u8);
    impl Uep4RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep4RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep4RxChainMaxNump {
            Uep4RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep4RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxChainNump(pub u8);
    impl Uep4RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep4RxChainNump {
        #[inline(always)]
        fn default() -> Uep4RxChainNump {
            Uep4RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep4RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxChainSt(pub u8);
    impl Uep4RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4RxChainSt {
        #[inline(always)]
        fn default() -> Uep4RxChainSt {
            Uep4RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep4RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 4 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxCr(pub u8);
    impl Uep4RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4RxCr {
        #[inline(always)]
        fn default() -> Uep4RxCr {
            Uep4RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep4RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxDma(pub u32);
    impl Uep4RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep4RxDma {
        #[inline(always)]
        fn default() -> Uep4RxDma {
            Uep4RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep4RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxDmaOfs(pub u16);
    impl Uep4RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep4RxDmaOfs {
            Uep4RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep4RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 4 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxSeq(pub u8);
    impl Uep4RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep4RxSeq {
        #[inline(always)]
        fn default() -> Uep4RxSeq {
            Uep4RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep4RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 4 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxSt(pub u8);
    impl Uep4RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4RxSt {
        #[inline(always)]
        fn default() -> Uep4RxSt {
            Uep4RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep4RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 4 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxCfg(pub u8);
    impl Uep4TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4TxCfg {
        #[inline(always)]
        fn default() -> Uep4TxCfg {
            Uep4TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep4TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxChainCr(pub u8);
    impl Uep4TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep4TxChainCr {
        #[inline(always)]
        fn default() -> Uep4TxChainCr {
            Uep4TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep4TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxChainExpNump(pub u8);
    impl Uep4TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep4TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep4TxChainExpNump {
            Uep4TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep4TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxChainLen(pub u16);
    impl Uep4TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep4TxChainLen {
        #[inline(always)]
        fn default() -> Uep4TxChainLen {
            Uep4TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep4TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxChainNump(pub u8);
    impl Uep4TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep4TxChainNump {
        #[inline(always)]
        fn default() -> Uep4TxChainNump {
            Uep4TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep4TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 4 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxChainSt(pub u8);
    impl Uep4TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4TxChainSt {
        #[inline(always)]
        fn default() -> Uep4TxChainSt {
            Uep4TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep4TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 4 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxCr(pub u8);
    impl Uep4TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4TxCr {
        #[inline(always)]
        fn default() -> Uep4TxCr {
            Uep4TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep4TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxDma(pub u32);
    impl Uep4TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep4TxDma {
        #[inline(always)]
        fn default() -> Uep4TxDma {
            Uep4TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep4TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxDmaOfs(pub u16);
    impl Uep4TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep4TxDmaOfs {
            Uep4TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep4TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 4 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxSeq(pub u8);
    impl Uep4TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep4TxSeq {
        #[inline(always)]
        fn default() -> Uep4TxSeq {
            Uep4TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep4TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 4 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxSt(pub u8);
    impl Uep4TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4TxSt {
        #[inline(always)]
        fn default() -> Uep4TxSt {
            Uep4TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep4TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 5 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxCfg(pub u8);
    impl Uep5RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5RxCfg {
        #[inline(always)]
        fn default() -> Uep5RxCfg {
            Uep5RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep5RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxChainCr(pub u8);
    impl Uep5RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep5RxChainCr {
        #[inline(always)]
        fn default() -> Uep5RxChainCr {
            Uep5RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep5RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxChainLen(pub u16);
    impl Uep5RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep5RxChainLen {
        #[inline(always)]
        fn default() -> Uep5RxChainLen {
            Uep5RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep5RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 5 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxChainMaxNump(pub u8);
    impl Uep5RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep5RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep5RxChainMaxNump {
            Uep5RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep5RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxChainNump(pub u8);
    impl Uep5RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep5RxChainNump {
        #[inline(always)]
        fn default() -> Uep5RxChainNump {
            Uep5RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep5RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxChainSt(pub u8);
    impl Uep5RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5RxChainSt {
        #[inline(always)]
        fn default() -> Uep5RxChainSt {
            Uep5RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep5RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 5 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxCr(pub u8);
    impl Uep5RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5RxCr {
        #[inline(always)]
        fn default() -> Uep5RxCr {
            Uep5RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep5RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxDma(pub u32);
    impl Uep5RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep5RxDma {
        #[inline(always)]
        fn default() -> Uep5RxDma {
            Uep5RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep5RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxDmaOfs(pub u16);
    impl Uep5RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep5RxDmaOfs {
            Uep5RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep5RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 5 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxSeq(pub u8);
    impl Uep5RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep5RxSeq {
        #[inline(always)]
        fn default() -> Uep5RxSeq {
            Uep5RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep5RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 5 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxSt(pub u8);
    impl Uep5RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5RxSt {
        #[inline(always)]
        fn default() -> Uep5RxSt {
            Uep5RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep5RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 5 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxCfg(pub u8);
    impl Uep5TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5TxCfg {
        #[inline(always)]
        fn default() -> Uep5TxCfg {
            Uep5TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep5TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxChainCr(pub u8);
    impl Uep5TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep5TxChainCr {
        #[inline(always)]
        fn default() -> Uep5TxChainCr {
            Uep5TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep5TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxChainExpNump(pub u8);
    impl Uep5TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep5TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep5TxChainExpNump {
            Uep5TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep5TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxChainLen(pub u16);
    impl Uep5TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep5TxChainLen {
        #[inline(always)]
        fn default() -> Uep5TxChainLen {
            Uep5TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep5TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxChainNump(pub u8);
    impl Uep5TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep5TxChainNump {
        #[inline(always)]
        fn default() -> Uep5TxChainNump {
            Uep5TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep5TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 5 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxChainSt(pub u8);
    impl Uep5TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5TxChainSt {
        #[inline(always)]
        fn default() -> Uep5TxChainSt {
            Uep5TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep5TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 5 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxCr(pub u8);
    impl Uep5TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5TxCr {
        #[inline(always)]
        fn default() -> Uep5TxCr {
            Uep5TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep5TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxDma(pub u32);
    impl Uep5TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep5TxDma {
        #[inline(always)]
        fn default() -> Uep5TxDma {
            Uep5TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep5TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxDmaOfs(pub u16);
    impl Uep5TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep5TxDmaOfs {
            Uep5TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep5TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 5 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxSeq(pub u8);
    impl Uep5TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep5TxSeq {
        #[inline(always)]
        fn default() -> Uep5TxSeq {
            Uep5TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep5TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 5 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxSt(pub u8);
    impl Uep5TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5TxSt {
        #[inline(always)]
        fn default() -> Uep5TxSt {
            Uep5TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep5TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 6 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxCfg(pub u8);
    impl Uep6RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6RxCfg {
        #[inline(always)]
        fn default() -> Uep6RxCfg {
            Uep6RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep6RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxChainCr(pub u8);
    impl Uep6RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep6RxChainCr {
        #[inline(always)]
        fn default() -> Uep6RxChainCr {
            Uep6RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep6RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxChainLen(pub u16);
    impl Uep6RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep6RxChainLen {
        #[inline(always)]
        fn default() -> Uep6RxChainLen {
            Uep6RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep6RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 6 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxChainMaxNump(pub u8);
    impl Uep6RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep6RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep6RxChainMaxNump {
            Uep6RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep6RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxChainNump(pub u8);
    impl Uep6RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep6RxChainNump {
        #[inline(always)]
        fn default() -> Uep6RxChainNump {
            Uep6RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep6RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxChainSt(pub u8);
    impl Uep6RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6RxChainSt {
        #[inline(always)]
        fn default() -> Uep6RxChainSt {
            Uep6RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep6RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 6 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxCr(pub u8);
    impl Uep6RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6RxCr {
        #[inline(always)]
        fn default() -> Uep6RxCr {
            Uep6RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep6RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxDma(pub u32);
    impl Uep6RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep6RxDma {
        #[inline(always)]
        fn default() -> Uep6RxDma {
            Uep6RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep6RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxDmaOfs(pub u16);
    impl Uep6RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep6RxDmaOfs {
            Uep6RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep6RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 6 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxSeq(pub u8);
    impl Uep6RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep6RxSeq {
        #[inline(always)]
        fn default() -> Uep6RxSeq {
            Uep6RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep6RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 6 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxSt(pub u8);
    impl Uep6RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6RxSt {
        #[inline(always)]
        fn default() -> Uep6RxSt {
            Uep6RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep6RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 6 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxCfg(pub u8);
    impl Uep6TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6TxCfg {
        #[inline(always)]
        fn default() -> Uep6TxCfg {
            Uep6TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep6TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxChainCr(pub u8);
    impl Uep6TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep6TxChainCr {
        #[inline(always)]
        fn default() -> Uep6TxChainCr {
            Uep6TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep6TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxChainExpNump(pub u8);
    impl Uep6TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep6TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep6TxChainExpNump {
            Uep6TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep6TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxChainLen(pub u16);
    impl Uep6TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep6TxChainLen {
        #[inline(always)]
        fn default() -> Uep6TxChainLen {
            Uep6TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep6TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxChainNump(pub u8);
    impl Uep6TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep6TxChainNump {
        #[inline(always)]
        fn default() -> Uep6TxChainNump {
            Uep6TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep6TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 6 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxChainSt(pub u8);
    impl Uep6TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6TxChainSt {
        #[inline(always)]
        fn default() -> Uep6TxChainSt {
            Uep6TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep6TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 6 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxCr(pub u8);
    impl Uep6TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6TxCr {
        #[inline(always)]
        fn default() -> Uep6TxCr {
            Uep6TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep6TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxDma(pub u32);
    impl Uep6TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep6TxDma {
        #[inline(always)]
        fn default() -> Uep6TxDma {
            Uep6TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep6TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxDmaOfs(pub u16);
    impl Uep6TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep6TxDmaOfs {
            Uep6TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep6TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 6 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxSeq(pub u8);
    impl Uep6TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep6TxSeq {
        #[inline(always)]
        fn default() -> Uep6TxSeq {
            Uep6TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep6TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 6 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxSt(pub u8);
    impl Uep6TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6TxSt {
        #[inline(always)]
        fn default() -> Uep6TxSt {
            Uep6TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep6TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 7 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxCfg(pub u8);
    impl Uep7RxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DP(PP=0) is received,. clearing all CHAIN_EN will result in an NRDY response to the subsequent DPH; Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a continuous burst packet is received,. PP=1 and not a short packet is not received, and DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7RxCfg {
        #[inline(always)]
        fn default() -> Uep7RxCfg {
            Uep7RxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep7RxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_eob_mode", &self.ep_rx_eob_mode())
                .field("ep_rx_tout_mode", &self.ep_rx_tout_mode())
                .field("ep_rx_fifo_cfg", &self.ep_rx_fifo_cfg())
                .field("ep_rx_fifo_mode", &self.ep_rx_fifo_mode())
                .field("ep_rx_chain_auto", &self.ep_rx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxChainCr(pub u8);
    impl Uep7RxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_ep_rx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep7RxChainCr {
        #[inline(always)]
        fn default() -> Uep7RxChainCr {
            Uep7RxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep7RxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxChainLen(pub u16);
    impl Uep7RxChainLen {
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the currently completed CHAIN to the last packet."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep7RxChainLen {
        #[inline(always)]
        fn default() -> Uep7RxChainLen {
            Uep7RxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep7RxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that Endpoint 7 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxChainMaxNump(pub u8);
    impl Uep7RxChainMaxNump {
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be received by the CHAIN."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep7RxChainMaxNump {
        #[inline(always)]
        fn default() -> Uep7RxChainMaxNump {
            Uep7RxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for Uep7RxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been received by Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxChainNump(pub u8);
    impl Uep7RxChainNump {
        #[doc = "The number of DPP packets that has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep7RxChainNump {
        #[inline(always)]
        fn default() -> Uep7RxChainNump {
            Uep7RxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep7RxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxChainSt(pub u8);
    impl Uep7RxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to currently receive the LPF status in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7RxChainSt {
        #[inline(always)]
        fn default() -> Uep7RxChainSt {
            Uep7RxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep7RxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 7 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxCr(pub u8);
    impl Uep7RxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7RxCr {
        #[inline(always)]
        fn default() -> Uep7RxCr {
            Uep7RxCr(0)
        }
    }
    impl core::fmt::Debug for Uep7RxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxDma(pub u32);
    impl Uep7RxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to receive data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep7RxDma {
        #[inline(always)]
        fn default() -> Uep7RxDma {
            Uep7RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep7RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxDma")
                .field("chain_rx_dma", &self.chain_rx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxDmaOfs(pub u16);
    impl Uep7RxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7RxDmaOfs {
        #[inline(always)]
        fn default() -> Uep7RxDmaOfs {
            Uep7RxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep7RxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 7 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxSeq(pub u8);
    impl Uep7RxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep7RxSeq {
        #[inline(always)]
        fn default() -> Uep7RxSeq {
            Uep7RxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep7RxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 7 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxSt(pub u8);
    impl Uep7RxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn er_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_er_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7RxSt {
        #[inline(always)]
        fn default() -> Uep7RxSt {
            Uep7RxSt(0)
        }
    }
    impl core::fmt::Debug for Uep7RxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxSt")
                .field("er_rx_chain_en", &self.er_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 7 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxCfg(pub u8);
    impl Uep7TxCfg {
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A value of 1 indicates that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: It is forbidden to write R16_EPn_ST->EP_SEQ_NUM software;. 0: allows the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY,. no software control required; It is recommended that the synchronization endpoint zero out this bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, EOB/LPF=0 in ACK-TP;If a short packet is sent, EOB/LPF=1 in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Access offset address 0xC~0xF, and the operation object."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If the bit is 1, the current endpoint uses FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches modes, and this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7TxCfg {
        #[inline(always)]
        fn default() -> Uep7TxCfg {
            Uep7TxCfg(0)
        }
    }
    impl core::fmt::Debug for Uep7TxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxCfg")
                .field("ep_tx_iso_mode", &self.ep_tx_iso_mode())
                .field("ep_tx_seq_auto", &self.ep_tx_seq_auto())
                .field("ep_tx_erdy_auto", &self.ep_tx_erdy_auto())
                .field("ep_tx_eob_mode", &self.ep_tx_eob_mode())
                .field("ep_tx_fifo_cfg", &self.ep_tx_fifo_cfg())
                .field("ep_tx_fifo_mode", &self.ep_tx_fifo_mode())
                .field("ep_tx_chain_auto", &self.ep_tx_chain_auto())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxChainCr(pub u8);
    impl Uep7TxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN status and configuration."]
        #[inline(always)]
        pub const fn set_tx_ret_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_force_ret(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit effectively forces the return of the selected CHAIN state machine configuration."]
        #[inline(always)]
        pub const fn set_tx_force_ret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The chain serial number of the current configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The chain serial number of the current configuration."]
        #[inline(always)]
        pub const fn set_tx_cur_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_use(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number currently in use."]
        #[inline(always)]
        pub const fn set_tx_cur_use(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Uep7TxChainCr {
        #[inline(always)]
        fn default() -> Uep7TxChainCr {
            Uep7TxChainCr(0)
        }
    }
    impl core::fmt::Debug for Uep7TxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxChainCr")
                .field("tx_ret_sel", &self.tx_ret_sel())
                .field("tx_force_ret", &self.tx_force_ret())
                .field("tx_cur_cfg", &self.tx_cur_cfg())
                .field("tx_cur_use", &self.tx_cur_use())
                .finish()
        }
    }
    #[doc = "Number of NUMPs expected to be sent by Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxChainExpNump(pub u8);
    impl Uep7TxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the currently completed CHAIN."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep7TxChainExpNump {
        #[inline(always)]
        fn default() -> Uep7TxChainExpNump {
            Uep7TxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for Uep7TxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxChainLen(pub u16);
    impl Uep7TxChainLen {
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The length of the last packet sent by the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep7TxChainLen {
        #[inline(always)]
        fn default() -> Uep7TxChainLen {
            Uep7TxChainLen(0)
        }
    }
    impl core::fmt::Debug for Uep7TxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs has been sent by Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxChainNump(pub u8);
    impl Uep7TxChainNump {
        #[doc = "The number of DPP packets that have been transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep7TxChainNump {
        #[inline(always)]
        fn default() -> Uep7TxChainNump {
            Uep7TxChainNump(0)
        }
    }
    impl core::fmt::Debug for Uep7TxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 7 CHAIN state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxChainSt(pub u8);
    impl Uep7TxChainSt {
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The serial number of the CHAIN that is currently interrupting."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the PP bits in the currently received DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IF THE NUMP IN THE CHAIN IS 0, THE POSITION IS 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EOB/LPF bits in the last packet of DPH in the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is only written, and 1 is written to release the current CHAIN_IF."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the automatic hardware setting of 1 after the UEP_CHAIN_NUMP register is configured,. and the CHAIN is automatically cleared to zero after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7TxChainSt {
        #[inline(always)]
        fn default() -> Uep7TxChainSt {
            Uep7TxChainSt(0)
        }
    }
    impl core::fmt::Debug for Uep7TxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 7 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxCr(pub u8);
    impl Uep7TxCr {
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The nump field of ERDY-TP is sent by the hardware,. and the value is generally set to the number of supported bursts. For example, if you support 16 bursts, the value of this field is 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 clears all configuration values and status of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint Stop, High Validity, Endpoint Stop Answering STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7TxCr {
        #[inline(always)]
        fn default() -> Uep7TxCr {
            Uep7TxCr(0)
        }
    }
    impl core::fmt::Debug for Uep7TxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address for Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxDma(pub u32);
    impl Uep7TxDma {
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Normal mode:The DMA start address of the CHAIN to send data.. FIFO mode:16~23 bits of the FIFO start address in SRAM;The FIFO end address is 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep7TxDma {
        #[inline(always)]
        fn default() -> Uep7TxDma {
            Uep7TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep7TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length for Endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxDmaOfs(pub u16);
    impl Uep7TxDmaOfs {
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in that CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7TxDmaOfs {
        #[inline(always)]
        fn default() -> Uep7TxDmaOfs {
            Uep7TxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for Uep7TxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint 7 Serial Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxSeq(pub u8);
    impl Uep7TxSeq {
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint,. writable in non-SEQ_AUTO mode, read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Uep7TxSeq {
        #[inline(always)]
        fn default() -> Uep7TxSeq {
            Uep7TxSeq(0)
        }
    }
    impl core::fmt::Debug for Uep7TxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 7 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxSt(pub u8);
    impl Uep7TxSt {
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The CHAIN enabled state, which corresponds to 4 independent CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 separate CHAINS."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the ERDY is currently being sent;. Writing 1 to that bit will send ERDY, which is used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is in the current throttling state, and write 1 is cleared to zero."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The current break flag for the endpoint, the bit is read-only,. and all CHAIN_IF are 0 if the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7TxSt {
        #[inline(always)]
        fn default() -> Uep7TxSt {
            Uep7TxSt(0)
        }
    }
    impl core::fmt::Debug for Uep7TxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint Receive Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoints 1~15 downpass enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_en(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "Endpoints 1~15 downpass enabled."]
        #[inline(always)]
        pub const fn set_ep_rx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
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
                .field("ep_rx_en", &self.ep_rx_en())
                .finish()
        }
    }
    #[doc = "Endpoint Sends Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoints 1~15 upload enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_en(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "Endpoints 1~15 upload enabled."]
        #[inline(always)]
        pub const fn set_ep_tx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u16) & 0x7fff) << 1usize);
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
                .field("ep_tx_en", &self.ep_tx_en())
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
        #[doc = "When DPH is transmitted in bursts, the next packet of data status."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When DPH is transmitted in bursts, the next packet of data status."]
        #[inline(always)]
        pub const fn set_dma_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SETUP transaction throttling."]
        #[must_use]
        #[inline(always)]
        pub const fn setup_flow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP transaction throttling."]
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
        #[doc = "Packet Pending control bit for sending TP/DP packets."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_hp_pend(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Packet Pending control bit for sending TP/DP packets."]
        #[inline(always)]
        pub const fn set_reg_hp_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Use in device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_erdy_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Use in device mode."]
        #[inline(always)]
        pub const fn set_tx_erdy_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
        #[doc = "Host Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_addr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Host Mode."]
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
                .field("setup_flow", &self.setup_flow())
                .field("itp_en", &self.itp_en())
                .field("host_mode", &self.host_mode())
                .field("reg_hp_pend", &self.reg_hp_pend())
                .field("tx_erdy_mode", &self.tx_erdy_mode())
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
