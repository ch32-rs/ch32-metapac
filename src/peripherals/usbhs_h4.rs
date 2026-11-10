#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs {}
unsafe impl Sync for Usbhs {}
impl Usbhs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control register."]
    #[inline(always)]
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
#[doc = "USBHS in device mode. Endpoint configuration / TX / RX registers (UEP* family)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsDevice {
    ptr: *mut u8,
}
unsafe impl Send for UsbhsDevice {}
unsafe impl Sync for UsbhsDevice {}
impl UsbhsDevice {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control register."]
    #[inline(always)]
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_t_tog_auto(self) -> crate::common::Reg<regs::UepTTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_r_tog_auto(self) -> crate::common::Reg<regs::UepRTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[inline(always)]
    pub const fn uep_t_burst(self) -> crate::common::Reg<regs::UepTBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB endpoint send the mode register."]
    #[inline(always)]
    pub const fn uep_t_burst_mode(
        self,
    ) -> crate::common::Reg<regs::UepTBurstMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[inline(always)]
    pub const fn uep_r_burst(self) -> crate::common::Reg<regs::UepRBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "USB endpoint reply mode register."]
    #[inline(always)]
    pub const fn uep_r_res_mode(self) -> crate::common::Reg<regs::UepRResMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[inline(always)]
    pub const fn uep_af_mode(self) -> crate::common::Reg<regs::UepAfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[inline(always)]
    pub const fn uep0_dma(self) -> crate::common::Reg<regs::Uep0Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "endpoint 1-7 receive the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep_rx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 1-7 send the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep_tx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[inline(always)]
    pub const fn uep0_max_len(self) -> crate::common::Reg<regs::Uep0MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "endpoint 1-7 max length packet register."]
    #[inline(always)]
    pub const fn uep_max_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepMaxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[inline(always)]
    pub const fn uep0_rx_len(self) -> crate::common::Reg<regs::Uep0RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "endpoint 1-7 receive the length register in a single pass."]
    #[inline(always)]
    pub const fn uep_rx_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoints 1-7."]
    #[inline(always)]
    pub const fn uep_r_size(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRSize, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 send the length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9fusize) as _) }
    }
    #[doc = "endpoint 1-7 send the length register."]
    #[inline(always)]
    pub const fn uep_t_len(self, n: usize) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 1-7 send control register."]
    #[inline(always)]
    pub const fn uep_tx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 1-7 receive control."]
    #[inline(always)]
    pub const fn uep_rx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa3usize + n * 4usize) as _) }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_t_iso(self) -> crate::common::Reg<regs::UepTIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_r_iso(self) -> crate::common::Reg<regs::UepRIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoints 1-7."]
    #[inline(always)]
    pub const fn uep_rx_fifo(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxFifo, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoints 1-7."]
    #[inline(always)]
    pub const fn uep_tx_fifo(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxFifo, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize + n * 4usize) as _) }
    }
}
#[doc = "USBHS in host mode. UH_* host control registers; offsets overlap with UEP* device-mode pairs."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsHost {
    ptr: *mut u8,
}
unsafe impl Send for UsbhsHost {}
unsafe impl Sync for UsbhsHost {}
impl UsbhsHost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control register."]
    #[inline(always)]
    pub const fn usb_ctrl(self) -> crate::common::Reg<regs::UsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "USB host Configuration register."]
    #[inline(always)]
    pub const fn uh_cfg(self) -> crate::common::Reg<regs::UhCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB host interrupt enable register."]
    #[inline(always)]
    pub const fn uh_int_en(self) -> crate::common::Reg<regs::UhIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "USB host device address register."]
    #[inline(always)]
    pub const fn uh_dev_ad(self) -> crate::common::Reg<regs::UhDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "USB host control register."]
    #[inline(always)]
    pub const fn uh_control(self) -> crate::common::Reg<regs::UhControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[inline(always)]
    pub const fn uh_int_flag(self) -> crate::common::Reg<regs::UhIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "USB host interrupt status."]
    #[inline(always)]
    pub const fn uh_int_st(self) -> crate::common::Reg<regs::UhIntSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "USB host miscellaneous status."]
    #[inline(always)]
    pub const fn uh_mis_st(self) -> crate::common::Reg<regs::UhMisSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "USB host power management data register."]
    #[inline(always)]
    pub const fn uh_lpm_data(self) -> crate::common::Reg<regs::UhLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "USB host SPLIT register."]
    #[inline(always)]
    pub const fn uh_split_data(self) -> crate::common::Reg<regs::UhSplitData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "USB host frame register."]
    #[inline(always)]
    pub const fn uh_frame(self) -> crate::common::Reg<regs::UhFrame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "USB host send length register."]
    #[inline(always)]
    pub const fn uh_tx_len(self) -> crate::common::Reg<regs::UhTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "USB host receive length register."]
    #[inline(always)]
    pub const fn uh_rx_len(self) -> crate::common::Reg<regs::UhRxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "USB host receives the maximum length register."]
    #[inline(always)]
    pub const fn uh_rx_max_len(self) -> crate::common::Reg<regs::UhRxMaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "DMA receive address register."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<regs::UhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "DMA send address register."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<regs::UhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "USB host port control register."]
    #[inline(always)]
    pub const fn uh_port_ctrl(self) -> crate::common::Reg<regs::UhPortCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "USB host port Configuration register."]
    #[inline(always)]
    pub const fn uh_port_cfg(self) -> crate::common::Reg<regs::UhPortCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "USB host port interrupt enable register."]
    #[inline(always)]
    pub const fn uh_port_int_en(self) -> crate::common::Reg<regs::UhPortIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0132usize) as _) }
    }
    #[doc = "USB host port test mode register."]
    #[inline(always)]
    pub const fn uh_port_test_ct(
        self,
    ) -> crate::common::Reg<regs::UhPortTestCt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0133usize) as _) }
    }
    #[doc = "USB host port status register."]
    #[inline(always)]
    pub const fn uh_port_st(self) -> crate::common::Reg<regs::UhPortSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "USB host port state charge register."]
    #[inline(always)]
    pub const fn uh_port_chg(self) -> crate::common::Reg<regs::UhPortChg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0136usize) as _) }
    }
    #[doc = "USB host BC charging control register."]
    #[inline(always)]
    pub const fn uh_bc_ctrl(self) -> crate::common::Reg<regs::UhBcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
}
pub mod regs {
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Dma(pub u32);
    impl Uep0Dma {
        #[doc = "The start address of the endpoint 0 buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The start address of the endpoint 0 buffer."]
        #[inline(always)]
        pub const fn set_uep0_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep0Dma {
        #[inline(always)]
        fn default() -> Uep0Dma {
            Uep0Dma(0)
        }
    }
    impl core::fmt::Debug for Uep0Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0Dma")
                .field("uep0_dma", &self.uep0_dma())
                .finish()
        }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0MaxLen(pub u32);
    impl Uep0MaxLen {
        #[doc = "endpoint 0 max acceptable offset length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 max acceptable offset length."]
        #[inline(always)]
        pub const fn set_uep0_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0MaxLen {
        #[inline(always)]
        fn default() -> Uep0MaxLen {
            Uep0MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep0MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0MaxLen")
                .field("uep0_max_len", &self.uep0_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u8);
    impl Uep0RxCtrl {
        #[doc = "endpoint 0 has control over the received response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 has control over the received response."]
        #[inline(always)]
        pub const fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_setup_is(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[inline(always)]
        pub const fn set_uep_r_setup_is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 receives the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end flag."]
        #[inline(always)]
        pub const fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("uep_r_res_mask", &self.uep_r_res_mask())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_setup_is", &self.uep_r_setup_is())
                .field("uep_r_tog_match", &self.uep_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("uep_r_done", &self.uep_r_done())
                .finish()
        }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxLen(pub u16);
    impl Uep0RxLen {
        #[doc = "endpoint 0 acceptable data length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_rx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 acceptable data length."]
        #[inline(always)]
        pub const fn set_uep0_rx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0RxLen {
        #[inline(always)]
        fn default() -> Uep0RxLen {
            Uep0RxLen(0)
        }
    }
    impl core::fmt::Debug for Uep0RxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxLen")
                .field("uep0_rx_len", &self.uep0_rx_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send the length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u16);
    impl Uep0TLen {
        #[doc = "endpoint 0 send the length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 send the length."]
        #[inline(always)]
        pub const fn set_uep0_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0TLen {
        #[inline(always)]
        fn default() -> Uep0TLen {
            Uep0TLen(0)
        }
    }
    impl core::fmt::Debug for Uep0TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TLen")
                .field("uep0_t_len", &self.uep0_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u8);
    impl Uep0TxCtrl {
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("uep_t_res_mask", &self.uep_t_res_mask())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("uep_t_done", &self.uep_t_done())
                .finish()
        }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepAfMode(pub u32);
    impl UepAfMode {
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_af(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[inline(always)]
        pub const fn set_uep_t_af(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for UepAfMode {
        #[inline(always)]
        fn default() -> UepAfMode {
            UepAfMode(0)
        }
    }
    impl core::fmt::Debug for UepAfMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepAfMode")
                .field("uep_t_af", &self.uep_t_af())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepMaxLen(pub u32);
    impl UepMaxLen {
        #[doc = "endpoint 1-7 max acceptable offset length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "endpoint 1-7 max acceptable offset length."]
        #[inline(always)]
        pub const fn set_uep_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UepMaxLen {
        #[inline(always)]
        fn default() -> UepMaxLen {
            UepMaxLen(0)
        }
    }
    impl core::fmt::Debug for UepMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepMaxLen")
                .field("uep_max_len", &self.uep_max_len())
                .finish()
        }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRBurst(pub u8);
    impl UepRBurst {
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[inline(always)]
        pub const fn set_uep_r_burst_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRBurst {
        #[inline(always)]
        fn default() -> UepRBurst {
            UepRBurst(0)
        }
    }
    impl core::fmt::Debug for UepRBurst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRBurst")
                .field("uep_r_burst_en", &self.uep_r_burst_en())
                .finish()
        }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRIso(pub u16);
    impl UepRIso {
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_iso_en(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[inline(always)]
        pub const fn set_uep_r_iso_en(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_fifo_en(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub const fn set_uep_r_fifo_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
        }
    }
    impl Default for UepRIso {
        #[inline(always)]
        fn default() -> UepRIso {
            UepRIso(0)
        }
    }
    impl core::fmt::Debug for UepRIso {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRIso")
                .field("uep_r_iso_en[0]", &self.uep_r_iso_en(0usize))
                .field("uep_r_iso_en[1]", &self.uep_r_iso_en(1usize))
                .field("uep_r_iso_en[2]", &self.uep_r_iso_en(2usize))
                .field("uep_r_iso_en[3]", &self.uep_r_iso_en(3usize))
                .field("uep_r_iso_en[4]", &self.uep_r_iso_en(4usize))
                .field("uep_r_iso_en[5]", &self.uep_r_iso_en(5usize))
                .field("uep_r_iso_en[6]", &self.uep_r_iso_en(6usize))
                .field("uep_r_fifo_en", &self.uep_r_fifo_en())
                .finish()
        }
    }
    #[doc = "USB endpoint reply mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRResMode(pub u8);
    impl UepRResMode {
        #[doc = "0 to 7 endpoint reply mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_res_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint reply mode."]
        #[inline(always)]
        pub const fn set_uep_r_res_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRResMode {
        #[inline(always)]
        fn default() -> UepRResMode {
            UepRResMode(0)
        }
    }
    impl core::fmt::Debug for UepRResMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRResMode")
                .field("uep_r_res_mode", &self.uep_r_res_mode())
                .finish()
        }
    }
    #[doc = "the length register of the total received data at endpoints 1-7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRSize(pub u16);
    impl UepRSize {
        #[doc = "the length of the total received data at endpoint 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 1."]
        #[inline(always)]
        pub const fn set_uep_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRSize {
        #[inline(always)]
        fn default() -> UepRSize {
            UepRSize(0)
        }
    }
    impl core::fmt::Debug for UepRSize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRSize")
                .field("uep_r_size", &self.uep_r_size())
                .finish()
        }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRTogAuto(pub u16);
    impl UepRTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_auto(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn set_uep_r_tog_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
    }
    impl Default for UepRTogAuto {
        #[inline(always)]
        fn default() -> UepRTogAuto {
            UepRTogAuto(0)
        }
    }
    impl core::fmt::Debug for UepRTogAuto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRTogAuto")
                .field("uep_r_tog_auto", &self.uep_r_tog_auto())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCtrl(pub u8);
    impl UepRxCtrl {
        #[doc = "endpoint 1-7 has control over the received response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1-7 has control over the received response."]
        #[inline(always)]
        pub const fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 1-7 expects a synchronous trigger. bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 1-7 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 1-7 for the received return NAK,packet type."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1-7 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 1-7 receive the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1-7 receive the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 1-7 receive the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1-7 receive the end flag."]
        #[inline(always)]
        pub const fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxCtrl {
        #[inline(always)]
        fn default() -> UepRxCtrl {
            UepRxCtrl(0)
        }
    }
    impl core::fmt::Debug for UepRxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxCtrl")
                .field("uep_r_res_mask", &self.uep_r_res_mask())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_tog_match", &self.uep_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("uep_r_done", &self.uep_r_done())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 receive the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxDma(pub u32);
    impl UepRxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn set_uep_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
                .field("uep_rx_dma", &self.uep_rx_dma())
                .finish()
        }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[inline(always)]
        pub const fn set_uep_rx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
                .field("uep_rx_en", &self.uep_rx_en())
                .finish()
        }
    }
    #[doc = "Receive FIFO address of usb endpoints 1-7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxFifo(pub u32);
    impl UepRxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for UepRxFifo {
        #[inline(always)]
        fn default() -> UepRxFifo {
            UepRxFifo(0)
        }
    }
    impl core::fmt::Debug for UepRxFifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxFifo")
                .field("uep_rx_fifo_s", &self.uep_rx_fifo_s())
                .field("uep_rx_fifo_e", &self.uep_rx_fifo_e())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 receive the length register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxLen(pub u16);
    impl UepRxLen {
        #[doc = "the length of data received by endpoint 1-7 in a single pass."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 1-7 in a single pass."]
        #[inline(always)]
        pub const fn set_uep_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRxLen {
        #[inline(always)]
        fn default() -> UepRxLen {
            UepRxLen(0)
        }
    }
    impl core::fmt::Debug for UepRxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxLen")
                .field("uep_rx_len", &self.uep_rx_len())
                .finish()
        }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurst(pub u8);
    impl UepTBurst {
        #[doc = "0 to 7 endpoint send burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send burst enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurst {
        #[inline(always)]
        fn default() -> UepTBurst {
            UepTBurst(0)
        }
    }
    impl core::fmt::Debug for UepTBurst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTBurst")
                .field("uep_t_burst_en", &self.uep_t_burst_en())
                .finish()
        }
    }
    #[doc = "USB endpoint send the mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurstMode(pub u8);
    impl UepTBurstMode {
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurstMode {
        #[inline(always)]
        fn default() -> UepTBurstMode {
            UepTBurstMode(0)
        }
    }
    impl core::fmt::Debug for UepTBurstMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTBurstMode")
                .field("uep_t_burst_mode", &self.uep_t_burst_mode())
                .finish()
        }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTIso(pub u16);
    impl UepTIso {
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_iso_en(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[inline(always)]
        pub const fn set_uep_t_iso_en(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_fifo_en(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub const fn set_uep_t_fifo_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
        }
    }
    impl Default for UepTIso {
        #[inline(always)]
        fn default() -> UepTIso {
            UepTIso(0)
        }
    }
    impl core::fmt::Debug for UepTIso {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTIso")
                .field("uep_t_iso_en[0]", &self.uep_t_iso_en(0usize))
                .field("uep_t_iso_en[1]", &self.uep_t_iso_en(1usize))
                .field("uep_t_iso_en[2]", &self.uep_t_iso_en(2usize))
                .field("uep_t_iso_en[3]", &self.uep_t_iso_en(3usize))
                .field("uep_t_iso_en[4]", &self.uep_t_iso_en(4usize))
                .field("uep_t_iso_en[5]", &self.uep_t_iso_en(5usize))
                .field("uep_t_iso_en[6]", &self.uep_t_iso_en(6usize))
                .field("uep_t_fifo_en", &self.uep_t_fifo_en())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u16);
    impl UepTLen {
        #[doc = "endpoint 1-7 send the length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 1-7 send the length."]
        #[inline(always)]
        pub const fn set_uep_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepTLen {
        #[inline(always)]
        fn default() -> UepTLen {
            UepTLen(0)
        }
    }
    impl core::fmt::Debug for UepTLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTLen")
                .field("uep_t_len", &self.uep_t_len())
                .finish()
        }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTTogAuto(pub u16);
    impl UepTTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_auto(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn set_uep_t_tog_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
    }
    impl Default for UepTTogAuto {
        #[inline(always)]
        fn default() -> UepTTogAuto {
            UepTTogAuto(0)
        }
    }
    impl core::fmt::Debug for UepTTogAuto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTTogAuto")
                .field("uep_t_tog_auto", &self.uep_t_tog_auto())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCtrl(pub u8);
    impl UepTxCtrl {
        #[doc = "endpoint 1-7 control of the send response to IN transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1-7 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 1-7 synchronous trigger bit for the sender to. prepare."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1-7 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 1-7 send the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1-7 send the end flag."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 1-7 send the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1-7 send the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxCtrl {
        #[inline(always)]
        fn default() -> UepTxCtrl {
            UepTxCtrl(0)
        }
    }
    impl core::fmt::Debug for UepTxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxCtrl")
                .field("uep_t_res_mask", &self.uep_t_res_mask())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("uep_t_done", &self.uep_t_done())
                .finish()
        }
    }
    #[doc = "endpoint 1-7 send the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxDma(pub u32);
    impl UepTxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn set_uep_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
                .field("uep_tx_dma", &self.uep_tx_dma())
                .finish()
        }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[inline(always)]
        pub const fn set_uep_tx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
                .field("uep_tx_en", &self.uep_tx_en())
                .finish()
        }
    }
    #[doc = "The sending FIFO address of usb endpoints 1-7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxFifo(pub u32);
    impl UepTxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for UepTxFifo {
        #[inline(always)]
        fn default() -> UepTxFifo {
            UepTxFifo(0)
        }
    }
    impl core::fmt::Debug for UepTxFifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxFifo")
                .field("uep_tx_fifo_s", &self.uep_tx_fifo_s())
                .field("uep_tx_fifo_e", &self.uep_tx_fifo_e())
                .finish()
        }
    }
    #[doc = "USB host BC charging control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhBcCtrl(pub u32);
    impl UhBcCtrl {
        #[doc = "UDP pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "UDP pin BC protocol comparator enables."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn bc_auto_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic mode enables."]
        #[inline(always)]
        pub const fn set_bc_auto_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub const fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub const fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_vsrc_act(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
        #[inline(always)]
        pub const fn set_udm_vsrc_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for UhBcCtrl {
        #[inline(always)]
        fn default() -> UhBcCtrl {
            UhBcCtrl(0)
        }
    }
    impl core::fmt::Debug for UhBcCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhBcCtrl")
                .field("udp_bc_cmpo", &self.udp_bc_cmpo())
                .field("udm_bc_cmpo", &self.udm_bc_cmpo())
                .field("udp_bc_cmpe", &self.udp_bc_cmpe())
                .field("udm_bc_cmpe", &self.udm_bc_cmpe())
                .field("bc_auto_mode", &self.bc_auto_mode())
                .field("udp_bc_vsrc", &self.udp_bc_vsrc())
                .field("udm_bc_vsrc", &self.udm_bc_vsrc())
                .field("udm_vsrc_act", &self.udm_vsrc_act())
                .finish()
        }
    }
    #[doc = "USB host Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhCfg(pub u8);
    impl UhCfg {
        #[doc = "USB connection controller module resets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rst_link(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection controller module resets."]
        #[inline(always)]
        pub const fn set_uh_rst_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rst_sie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn set_uh_rst_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_clr_all(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn set_uh_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[inline(always)]
        pub const fn set_uh_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn set_uh_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending function enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_sof_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending function enabled."]
        #[inline(always)]
        pub const fn set_uh_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "forced full speed FS."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_force_fs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "forced full speed FS."]
        #[inline(always)]
        pub const fn set_uh_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "LPM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn set_uh_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhCfg {
        #[inline(always)]
        fn default() -> UhCfg {
            UhCfg(0)
        }
    }
    impl core::fmt::Debug for UhCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhCfg")
                .field("uh_rst_link", &self.uh_rst_link())
                .field("uh_rst_sie", &self.uh_rst_sie())
                .field("uh_clr_all", &self.uh_clr_all())
                .field("uh_phy_suspendm", &self.uh_phy_suspendm())
                .field("uh_dma_en", &self.uh_dma_en())
                .field("uh_sof_en", &self.uh_sof_en())
                .field("uh_force_fs", &self.uh_force_fs())
                .field("uh_lpm_en", &self.uh_lpm_en())
                .finish()
        }
    }
    #[doc = "USB host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhControl(pub u32);
    impl UhControl {
        #[doc = "transaction token packet PID."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_token_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet PID."]
        #[inline(always)]
        pub const fn set_uh_t_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "transaction token packet endpoint number."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_endp_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet endpoint number."]
        #[inline(always)]
        pub const fn set_uh_t_endp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "send data PID."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "send data PID."]
        #[inline(always)]
        pub const fn set_uh_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "DATA cache control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_buf_mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DATA cache control bit."]
        #[inline(always)]
        pub const fn set_uh_buf_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "host enables the transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_host_action(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "host enables the transaction."]
        #[inline(always)]
        pub const fn set_uh_host_action(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "valid to send LMP packet."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_lpm_valid(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send LMP packet."]
        #[inline(always)]
        pub const fn set_uh_lpm_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "valid to send SPLIT packet."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_split_valid(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send SPLIT packet."]
        #[inline(always)]
        pub const fn set_uh_split_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_pre_pid_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[inline(always)]
        pub const fn set_uh_pre_pid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_no_data(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_uh_tx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_no_data(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_uh_rx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_no_res(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_uh_tx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_no_res(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_uh_rx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for UhControl {
        #[inline(always)]
        fn default() -> UhControl {
            UhControl(0)
        }
    }
    impl core::fmt::Debug for UhControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhControl")
                .field("uh_t_token_mask", &self.uh_t_token_mask())
                .field("uh_t_endp_mask", &self.uh_t_endp_mask())
                .field("uh_t_tog_mask", &self.uh_t_tog_mask())
                .field("uh_buf_mode", &self.uh_buf_mode())
                .field("uh_host_action", &self.uh_host_action())
                .field("uh_lpm_valid", &self.uh_lpm_valid())
                .field("uh_split_valid", &self.uh_split_valid())
                .field("uh_pre_pid_en", &self.uh_pre_pid_en())
                .field("uh_tx_no_data", &self.uh_tx_no_data())
                .field("uh_rx_no_data", &self.uh_rx_no_data())
                .field("uh_tx_no_res", &self.uh_tx_no_res())
                .field("uh_rx_no_res", &self.uh_rx_no_res())
                .finish()
        }
    }
    #[doc = "USB host device address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhDevAd(pub u8);
    impl UhDevAd {
        #[doc = "device address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_dev_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "device address."]
        #[inline(always)]
        pub const fn set_uh_dev_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for UhDevAd {
        #[inline(always)]
        fn default() -> UhDevAd {
            UhDevAd(0)
        }
    }
    impl core::fmt::Debug for UhDevAd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhDevAd")
                .field("uh_dev_addr", &self.uh_dev_addr())
                .finish()
        }
    }
    #[doc = "USB host frame register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhFrame(pub u32);
    impl UhFrame {
        #[doc = "the frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "the frame number."]
        #[inline(always)]
        pub const fn set_uh_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "microfame number."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_mframe_no(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "microfame number."]
        #[inline(always)]
        pub const fn set_uh_mframe_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "SOF count enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_sof_cnt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SOF count enabled."]
        #[inline(always)]
        pub const fn set_uh_sof_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "the SOF count is cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_sof_cnt_clr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "the SOF count is cleared."]
        #[inline(always)]
        pub const fn set_uh_sof_cnt_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for UhFrame {
        #[inline(always)]
        fn default() -> UhFrame {
            UhFrame(0)
        }
    }
    impl core::fmt::Debug for UhFrame {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhFrame")
                .field("uh_frame_no", &self.uh_frame_no())
                .field("uh_mframe_no", &self.uh_mframe_no())
                .field("uh_sof_cnt_en", &self.uh_sof_cnt_en())
                .field("uh_sof_cnt_clr", &self.uh_sof_cnt_clr())
                .finish()
        }
    }
    #[doc = "USB host interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntEn(pub u8);
    impl UhIntEn {
        #[doc = "Wakeup interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_wkup_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_wkup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_resume_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[inline(always)]
        pub const fn set_uhie_resume_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[inline(always)]
        pub const fn set_uhie_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_tx_halt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt enable."]
        #[inline(always)]
        pub const fn set_uhie_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_uhie_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntEn {
        #[inline(always)]
        fn default() -> UhIntEn {
            UhIntEn(0)
        }
    }
    impl core::fmt::Debug for UhIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntEn")
                .field("uhie_wkup_act", &self.uhie_wkup_act())
                .field("uhie_resume_act", &self.uhie_resume_act())
                .field("uhie_transfer", &self.uhie_transfer())
                .field("uhie_sof_act", &self.uhie_sof_act())
                .field("uhie_tx_halt", &self.uhie_tx_halt())
                .field("uhie_fifo_over", &self.uhie_fifo_over())
                .finish()
        }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntFlag(pub u8);
    impl UhIntFlag {
        #[doc = "Wakeup interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_wkup_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_wkup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_resume_act_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_resume_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_tx_halt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_uhif_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntFlag {
        #[inline(always)]
        fn default() -> UhIntFlag {
            UhIntFlag(0)
        }
    }
    impl core::fmt::Debug for UhIntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntFlag")
                .field("uhif_wkup_act", &self.uhif_wkup_act())
                .field("uhif_resume_act_if", &self.uhif_resume_act_if())
                .field("uhif_transfer", &self.uhif_transfer())
                .field("uhif_sof_act", &self.uhif_sof_act())
                .field("uhif_tx_halt", &self.uhif_tx_halt())
                .field("uhif_fifo_over", &self.uhif_fifo_over())
                .finish()
        }
    }
    #[doc = "USB host interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntSt(pub u8);
    impl UhIntSt {
        #[doc = "received PID."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_r_token_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "received PID."]
        #[inline(always)]
        pub const fn set_uh_r_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_rx_resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[inline(always)]
        pub const fn set_uhis_port_rx_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UhIntSt {
        #[inline(always)]
        fn default() -> UhIntSt {
            UhIntSt(0)
        }
    }
    impl core::fmt::Debug for UhIntSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntSt")
                .field("uh_r_token_mask", &self.uh_r_token_mask())
                .field("uhis_port_rx_resume", &self.uhis_port_rx_resume())
                .finish()
        }
    }
    #[doc = "USB host power management data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhLpmData(pub u32);
    impl UhLpmData {
        #[doc = "power management data."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "power management data."]
        #[inline(always)]
        pub const fn set_uh_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhLpmData {
        #[inline(always)]
        fn default() -> UhLpmData {
            UhLpmData(0)
        }
    }
    impl core::fmt::Debug for UhLpmData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhLpmData")
                .field("uh_lpm_data", &self.uh_lpm_data())
                .finish()
        }
    }
    #[doc = "USB host miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhMisSt(pub u8);
    impl UhMisSt {
        #[doc = "port enabled state."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_sof_free(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled state."]
        #[inline(always)]
        pub const fn set_uhms_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_sof_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
        #[inline(always)]
        pub const fn set_uhms_sof_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus SOF status."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_sof_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus SOF status."]
        #[inline(always)]
        pub const fn set_uhms_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB bus wakes up."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_usb_wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus wakes up."]
        #[inline(always)]
        pub const fn set_uhms_usb_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB bus status."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_linestate(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USB bus status."]
        #[inline(always)]
        pub const fn set_uhms_linestate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "J on USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_bus_j(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "J on USB bus."]
        #[inline(always)]
        pub const fn set_uhms_bus_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SE0 on USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn uhms_bus_se0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SE0 on USB bus."]
        #[inline(always)]
        pub const fn set_uhms_bus_se0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhMisSt {
        #[inline(always)]
        fn default() -> UhMisSt {
            UhMisSt(0)
        }
    }
    impl core::fmt::Debug for UhMisSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhMisSt")
                .field("uhms_sof_free", &self.uhms_sof_free())
                .field("uhms_sof_pre", &self.uhms_sof_pre())
                .field("uhms_sof_act", &self.uhms_sof_act())
                .field("uhms_usb_wakeup", &self.uhms_usb_wakeup())
                .field("uhms_linestate", &self.uhms_linestate())
                .field("uhms_bus_j", &self.uhms_bus_j())
                .field("uhms_bus_se0", &self.uhms_bus_se0())
                .finish()
        }
    }
    #[doc = "USB host port Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortCfg(pub u8);
    impl UhPortCfg {
        #[doc = "USB port mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_host_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB port mode selection."]
        #[inline(always)]
        pub const fn set_uh_host_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_pd_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
        #[inline(always)]
        pub const fn set_uh_pd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhPortCfg {
        #[inline(always)]
        fn default() -> UhPortCfg {
            UhPortCfg(0)
        }
    }
    impl core::fmt::Debug for UhPortCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortCfg")
                .field("uh_host_en", &self.uh_host_en())
                .field("uh_pd_en", &self.uh_pd_en())
                .finish()
        }
    }
    #[doc = "USB host port state charge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortChg(pub u8);
    impl UhPortChg {
        #[doc = "port connection state charge."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_port_connect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state charge."]
        #[inline(always)]
        pub const fn set_uhif_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "port enable state charge."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enable state charge."]
        #[inline(always)]
        pub const fn set_uhif_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "port pause state charge."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state charge."]
        #[inline(always)]
        pub const fn set_uhif_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "port reset state charge."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_port_reset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset state charge."]
        #[inline(always)]
        pub const fn set_uhif_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "port sleep state charge."]
        #[must_use]
        #[inline(always)]
        pub const fn uhif_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep state charge."]
        #[inline(always)]
        pub const fn set_uhif_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for UhPortChg {
        #[inline(always)]
        fn default() -> UhPortChg {
            UhPortChg(0)
        }
    }
    impl core::fmt::Debug for UhPortChg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortChg")
                .field("uhif_port_connect", &self.uhif_port_connect())
                .field("uhif_port_en", &self.uhif_port_en())
                .field("uhif_port_susp", &self.uhif_port_susp())
                .field("uhif_port_reset", &self.uhif_port_reset())
                .field("uhif_port_slp", &self.uhif_port_slp())
                .finish()
        }
    }
    #[doc = "USB host port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortCtrl(pub u32);
    impl UhPortCtrl {
        #[doc = "PORT send reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_set_port_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PORT send reset."]
        #[inline(always)]
        pub const fn set_uh_set_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the PORT is in a suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_set_port_susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT is in a suspended state."]
        #[inline(always)]
        pub const fn set_uh_set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "the PORT exits the suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_clr_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the suspended state."]
        #[inline(always)]
        pub const fn set_uh_clr_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_set_port_sleep(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[inline(always)]
        pub const fn set_uh_set_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_clr_port_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[inline(always)]
        pub const fn set_uh_clr_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "bring the PORT into the port state."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_clr_port_connect(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "bring the PORT into the port state."]
        #[inline(always)]
        pub const fn set_uh_clr_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_clr_port_sleep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[inline(always)]
        pub const fn set_uh_clr_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "wake-up time control."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_port_sleep_besl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "wake-up time control."]
        #[inline(always)]
        pub const fn set_uh_port_sleep_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "BUS reset time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_bus_rst_long(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUS reset time selection."]
        #[inline(always)]
        pub const fn set_uh_bus_rst_long(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for UhPortCtrl {
        #[inline(always)]
        fn default() -> UhPortCtrl {
            UhPortCtrl(0)
        }
    }
    impl core::fmt::Debug for UhPortCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortCtrl")
                .field("uh_set_port_reset", &self.uh_set_port_reset())
                .field("uh_set_port_susp", &self.uh_set_port_susp())
                .field("uh_clr_port_susp", &self.uh_clr_port_susp())
                .field("uh_set_port_sleep", &self.uh_set_port_sleep())
                .field("uh_clr_port_en", &self.uh_clr_port_en())
                .field("uh_clr_port_connect", &self.uh_clr_port_connect())
                .field("uh_clr_port_sleep", &self.uh_clr_port_sleep())
                .field("uh_port_sleep_besl", &self.uh_port_sleep_besl())
                .field("uh_bus_rst_long", &self.uh_bus_rst_long())
                .finish()
        }
    }
    #[doc = "USB host port interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortIntEn(pub u8);
    impl UhPortIntEn {
        #[doc = "port connection state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_port_connect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "port enable state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enable state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "port pause state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "port reset state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_port_reset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "port sleep state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhie_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_uhie_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for UhPortIntEn {
        #[inline(always)]
        fn default() -> UhPortIntEn {
            UhPortIntEn(0)
        }
    }
    impl core::fmt::Debug for UhPortIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortIntEn")
                .field("uhie_port_connect", &self.uhie_port_connect())
                .field("uhie_port_en", &self.uhie_port_en())
                .field("uhie_port_susp", &self.uhie_port_susp())
                .field("uhie_port_reset", &self.uhie_port_reset())
                .field("uhie_port_slp", &self.uhie_port_slp())
                .finish()
        }
    }
    #[doc = "USB host port status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortSt(pub u16);
    impl UhPortSt {
        #[doc = "port connection state."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_c0nnect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state."]
        #[inline(always)]
        pub const fn set_uhis_port_c0nnect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "port enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled."]
        #[inline(always)]
        pub const fn set_uhis_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "port pause state."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state."]
        #[inline(always)]
        pub const fn set_uhis_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "port reset status."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset status."]
        #[inline(always)]
        pub const fn set_uhis_port_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "port sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep."]
        #[inline(always)]
        pub const fn set_uhis_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "whether the port connection speed is low."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_ls(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is low."]
        #[inline(always)]
        pub const fn set_uhis_port_ls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "whether the port connection speed is hign."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_hs(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is hign."]
        #[inline(always)]
        pub const fn set_uhis_port_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "whether the port is in test mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uhis_port_test(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port is in test mode."]
        #[inline(always)]
        pub const fn set_uhis_port_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for UhPortSt {
        #[inline(always)]
        fn default() -> UhPortSt {
            UhPortSt(0)
        }
    }
    impl core::fmt::Debug for UhPortSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortSt")
                .field("uhis_port_c0nnect", &self.uhis_port_c0nnect())
                .field("uhis_port_en", &self.uhis_port_en())
                .field("uhis_port_susp", &self.uhis_port_susp())
                .field("uhis_port_rst", &self.uhis_port_rst())
                .field("uhis_port_slp", &self.uhis_port_slp())
                .field("uhis_port_ls", &self.uhis_port_ls())
                .field("uhis_port_hs", &self.uhis_port_hs())
                .field("uhis_port_test", &self.uhis_port_test())
                .finish()
        }
    }
    #[doc = "USB host port test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortTestCt(pub u8);
    impl UhPortTestCt {
        #[doc = "test output J."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test output J."]
        #[inline(always)]
        pub const fn set_uh_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test output K."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test output K."]
        #[inline(always)]
        pub const fn set_uh_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_test_force_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode was enabled."]
        #[inline(always)]
        pub const fn set_uh_test_force_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "test packet."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_test_packet(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "test packet."]
        #[inline(always)]
        pub const fn set_uh_test_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "test SE0 NAK."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_test_se0_nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "test SE0 NAK."]
        #[inline(always)]
        pub const fn set_uh_test_se0_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UhPortTestCt {
        #[inline(always)]
        fn default() -> UhPortTestCt {
            UhPortTestCt(0)
        }
    }
    impl core::fmt::Debug for UhPortTestCt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhPortTestCt")
                .field("uh_test_j", &self.uh_test_j())
                .field("uh_test_k", &self.uh_test_k())
                .field("uh_test_force_en", &self.uh_test_force_en())
                .field("uh_test_packet", &self.uh_test_packet())
                .field("uh_test_se0_nak", &self.uh_test_se0_nak())
                .finish()
        }
    }
    #[doc = "DMA receive address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDma(pub u32);
    impl UhRxDma {
        #[doc = "Received address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Received address."]
        #[inline(always)]
        pub const fn set_uh_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
    #[doc = "USB host receive length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxLen(pub u32);
    impl UhRxLen {
        #[doc = "Received data length."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received data length."]
        #[inline(always)]
        pub const fn set_uh_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxLen {
        #[inline(always)]
        fn default() -> UhRxLen {
            UhRxLen(0)
        }
    }
    impl core::fmt::Debug for UhRxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxLen")
                .field("uh_rx_len", &self.uh_rx_len())
                .finish()
        }
    }
    #[doc = "USB host receives the maximum length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxMaxLen(pub u32);
    impl UhRxMaxLen {
        #[doc = "receives the maximum length."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "receives the maximum length."]
        #[inline(always)]
        pub const fn set_uh_rx_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxMaxLen {
        #[inline(always)]
        fn default() -> UhRxMaxLen {
            UhRxMaxLen(0)
        }
    }
    impl core::fmt::Debug for UhRxMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxMaxLen")
                .field("uh_rx_max_len", &self.uh_rx_max_len())
                .finish()
        }
    }
    #[doc = "USB host SPLIT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSplitData(pub u32);
    impl UhSplitData {
        #[doc = "SPLIT management data."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_split_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "SPLIT management data."]
        #[inline(always)]
        pub const fn set_uh_split_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for UhSplitData {
        #[inline(always)]
        fn default() -> UhSplitData {
            UhSplitData(0)
        }
    }
    impl core::fmt::Debug for UhSplitData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhSplitData")
                .field("uh_split_data", &self.uh_split_data())
                .finish()
        }
    }
    #[doc = "DMA send address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDma(pub u32);
    impl UhTxDma {
        #[doc = "Send address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Send address."]
        #[inline(always)]
        pub const fn set_uh_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
    #[doc = "USB host send length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxLen(pub u32);
    impl UhTxLen {
        #[doc = "send data length."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "send data length."]
        #[inline(always)]
        pub const fn set_uh_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhTxLen {
        #[inline(always)]
        fn default() -> UhTxLen {
            UhTxLen(0)
        }
    }
    impl core::fmt::Debug for UhTxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxLen")
                .field("uh_tx_len", &self.uh_tx_len())
                .finish()
        }
    }
    #[doc = "USB mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbBaseMode(pub u8);
    impl UsbBaseMode {
        #[doc = "The desired speed mode of the device."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_speed_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The desired speed mode of the device."]
        #[inline(always)]
        pub const fn set_ud_speed_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for UsbBaseMode {
        #[inline(always)]
        fn default() -> UsbBaseMode {
            UsbBaseMode(0)
        }
    }
    impl core::fmt::Debug for UsbBaseMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbBaseMode")
                .field("ud_speed_type", &self.ud_speed_type())
                .finish()
        }
    }
    #[doc = "USB bus."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbBus(pub u16);
    impl UsbBus {
        #[doc = "USB wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_wakeup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB wakeup."]
        #[inline(always)]
        pub const fn set_usb_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "UDP status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_dp_st(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UDP status."]
        #[inline(always)]
        pub const fn set_usb_dp_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "UDM status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_dm_st(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UDM status."]
        #[inline(always)]
        pub const fn set_usb_dm_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
    }
    impl Default for UsbBus {
        #[inline(always)]
        fn default() -> UsbBus {
            UsbBus(0)
        }
    }
    impl core::fmt::Debug for UsbBus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbBus")
                .field("usb_wakeup", &self.usb_wakeup())
                .field("usb_dp_st", &self.usb_dp_st())
                .field("usb_dm_st", &self.usb_dm_st())
                .finish()
        }
    }
    #[doc = "USB base control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbCtrl(pub u8);
    impl UsbCtrl {
        #[doc = "LINK layer reset,highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_rst_link(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK layer reset,highly effective."]
        #[inline(always)]
        pub const fn set_ud_rst_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_rst_sie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn set_ud_rst_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_clr_all(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn set_ud_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB PHY suspend."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB PHY suspend."]
        #[inline(always)]
        pub const fn set_ud_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn set_ud_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_dev_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enabled."]
        #[inline(always)]
        pub const fn set_ud_dev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "LPM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn set_ud_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbCtrl {
        #[inline(always)]
        fn default() -> UsbCtrl {
            UsbCtrl(0)
        }
    }
    impl core::fmt::Debug for UsbCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbCtrl")
                .field("ud_rst_link", &self.ud_rst_link())
                .field("ud_rst_sie", &self.ud_rst_sie())
                .field("ud_clr_all", &self.ud_clr_all())
                .field("ud_phy_suspendm", &self.ud_phy_suspendm())
                .field("ud_dma_en", &self.ud_dma_en())
                .field("ud_dev_en", &self.ud_dev_en())
                .field("ud_lpm_en", &self.ud_lpm_en())
                .finish()
        }
    }
    #[doc = "USB device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbDevAd(pub u8);
    impl UsbDevAd {
        #[doc = "bit mask for USB device address."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_usb_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub const fn set_mask_usb_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for UsbDevAd {
        #[inline(always)]
        fn default() -> UsbDevAd {
            UsbDevAd(0)
        }
    }
    impl core::fmt::Debug for UsbDevAd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbDevAd")
                .field("mask_usb_addr", &self.mask_usb_addr())
                .finish()
        }
    }
    #[doc = "USB frame number."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbFramNo(pub u16);
    impl UsbFramNo {
        #[doc = "Received frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received frame number."]
        #[inline(always)]
        pub const fn set_ud_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Received micro frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_mframe_no(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Received micro frame number."]
        #[inline(always)]
        pub const fn set_ud_mframe_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for UsbFramNo {
        #[inline(always)]
        fn default() -> UsbFramNo {
            UsbFramNo(0)
        }
    }
    impl core::fmt::Debug for UsbFramNo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbFramNo")
                .field("ud_frame_no", &self.ud_frame_no())
                .field("ud_mframe_no", &self.ud_mframe_no())
                .finish()
        }
    }
    #[doc = "USB interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntEn(pub u8);
    impl UsbIntEn {
        #[doc = "USB bus reset interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn set_udie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[inline(always)]
        pub const fn set_udie_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt enable."]
        #[inline(always)]
        pub const fn set_udie_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udie_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_udie_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbIntEn {
        #[inline(always)]
        fn default() -> UsbIntEn {
            UsbIntEn(0)
        }
    }
    impl core::fmt::Debug for UsbIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbIntEn")
                .field("udie_bus_rst", &self.udie_bus_rst())
                .field("udie_suspend", &self.udie_suspend())
                .field("udie_bus_sleep", &self.udie_bus_sleep())
                .field("udie_lpm_act", &self.udie_lpm_act())
                .field("udie_transfer", &self.udie_transfer())
                .field("udie_sof_act", &self.udie_sof_act())
                .field("udie_link_rdy", &self.udie_link_rdy())
                .field("udie_fifo_over", &self.udie_fifo_over())
                .finish()
        }
    }
    #[doc = "USB interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntFg(pub u8);
    impl UsbIntFg {
        #[doc = "USB bus reset interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transmission end interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_rtx_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transmission end interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_rtx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_rx_sof(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_rx_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn udif_fifo_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_udif_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbIntFg {
        #[inline(always)]
        fn default() -> UsbIntFg {
            UsbIntFg(0)
        }
    }
    impl core::fmt::Debug for UsbIntFg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbIntFg")
                .field("udif_bus_rst", &self.udif_bus_rst())
                .field("udif_suspend", &self.udif_suspend())
                .field("udif_bus_sleep", &self.udif_bus_sleep())
                .field("udif_lpm_act", &self.udif_lpm_act())
                .field("udif_rtx_act", &self.udif_rtx_act())
                .field("udif_rx_sof", &self.udif_rx_sof())
                .field("udif_link_rdy", &self.udif_link_rdy())
                .field("udif_fifo_ov", &self.udif_fifo_ov())
                .finish()
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntSt(pub u8);
    impl UsbIntSt {
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[must_use]
        #[inline(always)]
        pub const fn udis_ep_id_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[inline(always)]
        pub const fn set_udis_ep_id_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "Endpoint data transmission direction."]
        #[must_use]
        #[inline(always)]
        pub const fn udis_ep_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint data transmission direction."]
        #[inline(always)]
        pub const fn set_udis_ep_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UsbIntSt {
        #[inline(always)]
        fn default() -> UsbIntSt {
            UsbIntSt(0)
        }
    }
    impl core::fmt::Debug for UsbIntSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbIntSt")
                .field("udis_ep_id_mask", &self.udis_ep_id_mask())
                .field("udis_ep_dir", &self.udis_ep_dir())
                .finish()
        }
    }
    #[doc = "USB power management register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbLpmData(pub u16);
    impl UsbLpmData {
        #[doc = "power management data."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "power management data."]
        #[inline(always)]
        pub const fn set_ud_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "power management busy."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_lpm_busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "power management busy."]
        #[inline(always)]
        pub const fn set_ud_lpm_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for UsbLpmData {
        #[inline(always)]
        fn default() -> UsbLpmData {
            UsbLpmData(0)
        }
    }
    impl core::fmt::Debug for UsbLpmData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbLpmData")
                .field("ud_lpm_data", &self.ud_lpm_data())
                .field("ud_lpm_busy", &self.ud_lpm_busy())
                .finish()
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbMisSt(pub u8);
    impl UsbMisSt {
        #[doc = "USB connection status."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection status."]
        #[inline(always)]
        pub const fn set_udms_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB suspend status."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend status."]
        #[inline(always)]
        pub const fn set_udms_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB sleep status."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB sleep status."]
        #[inline(always)]
        pub const fn set_udms_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB free status."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_sie_free(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB free status."]
        #[inline(always)]
        pub const fn set_udms_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB suspends the request."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_susp_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspends the request."]
        #[inline(always)]
        pub const fn set_udms_susp_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "whether the host is high-speed."]
        #[must_use]
        #[inline(always)]
        pub const fn udms_hs_mod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "whether the host is high-speed."]
        #[inline(always)]
        pub const fn set_udms_hs_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbMisSt {
        #[inline(always)]
        fn default() -> UsbMisSt {
            UsbMisSt(0)
        }
    }
    impl core::fmt::Debug for UsbMisSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbMisSt")
                .field("udms_ready", &self.udms_ready())
                .field("udms_suspend", &self.udms_suspend())
                .field("udms_sleep", &self.udms_sleep())
                .field("udms_sie_free", &self.udms_sie_free())
                .field("udms_susp_req", &self.udms_susp_req())
                .field("udms_hs_mod", &self.udms_hs_mod())
                .finish()
        }
    }
    #[doc = "USB test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbTestMode(pub u8);
    impl UsbTestMode {
        #[doc = "test mode,output J."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output J."]
        #[inline(always)]
        pub const fn set_ud_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test mode,output K."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output K."]
        #[inline(always)]
        pub const fn set_ud_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode,output a packet."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_test_pkt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output a packet."]
        #[inline(always)]
        pub const fn set_ud_test_pkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "test mode,output SEO."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_test_se0nak(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output SEO."]
        #[inline(always)]
        pub const fn set_ud_test_se0nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "test mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_test_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "test mode enable."]
        #[inline(always)]
        pub const fn set_ud_test_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbTestMode {
        #[inline(always)]
        fn default() -> UsbTestMode {
            UsbTestMode(0)
        }
    }
    impl core::fmt::Debug for UsbTestMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbTestMode")
                .field("ud_test_j", &self.ud_test_j())
                .field("ud_test_k", &self.ud_test_k())
                .field("ud_test_pkt", &self.ud_test_pkt())
                .field("ud_test_se0nak", &self.ud_test_se0nak())
                .field("ud_test_en", &self.ud_test_en())
                .finish()
        }
    }
    #[doc = "USB remote wake up register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbWakeCtrl(pub u8);
    impl UsbWakeCtrl {
        #[doc = "remote wake up."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_remote_wkup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "remote wake up."]
        #[inline(always)]
        pub const fn set_ud_remote_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for UsbWakeCtrl {
        #[inline(always)]
        fn default() -> UsbWakeCtrl {
            UsbWakeCtrl(0)
        }
    }
    impl core::fmt::Debug for UsbWakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbWakeCtrl")
                .field("ud_remote_wkup", &self.ud_remote_wkup())
                .finish()
        }
    }
}
