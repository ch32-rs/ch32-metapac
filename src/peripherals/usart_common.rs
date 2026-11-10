#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal synchronous asynchronous receiver transmitter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart {
    ptr: *mut u8,
}
unsafe impl Send for Usart {}
unsafe impl Sync for Usart {}
impl Usart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn statr(self) -> crate::common::Reg<regs::Statr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Baud rate register."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control register 1."]
    #[inline(always)]
    pub const fn ctlr1(self) -> crate::common::Reg<regs::Ctlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control register 2."]
    #[inline(always)]
    pub const fn ctlr2(self) -> crate::common::Reg<regs::Ctlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Control register 3."]
    #[inline(always)]
    pub const fn ctlr3(self) -> crate::common::Reg<regs::Ctlr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Guard time and prescaler register."]
    #[inline(always)]
    pub const fn gpr(self) -> crate::common::Reg<regs::Gpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Baud rate register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "fraction of USARTDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div_fraction(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "fraction of USARTDIV."]
        #[inline(always)]
        pub const fn set_div_fraction(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "mantissa of USARTDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div_mantissa(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "mantissa of USARTDIV."]
        #[inline(always)]
        pub const fn set_div_mantissa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
                .field("div_fraction", &self.div_fraction())
                .field("div_mantissa", &self.div_mantissa())
                .finish()
        }
    }
    #[doc = "Control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr1(pub u32);
    impl Ctlr1 {
        #[doc = "Send break."]
        #[must_use]
        #[inline(always)]
        pub const fn sbk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send break."]
        #[inline(always)]
        pub const fn set_sbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn rwu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver wakeup."]
        #[inline(always)]
        pub const fn set_rwu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn idleie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE interrupt enable."]
        #[inline(always)]
        pub const fn set_idleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXNE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable."]
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete interrupt enable."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TXE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TXE interrupt enable."]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn peie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PE interrupt enable."]
        #[inline(always)]
        pub const fn set_peie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Parity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Parity selection."]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Parity control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pce(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Parity control enable."]
        #[inline(always)]
        pub const fn set_pce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wakeup method."]
        #[must_use]
        #[inline(always)]
        pub const fn wake(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup method."]
        #[inline(always)]
        pub const fn set_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Word length."]
        #[must_use]
        #[inline(always)]
        pub const fn m(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Word length."]
        #[inline(always)]
        pub const fn set_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable."]
        #[inline(always)]
        pub const fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Ctlr1 {
        #[inline(always)]
        fn default() -> Ctlr1 {
            Ctlr1(0)
        }
    }
    impl core::fmt::Debug for Ctlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr1")
                .field("sbk", &self.sbk())
                .field("rwu", &self.rwu())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("idleie", &self.idleie())
                .field("rxneie", &self.rxneie())
                .field("tcie", &self.tcie())
                .field("txeie", &self.txeie())
                .field("peie", &self.peie())
                .field("ps", &self.ps())
                .field("pce", &self.pce())
                .field("wake", &self.wake())
                .field("m", &self.m())
                .field("ue", &self.ue())
                .finish()
        }
    }
    #[doc = "Control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr2(pub u32);
    impl Ctlr2 {
        #[doc = "Address of the USART node."]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node."]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "lin break detection length."]
        #[must_use]
        #[inline(always)]
        pub const fn lbdl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "lin break detection length."]
        #[inline(always)]
        pub const fn set_lbdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable."]
        #[inline(always)]
        pub const fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last bit clock pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn lbcl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last bit clock pulse."]
        #[inline(always)]
        pub const fn set_lbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock phase."]
        #[must_use]
        #[inline(always)]
        pub const fn cpha(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clock phase."]
        #[inline(always)]
        pub const fn set_cpha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn cpol(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock polarity."]
        #[inline(always)]
        pub const fn set_cpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable."]
        #[inline(always)]
        pub const fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP bits."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "STOP bits."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable."]
        #[inline(always)]
        pub const fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Ctlr2 {
        #[inline(always)]
        fn default() -> Ctlr2 {
            Ctlr2(0)
        }
    }
    impl core::fmt::Debug for Ctlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr2")
                .field("add", &self.add())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("lbcl", &self.lbcl())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("clken", &self.clken())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .finish()
        }
    }
    #[doc = "Control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctlr3(pub u32);
    impl Ctlr3 {
        #[doc = "Error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable."]
        #[inline(always)]
        pub const fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable."]
        #[inline(always)]
        pub const fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power."]
        #[must_use]
        #[inline(always)]
        pub const fn irlp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA low-power."]
        #[inline(always)]
        pub const fn set_irlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection."]
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Smartcard NACK enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard NACK enable."]
        #[inline(always)]
        pub const fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Smartcard mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard mode enable."]
        #[inline(always)]
        pub const fn set_scen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA enable receiver."]
        #[must_use]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver."]
        #[inline(always)]
        pub const fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter."]
        #[must_use]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter."]
        #[inline(always)]
        pub const fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTS enable."]
        #[inline(always)]
        pub const fn set_rtse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ctse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS enable."]
        #[inline(always)]
        pub const fn set_ctse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ctsie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt enable."]
        #[inline(always)]
        pub const fn set_ctsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ctlr3 {
        #[inline(always)]
        fn default() -> Ctlr3 {
            Ctlr3(0)
        }
    }
    impl core::fmt::Debug for Ctlr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctlr3")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("nack", &self.nack())
                .field("scen", &self.scen())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .field("rtse", &self.rtse())
                .field("ctse", &self.ctse())
                .field("ctsie", &self.ctsie())
                .finish()
        }
    }
    #[doc = "Data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u32);
    impl Datar {
        #[doc = "Data value."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data value."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    impl core::fmt::Debug for Datar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Datar").field("dr", &self.dr()).finish()
        }
    }
    #[doc = "Guard time and prescaler register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr(pub u32);
    impl Gpr {
        #[doc = "Prescaler value."]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler value."]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Guard time value."]
        #[must_use]
        #[inline(always)]
        pub const fn gt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Guard time value."]
        #[inline(always)]
        pub const fn set_gt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Gpr {
        #[inline(always)]
        fn default() -> Gpr {
            Gpr(0)
        }
    }
    impl core::fmt::Debug for Gpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpr")
                .field("psc", &self.psc())
                .field("gt", &self.gt())
                .finish()
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statr(pub u32);
    impl Statr {
        #[doc = "Parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise error flag."]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error."]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error."]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE line detected."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE line detected."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read data register not empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmit data register empty."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LIN break detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lbd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection flag."]
        #[inline(always)]
        pub const fn set_lbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS flag."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Statr {
        #[inline(always)]
        fn default() -> Statr {
            Statr(0)
        }
    }
    impl core::fmt::Debug for Statr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Statr")
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("idle", &self.idle())
                .field("rxne", &self.rxne())
                .field("tc", &self.tc())
                .field("txe", &self.txe())
                .field("lbd", &self.lbd())
                .field("cts", &self.cts())
                .finish()
        }
    }
}
