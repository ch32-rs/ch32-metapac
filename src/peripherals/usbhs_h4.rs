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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_t_tog_auto(self) -> crate::common::Reg<regs::UepTTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_r_tog_auto(self) -> crate::common::Reg<regs::UepRTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[inline(always)]
    pub const fn uep_t_burst(self) -> crate::common::Reg<regs::UepTBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "USB endpoint send the mode register."]
    #[inline(always)]
    pub const fn uep_t_burst_mode(
        self,
    ) -> crate::common::Reg<regs::UepTBurstMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x19usize) as _) }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[inline(always)]
    pub const fn uep_r_burst(self) -> crate::common::Reg<regs::UepRBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "USB endpoint reply mode register."]
    #[inline(always)]
    pub const fn uep_r_res_mode(self) -> crate::common::Reg<regs::UepRResMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1busize) as _) }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[inline(always)]
    pub const fn uep_af_mode(self) -> crate::common::Reg<regs::UepAfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[inline(always)]
    pub const fn uep0_dma(self) -> crate::common::Reg<regs::Uep0Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "endpoint 1 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep1_rx_dma(self) -> crate::common::Reg<regs::Uep1RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "endpoint 2 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep2_rx_dma(self) -> crate::common::Reg<regs::Uep2RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "endpoint 3 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep3_rx_dma(self) -> crate::common::Reg<regs::Uep3RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "endpoint 4 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep4_rx_dma(self) -> crate::common::Reg<regs::Uep4RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "endpoint 5 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep5_rx_dma(self) -> crate::common::Reg<regs::Uep5RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "endpoint 6 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep6_rx_dma(self) -> crate::common::Reg<regs::Uep6RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "endpoint 7 receives the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep7_rx_dma(self) -> crate::common::Reg<regs::Uep7RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "endpoint 1 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep1_tx_dma(self) -> crate::common::Reg<regs::Uep1TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "endpoint 2 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep2_tx_dma(self) -> crate::common::Reg<regs::Uep2TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "endpoint 3 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep3_tx_dma(self) -> crate::common::Reg<regs::Uep3TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "endpoint 4 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep4_tx_dma(self) -> crate::common::Reg<regs::Uep4TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "endpoint 5 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep5_tx_dma(self) -> crate::common::Reg<regs::Uep5TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "endpoint 6 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep6_tx_dma(self) -> crate::common::Reg<regs::Uep6TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "endpoint 7 sends the start address register of the buffer."]
    #[inline(always)]
    pub const fn uep7_tx_dma(self) -> crate::common::Reg<regs::Uep7TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[inline(always)]
    pub const fn uep0_max_len(self) -> crate::common::Reg<regs::Uep0MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "endpoint 1 max length packet register."]
    #[inline(always)]
    pub const fn uep1_max_len(self) -> crate::common::Reg<regs::Uep1MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "endpoint 2 max length packet register."]
    #[inline(always)]
    pub const fn uep2_max_len(self) -> crate::common::Reg<regs::Uep2MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "endpoint 3 max length packet register."]
    #[inline(always)]
    pub const fn uep3_max_len(self) -> crate::common::Reg<regs::Uep3MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "endpoint 4 max length packet register."]
    #[inline(always)]
    pub const fn uep4_max_len(self) -> crate::common::Reg<regs::Uep4MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "endpoint 5 max length packet register."]
    #[inline(always)]
    pub const fn uep5_max_len(self) -> crate::common::Reg<regs::Uep5MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "endpoint 6 max length packet register."]
    #[inline(always)]
    pub const fn uep6_max_len(self) -> crate::common::Reg<regs::Uep6MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "endpoint 7 max length packet register."]
    #[inline(always)]
    pub const fn uep7_max_len(self) -> crate::common::Reg<regs::Uep7MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[inline(always)]
    pub const fn uep0_rx_len(self) -> crate::common::Reg<regs::Uep0RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "endpoint 1 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep1_rx_len(self) -> crate::common::Reg<regs::Uep1RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 1."]
    #[inline(always)]
    pub const fn uep1_r_size(self) -> crate::common::Reg<regs::Uep1RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x82usize) as _) }
    }
    #[doc = "endpoint 2 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep2_rx_len(self) -> crate::common::Reg<regs::Uep2RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 2."]
    #[inline(always)]
    pub const fn uep2_r_size(self) -> crate::common::Reg<regs::Uep2RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x86usize) as _) }
    }
    #[doc = "endpoint 3 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep3_rx_len(self) -> crate::common::Reg<regs::Uep3RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 3."]
    #[inline(always)]
    pub const fn uep3_r_size(self) -> crate::common::Reg<regs::Uep3RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8ausize) as _) }
    }
    #[doc = "endpoint 4 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep4_rx_len(self) -> crate::common::Reg<regs::Uep4RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 4."]
    #[inline(always)]
    pub const fn uep4_r_size(self) -> crate::common::Reg<regs::Uep4RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8eusize) as _) }
    }
    #[doc = "endpoint 5 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep5_rx_len(self) -> crate::common::Reg<regs::Uep5RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 5."]
    #[inline(always)]
    pub const fn uep5_r_size(self) -> crate::common::Reg<regs::Uep5RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x92usize) as _) }
    }
    #[doc = "endpoint 6 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep6_rx_len(self) -> crate::common::Reg<regs::Uep6RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 6."]
    #[inline(always)]
    pub const fn uep6_r_size(self) -> crate::common::Reg<regs::Uep6RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x96usize) as _) }
    }
    #[doc = "endpoint 7 receives the lenth register in a single pass."]
    #[inline(always)]
    pub const fn uep7_rx_len(self) -> crate::common::Reg<regs::Uep7RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint 7."]
    #[inline(always)]
    pub const fn uep7_r_size(self) -> crate::common::Reg<regs::Uep7RSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9ausize) as _) }
    }
    #[doc = "endpoint 0 send the length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9eusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9fusize) as _) }
    }
    #[doc = "endpoint 1 send the length register."]
    #[inline(always)]
    pub const fn uep1_t_len(self) -> crate::common::Reg<regs::Uep1TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "endpoint 1 send control register."]
    #[inline(always)]
    pub const fn uep1_tx_ctrl(self) -> crate::common::Reg<regs::Uep1TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa2usize) as _) }
    }
    #[doc = "endpoint 1 receive control."]
    #[inline(always)]
    pub const fn uep1_rx_ctrl(self) -> crate::common::Reg<regs::Uep1RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa3usize) as _) }
    }
    #[doc = "endpoint 2 send the length register."]
    #[inline(always)]
    pub const fn uep2_t_len(self) -> crate::common::Reg<regs::Uep2TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "endpoint 2 send control register."]
    #[inline(always)]
    pub const fn uep2_tx_ctrl(self) -> crate::common::Reg<regs::Uep2TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa6usize) as _) }
    }
    #[doc = "endpoint 2 receive control."]
    #[inline(always)]
    pub const fn uep2_rx_ctrl(self) -> crate::common::Reg<regs::Uep2RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa7usize) as _) }
    }
    #[doc = "endpoint 3 send the length register."]
    #[inline(always)]
    pub const fn uep3_t_len(self) -> crate::common::Reg<regs::Uep3TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "endpoint 3 send control register."]
    #[inline(always)]
    pub const fn uep3_tx_ctrl(self) -> crate::common::Reg<regs::Uep3TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xaausize) as _) }
    }
    #[doc = "endpoint 3 receive control."]
    #[inline(always)]
    pub const fn uep3_rx_ctrl(self) -> crate::common::Reg<regs::Uep3RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xabusize) as _) }
    }
    #[doc = "endpoint 4 send the length register."]
    #[inline(always)]
    pub const fn uep4_t_len(self) -> crate::common::Reg<regs::Uep4TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "endpoint 4 send control register."]
    #[inline(always)]
    pub const fn uep4_tx_ctrl(self) -> crate::common::Reg<regs::Uep4TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xaeusize) as _) }
    }
    #[doc = "endpoint 4 receive control."]
    #[inline(always)]
    pub const fn uep4_rx_ctrl(self) -> crate::common::Reg<regs::Uep4RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xafusize) as _) }
    }
    #[doc = "endpoint 5 send the length register."]
    #[inline(always)]
    pub const fn uep5_t_len(self) -> crate::common::Reg<regs::Uep5TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "endpoint 5 send control register."]
    #[inline(always)]
    pub const fn uep5_tx_ctrl(self) -> crate::common::Reg<regs::Uep5TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb2usize) as _) }
    }
    #[doc = "endpoint 5 receive control."]
    #[inline(always)]
    pub const fn uep5_rx_ctrl(self) -> crate::common::Reg<regs::Uep5RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb3usize) as _) }
    }
    #[doc = "endpoint 6 send the length register."]
    #[inline(always)]
    pub const fn uep6_t_len(self) -> crate::common::Reg<regs::Uep6TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "endpoint 6 send control register."]
    #[inline(always)]
    pub const fn uep6_tx_ctrl(self) -> crate::common::Reg<regs::Uep6TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb6usize) as _) }
    }
    #[doc = "endpoint 6 receive control."]
    #[inline(always)]
    pub const fn uep6_rx_ctrl(self) -> crate::common::Reg<regs::Uep6RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb7usize) as _) }
    }
    #[doc = "endpoint 7 send the length register."]
    #[inline(always)]
    pub const fn uep7_t_len(self) -> crate::common::Reg<regs::Uep7TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "endpoint 7 send control register."]
    #[inline(always)]
    pub const fn uep7_tx_ctrl(self) -> crate::common::Reg<regs::Uep7TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbausize) as _) }
    }
    #[doc = "endpoint 7 receive control."]
    #[inline(always)]
    pub const fn uep7_rx_ctrl(self) -> crate::common::Reg<regs::Uep7RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbbusize) as _) }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_t_iso(self) -> crate::common::Reg<regs::UepTIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_r_iso(self) -> crate::common::Reg<regs::UepRIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbeusize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 1."]
    #[inline(always)]
    pub const fn uep1_rx_fifo(self) -> crate::common::Reg<regs::Uep1RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 2."]
    #[inline(always)]
    pub const fn uep2_rx_fifo(self) -> crate::common::Reg<regs::Uep2RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 3."]
    #[inline(always)]
    pub const fn uep3_rx_fifo(self) -> crate::common::Reg<regs::Uep3RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 4."]
    #[inline(always)]
    pub const fn uep4_rx_fifo(self) -> crate::common::Reg<regs::Uep4RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 5."]
    #[inline(always)]
    pub const fn uep5_rx_fifo(self) -> crate::common::Reg<regs::Uep5RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 6."]
    #[inline(always)]
    pub const fn uep6_rx_fifo(self) -> crate::common::Reg<regs::Uep6RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Receive FIFO address of usb endpoint 7."]
    #[inline(always)]
    pub const fn uep7_rx_fifo(self) -> crate::common::Reg<regs::Uep7RxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 1."]
    #[inline(always)]
    pub const fn uep1_tx_fifo(self) -> crate::common::Reg<regs::Uep1TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 2."]
    #[inline(always)]
    pub const fn uep2_tx_fifo(self) -> crate::common::Reg<regs::Uep2TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 3."]
    #[inline(always)]
    pub const fn uep3_tx_fifo(self) -> crate::common::Reg<regs::Uep3TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 4."]
    #[inline(always)]
    pub const fn uep4_tx_fifo(self) -> crate::common::Reg<regs::Uep4TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 5."]
    #[inline(always)]
    pub const fn uep5_tx_fifo(self) -> crate::common::Reg<regs::Uep5TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 6."]
    #[inline(always)]
    pub const fn uep6_tx_fifo(self) -> crate::common::Reg<regs::Uep6TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "The sending FIFO address of usb endpoint 7."]
    #[inline(always)]
    pub const fn uep7_tx_fifo(self) -> crate::common::Reg<regs::Uep7TxFifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn usb_base_mode(self) -> crate::common::Reg<regs::UsbBaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn usb_int_en(self) -> crate::common::Reg<regs::UsbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn usb_dev_ad(self) -> crate::common::Reg<regs::UsbDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn usb_wake_ctrl(self) -> crate::common::Reg<regs::UsbWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn usb_test_mode(self) -> crate::common::Reg<regs::UsbTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn usb_lpm_data(self) -> crate::common::Reg<regs::UsbLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn usb_int_fg(self) -> crate::common::Reg<regs::UsbIntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn usb_int_st(self) -> crate::common::Reg<regs::UsbIntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn usb_mis_st(self) -> crate::common::Reg<regs::UsbMisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn usb_fram_no(self) -> crate::common::Reg<regs::UsbFramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn usb_bus(self) -> crate::common::Reg<regs::UsbBus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "USB host Configuration register."]
    #[inline(always)]
    pub const fn uh_cfg(self) -> crate::common::Reg<regs::UhCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "USB host interrupt enable register."]
    #[inline(always)]
    pub const fn uh_int_en(self) -> crate::common::Reg<regs::UhIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0102usize) as _) }
    }
    #[doc = "USB host device address register."]
    #[inline(always)]
    pub const fn uh_dev_ad(self) -> crate::common::Reg<regs::UhDevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0103usize) as _) }
    }
    #[doc = "USB host control register."]
    #[inline(always)]
    pub const fn uh_control(self) -> crate::common::Reg<regs::UhControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[inline(always)]
    pub const fn uh_int_flag(self) -> crate::common::Reg<regs::UhIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "USB host interrupt status."]
    #[inline(always)]
    pub const fn uh_int_st(self) -> crate::common::Reg<regs::UhIntSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0109usize) as _) }
    }
    #[doc = "USB host miscellaneous status."]
    #[inline(always)]
    pub const fn uh_mis_st(self) -> crate::common::Reg<regs::UhMisSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010ausize) as _) }
    }
    #[doc = "USB host power management data register."]
    #[inline(always)]
    pub const fn uh_lpm_data(self) -> crate::common::Reg<regs::UhLpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "USB host SPLIT register."]
    #[inline(always)]
    pub const fn uh_split_data(self) -> crate::common::Reg<regs::UhSplitData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "USB host frame register."]
    #[inline(always)]
    pub const fn uh_frame(self) -> crate::common::Reg<regs::UhFrame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "USB host send length register."]
    #[inline(always)]
    pub const fn uh_tx_len(self) -> crate::common::Reg<regs::UhTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "USB host receive length register."]
    #[inline(always)]
    pub const fn uh_rx_len(self) -> crate::common::Reg<regs::UhRxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "USB host receives the maximum length register."]
    #[inline(always)]
    pub const fn uh_rx_max_len(self) -> crate::common::Reg<regs::UhRxMaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "DMA receive address register."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<regs::UhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "DMA send address register."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<regs::UhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "USB host port control register."]
    #[inline(always)]
    pub const fn uh_port_ctrl(self) -> crate::common::Reg<regs::UhPortCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "USB host port Configuration register."]
    #[inline(always)]
    pub const fn uh_port_cfg(self) -> crate::common::Reg<regs::UhPortCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "USB host port interrupt enable register."]
    #[inline(always)]
    pub const fn uh_port_int_en(self) -> crate::common::Reg<regs::UhPortIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0132usize) as _) }
    }
    #[doc = "USB host port test mode register."]
    #[inline(always)]
    pub const fn uh_port_test_ct(
        self,
    ) -> crate::common::Reg<regs::UhPortTestCt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0133usize) as _) }
    }
    #[doc = "USB host port status register."]
    #[inline(always)]
    pub const fn uh_port_st(self) -> crate::common::Reg<regs::UhPortSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "USB host port state charge register."]
    #[inline(always)]
    pub const fn uh_port_chg(self) -> crate::common::Reg<regs::UhPortChg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0136usize) as _) }
    }
    #[doc = "USB host BC charging control register."]
    #[inline(always)]
    pub const fn uh_bc_ctrl(self) -> crate::common::Reg<regs::UhBcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
}
pub mod regs {
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Dma(pub u32);
    impl Uep0Dma {
        #[doc = "The start address of the endpoint 0 buffer."]
        #[inline(always)]
        pub const fn uep0_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The start address of the endpoint 0 buffer."]
        #[inline(always)]
        pub fn set_uep0_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep0Dma {
        #[inline(always)]
        fn default() -> Uep0Dma {
            Uep0Dma(0)
        }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0MaxLen(pub u32);
    impl Uep0MaxLen {
        #[doc = "endpoint 0 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep0_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "endpoint 0 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep0_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Uep0MaxLen {
        #[inline(always)]
        fn default() -> Uep0MaxLen {
            Uep0MaxLen(0)
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u8);
    impl Uep0RxCtrl {
        #[doc = "endpoint 0 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[inline(always)]
        pub const fn uep_r_setup_is(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[inline(always)]
        pub fn set_uep_r_setup_is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep0RxCtrl {
        #[inline(always)]
        fn default() -> Uep0RxCtrl {
            Uep0RxCtrl(0)
        }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxLen(pub u16);
    impl Uep0RxLen {
        #[doc = "endpoint 0 acceptable data length."]
        #[inline(always)]
        pub const fn uep0_rx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 acceptable data length."]
        #[inline(always)]
        pub fn set_uep0_rx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0RxLen {
        #[inline(always)]
        fn default() -> Uep0RxLen {
            Uep0RxLen(0)
        }
    }
    #[doc = "endpoint 0 send the length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u16);
    impl Uep0TLen {
        #[doc = "endpoint 0 send the length."]
        #[inline(always)]
        pub const fn uep0_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 send the length."]
        #[inline(always)]
        pub fn set_uep0_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0TLen {
        #[inline(always)]
        fn default() -> Uep0TLen {
            Uep0TLen(0)
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u8);
    impl Uep0TxCtrl {
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep0TxCtrl {
        #[inline(always)]
        fn default() -> Uep0TxCtrl {
            Uep0TxCtrl(0)
        }
    }
    #[doc = "endpoint 1 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1MaxLen(pub u32);
    impl Uep1MaxLen {
        #[doc = "endpoint 1 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep1_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 1 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep1_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep1MaxLen {
        #[inline(always)]
        fn default() -> Uep1MaxLen {
            Uep1MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RSize(pub u16);
    impl Uep1RSize {
        #[doc = "the length of the total received data at endpoint 1."]
        #[inline(always)]
        pub const fn uep1_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 1."]
        #[inline(always)]
        pub fn set_uep1_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1RSize {
        #[inline(always)]
        fn default() -> Uep1RSize {
            Uep1RSize(0)
        }
    }
    #[doc = "endpoint 1 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxCtrl(pub u8);
    impl Uep1RxCtrl {
        #[doc = "endpoint 1 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 1 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 1 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 1 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 1 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 1 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1RxCtrl {
        #[inline(always)]
        fn default() -> Uep1RxCtrl {
            Uep1RxCtrl(0)
        }
    }
    #[doc = "endpoint 1 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxDma(pub u32);
    impl Uep1RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep1_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep1_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep1RxDma {
        #[inline(always)]
        fn default() -> Uep1RxDma {
            Uep1RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxFifo(pub u32);
    impl Uep1RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep1RxFifo {
        #[inline(always)]
        fn default() -> Uep1RxFifo {
            Uep1RxFifo(0)
        }
    }
    #[doc = "endpoint 1 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxLen(pub u16);
    impl Uep1RxLen {
        #[doc = "the length of data received by endpoint 1 in a single pass."]
        #[inline(always)]
        pub const fn uep1_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 1 in a single pass."]
        #[inline(always)]
        pub fn set_uep1_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1RxLen {
        #[inline(always)]
        fn default() -> Uep1RxLen {
            Uep1RxLen(0)
        }
    }
    #[doc = "endpoint 1 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TLen(pub u16);
    impl Uep1TLen {
        #[doc = "endpoint 1 send the length."]
        #[inline(always)]
        pub const fn uep1_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 1 send the length."]
        #[inline(always)]
        pub fn set_uep1_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1TLen {
        #[inline(always)]
        fn default() -> Uep1TLen {
            Uep1TLen(0)
        }
    }
    #[doc = "endpoint 1 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxCtrl(pub u8);
    impl Uep1TxCtrl {
        #[doc = "endpoint 1 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 1 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 1 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 1 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 1 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 1 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1TxCtrl {
        #[inline(always)]
        fn default() -> Uep1TxCtrl {
            Uep1TxCtrl(0)
        }
    }
    #[doc = "endpoint 1 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxDma(pub u32);
    impl Uep1TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep1_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep1_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep1TxDma {
        #[inline(always)]
        fn default() -> Uep1TxDma {
            Uep1TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxFifo(pub u32);
    impl Uep1TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep1TxFifo {
        #[inline(always)]
        fn default() -> Uep1TxFifo {
            Uep1TxFifo(0)
        }
    }
    #[doc = "endpoint 2 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2MaxLen(pub u32);
    impl Uep2MaxLen {
        #[doc = "endpoint 2 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep2_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 2 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep2_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep2MaxLen {
        #[inline(always)]
        fn default() -> Uep2MaxLen {
            Uep2MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RSize(pub u16);
    impl Uep2RSize {
        #[doc = "the length of the total received data at endpoint 2."]
        #[inline(always)]
        pub const fn uep2_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 2."]
        #[inline(always)]
        pub fn set_uep2_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2RSize {
        #[inline(always)]
        fn default() -> Uep2RSize {
            Uep2RSize(0)
        }
    }
    #[doc = "endpoint 2 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxCtrl(pub u8);
    impl Uep2RxCtrl {
        #[doc = "endpoint 2 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 2 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 2 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 2 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 2 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 2 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 2 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 2 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 2 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 2 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2RxCtrl {
        #[inline(always)]
        fn default() -> Uep2RxCtrl {
            Uep2RxCtrl(0)
        }
    }
    #[doc = "endpoint 2 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxDma(pub u32);
    impl Uep2RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep2_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep2_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep2RxDma {
        #[inline(always)]
        fn default() -> Uep2RxDma {
            Uep2RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxFifo(pub u32);
    impl Uep2RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep2RxFifo {
        #[inline(always)]
        fn default() -> Uep2RxFifo {
            Uep2RxFifo(0)
        }
    }
    #[doc = "endpoint 2 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxLen(pub u16);
    impl Uep2RxLen {
        #[doc = "the length of data received by endpoint 2 in a single pass."]
        #[inline(always)]
        pub const fn uep2_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 2 in a single pass."]
        #[inline(always)]
        pub fn set_uep2_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2RxLen {
        #[inline(always)]
        fn default() -> Uep2RxLen {
            Uep2RxLen(0)
        }
    }
    #[doc = "endpoint 2 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TLen(pub u16);
    impl Uep2TLen {
        #[doc = "endpoint 2 send the length."]
        #[inline(always)]
        pub const fn uep2_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 2 send the length."]
        #[inline(always)]
        pub fn set_uep2_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2TLen {
        #[inline(always)]
        fn default() -> Uep2TLen {
            Uep2TLen(0)
        }
    }
    #[doc = "endpoint 2 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxCtrl(pub u8);
    impl Uep2TxCtrl {
        #[doc = "endpoint 2 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 2 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 2 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mas(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 2 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 2 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 2 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 2 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 2 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2TxCtrl {
        #[inline(always)]
        fn default() -> Uep2TxCtrl {
            Uep2TxCtrl(0)
        }
    }
    #[doc = "endpoint 2 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxDma(pub u32);
    impl Uep2TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep2_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep2_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep2TxDma {
        #[inline(always)]
        fn default() -> Uep2TxDma {
            Uep2TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxFifo(pub u32);
    impl Uep2TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep2TxFifo {
        #[inline(always)]
        fn default() -> Uep2TxFifo {
            Uep2TxFifo(0)
        }
    }
    #[doc = "endpoint 3 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3MaxLen(pub u32);
    impl Uep3MaxLen {
        #[doc = "endpoint 3 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep3_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 3 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep3_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep3MaxLen {
        #[inline(always)]
        fn default() -> Uep3MaxLen {
            Uep3MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RSize(pub u16);
    impl Uep3RSize {
        #[doc = "the length of the total received data at endpoint 3."]
        #[inline(always)]
        pub const fn uep3_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 3."]
        #[inline(always)]
        pub fn set_uep3_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3RSize {
        #[inline(always)]
        fn default() -> Uep3RSize {
            Uep3RSize(0)
        }
    }
    #[doc = "endpoint 3 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxCtrl(pub u8);
    impl Uep3RxCtrl {
        #[doc = "endpoint 3 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 3 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 3 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 3 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 3 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 3 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 3 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 3 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 3 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 3 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3RxCtrl {
        #[inline(always)]
        fn default() -> Uep3RxCtrl {
            Uep3RxCtrl(0)
        }
    }
    #[doc = "endpoint 3 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxDma(pub u32);
    impl Uep3RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep3_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep3_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep3RxDma {
        #[inline(always)]
        fn default() -> Uep3RxDma {
            Uep3RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxFifo(pub u32);
    impl Uep3RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep3RxFifo {
        #[inline(always)]
        fn default() -> Uep3RxFifo {
            Uep3RxFifo(0)
        }
    }
    #[doc = "endpoint 3 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxLen(pub u16);
    impl Uep3RxLen {
        #[doc = "the length of data received by endpoint 3 in a single pass."]
        #[inline(always)]
        pub const fn uep3_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 3 in a single pass."]
        #[inline(always)]
        pub fn set_uep3_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3RxLen {
        #[inline(always)]
        fn default() -> Uep3RxLen {
            Uep3RxLen(0)
        }
    }
    #[doc = "endpoint 3 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TLen(pub u16);
    impl Uep3TLen {
        #[doc = "endpoint 3 send the length."]
        #[inline(always)]
        pub const fn uep3_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 3 send the length."]
        #[inline(always)]
        pub fn set_uep3_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3TLen {
        #[inline(always)]
        fn default() -> Uep3TLen {
            Uep3TLen(0)
        }
    }
    #[doc = "endpoint 3 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxCtrl(pub u8);
    impl Uep3TxCtrl {
        #[doc = "endpoint 3 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 3 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 3 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 3 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 3 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 3 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 3 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 3 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3TxCtrl {
        #[inline(always)]
        fn default() -> Uep3TxCtrl {
            Uep3TxCtrl(0)
        }
    }
    #[doc = "endpoint 3 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxDma(pub u32);
    impl Uep3TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep3_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep3_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep3TxDma {
        #[inline(always)]
        fn default() -> Uep3TxDma {
            Uep3TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxFifo(pub u32);
    impl Uep3TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep3TxFifo {
        #[inline(always)]
        fn default() -> Uep3TxFifo {
            Uep3TxFifo(0)
        }
    }
    #[doc = "endpoint 4 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4MaxLen(pub u32);
    impl Uep4MaxLen {
        #[doc = "endpoint 4 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep4_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 4 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep4_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep4MaxLen {
        #[inline(always)]
        fn default() -> Uep4MaxLen {
            Uep4MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RSize(pub u16);
    impl Uep4RSize {
        #[doc = "the length of the total received data at endpoint 4."]
        #[inline(always)]
        pub const fn uep4_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 4."]
        #[inline(always)]
        pub fn set_uep4_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4RSize {
        #[inline(always)]
        fn default() -> Uep4RSize {
            Uep4RSize(0)
        }
    }
    #[doc = "endpoint 4 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxCtrl(pub u8);
    impl Uep4RxCtrl {
        #[doc = "endpoint 4 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 4 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 4 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 4 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 4 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 4 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 4 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 4 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 4 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 4 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4RxCtrl {
        #[inline(always)]
        fn default() -> Uep4RxCtrl {
            Uep4RxCtrl(0)
        }
    }
    #[doc = "endpoint 4 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxDma(pub u32);
    impl Uep4RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep4_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep4_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep4RxDma {
        #[inline(always)]
        fn default() -> Uep4RxDma {
            Uep4RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxFifo(pub u32);
    impl Uep4RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep4RxFifo {
        #[inline(always)]
        fn default() -> Uep4RxFifo {
            Uep4RxFifo(0)
        }
    }
    #[doc = "endpoint 4 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxLen(pub u16);
    impl Uep4RxLen {
        #[doc = "the length of data received by endpoint 4 in a single pass."]
        #[inline(always)]
        pub const fn uep4_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 4 in a single pass."]
        #[inline(always)]
        pub fn set_uep4_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4RxLen {
        #[inline(always)]
        fn default() -> Uep4RxLen {
            Uep4RxLen(0)
        }
    }
    #[doc = "endpoint 4 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TLen(pub u16);
    impl Uep4TLen {
        #[doc = "endpoint 4 send the length."]
        #[inline(always)]
        pub const fn uep4_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 4 send the length."]
        #[inline(always)]
        pub fn set_uep4_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4TLen {
        #[inline(always)]
        fn default() -> Uep4TLen {
            Uep4TLen(0)
        }
    }
    #[doc = "endpoint 4 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxCtrl(pub u8);
    impl Uep4TxCtrl {
        #[doc = "endpoint 4 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 4 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 4 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 4 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 4 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 4 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 4 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 4 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4TxCtrl {
        #[inline(always)]
        fn default() -> Uep4TxCtrl {
            Uep4TxCtrl(0)
        }
    }
    #[doc = "endpoint 4 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxDma(pub u32);
    impl Uep4TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep4_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep4_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep4TxDma {
        #[inline(always)]
        fn default() -> Uep4TxDma {
            Uep4TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxFifo(pub u32);
    impl Uep4TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep4TxFifo {
        #[inline(always)]
        fn default() -> Uep4TxFifo {
            Uep4TxFifo(0)
        }
    }
    #[doc = "endpoint 5 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5MaxLen(pub u32);
    impl Uep5MaxLen {
        #[doc = "endpoint 5 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep5_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 5 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep5_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep5MaxLen {
        #[inline(always)]
        fn default() -> Uep5MaxLen {
            Uep5MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RSize(pub u16);
    impl Uep5RSize {
        #[doc = "the length of the total received data at endpoint 5."]
        #[inline(always)]
        pub const fn uep5_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 5."]
        #[inline(always)]
        pub fn set_uep5_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5RSize {
        #[inline(always)]
        fn default() -> Uep5RSize {
            Uep5RSize(0)
        }
    }
    #[doc = "endpoint 5 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxCtrl(pub u8);
    impl Uep5RxCtrl {
        #[doc = "endpoint 5 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 5 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 5 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 5 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 5 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 5 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 5 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 5 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 5 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 5 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5RxCtrl {
        #[inline(always)]
        fn default() -> Uep5RxCtrl {
            Uep5RxCtrl(0)
        }
    }
    #[doc = "endpoint 5 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxDma(pub u32);
    impl Uep5RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep5_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep5_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep5RxDma {
        #[inline(always)]
        fn default() -> Uep5RxDma {
            Uep5RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxFifo(pub u32);
    impl Uep5RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep5RxFifo {
        #[inline(always)]
        fn default() -> Uep5RxFifo {
            Uep5RxFifo(0)
        }
    }
    #[doc = "endpoint 5 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxLen(pub u16);
    impl Uep5RxLen {
        #[doc = "the length of data received by endpoint 5 in a single pass."]
        #[inline(always)]
        pub const fn uep5_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 5 in a single pass."]
        #[inline(always)]
        pub fn set_uep5_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5RxLen {
        #[inline(always)]
        fn default() -> Uep5RxLen {
            Uep5RxLen(0)
        }
    }
    #[doc = "endpoint 5 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TLen(pub u16);
    impl Uep5TLen {
        #[doc = "endpoint 5 send the length."]
        #[inline(always)]
        pub const fn uep5_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 5 send the length."]
        #[inline(always)]
        pub fn set_uep5_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5TLen {
        #[inline(always)]
        fn default() -> Uep5TLen {
            Uep5TLen(0)
        }
    }
    #[doc = "endpoint 5 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxCtrl(pub u8);
    impl Uep5TxCtrl {
        #[doc = "endpoint 5 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 5 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 5 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 5 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 5 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 5 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 5 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 5 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5TxCtrl {
        #[inline(always)]
        fn default() -> Uep5TxCtrl {
            Uep5TxCtrl(0)
        }
    }
    #[doc = "endpoint 5 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxDma(pub u32);
    impl Uep5TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep5_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep5_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep5TxDma {
        #[inline(always)]
        fn default() -> Uep5TxDma {
            Uep5TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxFifo(pub u32);
    impl Uep5TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep5TxFifo {
        #[inline(always)]
        fn default() -> Uep5TxFifo {
            Uep5TxFifo(0)
        }
    }
    #[doc = "endpoint 6 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6MaxLen(pub u32);
    impl Uep6MaxLen {
        #[doc = "endpoint 6 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep6_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 6 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep6_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep6MaxLen {
        #[inline(always)]
        fn default() -> Uep6MaxLen {
            Uep6MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RSize(pub u16);
    impl Uep6RSize {
        #[doc = "the length of the total received data at endpoint 6."]
        #[inline(always)]
        pub const fn uep6_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 6."]
        #[inline(always)]
        pub fn set_uep6_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6RSize {
        #[inline(always)]
        fn default() -> Uep6RSize {
            Uep6RSize(0)
        }
    }
    #[doc = "endpoint 6 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxCtrl(pub u8);
    impl Uep6RxCtrl {
        #[doc = "endpoint 6 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 6 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 6 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 6 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 6 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 6 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 6 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 6 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 6 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 6 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6RxCtrl {
        #[inline(always)]
        fn default() -> Uep6RxCtrl {
            Uep6RxCtrl(0)
        }
    }
    #[doc = "endpoint 6 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxDma(pub u32);
    impl Uep6RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep6_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep6_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep6RxDma {
        #[inline(always)]
        fn default() -> Uep6RxDma {
            Uep6RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxFifo(pub u32);
    impl Uep6RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep6RxFifo {
        #[inline(always)]
        fn default() -> Uep6RxFifo {
            Uep6RxFifo(0)
        }
    }
    #[doc = "endpoint 6 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxLen(pub u16);
    impl Uep6RxLen {
        #[doc = "the length of data received by endpoint 6 in a single pass."]
        #[inline(always)]
        pub const fn uep6_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 6 in a single pass."]
        #[inline(always)]
        pub fn set_uep6_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6RxLen {
        #[inline(always)]
        fn default() -> Uep6RxLen {
            Uep6RxLen(0)
        }
    }
    #[doc = "endpoint 6 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TLen(pub u16);
    impl Uep6TLen {
        #[doc = "endpoint 6 send the length."]
        #[inline(always)]
        pub const fn uep6_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 6 send the length."]
        #[inline(always)]
        pub fn set_uep6_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6TLen {
        #[inline(always)]
        fn default() -> Uep6TLen {
            Uep6TLen(0)
        }
    }
    #[doc = "endpoint 6 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxCtrl(pub u8);
    impl Uep6TxCtrl {
        #[doc = "endpoint 6 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 6 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 6 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 6 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 6 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 6 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 6 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 6 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6TxCtrl {
        #[inline(always)]
        fn default() -> Uep6TxCtrl {
            Uep6TxCtrl(0)
        }
    }
    #[doc = "endpoint 6 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxDma(pub u32);
    impl Uep6TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep6_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep6_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep6TxDma {
        #[inline(always)]
        fn default() -> Uep6TxDma {
            Uep6TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxFifo(pub u32);
    impl Uep6TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep6TxFifo {
        #[inline(always)]
        fn default() -> Uep6TxFifo {
            Uep6TxFifo(0)
        }
    }
    #[doc = "endpoint 7 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7MaxLen(pub u32);
    impl Uep7MaxLen {
        #[doc = "endpoint 7 max acceptable offset length."]
        #[inline(always)]
        pub const fn uep7_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 7 max acceptable offset length."]
        #[inline(always)]
        pub fn set_uep7_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep7MaxLen {
        #[inline(always)]
        fn default() -> Uep7MaxLen {
            Uep7MaxLen(0)
        }
    }
    #[doc = "the length register of the total received data at endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RSize(pub u16);
    impl Uep7RSize {
        #[doc = "the length of the total received data at endpoint 7."]
        #[inline(always)]
        pub const fn uep7_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint 7."]
        #[inline(always)]
        pub fn set_uep7_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7RSize {
        #[inline(always)]
        fn default() -> Uep7RSize {
            Uep7RSize(0)
        }
    }
    #[doc = "endpoint 7 receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxCtrl(pub u8);
    impl Uep7RxCtrl {
        #[doc = "endpoint 7 has control over the received response."]
        #[inline(always)]
        pub const fn uep_r_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 7 has control over the received response."]
        #[inline(always)]
        pub fn set_uep_r_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 7 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint 7 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn uep_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub fn set_uep_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 7 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 7 for the received return NAK,packet type."]
        #[inline(always)]
        pub fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 7 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 7 receives the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 7 receives the end flag."]
        #[inline(always)]
        pub const fn uep_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 7 receives the end flag."]
        #[inline(always)]
        pub fn set_uep_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7RxCtrl {
        #[inline(always)]
        fn default() -> Uep7RxCtrl {
            Uep7RxCtrl(0)
        }
    }
    #[doc = "endpoint 7 receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxDma(pub u32);
    impl Uep7RxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep7_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep7_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep7RxDma {
        #[inline(always)]
        fn default() -> Uep7RxDma {
            Uep7RxDma(0)
        }
    }
    #[doc = "Receive FIFO address of usb endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxFifo(pub u32);
    impl Uep7RxFifo {
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub const fn uep_rx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the receiving endpoint."]
        #[inline(always)]
        pub fn set_uep_rx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep7RxFifo {
        #[inline(always)]
        fn default() -> Uep7RxFifo {
            Uep7RxFifo(0)
        }
    }
    #[doc = "endpoint 7 receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxLen(pub u16);
    impl Uep7RxLen {
        #[doc = "the length of data received by endpoint 7 in a single pass."]
        #[inline(always)]
        pub const fn uep7_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint 7 in a single pass."]
        #[inline(always)]
        pub fn set_uep7_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7RxLen {
        #[inline(always)]
        fn default() -> Uep7RxLen {
            Uep7RxLen(0)
        }
    }
    #[doc = "endpoint 7 send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TLen(pub u16);
    impl Uep7TLen {
        #[doc = "endpoint 7 send the length."]
        #[inline(always)]
        pub const fn uep7_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 7 send the length."]
        #[inline(always)]
        pub fn set_uep7_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7TLen {
        #[inline(always)]
        fn default() -> Uep7TLen {
            Uep7TLen(0)
        }
    }
    #[doc = "endpoint 7 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxCtrl(pub u8);
    impl Uep7TxCtrl {
        #[doc = "endpoint 7 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn uep_t_res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 7 control of the send response to IN transactions."]
        #[inline(always)]
        pub fn set_uep_t_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 7 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 7 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 7 sends the end flag."]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 7 sends the end flag."]
        #[inline(always)]
        pub fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 7 sends the end of NAK flag."]
        #[inline(always)]
        pub const fn uep_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 7 sends the end of NAK flag."]
        #[inline(always)]
        pub fn set_uep_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7TxCtrl {
        #[inline(always)]
        fn default() -> Uep7TxCtrl {
            Uep7TxCtrl(0)
        }
    }
    #[doc = "endpoint 7 sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxDma(pub u32);
    impl Uep7TxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn uep7_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub fn set_uep7_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uep7TxDma {
        #[inline(always)]
        fn default() -> Uep7TxDma {
            Uep7TxDma(0)
        }
    }
    #[doc = "The sending FIFO address of usb endpoint 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxFifo(pub u32);
    impl Uep7TxFifo {
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The FIFO start address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub const fn uep_tx_fifo_e(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The end FIFO address of the sending endpoint."]
        #[inline(always)]
        pub fn set_uep_tx_fifo_e(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Uep7TxFifo {
        #[inline(always)]
        fn default() -> Uep7TxFifo {
            Uep7TxFifo(0)
        }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepAfMode(pub u32);
    impl UepAfMode {
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[inline(always)]
        pub const fn uep_t_af(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[inline(always)]
        pub fn set_uep_t_af(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for UepAfMode {
        #[inline(always)]
        fn default() -> UepAfMode {
            UepAfMode(0)
        }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRBurst(pub u8);
    impl UepRBurst {
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[inline(always)]
        pub const fn uep_r_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[inline(always)]
        pub fn set_uep_r_burst_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRBurst {
        #[inline(always)]
        fn default() -> UepRBurst {
            UepRBurst(0)
        }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRIso(pub u16);
    impl UepRIso {
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[inline(always)]
        pub const fn uepn_r_iso_en(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[inline(always)]
        pub fn set_uepn_r_iso_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub const fn uep_r_fifo_en(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub fn set_uep_r_fifo_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
        }
    }
    impl Default for UepRIso {
        #[inline(always)]
        fn default() -> UepRIso {
            UepRIso(0)
        }
    }
    #[doc = "USB endpoint reply mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRResMode(pub u8);
    impl UepRResMode {
        #[doc = "0 to 7 endpoint reply mode."]
        #[inline(always)]
        pub const fn uep_r_res_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint reply mode."]
        #[inline(always)]
        pub fn set_uep_r_res_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRResMode {
        #[inline(always)]
        fn default() -> UepRResMode {
            UepRResMode(0)
        }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRTogAuto(pub u16);
    impl UepRTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn uep_r_tog_auto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub fn set_uep_r_tog_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for UepRTogAuto {
        #[inline(always)]
        fn default() -> UepRTogAuto {
            UepRTogAuto(0)
        }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[inline(always)]
        pub const fn uep_rx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[inline(always)]
        pub fn set_uep_rx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRxEn {
        #[inline(always)]
        fn default() -> UepRxEn {
            UepRxEn(0)
        }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurst(pub u8);
    impl UepTBurst {
        #[doc = "0 to 7 endpoint send burst enable."]
        #[inline(always)]
        pub const fn uep_t_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send burst enable."]
        #[inline(always)]
        pub fn set_uep_t_burst_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurst {
        #[inline(always)]
        fn default() -> UepTBurst {
            UepTBurst(0)
        }
    }
    #[doc = "USB endpoint send the mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurstMode(pub u8);
    impl UepTBurstMode {
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[inline(always)]
        pub const fn uep_t_burst_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[inline(always)]
        pub fn set_uep_t_burst_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurstMode {
        #[inline(always)]
        fn default() -> UepTBurstMode {
            UepTBurstMode(0)
        }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTIso(pub u16);
    impl UepTIso {
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[inline(always)]
        pub const fn uepn_t_iso_en(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[inline(always)]
        pub fn set_uepn_t_iso_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub const fn uep_t_fifo_en(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "The FIFO mode of the TX of the endpoint is enabled."]
        #[inline(always)]
        pub fn set_uep_t_fifo_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u16) & 0x7f) << 9usize);
        }
    }
    impl Default for UepTIso {
        #[inline(always)]
        fn default() -> UepTIso {
            UepTIso(0)
        }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTTogAuto(pub u16);
    impl UepTTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn uep_t_tog_auto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub fn set_uep_t_tog_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for UepTTogAuto {
        #[inline(always)]
        fn default() -> UepTTogAuto {
            UepTTogAuto(0)
        }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[inline(always)]
        pub const fn uep_tx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[inline(always)]
        pub fn set_uep_tx_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepTxEn {
        #[inline(always)]
        fn default() -> UepTxEn {
            UepTxEn(0)
        }
    }
    #[doc = "USB host BC charging control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhBcCtrl(pub u32);
    impl UhBcCtrl {
        #[doc = "UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "UDP pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator enables."]
        #[inline(always)]
        pub fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[inline(always)]
        pub fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic mode enables."]
        #[inline(always)]
        pub const fn bc_auto_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic mode enables."]
        #[inline(always)]
        pub fn set_bc_auto_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
        #[inline(always)]
        pub const fn udm_vsrc_act(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
        #[inline(always)]
        pub fn set_udm_vsrc_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for UhBcCtrl {
        #[inline(always)]
        fn default() -> UhBcCtrl {
            UhBcCtrl(0)
        }
    }
    #[doc = "USB host Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhCfg(pub u8);
    impl UhCfg {
        #[doc = "USB connection controller module resets."]
        #[inline(always)]
        pub const fn uh_rst_link(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection controller module resets."]
        #[inline(always)]
        pub fn set_uh_rst_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn uh_rst_sie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub fn set_uh_rst_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn uh_clr_all(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub fn set_uh_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[inline(always)]
        pub const fn uh_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[inline(always)]
        pub fn set_uh_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn uh_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub fn set_uh_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending function enabled."]
        #[inline(always)]
        pub const fn uh_sof_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending function enabled."]
        #[inline(always)]
        pub fn set_uh_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "forced full speed FS."]
        #[inline(always)]
        pub const fn uh_force_fs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "forced full speed FS."]
        #[inline(always)]
        pub fn set_uh_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn uh_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub fn set_uh_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhCfg {
        #[inline(always)]
        fn default() -> UhCfg {
            UhCfg(0)
        }
    }
    #[doc = "USB host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhControl(pub u32);
    impl UhControl {
        #[doc = "transaction token packet PID."]
        #[inline(always)]
        pub const fn uh_t_token_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet PID."]
        #[inline(always)]
        pub fn set_uh_t_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "transaction token packet endpoint number."]
        #[inline(always)]
        pub const fn uh_t_endp_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet endpoint number."]
        #[inline(always)]
        pub fn set_uh_t_endp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "send data PID."]
        #[inline(always)]
        pub const fn uh_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "send data PID."]
        #[inline(always)]
        pub fn set_uh_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "DATA cache control bit."]
        #[inline(always)]
        pub const fn uh_buf_mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DATA cache control bit."]
        #[inline(always)]
        pub fn set_uh_buf_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "host enables the transaction."]
        #[inline(always)]
        pub const fn uh_host_action(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "host enables the transaction."]
        #[inline(always)]
        pub fn set_uh_host_action(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "valid to send LMP packet."]
        #[inline(always)]
        pub const fn uh_lpm_valid(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send LMP packet."]
        #[inline(always)]
        pub fn set_uh_lpm_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "valid to send SPLIT packet."]
        #[inline(always)]
        pub const fn uh_split_valid(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send SPLIT packet."]
        #[inline(always)]
        pub fn set_uh_split_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[inline(always)]
        pub const fn uh_pre_pid_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[inline(always)]
        pub fn set_uh_pre_pid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn uh_tx_no_data(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[inline(always)]
        pub fn set_uh_tx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn uh_rx_no_data(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[inline(always)]
        pub fn set_uh_rx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn uh_tx_no_res(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[inline(always)]
        pub fn set_uh_tx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn uh_rx_no_res(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[inline(always)]
        pub fn set_uh_rx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for UhControl {
        #[inline(always)]
        fn default() -> UhControl {
            UhControl(0)
        }
    }
    #[doc = "USB host device address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhDevAd(pub u8);
    impl UhDevAd {
        #[doc = "device address."]
        #[inline(always)]
        pub const fn uh_dev_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "device address."]
        #[inline(always)]
        pub fn set_uh_dev_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for UhDevAd {
        #[inline(always)]
        fn default() -> UhDevAd {
            UhDevAd(0)
        }
    }
    #[doc = "USB host frame register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhFrame(pub u32);
    impl UhFrame {
        #[doc = "the frame number."]
        #[inline(always)]
        pub const fn uh_frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "the frame number."]
        #[inline(always)]
        pub fn set_uh_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "microfame number."]
        #[inline(always)]
        pub const fn uh_mframe_no(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "microfame number."]
        #[inline(always)]
        pub fn set_uh_mframe_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "SOF count enabled."]
        #[inline(always)]
        pub const fn uh_sof_cnt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SOF count enabled."]
        #[inline(always)]
        pub fn set_uh_sof_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "the SOF count is cleared."]
        #[inline(always)]
        pub const fn uh_sof_cnt_clr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "the SOF count is cleared."]
        #[inline(always)]
        pub fn set_uh_sof_cnt_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for UhFrame {
        #[inline(always)]
        fn default() -> UhFrame {
            UhFrame(0)
        }
    }
    #[doc = "USB host interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntEn(pub u8);
    impl UhIntEn {
        #[doc = "Wakeup interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_wkup_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_wkup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[inline(always)]
        pub const fn uhie_resume_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[inline(always)]
        pub fn set_uhie_resume_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[inline(always)]
        pub const fn uhie_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[inline(always)]
        pub fn set_uhie_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt enable."]
        #[inline(always)]
        pub const fn uhie_tx_halt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt enable."]
        #[inline(always)]
        pub fn set_uhie_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn uhie_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[inline(always)]
        pub fn set_uhie_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntEn {
        #[inline(always)]
        fn default() -> UhIntEn {
            UhIntEn(0)
        }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntFlag(pub u8);
    impl UhIntFlag {
        #[doc = "Wakeup interrupt flag."]
        #[inline(always)]
        pub const fn uhif_wkup_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_wkup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt flag."]
        #[inline(always)]
        pub const fn uhif_resume_act_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_resume_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[inline(always)]
        pub const fn uhif_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[inline(always)]
        pub const fn uhif_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt flag."]
        #[inline(always)]
        pub const fn uhif_tx_halt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_tx_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[inline(always)]
        pub const fn uhif_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[inline(always)]
        pub fn set_uhif_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntFlag {
        #[inline(always)]
        fn default() -> UhIntFlag {
            UhIntFlag(0)
        }
    }
    #[doc = "USB host interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntSt(pub u8);
    impl UhIntSt {
        #[doc = "received PID."]
        #[inline(always)]
        pub const fn uh_r_token_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "received PID."]
        #[inline(always)]
        pub fn set_uh_r_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[inline(always)]
        pub const fn uhis_port_rx_resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[inline(always)]
        pub fn set_uhis_port_rx_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UhIntSt {
        #[inline(always)]
        fn default() -> UhIntSt {
            UhIntSt(0)
        }
    }
    #[doc = "USB host power management data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhLpmData(pub u32);
    impl UhLpmData {
        #[doc = "power management data."]
        #[inline(always)]
        pub const fn uh_lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "power management data."]
        #[inline(always)]
        pub fn set_uh_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhLpmData {
        #[inline(always)]
        fn default() -> UhLpmData {
            UhLpmData(0)
        }
    }
    #[doc = "USB host miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhMisSt(pub u8);
    impl UhMisSt {
        #[doc = "port enabled state."]
        #[inline(always)]
        pub const fn uhms_sof_free(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled state."]
        #[inline(always)]
        pub fn set_uhms_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
        #[inline(always)]
        pub const fn uhms_sof_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
        #[inline(always)]
        pub fn set_uhms_sof_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus SOF status."]
        #[inline(always)]
        pub const fn uhms_sof_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus SOF status."]
        #[inline(always)]
        pub fn set_uhms_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB bus wakes up."]
        #[inline(always)]
        pub const fn uhms_usb_wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus wakes up."]
        #[inline(always)]
        pub fn set_uhms_usb_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB bus status."]
        #[inline(always)]
        pub const fn uhms_linestate(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USB bus status."]
        #[inline(always)]
        pub fn set_uhms_linestate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "J on USB bus."]
        #[inline(always)]
        pub const fn uhms_bus_j(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "J on USB bus."]
        #[inline(always)]
        pub fn set_uhms_bus_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SE0 on USB bus."]
        #[inline(always)]
        pub const fn uhms_bus_se0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SE0 on USB bus."]
        #[inline(always)]
        pub fn set_uhms_bus_se0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhMisSt {
        #[inline(always)]
        fn default() -> UhMisSt {
            UhMisSt(0)
        }
    }
    #[doc = "USB host port Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortCfg(pub u8);
    impl UhPortCfg {
        #[doc = "USB port mode selection."]
        #[inline(always)]
        pub const fn uh_host_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB port mode selection."]
        #[inline(always)]
        pub fn set_uh_host_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
        #[inline(always)]
        pub const fn uh_pd_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
        #[inline(always)]
        pub fn set_uh_pd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhPortCfg {
        #[inline(always)]
        fn default() -> UhPortCfg {
            UhPortCfg(0)
        }
    }
    #[doc = "USB host port state charge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortChg(pub u16);
    impl UhPortChg {
        #[doc = "port connection state charge."]
        #[inline(always)]
        pub const fn uhif_port_connect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state charge."]
        #[inline(always)]
        pub fn set_uhif_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "port enable state charge."]
        #[inline(always)]
        pub const fn uhif_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enable state charge."]
        #[inline(always)]
        pub fn set_uhif_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "port pause state charge."]
        #[inline(always)]
        pub const fn uhif_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state charge."]
        #[inline(always)]
        pub fn set_uhif_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "port reset state charge."]
        #[inline(always)]
        pub const fn uhif_port_reset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset state charge."]
        #[inline(always)]
        pub fn set_uhif_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "port sleep state charge."]
        #[inline(always)]
        pub const fn uhif_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep state charge."]
        #[inline(always)]
        pub fn set_uhif_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
    }
    impl Default for UhPortChg {
        #[inline(always)]
        fn default() -> UhPortChg {
            UhPortChg(0)
        }
    }
    #[doc = "USB host port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortCtrl(pub u32);
    impl UhPortCtrl {
        #[doc = "PORT send reset."]
        #[inline(always)]
        pub const fn uh_set_port_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PORT send reset."]
        #[inline(always)]
        pub fn set_uh_set_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the PORT is in a suspended state."]
        #[inline(always)]
        pub const fn uh_set_port_susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT is in a suspended state."]
        #[inline(always)]
        pub fn set_uh_set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "the PORT exits the suspended state."]
        #[inline(always)]
        pub const fn uh_clr_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the suspended state."]
        #[inline(always)]
        pub fn set_uh_clr_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[inline(always)]
        pub const fn uh_set_port_sleep(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[inline(always)]
        pub fn set_uh_set_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[inline(always)]
        pub const fn uh_clr_port_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[inline(always)]
        pub fn set_uh_clr_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "bring the PORT into the port state."]
        #[inline(always)]
        pub const fn uh_clr_port_connect(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "bring the PORT into the port state."]
        #[inline(always)]
        pub fn set_uh_clr_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[inline(always)]
        pub const fn uh_clr_port_sleep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[inline(always)]
        pub fn set_uh_clr_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "wake-up time control."]
        #[inline(always)]
        pub const fn uh_port_sleep_besl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "wake-up time control."]
        #[inline(always)]
        pub fn set_uh_port_sleep_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "BUS reset time selection."]
        #[inline(always)]
        pub const fn uh_bus_rst_long(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUS reset time selection."]
        #[inline(always)]
        pub fn set_uh_bus_rst_long(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for UhPortCtrl {
        #[inline(always)]
        fn default() -> UhPortCtrl {
            UhPortCtrl(0)
        }
    }
    #[doc = "USB host port interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortIntEn(pub u8);
    impl UhPortIntEn {
        #[doc = "port connection state change interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_port_connect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state change interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "port enable state change interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enable state change interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "port pause state change interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state change interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "port reset state change interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_port_reset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset state change interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "port sleep state change interrupt enabled."]
        #[inline(always)]
        pub const fn uhie_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep state change interrupt enabled."]
        #[inline(always)]
        pub fn set_uhie_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for UhPortIntEn {
        #[inline(always)]
        fn default() -> UhPortIntEn {
            UhPortIntEn(0)
        }
    }
    #[doc = "USB host port status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortSt(pub u16);
    impl UhPortSt {
        #[doc = "port connection state."]
        #[inline(always)]
        pub const fn uhis_port_c0nnect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state."]
        #[inline(always)]
        pub fn set_uhis_port_c0nnect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "port enabled."]
        #[inline(always)]
        pub const fn uhis_port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled."]
        #[inline(always)]
        pub fn set_uhis_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "port pause state."]
        #[inline(always)]
        pub const fn uhis_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state."]
        #[inline(always)]
        pub fn set_uhis_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "port reset status."]
        #[inline(always)]
        pub const fn uhis_port_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset status."]
        #[inline(always)]
        pub fn set_uhis_port_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "port sleep."]
        #[inline(always)]
        pub const fn uhis_port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep."]
        #[inline(always)]
        pub fn set_uhis_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "whether the port connection speed is low."]
        #[inline(always)]
        pub const fn uhis_port_ls(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is low."]
        #[inline(always)]
        pub fn set_uhis_port_ls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "whether the port connection speed is hign."]
        #[inline(always)]
        pub const fn uhis_port_hs(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is hign."]
        #[inline(always)]
        pub fn set_uhis_port_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "whether the port is in test mode."]
        #[inline(always)]
        pub const fn uhis_port_test(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port is in test mode."]
        #[inline(always)]
        pub fn set_uhis_port_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for UhPortSt {
        #[inline(always)]
        fn default() -> UhPortSt {
            UhPortSt(0)
        }
    }
    #[doc = "USB host port test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhPortTestCt(pub u8);
    impl UhPortTestCt {
        #[doc = "test output J."]
        #[inline(always)]
        pub const fn uh_test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test output J."]
        #[inline(always)]
        pub fn set_uh_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test output K."]
        #[inline(always)]
        pub const fn uh_test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test output K."]
        #[inline(always)]
        pub fn set_uh_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode was enabled."]
        #[inline(always)]
        pub const fn uh_test_force_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode was enabled."]
        #[inline(always)]
        pub fn set_uh_test_force_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "test packet."]
        #[inline(always)]
        pub const fn uh_test_packet(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "test packet."]
        #[inline(always)]
        pub fn set_uh_test_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "test SE0 NAK."]
        #[inline(always)]
        pub const fn uh_test_se0_nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "test SE0 NAK."]
        #[inline(always)]
        pub fn set_uh_test_se0_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UhPortTestCt {
        #[inline(always)]
        fn default() -> UhPortTestCt {
            UhPortTestCt(0)
        }
    }
    #[doc = "DMA receive address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDma(pub u32);
    impl UhRxDma {
        #[doc = "Received address."]
        #[inline(always)]
        pub const fn uh_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Received address."]
        #[inline(always)]
        pub fn set_uh_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for UhRxDma {
        #[inline(always)]
        fn default() -> UhRxDma {
            UhRxDma(0)
        }
    }
    #[doc = "USB host receive length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxLen(pub u32);
    impl UhRxLen {
        #[doc = "Received data length."]
        #[inline(always)]
        pub const fn uh_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received data length."]
        #[inline(always)]
        pub fn set_uh_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxLen {
        #[inline(always)]
        fn default() -> UhRxLen {
            UhRxLen(0)
        }
    }
    #[doc = "USB host receives the maximum length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxMaxLen(pub u32);
    impl UhRxMaxLen {
        #[doc = "receives the maximum length."]
        #[inline(always)]
        pub const fn uh_rx_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "receives the maximum length."]
        #[inline(always)]
        pub fn set_uh_rx_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxMaxLen {
        #[inline(always)]
        fn default() -> UhRxMaxLen {
            UhRxMaxLen(0)
        }
    }
    #[doc = "USB host SPLIT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSplitData(pub u32);
    impl UhSplitData {
        #[doc = "SPLIT management data."]
        #[inline(always)]
        pub const fn uh_split_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "SPLIT management data."]
        #[inline(always)]
        pub fn set_uh_split_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for UhSplitData {
        #[inline(always)]
        fn default() -> UhSplitData {
            UhSplitData(0)
        }
    }
    #[doc = "DMA send address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDma(pub u32);
    impl UhTxDma {
        #[doc = "Send address."]
        #[inline(always)]
        pub const fn uh_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Send address."]
        #[inline(always)]
        pub fn set_uh_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for UhTxDma {
        #[inline(always)]
        fn default() -> UhTxDma {
            UhTxDma(0)
        }
    }
    #[doc = "USB host send length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxLen(pub u32);
    impl UhTxLen {
        #[doc = "send data length."]
        #[inline(always)]
        pub const fn uh_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "send data length."]
        #[inline(always)]
        pub fn set_uh_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhTxLen {
        #[inline(always)]
        fn default() -> UhTxLen {
            UhTxLen(0)
        }
    }
    #[doc = "USB mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbBaseMode(pub u8);
    impl UsbBaseMode {
        #[doc = "The desired speed mode of the device."]
        #[inline(always)]
        pub const fn ud_speed_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The desired speed mode of the device."]
        #[inline(always)]
        pub fn set_ud_speed_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for UsbBaseMode {
        #[inline(always)]
        fn default() -> UsbBaseMode {
            UsbBaseMode(0)
        }
    }
    #[doc = "USB bus."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbBus(pub u16);
    impl UsbBus {
        #[doc = "USB wakeup."]
        #[inline(always)]
        pub const fn usb_wakeup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB wakeup."]
        #[inline(always)]
        pub fn set_usb_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "UDP status."]
        #[inline(always)]
        pub const fn usb_dp_st(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UDP status."]
        #[inline(always)]
        pub fn set_usb_dp_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "UDM status."]
        #[inline(always)]
        pub const fn usb_dm_st(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UDM status."]
        #[inline(always)]
        pub fn set_usb_dm_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
    }
    impl Default for UsbBus {
        #[inline(always)]
        fn default() -> UsbBus {
            UsbBus(0)
        }
    }
    #[doc = "USB base control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbCtrl(pub u8);
    impl UsbCtrl {
        #[doc = "LINK layer reset,highly effective."]
        #[inline(always)]
        pub const fn ud_rst_link(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK layer reset,highly effective."]
        #[inline(always)]
        pub fn set_ud_rst_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn ud_rst_sie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub fn set_ud_rst_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn ud_clr_all(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub fn set_ud_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB PHY suspend."]
        #[inline(always)]
        pub const fn ud_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB PHY suspend."]
        #[inline(always)]
        pub fn set_ud_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn ud_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub fn set_ud_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enabled."]
        #[inline(always)]
        pub const fn ud_dev_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enabled."]
        #[inline(always)]
        pub fn set_ud_dev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn ud_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub fn set_ud_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbCtrl {
        #[inline(always)]
        fn default() -> UsbCtrl {
            UsbCtrl(0)
        }
    }
    #[doc = "USB device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbDevAd(pub u8);
    impl UsbDevAd {
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub const fn mask_usb_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub fn set_mask_usb_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for UsbDevAd {
        #[inline(always)]
        fn default() -> UsbDevAd {
            UsbDevAd(0)
        }
    }
    #[doc = "USB frame number."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbFramNo(pub u16);
    impl UsbFramNo {
        #[doc = "Received frame number."]
        #[inline(always)]
        pub const fn ud_frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received frame number."]
        #[inline(always)]
        pub fn set_ud_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Received micro frame number."]
        #[inline(always)]
        pub const fn ud_mframe_no(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Received micro frame number."]
        #[inline(always)]
        pub fn set_ud_mframe_no(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for UsbFramNo {
        #[inline(always)]
        fn default() -> UsbFramNo {
            UsbFramNo(0)
        }
    }
    #[doc = "USB interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntEn(pub u8);
    impl UsbIntEn {
        #[doc = "USB bus reset interrupt enabled."]
        #[inline(always)]
        pub const fn udie_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt enabled."]
        #[inline(always)]
        pub fn set_udie_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[inline(always)]
        pub const fn udie_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[inline(always)]
        pub fn set_udie_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[inline(always)]
        pub const fn udie_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[inline(always)]
        pub fn set_udie_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn udie_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[inline(always)]
        pub fn set_udie_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn udie_transfer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub fn set_udie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[inline(always)]
        pub const fn udie_sof_act(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[inline(always)]
        pub fn set_udie_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interrupt enable."]
        #[inline(always)]
        pub const fn udie_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt enable."]
        #[inline(always)]
        pub fn set_udie_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt enable."]
        #[inline(always)]
        pub const fn udie_fifo_over(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt enable."]
        #[inline(always)]
        pub fn set_udie_fifo_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbIntEn {
        #[inline(always)]
        fn default() -> UsbIntEn {
            UsbIntEn(0)
        }
    }
    #[doc = "USB interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntFg(pub u8);
    impl UsbIntFg {
        #[doc = "USB bus reset interrupt flag."]
        #[inline(always)]
        pub const fn udif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt flag."]
        #[inline(always)]
        pub fn set_udif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[inline(always)]
        pub const fn udif_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[inline(always)]
        pub fn set_udif_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[inline(always)]
        pub const fn udif_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[inline(always)]
        pub fn set_udif_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[inline(always)]
        pub const fn udif_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[inline(always)]
        pub fn set_udif_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transmission end interrupt flag."]
        #[inline(always)]
        pub const fn udif_rtx_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transmission end interrupt flag."]
        #[inline(always)]
        pub fn set_udif_rtx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[inline(always)]
        pub const fn udif_rx_sof(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[inline(always)]
        pub fn set_udif_rx_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interrupt flag."]
        #[inline(always)]
        pub const fn udif_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt flag."]
        #[inline(always)]
        pub fn set_udif_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt flag."]
        #[inline(always)]
        pub const fn udif_fifo_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt flag."]
        #[inline(always)]
        pub fn set_udif_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbIntFg {
        #[inline(always)]
        fn default() -> UsbIntFg {
            UsbIntFg(0)
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbIntSt(pub u8);
    impl UsbIntSt {
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[inline(always)]
        pub const fn udis_ep_id_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[inline(always)]
        pub fn set_udis_ep_id_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "Endpoint data transmission direction."]
        #[inline(always)]
        pub const fn udis_ep_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint data transmission direction."]
        #[inline(always)]
        pub fn set_udis_ep_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UsbIntSt {
        #[inline(always)]
        fn default() -> UsbIntSt {
            UsbIntSt(0)
        }
    }
    #[doc = "USB power management register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbLpmData(pub u16);
    impl UsbLpmData {
        #[doc = "power management data."]
        #[inline(always)]
        pub const fn ud_lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "power management data."]
        #[inline(always)]
        pub fn set_ud_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "power management busy."]
        #[inline(always)]
        pub const fn ud_lpm_busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "power management busy."]
        #[inline(always)]
        pub fn set_ud_lpm_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for UsbLpmData {
        #[inline(always)]
        fn default() -> UsbLpmData {
            UsbLpmData(0)
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbMisSt(pub u8);
    impl UsbMisSt {
        #[doc = "USB connection status."]
        #[inline(always)]
        pub const fn udms_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection status."]
        #[inline(always)]
        pub fn set_udms_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB suspend status."]
        #[inline(always)]
        pub const fn udms_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend status."]
        #[inline(always)]
        pub fn set_udms_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB sleep status."]
        #[inline(always)]
        pub const fn udms_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB sleep status."]
        #[inline(always)]
        pub fn set_udms_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB free status."]
        #[inline(always)]
        pub const fn udms_sie_free(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB free status."]
        #[inline(always)]
        pub fn set_udms_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB suspends the request."]
        #[inline(always)]
        pub const fn udms_susp_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspends the request."]
        #[inline(always)]
        pub fn set_udms_susp_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "whether the host is high-speed."]
        #[inline(always)]
        pub const fn udms_hs_mod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "whether the host is high-speed."]
        #[inline(always)]
        pub fn set_udms_hs_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbMisSt {
        #[inline(always)]
        fn default() -> UsbMisSt {
            UsbMisSt(0)
        }
    }
    #[doc = "USB test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbTestMode(pub u8);
    impl UsbTestMode {
        #[doc = "test mode,output J."]
        #[inline(always)]
        pub const fn ud_test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output J."]
        #[inline(always)]
        pub fn set_ud_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test mode,output K."]
        #[inline(always)]
        pub const fn ud_test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output K."]
        #[inline(always)]
        pub fn set_ud_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode,output a packet."]
        #[inline(always)]
        pub const fn ud_test_pkt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output a packet."]
        #[inline(always)]
        pub fn set_ud_test_pkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "test mode,output SEO."]
        #[inline(always)]
        pub const fn ud_test_se0nak(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output SEO."]
        #[inline(always)]
        pub fn set_ud_test_se0nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "test mode enable."]
        #[inline(always)]
        pub const fn ud_test_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "test mode enable."]
        #[inline(always)]
        pub fn set_ud_test_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UsbTestMode {
        #[inline(always)]
        fn default() -> UsbTestMode {
            UsbTestMode(0)
        }
    }
    #[doc = "USB remote wake up register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbWakeCtrl(pub u16);
    impl UsbWakeCtrl {
        #[doc = "remote wake up."]
        #[inline(always)]
        pub const fn ud_remote_wkup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "remote wake up."]
        #[inline(always)]
        pub fn set_ud_remote_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
    }
    impl Default for UsbWakeCtrl {
        #[inline(always)]
        fn default() -> UsbWakeCtrl {
            UsbWakeCtrl(0)
        }
    }
}
