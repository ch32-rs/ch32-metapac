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
    #[doc = "LINK Control Register."]
    #[inline(always)]
    pub const fn link_ctrl(self) -> crate::common::Reg<regs::LinkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LINK Interrupt Enable Register."]
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
    #[doc = "LINK ITP timeout mode register."]
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
    #[doc = "U1 Wake Up LFPS Effective Time Register."]
    #[inline(always)]
    pub const fn link_u1_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU1WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "U2 Wake LFPS Effective Time Register."]
    #[inline(always)]
    pub const fn link_u2_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU2WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "U3 Wake LFPS Effective Time Register."]
    #[inline(always)]
    pub const fn link_u3_wkup_filter(
        self,
    ) -> crate::common::Reg<regs::LinkU3WkupFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
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
    #[doc = "PORT_CAP Register."]
    #[inline(always)]
    pub const fn link_lmp_port_cap(
        self,
    ) -> crate::common::Reg<regs::LinkLmpPortCap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "LMP Receive Data 0 Register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data0(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "LMP Receives Data 1 Register."]
    #[inline(always)]
    pub const fn link_lmp_rx_data1(
        self,
    ) -> crate::common::Reg<regs::LinkLmpRxData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "LMP Receives Data 2 Register."]
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
    #[doc = "Flash capecity register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "USBSS Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "The Interval register for the ITP package in USB."]
    #[inline(always)]
    pub const fn itp(self) -> crate::common::Reg<regs::Itp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interval adaptive registers for ITP packets in USB."]
    #[inline(always)]
    pub const fn itp_adj(self) -> crate::common::Reg<regs::ItpAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Endpoint Send Enable Register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Endpoint Receive Enable Register."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Endpoint 0 Transmit Control Register."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Endpoint 0 Receives Control Register."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Sending Buffer Address Register."]
    #[inline(always)]
    pub const fn uep0_tx_dma(self) -> crate::common::Reg<regs::Uep0TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Receive Buffer Address Register."]
    #[inline(always)]
    pub const fn uep0_rx_dma(self) -> crate::common::Reg<regs::Uep0RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "endpoint 0 transmit DMA offset."]
    #[inline(always)]
    pub const fn uep0_tx_dma_ofs(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "endpoint 0 receive DMA offset."]
    #[inline(always)]
    pub const fn uep0_rx_dma_ofs(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Endpoint 1 configures the registers."]
    #[inline(always)]
    pub const fn uep_tx_cfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxCfg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 Control Register."]
    #[inline(always)]
    pub const fn uep_tx_cr(self, n: usize) -> crate::common::Reg<regs::UepTxCr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc1usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 serial number register."]
    #[inline(always)]
    pub const fn uep_tx_seq(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxSeq, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 state register."]
    #[inline(always)]
    pub const fn uep_tx_st(self, n: usize) -> crate::common::Reg<regs::UepTxSt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc3usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[inline(always)]
    pub const fn uep_tx_chain_cr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxChainCr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN status register."]
    #[inline(always)]
    pub const fn uep_tx_chain_st(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxChainSt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc5usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[inline(always)]
    pub const fn uep_tx_chain_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxChainLen, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize + n * 32usize) as _) }
    }
    #[doc = "Number of NUMPs expected to be sent by endpoint 1."]
    #[inline(always)]
    pub const fn uep_tx_chain_exp_nump(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxChainExpNump, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize + n * 32usize) as _) }
    }
    #[doc = "Number of NUMPs sent by endpoint 1."]
    #[inline(always)]
    pub const fn uep_tx_chain_nump(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxChainNump, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc9usize + n * 32usize) as _) }
    }
    #[doc = "DMA offset length of endpoint 1."]
    #[inline(always)]
    pub const fn uep_tx_dma_ofs(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxDmaOfs, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize + n * 32usize) as _) }
    }
    #[doc = "DMA start address of endpoint 1."]
    #[inline(always)]
    pub const fn uep_tx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxDma, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[inline(always)]
    pub const fn uep_rx_cfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxCfg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 Control Register."]
    #[inline(always)]
    pub const fn uep_rx_cr(self, n: usize) -> crate::common::Reg<regs::UepRxCr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd1usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 serial number register."]
    #[inline(always)]
    pub const fn uep_rx_seq(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxSeq, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd2usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 state register."]
    #[inline(always)]
    pub const fn uep_rx_st(self, n: usize) -> crate::common::Reg<regs::UepRxSt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd3usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN control register."]
    #[inline(always)]
    pub const fn uep_rx_chain_cr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxChainCr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize + n * 32usize) as _) }
    }
    #[doc = "Endpoint 1 CHAIN status register."]
    #[inline(always)]
    pub const fn uep_rx_chain_st(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxChainSt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd5usize + n * 32usize) as _) }
    }
    #[doc = "The length of the last packet received by the endpoint 1 CHAIN."]
    #[inline(always)]
    pub const fn uep_rx_chain_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxChainLen, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd6usize + n * 32usize) as _) }
    }
    #[doc = "Number of NUMPs that endpoint 1 can receive."]
    #[inline(always)]
    pub const fn uep_rx_chain_max_nump(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxChainMaxNump, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize + n * 32usize) as _) }
    }
    #[doc = "Number of NUMPs received by endpoint 1."]
    #[inline(always)]
    pub const fn uep_rx_chain_nump(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxChainNump, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd9usize + n * 32usize) as _) }
    }
    #[doc = "DMA offset length of endpoint 1."]
    #[inline(always)]
    pub const fn uep_rx_dma_ofs(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxDmaOfs, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdausize + n * 32usize) as _) }
    }
    #[doc = "DMA start address of endpoint 1."]
    #[inline(always)]
    pub const fn uep_rx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxDma, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize + n * 32usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash capecity register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
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
        #[doc = "Reset all software configuration registers, high efficiency, need software clearing."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reset all software configuration registers, high efficiency, need software clearing."]
        #[inline(always)]
        pub const fn set_usb_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "To reset the protocol layer and FIFO module, software clearing is required."]
        #[must_use]
        #[inline(always)]
        pub const fn force_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "To reset the protocol layer and FIFO module, software clearing is required."]
        #[inline(always)]
        pub const fn set_force_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When performing burst DPH transmission, the operation of the next packet."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When performing burst DPH transmission, the operation of the next packet."]
        #[inline(always)]
        pub const fn set_dma_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SETUP Transaction Flow Control."]
        #[must_use]
        #[inline(always)]
        pub const fn setup_flow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP Transaction Flow Control."]
        #[inline(always)]
        pub const fn set_setup_flow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
        #[doc = "USB transaction completion interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_transfer(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction completion interrupt enabled."]
        #[inline(always)]
        pub const fn set_uie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Device mode: Receive SETUP transaction to complete interrupt enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_setup(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Device mode: Receive SETUP transaction to complete interrupt enablement."]
        #[inline(always)]
        pub const fn set_udie_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Device mode: Receive STATUS transaction to complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_status(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Device mode: Receive STATUS transaction to complete interrupt enable."]
        #[inline(always)]
        pub const fn set_udie_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receive PING-TP to complete the interrupt enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_rx_ping(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receive PING-TP to complete the interrupt enablement."]
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
        #[doc = "Sending FIFO Overflow Interrupt Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_fifo_txov(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Sending FIFO Overflow Interrupt Enabled."]
        #[inline(always)]
        pub const fn set_uie_fifo_txov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Receive FIFO overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_fifo_rxov(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_uie_fifo_rxov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The address of the USB device."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_addr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The address of the USB device."]
        #[inline(always)]
        pub const fn set_dev_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("dma_en", &self.dma_en())
                .field("usb_clr_all", &self.usb_clr_all())
                .field("force_rst", &self.force_rst())
                .field("dma_mode", &self.dma_mode())
                .field("setup_flow", &self.setup_flow())
                .field("tx_erdy_mode", &self.tx_erdy_mode())
                .field("uie_transfer", &self.uie_transfer())
                .field("udie_setup", &self.udie_setup())
                .field("udie_status", &self.udie_status())
                .field("uie_rx_ping", &self.uie_rx_ping())
                .field("uie_itp", &self.uie_itp())
                .field("uie_fifo_txov", &self.uie_fifo_txov())
                .field("uie_fifo_rxov", &self.uie_fifo_rxov())
                .field("dev_addr", &self.dev_addr())
                .finish()
        }
    }
    #[doc = "The Interval register for the ITP package in USB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itp(pub u32);
    impl Itp {
        #[doc = "The Bus interval Counter field in the received ITP."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_itp_interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "The Bus interval Counter field in the received ITP."]
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
    #[doc = "Interval adaptive registers for ITP packets in USB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ItpAdj(pub u32);
    impl ItpAdj {
        #[doc = "In device mode, the Bus Interval Adjustment Control field in the received ITP."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_adj_cr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "In device mode, the Bus Interval Adjustment Control field in the received ITP."]
        #[inline(always)]
        pub const fn set_itp_adj_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "In device mode, the delayed bit of the Link Control Word in the received ITP is 1 when the ITP is delayed."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_delayed(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "In device mode, the delayed bit of the Link Control Word in the received ITP is 1 when the ITP is delayed."]
        #[inline(always)]
        pub const fn set_itp_delayed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The high 13bit (delta) of ITS in the ITP received in device mode represents the time difference from. the start of the current ITP packet to the boundary of the previous bus interval."]
        #[must_use]
        #[inline(always)]
        pub const fn itp_delta(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x1fff;
            val as u16
        }
        #[doc = "The high 13bit (delta) of ITS in the ITP received in device mode represents the time difference from. the start of the current ITP packet to the boundary of the previous bus interval."]
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
        #[doc = "Receiving terminal resistance control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_term_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiving terminal resistance control."]
        #[inline(always)]
        pub const fn set_link_rx_term_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Swap SSTX and SSRX polarity as follows."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ss_plr_swap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Swap SSTX and SSRX polarity as follows."]
        #[inline(always)]
        pub const fn set_link_ss_plr_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PIPE Interface Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn link_phy_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PIPE Interface Reset."]
        #[inline(always)]
        pub const fn set_link_phy_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "POLLING_LFPS whether the timeout enters COMPLIANCE mode."]
        #[must_use]
        #[inline(always)]
        pub const fn link_compliance_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "POLLING_LFPS whether the timeout enters COMPLIANCE mode."]
        #[inline(always)]
        pub const fn set_link_compliance_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LFPS receive control, where the bit 1 indicates that LFPS receive is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lfps_rx_pd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LFPS receive control, where the bit 1 indicates that LFPS receive is disabled."]
        #[inline(always)]
        pub const fn set_link_lfps_rx_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receiver equalization enables control, protocol specifics are optional."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_eq_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver equalization enables control, protocol specifics are optional."]
        #[inline(always)]
        pub const fn set_link_rx_eq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmitter signal swing control, low swing power consumption, but affect the transmission distance."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_swing(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter signal swing control, low swing power consumption, but affect the transmission distance."]
        #[inline(always)]
        pub const fn set_link_tx_swing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmitter De-Emphasis control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_deemph(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Transmitter De-Emphasis control."]
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
        #[doc = "Detection method of connected devices in U2 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_det_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Detection method of connected devices in U2 state."]
        #[inline(always)]
        pub const fn set_link_u2_det_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Allow to enter the LOOPBACK enable bit, high efficiency, use with the LOOKBACK enable in TX/RX_TS_CFG \\[3\\], both are valid,. LINK can enter the LOOKBACK mode (used to enable the LOOPBACK slave data loop and error counting, the LOOPBACK host maintains 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_loopback_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Allow to enter the LOOPBACK enable bit, high efficiency, use with the LOOKBACK enable in TX/RX_TS_CFG \\[3\\], both are valid,. LINK can enter the LOOKBACK mode (used to enable the LOOPBACK slave data loop and error counting, the LOOPBACK host maintains 0)."]
        #[inline(always)]
        pub const fn set_link_loopback_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Used in LOOPBACK mode for LOOPBACK host control pattern transmission (LOOPBACK slave holds 0),. ACT is a high-level signal that lasts for a period of time."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lookback_act(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Used in LOOPBACK mode for LOOPBACK host control pattern transmission (LOOPBACK slave holds 0),. ACT is a high-level signal that lasts for a period of time."]
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
        #[doc = "High efficiency, after receiving the LGO_U1, the response LAU is allowed to enter the U1 state,. otherwise the response LXU refuses to enter the U1 state after receiving the LGO_U1."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u1_allow(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "High efficiency, after receiving the LGO_U1, the response LAU is allowed to enter the U1 state,. otherwise the response LXU refuses to enter the U1 state after receiving the LGO_U1."]
        #[inline(always)]
        pub const fn set_link_u1_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "High efficiency, after receiving the LGO_U2,. the response LAU allows it to enter the U2 state, otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_allow(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "High efficiency, after receiving the LGO_U2,. the response LAU allows it to enter the U2 state, otherwise after receiving the LGO_U2, the response LXU refuses to enter the U2 state."]
        #[inline(always)]
        pub const fn set_link_u2_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SKP rejection control for descrambling."]
        #[must_use]
        #[inline(always)]
        pub const fn link_skp_mode(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "SKP rejection control for descrambling."]
        #[inline(always)]
        pub const fn set_link_skp_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "U1 sends PING_FPFS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u1_ping_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "U1 sends PING_FPFS enable."]
        #[inline(always)]
        pub const fn set_link_u1_ping_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "protocol version."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tout_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "protocol version."]
        #[inline(always)]
        pub const fn set_link_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "LINK reset, including the reset status machine and all interrupt flags, is highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LINK reset, including the reset status machine and all interrupt flags, is highly effective."]
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
                .field("link_skp_mode", &self.link_skp_mode())
                .field("link_u1_ping_en", &self.link_u1_ping_en())
                .field("link_tout_mode", &self.link_tout_mode())
                .field("link_reset", &self.link_reset())
                .finish()
        }
    }
    #[doc = "LINK Control Register."]
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
        #[doc = "RW Set LINK to SS.DISABLED, which is highly effective and requires software to clear it.. If the hardware automatically goes to SS.DIABLED, then the bit is automatically set to 1 by the hardware. In device mode, if the USB2.0 bus reset is received, the software should first set the PD_MODE to P2 mode, and then clear this bit, LTSSM will automatically enter the SS.RX_DETECT, the device will be detected 8 times (every 12ms), if no TERM is detected, it will enter the DISABLE state, and communicate in USB2.0 mode at the same time. In host mode, a BUS_RESET signal should be sent through the USB2.0 controller, the software should first set the PD_MODE to P2 mode, then clear this bit, and if a TERM is detected, a POLLING_LFPS handshake will be made. Otherwise, the software should set the power mode to P3 and communicate in USB2.0 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_disabled(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW Set LINK to SS.DISABLED, which is highly effective and requires software to clear it.. If the hardware automatically goes to SS.DIABLED, then the bit is automatically set to 1 by the hardware. In device mode, if the USB2.0 bus reset is received, the software should first set the PD_MODE to P2 mode, and then clear this bit, LTSSM will automatically enter the SS.RX_DETECT, the device will be detected 8 times (every 12ms), if no TERM is detected, it will enter the DISABLE state, and communicate in USB2.0 mode at the same time. In host mode, a BUS_RESET signal should be sent through the USB2.0 controller, the software should first set the PD_MODE to P2 mode, then clear this bit, and if a TERM is detected, a POLLING_LFPS handshake will be made. Otherwise, the software should set the power mode to P3 and communicate in USB2.0 mode."]
        #[inline(always)]
        pub const fn set_link_go_disabled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Set LINK to SS.INACTIVE, high efficiency, hardware automatic clearing."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_inactive(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Set LINK to SS.INACTIVE, high efficiency, hardware automatic clearing."]
        #[inline(always)]
        pub const fn set_link_go_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Set the LINK to SS.RECOVERY, which is highly effective and the hardware is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_recovery(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Set the LINK to SS.RECOVERY, which is highly effective and the hardware is automatically cleared."]
        #[inline(always)]
        pub const fn set_link_go_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Before setting this bit, the PD_MODE should be set to P2 mode, and the LINK should be set to enter the SS.RX_DETECT,. and the software query TERM_PRESENT know whether there is a connection or whether the connection is disconnected. High effectiveness, automatic clearing."]
        #[must_use]
        #[inline(always)]
        pub const fn link_go_rx_det(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Before setting this bit, the PD_MODE should be set to P2 mode, and the LINK should be set to enter the SS.RX_DETECT,. and the software query TERM_PRESENT know whether there is a connection or whether the connection is disconnected. High effectiveness, automatic clearing."]
        #[inline(always)]
        pub const fn set_link_go_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High effectiveness, cleared by software. This bit is valid and will send WARM_RESET.. In device mode: After detecting that the host has continuously sent WARM_RESET for 18 ms, the device controller will automatically declare that it has detected WARM_RESET and reset it."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_warm_rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High effectiveness, cleared by software. This bit is valid and will send WARM_RESET.. In device mode: After detecting that the host has continuously sent WARM_RESET for 18 ms, the device controller will automatically declare that it has detected WARM_RESET and reset it."]
        #[inline(always)]
        pub const fn set_link_tx_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "High efficiency, hardware automatic clearing."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_ux_exit(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "High efficiency, hardware automatic clearing."]
        #[inline(always)]
        pub const fn set_link_tx_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive U3 exti LPFS signal enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u3_wkup_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Receive U3 exti LPFS signal enable."]
        #[inline(always)]
        pub const fn set_link_u3_wkup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "High validity, if the SS.RX_DETECT detects the TERM, the POLLING handshake will be performed after the software sets the bit to be valid."]
        #[must_use]
        #[inline(always)]
        pub const fn link_polling_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "High validity, if the SS.RX_DETECT detects the TERM, the POLLING handshake will be performed after the software sets the bit to be valid."]
        #[inline(always)]
        pub const fn set_link_polling_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "After the bit is valid, it will send LGO_U1, high efficiency, and the hardware will be automatically cleared.. If the other party responds with LXU, it indicates that it refuses to enter U1 mode; If the query LINK status is U1, it indicates that the other party accepts to enter the U1 low-power state, and the software should change the current power mode to P1."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, it will send LGO_U1, high efficiency, and the hardware will be automatically cleared.. If the other party responds with LXU, it indicates that it refuses to enter U1 mode; If the query LINK status is U1, it indicates that the other party accepts to enter the U1 low-power state, and the software should change the current power mode to P1."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "After the bit is valid, it will send LGO_U2, high efficiency, and the hardware will be automatically cleared;. If the other party responds with LXU, it indicates that it refuses to enter the U2 state; If the query LINK status is U2, it indicates that the other party accepts to enter the U2 low-power state, and the software should change the current power mode to P2."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, it will send LGO_U2, high efficiency, and the hardware will be automatically cleared;. If the other party responds with LXU, it indicates that it refuses to enter the U2 state; If the query LINK status is U2, it indicates that the other party accepts to enter the U2 low-power state, and the software should change the current power mode to P2."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "After the bit is valid, it will be sent LGO_U3, high effectiveness, and the hardware will be automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_lgo_u3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "After the bit is valid, it will be sent LGO_U3, high effectiveness, and the hardware will be automatically cleared."]
        #[inline(always)]
        pub const fn set_link_tx_lgo_u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Link Configuration for Sending TS1/TS2 Training Sequences."]
        #[must_use]
        #[inline(always)]
        pub const fn link_tx_ts_cfg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Link Configuration for Sending TS1/TS2 Training Sequences."]
        #[inline(always)]
        pub const fn set_link_tx_ts_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Received link control."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_ts_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Received link control."]
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
                .field("link_u3_wkup_en", &self.link_u3_wkup_en())
                .field("link_polling_en", &self.link_polling_en())
                .field("link_tx_lgo_u1", &self.link_tx_lgo_u1())
                .field("link_tx_lgo_u2", &self.link_tx_lgo_u2())
                .field("link_tx_lgo_u3", &self.link_tx_lgo_u3())
                .field("link_tx_ts_cfg", &self.link_tx_ts_cfg())
                .field("link_rx_ts_cfg", &self.link_rx_ts_cfg())
                .finish()
        }
    }
    #[doc = "LINK Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkIntCtrl(pub u32);
    impl LinkIntCtrl {
        #[doc = "LINK is initialized, including two ports before (Heade Sequence Number Advertisement). and (RX Headr Buffer Credit Advertisement) interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is initialized, including two ports before (Heade Sequence Number Advertisement). and (RX Headr Buffer Credit Advertisement) interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Due to the error LINK in RECOVERY interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_recovery(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Due to the error LINK in RECOVERY interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LINK is in INACTIVE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_inactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is in INACTIVE interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LINK in DISABLED interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LINK in DISABLED interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LINK is in U3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is in U3 interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_go_u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LINK is in 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is in 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_go_u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LINK is in U1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LINK is in U1 interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_go_u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LINK in U0 interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_go_u0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LINK in U0 interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_go_u0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "When requesting an exit U3 command timeout interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u3_wk_tout(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "When requesting an exit U3 command timeout interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_u3_wk_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Send LGO_ Ux, receive LXU interrupt enablement that refuses to enter the ux send."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_rej(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Send LGO_ Ux, receive LXU interrupt enablement that refuses to enter the ux send."]
        #[inline(always)]
        pub const fn set_link_ie_ux_rej(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LINK entry RX_DETECT to detect the impedance detection interruption of the trace segment of the remote receiver enables the following functions."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_term_pres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LINK entry RX_DETECT to detect the impedance detection interruption of the trace segment of the remote receiver enables the following functions."]
        #[inline(always)]
        pub const fn set_link_ie_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LINK into POLLING_RXEQ for receiver equalization training interrupt enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_txeq(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LINK into POLLING_RXEQ for receiver equalization training interrupt enablement."]
        #[inline(always)]
        pub const fn set_link_ie_txeq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Receiving a request to exit the UX interrupts the enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_exit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Receiving a request to exit the UX interrupts the enable."]
        #[inline(always)]
        pub const fn set_link_ie_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Warm reset (unconnected) interrupt enable with LPFS."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_warm_rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Warm reset (unconnected) interrupt enable with LPFS."]
        #[inline(always)]
        pub const fn set_link_ie_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "In the U3 state, receive the low-frequency periodic signal (LFPS) wake-up interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u3_wakeup(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "In the U3 state, receive the low-frequency periodic signal (LFPS) wake-up interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_u3_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "HOT RESET, which uses the reset interrupt of the TS1/TS2 ordered set."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hot_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "HOT RESET, which uses the reset interrupt of the TS1/TS2 ordered set."]
        #[inline(always)]
        pub const fn set_link_ie_hot_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HP_BUF send FIFO null interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HP_BUF send FIFO null interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_hpbuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HP_BUF send FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_hpbuf_full(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HP_BUF send FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_hpbuf_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Link entry compliance tests, interrupted with compatibility tests or physical layer conformance tests."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_compliance(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Link entry compliance tests, interrupted with compatibility tests or physical layer conformance tests."]
        #[inline(always)]
        pub const fn set_link_ie_compliance(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Link enters loopback mode for testing and fault isolation interrupt enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_loopback(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Link enters loopback mode for testing and fault isolation interrupt enablement."]
        #[inline(always)]
        pub const fn set_link_ie_loopback(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Link enters Rx.Detect state interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_det(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Link enters Rx.Detect state interrupt enabled."]
        #[inline(always)]
        pub const fn set_link_ie_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Receive a link command flag interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_lmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Receive a link command flag interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_rx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Successfully entering U0, you can send HP packet interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_tx_lmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Successfully entering U0, you can send HP packet interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_tx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Receive LMP timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_rx_lmp_tout(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LMP timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_rx_lmp_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The CAP of the other party receiving the link cannot be combined with its own CAP to make an interrupt enable of the uplink and downlink ports."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cap_fail_int_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The CAP of the other party receiving the link cannot be combined with its own CAP to make an interrupt enable of the uplink and downlink ports."]
        #[inline(always)]
        pub const fn set_rx_cap_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Link receives or sends to PORT_Capability LMP interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn config_done_int_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Link receives or sends to PORT_Capability LMP interrupt enable."]
        #[inline(always)]
        pub const fn set_config_done_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Exit UX failure interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_exit_fail(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Exit UX failure interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_ux_exit_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Send WARM_RESET end interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_tx_warmrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Send WARM_RESET end interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_tx_warmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "UX conversion failure interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_ux_fail(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "UX conversion failure interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_ux_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "U2 timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u2_tout(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "U2 timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_link_ie_u2_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "U1 Timeout Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_u1_tout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "U1 Timeout Interrupt Enable."]
        #[inline(always)]
        pub const fn set_link_ie_u1_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Link state machine change flag interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ie_state_chg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Link state machine change flag interrupt enable."]
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
                .field("rx_cap_fail_int_en", &self.rx_cap_fail_int_en())
                .field("config_done_int_en", &self.config_done_int_en())
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
        #[doc = "LINK enters the U0 state and completes the LINK initialization interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the U0 state and completes the LINK initialization interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LINK enters the SS.RECOVERY status interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_recovery(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the SS.RECOVERY status interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_recovery(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LINK enters the SS.INACTIVE state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_inactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the SS.INACTIVE state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_inactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LINK to enter the SS.DISABLE state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LINK to enter the SS.DISABLE state interrupt flag."]
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
        #[doc = "Wake up timeout interrupt flag (10ms) from U3."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u3_wk_tout(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up timeout interrupt flag (10ms) from U3."]
        #[inline(always)]
        pub const fn set_link_if_u3_wk_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LINK refuses to enter low power mode (U1/U2) interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_rej(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LINK refuses to enter low power mode (U1/U2) interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_ux_rej(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TERM disconnected or connection interrupted flag detected."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_term_pres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TERM disconnected or connection interrupted flag detected."]
        #[inline(always)]
        pub const fn set_link_if_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LINK enters TXEQ state outage flag:. Indicates completion of the POLLING handshake and waits for the software to set the PWR_MODE to P0 in order to enter the TXEQ stage."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_txeq(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters TXEQ state outage flag:. Indicates completion of the POLLING handshake and waits for the software to set the PWR_MODE to P0 in order to enter the TXEQ stage."]
        #[inline(always)]
        pub const fn set_link_if_txeq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag,. after this position 1, the software should set the PWR_MODE to P0, after setting the PWR_MODE to P0, the UX exit timer starts to time, after reaching U1_EXIT_TIMER, U2_EXIT_TIMER, U3_WAKE_TIMER, LINK exits the Ux state, Go to Recovery."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_exit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LINK receives the LFPS ready to exit U1/U2/U3 request interrupt flag,. after this position 1, the software should set the PWR_MODE to P0, after setting the PWR_MODE to P0, the UX exit timer starts to time, after reaching U1_EXIT_TIMER, U2_EXIT_TIMER, U3_WAKE_TIMER, LINK exits the Ux state, Go to Recovery."]
        #[inline(always)]
        pub const fn set_link_if_ux_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The WARM_RESET state change (valid-> invalid or invalid->valid). received by the device interrupted the flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_warm_rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The WARM_RESET state change (valid-> invalid or invalid->valid). received by the device interrupted the flag."]
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
        #[doc = "LINK enters the COMPLIANCE status interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_compliance(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the COMPLIANCE status interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_compliance(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LINK enters the LOOPBACK state interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_loopback(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LINK enters the LOOPBACK state interrupt flag."]
        #[inline(always)]
        pub const fn set_link_if_loopback(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "After entering the RX_DETECT state interrupt flag, this position 1,. the software will set the PWR_MODE to P2, ready for RX_DETECT."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_det(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "After entering the RX_DETECT state interrupt flag, this position 1,. the software will set the PWR_MODE to P2, ready for RX_DETECT."]
        #[inline(always)]
        pub const fn set_link_if_rx_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Receiving LMP interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_lmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Receiving LMP interrupt flags."]
        #[inline(always)]
        pub const fn set_link_if_rx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "LINK initializes the interrupt flag,. and the software configures to send Port Capabilities LMP for port capability switching."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_tx_lmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "LINK initializes the interrupt flag,. and the software configures to send Port Capabilities LMP for port capability switching."]
        #[inline(always)]
        pub const fn set_link_if_tx_lmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "After LINK initialization is complete,. the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_rx_lmp_tout(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "After LINK initialization is complete,. the Port Capabilities/Configuration LMPs timeout (20us) interrupt flag is exchanged."]
        #[inline(always)]
        pub const fn set_link_if_rx_lmp_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The CAP of the other party received by the link cannot be combined with its own CAP to form an interrupt. at the uplink and downlink ports (write 1 to clear it)."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cap_fail_flag(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The CAP of the other party received by the link cannot be combined with its own CAP to form an interrupt. at the uplink and downlink ports (write 1 to clear it)."]
        #[inline(always)]
        pub const fn set_rx_cap_fail_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "The link receives or sends to PORT_Capability LMP interrupt (write 1 to zero)."]
        #[must_use]
        #[inline(always)]
        pub const fn config_done_flag(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "The link receives or sends to PORT_Capability LMP interrupt (write 1 to zero)."]
        #[inline(always)]
        pub const fn set_config_done_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Exit UX fails to interrupt; (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_exit_fail(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Exit UX fails to interrupt; (write 1 clear)."]
        #[inline(always)]
        pub const fn set_link_if_ux_exit_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Interruption at the end of the WARM_RESET send; (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_tx_warmrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Interruption at the end of the WARM_RESET send; (write 1 clear)."]
        #[inline(always)]
        pub const fn set_link_if_tx_warmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Failed to enter the UX interrupted; (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_ux_fail(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Failed to enter the UX interrupted; (write 1 clear)."]
        #[inline(always)]
        pub const fn set_link_if_ux_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "U2 timeout interrupt; (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u2_tout(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "U2 timeout interrupt; (write 1 clear)."]
        #[inline(always)]
        pub const fn set_link_if_u2_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "U1 timeout interrupt; (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_u1_tout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "U1 timeout interrupt; (write 1 clear)."]
        #[inline(always)]
        pub const fn set_link_if_u1_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "LINK_RESET reset to 0 (write 1 clear)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_if_state_chg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LINK_RESET reset to 0 (write 1 clear)."]
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
                .field("rx_cap_fail_flag", &self.rx_cap_fail_flag())
                .field("config_done_flag", &self.config_done_flag())
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
        #[doc = "The delay time for serial bit stream parsing to parallel data is 40ns by default,. as the device writes the delay information to the register, which is invalid by 3 bits lower (because the clock frequency is 125MHz, the delay time needs to be set to 8*n)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_isoch_dly(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The delay time for serial bit stream parsing to parallel data is 40ns by default,. as the device writes the delay information to the register, which is invalid by 3 bits lower (because the clock frequency is 125MHz, the delay time needs to be set to 8*n)."]
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
    #[doc = "LINK ITP timeout mode register."]
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
    #[doc = "PORT_CAP Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpPortCap(pub u32);
    impl LinkLmpPortCap {
        #[doc = "When the automatic configuration of Port Capability fails, the bit is automatically cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_cap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When the automatic configuration of Port Capability fails, the bit is automatically cleared."]
        #[inline(always)]
        pub const fn set_auto_cap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configure the OTG bits in the LMP of the port capability of this port; Used for whether to. go to this OTG function."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_otg_capable(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configure the OTG bits in the LMP of the port capability of this port; Used for whether to. go to this OTG function."]
        #[inline(always)]
        pub const fn set_cfg_otg_capable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Configure the Direction low in the LMP of the port capability."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_down_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Configure the Direction low in the LMP of the port capability."]
        #[inline(always)]
        pub const fn set_cfg_down_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Configure the Direction bit in the LMP of the port capability of this port."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_up_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Configure the Direction bit in the LMP of the port capability of this port."]
        #[inline(always)]
        pub const fn set_cfg_up_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Configure the Tiebreaker bit in the LMP of the Port Capability of this port;. The higher the value, the lower exit."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_tiebreaker(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Configure the Tiebreaker bit in the LMP of the Port Capability of this port;. The higher the value, the lower exit."]
        #[inline(always)]
        pub const fn set_cfg_tiebreaker(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Decide on your port configuration according to your and the other party's PORT Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn link_down_mode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Decide on your port configuration according to your and the other party's PORT Capability."]
        #[inline(always)]
        pub const fn set_link_down_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "17\\]: If the position is 1, it means that the port can be configured as a downbound port;. \\[18\\]: If the position is 1, it means that the port can be configured as an upbound port."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cap_dir(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "17\\]: If the position is 1, it means that the port can be configured as a downbound port;. \\[18\\]: If the position is 1, it means that the port can be configured as an upbound port."]
        #[inline(always)]
        pub const fn set_rx_cap_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "OTG bits in LMP that receive Port Capability; Used for whether to go to this OTG function."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cap_otg(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OTG bits in LMP that receive Port Capability; Used for whether to go to this OTG function."]
        #[inline(always)]
        pub const fn set_rx_cap_otg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Tiebreaker, used for PORT type quorum when two PORTs (REG_PORT_CAP\\[17:16\\]=11) with both uplink and. downlink capabilities are connected, the port with the higher value becomes the downstream port, and if the tiebreakers of the two PORTs are equal, the PORT_CAP will be re-exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cap_tiebrk(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Tiebreaker, used for PORT type quorum when two PORTs (REG_PORT_CAP\\[17:16\\]=11) with both uplink and. downlink capabilities are connected, the port with the higher value becomes the downstream port, and if the tiebreakers of the two PORTs are equal, the PORT_CAP will be re-exchanged."]
        #[inline(always)]
        pub const fn set_rx_cap_tiebrk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
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
        #[doc = "PORT Capability configuration completion flag (software operation, write 1 clear 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_cap_vld(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PORT Capability configuration completion flag (software operation, write 1 clear 0)."]
        #[inline(always)]
        pub const fn set_link_lmp_tx_cap_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Upon receipt of a valid PORT_CAP_LMP, the protocol stipulates that. the two parties will exchange PORT_CAP_LMP within 20us after entering the U0 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_cap_vld(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Upon receipt of a valid PORT_CAP_LMP, the protocol stipulates that. the two parties will exchange PORT_CAP_LMP within 20us after entering the U0 state."]
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
                .field("auto_cap", &self.auto_cap())
                .field("cfg_otg_capable", &self.cfg_otg_capable())
                .field("cfg_down_mode", &self.cfg_down_mode())
                .field("cfg_up_mode", &self.cfg_up_mode())
                .field("cfg_tiebreaker", &self.cfg_tiebreaker())
                .field("link_down_mode", &self.link_down_mode())
                .field("rx_cap_dir", &self.rx_cap_dir())
                .field("rx_cap_otg", &self.rx_cap_otg())
                .field("rx_cap_tiebrk", &self.rx_cap_tiebrk())
                .field("link_speed", &self.link_speed())
                .field("link_lmp_tx_cap_vld", &self.link_lmp_tx_cap_vld())
                .field("link_lmp_rx_cap_vld", &self.link_lmp_rx_cap_vld())
                .finish()
        }
    }
    #[doc = "LMP Receive Data 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData0(pub u32);
    impl LinkLmpRxData0 {
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[31:0\\]."]
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
    #[doc = "LMP Receives Data 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData1(pub u32);
    impl LinkLmpRxData1 {
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[63:32\\]."]
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
    #[doc = "LMP Receives Data 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpRxData2(pub u32);
    impl LinkLmpRxData2 {
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[95:64\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_rx_data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "After LMP reception is complete, HP data is stored in this register \\[95:64\\]."]
        #[inline(always)]
        pub const fn set_link_lmp_rx_data2(&mut self, val: u32) {
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
                .field("link_lmp_rx_data2", &self.link_lmp_rx_data2())
                .finish()
        }
    }
    #[doc = "USB Custom HP Data 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkLmpTxData0(pub u32);
    impl LinkLmpTxData0 {
        #[doc = "User-defined data sent to HP \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User-defined data sent to HP \\[31:0\\]."]
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
        #[doc = "User-defined data sent to HP \\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User-defined data sent to HP \\[63:32\\]."]
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
        #[doc = "User-defined data sent to HP \\[95:64\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn link_lmp_tx_data2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User-defined data sent to HP \\[95:64\\]."]
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
        #[doc = "Detect external device counter registers."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdet_exp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Detect external device counter registers."]
        #[inline(always)]
        pub const fn set_rxdet_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "Reset signal for LPM-related registers."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signal for LPM-related registers."]
        #[inline(always)]
        pub const fn set_lpm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "CLKLPM is fixed at 12MHz, when LINK_PD_MODE=3 and LPM_EN=0 and OSCOUTEN=0 and PLLALIV=0,. CLKOSCO will stop and the crystal oscillator will stop."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CLKLPM is fixed at 12MHz, when LINK_PD_MODE=3 and LPM_EN=0 and OSCOUTEN=0 and PLLALIV=0,. CLKOSCO will stop and the crystal oscillator will stop."]
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
        #[doc = "The PHY layer detects the device connection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_term_present(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The PHY layer detects the device connection."]
        #[inline(always)]
        pub const fn set_lpm_term_present(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "When the LPM count reaches the desired value, it is confirmed that a device is connected to the PHY RXDET_EXP this bit is elevated."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_rxdet_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "When the LPM count reaches the desired value, it is confirmed that a device is connected to the PHY RXDET_EXP this bit is elevated."]
        #[inline(always)]
        pub const fn set_lpm_rxdet_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "The channel type of the PHY layer is automatically selected."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_chsel_auto(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The channel type of the PHY layer is automatically selected."]
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
        #[doc = "After RX_DETECT, if there is a receiver terminal resistor, the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_term_pres(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "After RX_DETECT, if there is a receiver terminal resistor, the bit is 1."]
        #[inline(always)]
        pub const fn set_link_rx_term_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receive a valid WARM_RESET signal from the host. (the hardware will automatically pull up after receiving the host's WARM_RESET 18ms)."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_warm_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receive a valid WARM_RESET signal from the host. (the hardware will automatically pull up after receiving the host's WARM_RESET 18ms)."]
        #[inline(always)]
        pub const fn set_link_rx_warm_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LINK busy state, the bit is 1 when the switching PD_MODE is completed,. and the hardware will automatically clear it after the switching is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn link_busy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LINK busy state, the bit is 1 when the switching PD_MODE is completed,. and the hardware will automatically clear it after the switching is completed."]
        #[inline(always)]
        pub const fn set_link_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "When the LINK enters the U0 state, the position 1 will be automatically. cleared after the initialization (broadcast) is completed, and the bit will be automatically cleared when it exits the U0 state."]
        #[must_use]
        #[inline(always)]
        pub const fn link_ready(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "When the LINK enters the U0 state, the position 1 will be automatically. cleared after the initialization (broadcast) is completed, and the bit will be automatically cleared when it exits the U0 state."]
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
        #[doc = "Received a request to exit the UX."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_ux_exit_req(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Received a request to exit the UX."]
        #[inline(always)]
        pub const fn set_link_rx_ux_exit_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The link is at P2 and is RX_DETECT underway."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rx_detect(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "The link is at P2 and is RX_DETECT underway."]
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
        #[doc = "The link is in the U2 state, allowing sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u2_sleep_allow(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in the U2 state, allowing sleep."]
        #[inline(always)]
        pub const fn set_link_u2_sleep_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "The link is in the U3 state, allowing sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn link_u3_sleep_allow(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "The link is in the U3 state, allowing sleep."]
        #[inline(always)]
        pub const fn set_link_u3_sleep_allow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "HP_BUF the status of sending FIFO IDLE."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_idle(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HP_BUF the status of sending FIFO IDLE."]
        #[inline(always)]
        pub const fn set_link_hpbuf_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "HP buffer is full."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_full(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "HP buffer is full."]
        #[inline(always)]
        pub const fn set_link_hpbuf_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "HP buffer is empty."]
        #[must_use]
        #[inline(always)]
        pub const fn link_hpbuf_empty(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "HP buffer is empty."]
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
    #[doc = "U1 Wake Up LFPS Effective Time Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU1WkupFilter(pub u8);
    impl LinkU1WkupFilter {
        #[doc = "The effective duration of the LFPS received when U1 exits.. When the receiving LFPS reaches this time, the received U1 EXIT can be considered valid, and the transmitted LFPS_LAST is raised, and the handshake is successful, which defaults to 600ns. U1_WKUP_FILTER \\[7\\]: the unit of time that controls U1_WKUP_FILTER \\[6:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn u1_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The effective duration of the LFPS received when U1 exits.. When the receiving LFPS reaches this time, the received U1 EXIT can be considered valid, and the transmitted LFPS_LAST is raised, and the handshake is successful, which defaults to 600ns. U1_WKUP_FILTER \\[7\\]: the unit of time that controls U1_WKUP_FILTER \\[6:0\\]."]
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
    #[doc = "LINK U2 Inactivity Timeout Counter Threshold Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU2InactTimer(pub u8);
    impl LinkU2InactTimer {
        #[doc = "Values of the inactivity timeout counter threshold for U2."]
        #[must_use]
        #[inline(always)]
        pub const fn u2_inactive_timer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Values of the inactivity timeout counter threshold for U2."]
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
    #[doc = "U2 Wake LFPS Effective Time Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU2WkupFilter(pub u8);
    impl LinkU2WkupFilter {
        #[doc = "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,. and the received U2 EXIT is considered valid, the duration is (n)us, and the default is 2us."]
        #[must_use]
        #[inline(always)]
        pub const fn u2_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,. and the received U2 EXIT is considered valid, the duration is (n)us, and the default is 2us."]
        #[inline(always)]
        pub const fn set_u2_wkup_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
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
    #[doc = "U3 Wake LFPS Effective Time Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LinkU3WkupFilter(pub u8);
    impl LinkU3WkupFilter {
        #[doc = "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,. and the received U3 EXIT is considered valid, the duration is (n)us, and the default is 100us."]
        #[must_use]
        #[inline(always)]
        pub const fn u3_wkup_filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "When the received LFPS reaches this time, the transmitted LFPS_LAST is raised and the handshake is successful,. and the received U3 EXIT is considered valid, the duration is (n)us, and the default is 100us."]
        #[inline(always)]
        pub const fn set_u3_wkup_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
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
    #[doc = "USBSS Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "USB transaction completion interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_transfer(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction completion interrupt flag bit."]
        #[inline(always)]
        pub const fn set_uif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Device mode: Receives the SETUP transaction completion interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_setup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Device mode: Receives the SETUP transaction completion interrupt flag bit."]
        #[inline(always)]
        pub const fn set_udif_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Device mode: Receives the STATUS transaction to complete the interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_status(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Device mode: Receives the STATUS transaction to complete the interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive PING-TP completion interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rx_ping(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive PING-TP completion interrupt flag bit."]
        #[inline(always)]
        pub const fn set_uif_rx_ping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Send ITP completion interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_itp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Send ITP completion interrupt flag bit."]
        #[inline(always)]
        pub const fn set_uif_itp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Send FIFO overflow interrupt flag bits."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_txov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Send FIFO overflow interrupt flag bits."]
        #[inline(always)]
        pub const fn set_uif_fifo_txov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive FIFO overflow interrupt flag bits."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_rxov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO overflow interrupt flag bits."]
        #[inline(always)]
        pub const fn set_uif_fifo_rxov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "If multiple endpoints have an interrupt flag at the same time,. the priority order is as follows (high-> low): Synchronous transmission > control transmission>block transmission/interrupted transmission, the smaller the endpoint number of the same endpoint type, the higher the priority."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_id(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "If multiple endpoints have an interrupt flag at the same time,. the priority order is as follows (high-> low): Synchronous transmission > control transmission>block transmission/interrupted transmission, the smaller the endpoint number of the same endpoint type, the higher the priority."]
        #[inline(always)]
        pub const fn set_ep_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "If the transmission and reception have an interrupt flag. at the same time, the bit is 1, which prioritizes transmission (SETUP/STATUS independent interrupt)."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_dir(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "If the transmission and reception have an interrupt flag. at the same time, the bit is 1, which prioritizes transmission (SETUP/STATUS independent interrupt)."]
        #[inline(always)]
        pub const fn set_ep_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("udif_status", &self.udif_status())
                .field("uif_rx_ping", &self.uif_rx_ping())
                .field("uif_itp", &self.uif_itp())
                .field("uif_fifo_txov", &self.uif_fifo_txov())
                .field("uif_fifo_rxov", &self.uif_fifo_rxov())
                .field("ep_id", &self.ep_id())
                .field("ep_dir", &self.ep_dir())
                .finish()
        }
    }
    #[doc = "Endpoint 0 Receives Control Register."]
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
        #[doc = "The serial number that the endpoint expects to receive."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The serial number that the endpoint expects to receive."]
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
        #[doc = "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_erdy(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent."]
        #[inline(always)]
        pub const fn set_ep0_rx_erdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "The PP bit in the received DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_rx_pp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "The PP bit in the received DPH."]
        #[inline(always)]
        pub const fn set_ep0_rx_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "The downstream transaction completes the interrupt flag,. the software writes 0 to clear zero, and the hardware sets it to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_ep0_rx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The downstream transaction completes the interrupt flag,. the software writes 0 to clear zero, and the hardware sets it to 1."]
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
    #[doc = "Receive Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxDma(pub u32);
    impl Uep0RxDma {
        #[doc = "Endpoint 0 receives the buffer start address."]
        #[must_use]
        #[inline(always)]
        pub const fn u3ep0_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endpoint 0 receives the buffer start address."]
        #[inline(always)]
        pub const fn set_u3ep0_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep0RxDma {
        #[inline(always)]
        fn default() -> Uep0RxDma {
            Uep0RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep0RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxDma")
                .field("u3ep0_rx_dma", &self.u3ep0_rx_dma())
                .finish()
        }
    }
    #[doc = "Endpoint 0 Transmit Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u32);
    impl Uep0TxCtrl {
        #[doc = "The endpoint sends length registers with a maximum value of 512B."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The endpoint sends length registers with a maximum value of 512B."]
        #[inline(always)]
        pub const fn set_ep0_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "The current sequence number of the endpoint is automatically cleared after receiving the SETUP transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_seq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "The current sequence number of the endpoint is automatically cleared after receiving the SETUP transaction."]
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
        #[doc = "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_erdy(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "The bit writes 1, sends ERDY, and the hardware of the bit is automatically cleared after the ERDY is sent."]
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
        #[doc = "Flag to complete NRDY-TP send (answer)."]
        #[must_use]
        #[inline(always)]
        pub const fn ep0_tx_flow(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Flag to complete NRDY-TP send (answer)."]
        #[inline(always)]
        pub const fn set_ep0_tx_flow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "The upload transaction is interrupted, the software writes 0 to clear zero, and the hardware sets it to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_ep0_tx_act(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "The upload transaction is interrupted, the software writes 0 to clear zero, and the hardware sets it to 1."]
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
    #[doc = "Sending Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxDma(pub u32);
    impl Uep0TxDma {
        #[doc = "Endpoint 0 sends the buffer start address."]
        #[must_use]
        #[inline(always)]
        pub const fn u3ep0_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endpoint 0 sends the buffer start address."]
        #[inline(always)]
        pub const fn set_u3ep0_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Uep0TxDma {
        #[inline(always)]
        fn default() -> Uep0TxDma {
            Uep0TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep0TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TxDma")
                .field("u3ep0_tx_dma", &self.u3ep0_tx_dma())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCfg(pub u8);
    impl UepRxCfg {
        #[doc = "1 means that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1 means that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Allow the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allow the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY, no software control required;. The synchronous endpoint recommends clearing the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY, no software control required;. The synchronous endpoint recommends clearing the bit."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "This bit is 1, and when a short packet or DPH (PP=0) is received,. all CHAIN_EN are cleared, which will cause the DPH to respond to NRDY. Software reconfiguration required."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nrdy_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when a short packet or DPH (PP=0) is received,. all CHAIN_EN are cleared, which will cause the DPH to respond to NRDY. Software reconfiguration required."]
        #[inline(always)]
        pub const fn set_ep_rx_nrdy_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "This bit is 1, and when the continuous burst packet is received, PP=1 and not a short packet,. the DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_tout_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, and when the continuous burst packet is received, PP=1 and not a short packet,. the DPH is not received after the timeout, resulting in TOUT_IF interrupt."]
        #[inline(always)]
        pub const fn set_ep_rx_tout_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "The object being operated on when the access offset address is 0xC~0xF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "The object being operated on when the access offset address is 0xC~0xF."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "A bit of 1 indicates that the current endpoint is using the FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "A bit of 1 indicates that the current endpoint is using the FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches mode, this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches mode, this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxCfg {
        #[inline(always)]
        fn default() -> UepRxCfg {
            UepRxCfg(0)
        }
    }
    impl core::fmt::Debug for UepRxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxCfg")
                .field("ep_rx_iso_mode", &self.ep_rx_iso_mode())
                .field("ep_rx_seq_auto", &self.ep_rx_seq_auto())
                .field("ep_rx_erdy_auto", &self.ep_rx_erdy_auto())
                .field("ep_rx_nrdy_mode", &self.ep_rx_nrdy_mode())
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
    pub struct UepRxChainCr(pub u8);
    impl UepRxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration."]
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
        #[doc = "The currently configured CHAIN serial number."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The currently configured CHAIN serial number."]
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
    impl Default for UepRxChainCr {
        #[inline(always)]
        fn default() -> UepRxChainCr {
            UepRxChainCr(0)
        }
    }
    impl core::fmt::Debug for UepRxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxChainCr")
                .field("ep_rx_ret_sel", &self.ep_rx_ret_sel())
                .field("ep_rx_force_ret", &self.ep_rx_force_ret())
                .field("ep_rx_cur_cfg", &self.ep_rx_cur_cfg())
                .field("ep_rx_cur_use", &self.ep_rx_cur_use())
                .finish()
        }
    }
    #[doc = "The length of the last packet received by the endpoint 1 CHAIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxChainLen(pub u16);
    impl UepRxChainLen {
        #[doc = "The currently completed CHAIN receives the length of the last package."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The currently completed CHAIN receives the length of the last package."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for UepRxChainLen {
        #[inline(always)]
        fn default() -> UepRxChainLen {
            UepRxChainLen(0)
        }
    }
    impl core::fmt::Debug for UepRxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxChainLen")
                .field("ep_rx_chain_rx_len", &self.ep_rx_chain_rx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs that endpoint 1 can receive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxChainMaxNump(pub u8);
    impl UepRxChainMaxNump {
        #[doc = "The number of DPP packets that the CHAIN can receive."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_chain_max_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that the CHAIN can receive."]
        #[inline(always)]
        pub const fn set_rx_chain_max_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRxChainMaxNump {
        #[inline(always)]
        fn default() -> UepRxChainMaxNump {
            UepRxChainMaxNump(0)
        }
    }
    impl core::fmt::Debug for UepRxChainMaxNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxChainMaxNump")
                .field("rx_chain_max_nump", &self.rx_chain_max_nump())
                .finish()
        }
    }
    #[doc = "Number of NUMPs received by endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxChainNump(pub u8);
    impl UepRxChainNump {
        #[doc = "The number of DPP packets that the CHAIN has received."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that the CHAIN has received."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRxChainNump {
        #[inline(always)]
        fn default() -> UepRxChainNump {
            UepRxChainNump(0)
        }
    }
    impl core::fmt::Debug for UepRxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxChainSt(pub u8);
    impl UepRxChainSt {
        #[doc = "The CHAIN serial number that is currently generating the outage."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number that is currently generating the outage."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The current status of the PP bit in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The current status of the PP bit in the DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If the NUMP in the current CHAIN is 0, then the position is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If the NUMP in the current CHAIN is 0, then the position is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used, and there is a CRC32 error in the currently receiving DPP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_iso_pkt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used, and there is a CRC32 error in the currently receiving DPP."]
        #[inline(always)]
        pub const fn set_ep_rx_iso_pkt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Only the synchronous downhaul endpoint is used to receive the LPF status in the DPH currently received."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_lpf_flag(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Only the synchronous downhaul endpoint is used to receive the LPF status in the DPH currently received."]
        #[inline(always)]
        pub const fn set_ep_rx_lpf_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The bit is only written, write 1 to release the current CHAIN_IF;. The interrupt occurs when any of the following conditions are met."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The bit is only written, write 1 to release the current CHAIN_IF;. The interrupt occurs when any of the following conditions are met."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the hardware to. automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the hardware to. automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared after the transmission is completed."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxChainSt {
        #[inline(always)]
        fn default() -> UepRxChainSt {
            UepRxChainSt(0)
        }
    }
    impl core::fmt::Debug for UepRxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxChainSt")
                .field("ep_rx_chain_no", &self.ep_rx_chain_no())
                .field("ep_rx_dph_pp", &self.ep_rx_dph_pp())
                .field("ep_rx_nump_empty", &self.ep_rx_nump_empty())
                .field("ep_rx_iso_pkt_err", &self.ep_rx_iso_pkt_err())
                .field("ep_rx_lpf_flag", &self.ep_rx_lpf_flag())
                .field("ep_rx_chain_if", &self.ep_rx_chain_if())
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCr(pub u8);
    impl UepRxCr {
        #[doc = "The hardware sends the NUMP field of ERDY-TP,. and the general configuration value is the number of bursts supported. For example, if you support level 16 bursts, the field is configured to be 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The hardware sends the NUMP field of ERDY-TP,. and the general configuration value is the number of bursts supported. For example, if you support level 16 bursts, the field is configured to be 16."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 to clear all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to clear all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_rx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint stop, high efficiency, endpoint stop response STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint stop, high efficiency, endpoint stop response STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_rx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxCr {
        #[inline(always)]
        fn default() -> UepRxCr {
            UepRxCr(0)
        }
    }
    impl core::fmt::Debug for UepRxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxCr")
                .field("ep_rx_erdy_nump", &self.ep_rx_erdy_nump())
                .field("ep_rx_chain_clr", &self.ep_rx_chain_clr())
                .field("ep_rx_clr", &self.ep_rx_clr())
                .field("ep_rx_halt", &self.ep_rx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address of endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxDma(pub u32);
    impl UepRxDma {
        #[doc = "In normal mode:. The DMA start address of the CHAIN receiving data. In FIFO mode: The FIFO start address is 16~23 bits of the mapping address in SRAM; The FIFO ends 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ba(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "In normal mode:. The DMA start address of the CHAIN receiving data. In FIFO mode: The FIFO start address is 16~23 bits of the mapping address in SRAM; The FIFO ends 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_dma_ba(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UepRxDma {
        #[inline(always)]
        fn default() -> UepRxDma {
            UepRxDma(0)
        }
    }
    impl core::fmt::Debug for UepRxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxDma")
                .field("chain_dma_ba", &self.chain_dma_ba())
                .finish()
        }
    }
    #[doc = "DMA offset length of endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxDmaOfs(pub u16);
    impl UepRxDmaOfs {
        #[doc = "The offset address of the DPP in the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRxDmaOfs {
        #[inline(always)]
        fn default() -> UepRxDmaOfs {
            UepRxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for UepRxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxDmaOfs")
                .field("chain_dma_ofs", &self.chain_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint Receive Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoint 1~15 downpass Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_en(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "Endpoint 1~15 downpass Enable."]
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
    #[doc = "Endpoint 1 serial number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxSeq(pub u8);
    impl UepRxSeq {
        #[doc = "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for UepRxSeq {
        #[inline(always)]
        fn default() -> UepRxSeq {
            UepRxSeq(0)
        }
    }
    impl core::fmt::Debug for UepRxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxSeq")
                .field("ep_rx_seq_num", &self.ep_rx_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 1 state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxSt(pub u8);
    impl UepRxSt {
        #[doc = "CHAIN enabled state, corresponding to 4 independent CHAINs."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CHAIN enabled state, corresponding to 4 independent CHAINs."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 independent CHAINs."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 independent CHAINs."]
        #[inline(always)]
        pub const fn set_ep_rx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_ep_rx_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is currently in the flow control state, write 1 to clear it."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is currently in the flow control state, write 1 to clear it."]
        #[inline(always)]
        pub const fn set_ep_rx_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_ep_rx_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxSt {
        #[inline(always)]
        fn default() -> UepRxSt {
            UepRxSt(0)
        }
    }
    impl core::fmt::Debug for UepRxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxSt")
                .field("ep_rx_chain_en", &self.ep_rx_chain_en())
                .field("ep_rx_chain_res", &self.ep_rx_chain_res())
                .field("ep_rx_erdy_req", &self.ep_rx_erdy_req())
                .field("ep_rx_fc_st", &self.ep_rx_fc_st())
                .field("ep_rx_int_flag", &self.ep_rx_int_flag())
                .finish()
        }
    }
    #[doc = "Endpoint 1 configures the registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCfg(pub u8);
    impl UepTxCfg {
        #[doc = "1 means that the current endpoint is a synchronous endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_iso_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1 means that the current endpoint is a synchronous endpoint."]
        #[inline(always)]
        pub const fn set_ep_tx_iso_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Allow the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_seq_auto(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allow the software to write R16_EPn_ST->EP_SEQ_NUM."]
        #[inline(always)]
        pub const fn set_ep_tx_seq_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY, no software control required;. The synchronous endpoint recommends clearing the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_auto(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERDY automatic mode, the hardware will send ERDY, no software control required;. The synchronous endpoint recommends clearing the bit."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If a short packet is sent, then EOB/LPF in ACK-TP."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_eob_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If a short packet is sent, then EOB/LPF in ACK-TP."]
        #[inline(always)]
        pub const fn set_ep_tx_eob_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "The object being operated on when the access offset address is 0xC~0xF."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_cfg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "The object being operated on when the access offset address is 0xC~0xF."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "A bit of 1 indicates that the current endpoint is using the FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_fifo_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "A bit of 1 indicates that the current endpoint is using the FIFO mode,. and the start and end addresses of the FIFO should be configured before the bit is 1."]
        #[inline(always)]
        pub const fn set_ep_tx_fifo_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "CHAIN automatically switches mode, this mode is recommended."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_auto(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN automatically switches mode, this mode is recommended."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxCfg {
        #[inline(always)]
        fn default() -> UepTxCfg {
            UepTxCfg(0)
        }
    }
    impl core::fmt::Debug for UepTxCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxCfg")
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
    pub struct UepTxChainCr(pub u8);
    impl UepTxChainCr {
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ret_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "When the FORCE_RET is valid, this bit indicates the returned CHAIN state and configuration."]
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
        #[doc = "The currently configured CHAIN serial number."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cur_cfg(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "The currently configured CHAIN serial number."]
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
    impl Default for UepTxChainCr {
        #[inline(always)]
        fn default() -> UepTxChainCr {
            UepTxChainCr(0)
        }
    }
    impl core::fmt::Debug for UepTxChainCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxChainCr")
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
    pub struct UepTxChainExpNump(pub u8);
    impl UepTxChainExpNump {
        #[doc = "The number of DPP packets that can be sent by the CHAIN that has been completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_exp_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that can be sent by the CHAIN that has been completed."]
        #[inline(always)]
        pub const fn set_tx_chain_exp_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTxChainExpNump {
        #[inline(always)]
        fn default() -> UepTxChainExpNump {
            UepTxChainExpNump(0)
        }
    }
    impl core::fmt::Debug for UepTxChainExpNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxChainExpNump")
                .field("tx_chain_exp_nump", &self.tx_chain_exp_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN sends the last packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxChainLen(pub u16);
    impl UepTxChainLen {
        #[doc = "The CHAIN sends the length of the last packet."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The CHAIN sends the length of the last packet."]
        #[inline(always)]
        pub const fn set_chain_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for UepTxChainLen {
        #[inline(always)]
        fn default() -> UepTxChainLen {
            UepTxChainLen(0)
        }
    }
    impl core::fmt::Debug for UepTxChainLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxChainLen")
                .field("chain_tx_len", &self.chain_tx_len())
                .finish()
        }
    }
    #[doc = "Number of NUMPs sent by endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxChainNump(pub u8);
    impl UepTxChainNump {
        #[doc = "The number of DPP packets that have been transmitted; When the CHIN_EN is 0,. the register is automatically cleared at the same time as the UEP_CHAIN_NUMP is written."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The number of DPP packets that have been transmitted; When the CHIN_EN is 0,. the register is automatically cleared at the same time as the UEP_CHAIN_NUMP is written."]
        #[inline(always)]
        pub const fn set_tx_chain_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTxChainNump {
        #[inline(always)]
        fn default() -> UepTxChainNump {
            UepTxChainNump(0)
        }
    }
    impl core::fmt::Debug for UepTxChainNump {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxChainNump")
                .field("tx_chain_nump", &self.tx_chain_nump())
                .finish()
        }
    }
    #[doc = "Endpoint 1 CHAIN status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxChainSt(pub u8);
    impl UepTxChainSt {
        #[doc = "The CHAIN serial number that is currently generating the outage."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_no(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The CHAIN serial number that is currently generating the outage."]
        #[inline(always)]
        pub const fn set_tx_chain_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "The current status of the PP bit in the DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dph_pp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The current status of the PP bit in the DPH."]
        #[inline(always)]
        pub const fn set_tx_dph_pp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "If the NUMP in the current CHAIN is 0, then the position is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_nump_empty(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "If the NUMP in the current CHAIN is 0, then the position is 1."]
        #[inline(always)]
        pub const fn set_tx_nump_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "The EOB/LPF bits in the last pack of DPH of the current CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_eob_lpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "The EOB/LPF bits in the last pack of DPH of the current CHAIN."]
        #[inline(always)]
        pub const fn set_tx_eob_lpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The bit is only written, write 1 to release the current CHAIN_IF;. The current CHAIN NUMP_EMPTY is 1 to cause this outage."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The bit is only written, write 1 to release the current CHAIN_IF;. The current CHAIN NUMP_EMPTY is 1 to cause this outage."]
        #[inline(always)]
        pub const fn set_tx_chain_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The currently used CHAIN enables the hardware to. automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared after the transmission is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The currently used CHAIN enables the hardware to. automatically set 1 after the UEP_CHAIN_NUMP register is configured, and the CHAIN is automatically cleared after the transmission is completed."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxChainSt {
        #[inline(always)]
        fn default() -> UepTxChainSt {
            UepTxChainSt(0)
        }
    }
    impl core::fmt::Debug for UepTxChainSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxChainSt")
                .field("tx_chain_no", &self.tx_chain_no())
                .field("tx_dph_pp", &self.tx_dph_pp())
                .field("tx_nump_empty", &self.tx_nump_empty())
                .field("tx_eob_lpf", &self.tx_eob_lpf())
                .field("tx_chain_if", &self.tx_chain_if())
                .field("tx_chain_en", &self.tx_chain_en())
                .finish()
        }
    }
    #[doc = "Endpoint 1 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCr(pub u8);
    impl UepTxCr {
        #[doc = "The hardware sends the NUMP field of ERDY-TP,. and the general configuration value is the number of bursts supported. For example, if you support level 16 bursts, the field is configured to be 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_erdy_nump(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The hardware sends the NUMP field of ERDY-TP,. and the general configuration value is the number of bursts supported. For example, if you support level 16 bursts, the field is configured to be 16."]
        #[inline(always)]
        pub const fn set_ep_tx_erdy_nump(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Write 1 to clear all CHAIN configuration values and statuses."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_chain_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to clear all CHAIN configuration values and statuses."]
        #[inline(always)]
        pub const fn set_ep_tx_chain_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_clr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to clear all configuration values and states of the endpoint, except for UEP_CFG."]
        #[inline(always)]
        pub const fn set_ep_tx_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint stop, high efficiency, endpoint stop response STALL to DPH."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_halt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint stop, high efficiency, endpoint stop response STALL to DPH."]
        #[inline(always)]
        pub const fn set_ep_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxCr {
        #[inline(always)]
        fn default() -> UepTxCr {
            UepTxCr(0)
        }
    }
    impl core::fmt::Debug for UepTxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxCr")
                .field("ep_tx_erdy_nump", &self.ep_tx_erdy_nump())
                .field("ep_tx_chain_clr", &self.ep_tx_chain_clr())
                .field("ep_tx_clr", &self.ep_tx_clr())
                .field("ep_tx_halt", &self.ep_tx_halt())
                .finish()
        }
    }
    #[doc = "DMA start address of endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxDma(pub u32);
    impl UepTxDma {
        #[doc = "In normal mode:. The DMA start address of the CHAIN sending data. In FIFO mode: The FIFO start address is 16~23 bits of the mapping address in SRAM; The FIFO ends 16~23 bits of the mapped address in SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "In normal mode:. The DMA start address of the CHAIN sending data. In FIFO mode: The FIFO start address is 16~23 bits of the mapping address in SRAM; The FIFO ends 16~23 bits of the mapped address in SRAM."]
        #[inline(always)]
        pub const fn set_chain_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UepTxDma {
        #[inline(always)]
        fn default() -> UepTxDma {
            UepTxDma(0)
        }
    }
    impl core::fmt::Debug for UepTxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxDma")
                .field("chain_tx_dma", &self.chain_tx_dma())
                .finish()
        }
    }
    #[doc = "DMA offset length of endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxDmaOfs(pub u16);
    impl UepTxDmaOfs {
        #[doc = "The offset address of the DPP in the CHAIN."]
        #[must_use]
        #[inline(always)]
        pub const fn chain_tx_dma_ofs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The offset address of the DPP in the CHAIN."]
        #[inline(always)]
        pub const fn set_chain_tx_dma_ofs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepTxDmaOfs {
        #[inline(always)]
        fn default() -> UepTxDmaOfs {
            UepTxDmaOfs(0)
        }
    }
    impl core::fmt::Debug for UepTxDmaOfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxDmaOfs")
                .field("chain_tx_dma_ofs", &self.chain_tx_dma_ofs())
                .finish()
        }
    }
    #[doc = "Endpoint Send Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoint 1~15 Upload Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx_en(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "Endpoint 1~15 Upload Enable."]
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
    #[doc = "Endpoint 1 serial number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxSeq(pub u8);
    impl UepTxSeq {
        #[doc = "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_seq_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "The current serial number of the endpoint is writable in non-SEQ_AUTO mode and read-only in SEQ_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_seq_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for UepTxSeq {
        #[inline(always)]
        fn default() -> UepTxSeq {
            UepTxSeq(0)
        }
    }
    impl core::fmt::Debug for UepTxSeq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxSeq")
                .field("tx_ep_seq_num", &self.tx_ep_seq_num())
                .finish()
        }
    }
    #[doc = "Endpoint 1 state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxSt(pub u8);
    impl UepTxSt {
        #[doc = "CHAIN enabled state, corresponding to 4 independent CHAINs."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CHAIN enabled state, corresponding to 4 independent CHAINs."]
        #[inline(always)]
        pub const fn set_tx_chain_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "CHAIN response state, corresponding to 4 independent CHAINs."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_chain_res(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CHAIN response state, corresponding to 4 independent CHAINs."]
        #[inline(always)]
        pub const fn set_tx_chain_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_erdy_req(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "It indicates that it is currently in the sending ERDY;. Writing 1 to that bit will send ERDY, used in non-ERDY_AUTO mode."]
        #[inline(always)]
        pub const fn set_tx_ep_erdy_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The endpoint is currently in the flow control state, write 1 to clear it."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_fc_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is currently in the flow control state, write 1 to clear it."]
        #[inline(always)]
        pub const fn set_tx_ep_fc_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ep_int_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The endpoint is currently interrupted, the bit is read-only, and if all CHAIN_IF is 0, the bit is 0, otherwise it is 1."]
        #[inline(always)]
        pub const fn set_tx_ep_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxSt {
        #[inline(always)]
        fn default() -> UepTxSt {
            UepTxSt(0)
        }
    }
    impl core::fmt::Debug for UepTxSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxSt")
                .field("tx_chain_en", &self.tx_chain_en())
                .field("tx_chain_res", &self.tx_chain_res())
                .field("tx_ep_erdy_req", &self.tx_ep_erdy_req())
                .field("tx_ep_fc_st", &self.tx_ep_fc_st())
                .field("tx_ep_int_flag", &self.tx_ep_int_flag())
                .finish()
        }
    }
}
