#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet MAC-10M+PHY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt enable register"]
    #[inline(always)]
    pub const fn eie(self) -> crate::common::Reg<regs::Eie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "Interrupt flag register"]
    #[inline(always)]
    pub const fn eir(self) -> crate::common::Reg<regs::Eir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn estat(self) -> crate::common::Reg<regs::Estat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "PHY analog parameter setting register"]
    #[inline(always)]
    pub const fn econ2(self) -> crate::common::Reg<regs::Econ2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Receive/transmit control register"]
    #[inline(always)]
    pub const fn econ1(self) -> crate::common::Reg<regs::Econ1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "Transmit DMA buffer start address register"]
    #[inline(always)]
    pub const fn etxst(self) -> crate::common::Reg<regs::Etxst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Transmission length register"]
    #[inline(always)]
    pub const fn etxln(self) -> crate::common::Reg<regs::Etxln, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Receive DMA buffer start address register"]
    #[inline(always)]
    pub const fn erxst(self) -> crate::common::Reg<regs::Erxst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Reception length register"]
    #[inline(always)]
    pub const fn erxln(self) -> crate::common::Reg<regs::Erxln, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Hash table low register"]
    #[inline(always)]
    pub const fn htl(self) -> crate::common::Reg<regs::Htl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Hash table high register"]
    #[inline(always)]
    pub const fn hth(self) -> crate::common::Reg<regs::Hth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Receive packet filter control register"]
    #[inline(always)]
    pub const fn erxfcon(self) -> crate::common::Reg<regs::Erxfcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Mac layer flow control register"]
    #[inline(always)]
    pub const fn macon1(self) -> crate::common::Reg<regs::Macon1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x19usize) as _) }
    }
    #[doc = "Mac layer packet control register"]
    #[inline(always)]
    pub const fn macon2(self) -> crate::common::Reg<regs::Macon2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Minimum packet interval register"]
    #[inline(always)]
    pub const fn mabbipg(self) -> crate::common::Reg<regs::Mabbipg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1busize) as _) }
    }
    #[doc = "Flow control pause frame time register"]
    #[inline(always)]
    pub const fn epaus(self) -> crate::common::Reg<regs::Epaus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Maximum receive packet length register"]
    #[inline(always)]
    pub const fn mamxfl(self) -> crate::common::Reg<regs::Mamxfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[doc = "MII read register"]
    #[inline(always)]
    pub const fn mird(self) -> crate::common::Reg<regs::Mird, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MII read register address"]
    #[inline(always)]
    pub const fn miregadr(self) -> crate::common::Reg<regs::Miregadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MII write register"]
    #[inline(always)]
    pub const fn miwr(self) -> crate::common::Reg<regs::Miwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MAC address low register"]
    #[inline(always)]
    pub const fn maadr0(self) -> crate::common::Reg<regs::Maadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "MAC address byte 1"]
    #[inline(always)]
    pub const fn maadr1(self) -> crate::common::Reg<regs::Maadr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x29usize) as _) }
    }
    #[doc = "MAC address byte 2"]
    #[inline(always)]
    pub const fn maadr2(self) -> crate::common::Reg<regs::Maadr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[doc = "MAC address byte 3"]
    #[inline(always)]
    pub const fn maadr3(self) -> crate::common::Reg<regs::Maadr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2busize) as _) }
    }
    #[doc = "MAC address byte 4"]
    #[inline(always)]
    pub const fn maadr4(self) -> crate::common::Reg<regs::Maadr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "MAC address high register"]
    #[inline(always)]
    pub const fn maadr5(self) -> crate::common::Reg<regs::Maadr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2dusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Econ1(pub u8);
    impl Econ1 {
        #[doc = "Receive enable"]
        #[inline(always)]
        pub const fn rx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive enable"]
        #[inline(always)]
        pub fn set_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Transmit start. Cleared automatically after the transmission is completed"]
        #[inline(always)]
        pub const fn tx_rts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit start. Cleared automatically after the transmission is completed"]
        #[inline(always)]
        pub fn set_tx_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Receive module reset"]
        #[inline(always)]
        pub const fn rx_rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive module reset"]
        #[inline(always)]
        pub fn set_rx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Transmit module reset"]
        #[inline(always)]
        pub const fn tx_rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit module reset"]
        #[inline(always)]
        pub fn set_tx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Econ1 {
        #[inline(always)]
        fn default() -> Econ1 {
            Econ1(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Econ2(pub u8);
    impl Econ2 {
        #[doc = "Transmitter energy-saving driver control"]
        #[inline(always)]
        pub const fn tx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter energy-saving driver control"]
        #[inline(always)]
        pub fn set_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Reserved. Must write 110b."]
        #[inline(always)]
        pub const fn rx_must(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Reserved. Must write 110b."]
        #[inline(always)]
        pub fn set_rx_must(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
        }
    }
    impl Default for Econ2 {
        #[inline(always)]
        fn default() -> Econ2 {
            Econ2(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eie(pub u8);
    impl Eie {
        #[doc = "Receive error interrupt enable"]
        #[inline(always)]
        pub const fn rxerie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive error interrupt enable"]
        #[inline(always)]
        pub fn set_rxerie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Transmit error interrupt enable"]
        #[inline(always)]
        pub const fn txerie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit error interrupt enable"]
        #[inline(always)]
        pub fn set_txerie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Built-in 50ohm impedance matching resistor enable"]
        #[inline(always)]
        pub const fn r_en50(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Built-in 50ohm impedance matching resistor enable"]
        #[inline(always)]
        pub fn set_r_en50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Transmit completed interrupt enable"]
        #[inline(always)]
        pub const fn txie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit completed interrupt enable"]
        #[inline(always)]
        pub fn set_txie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Link change interrupt enable"]
        #[inline(always)]
        pub const fn linkie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Link change interrupt enable"]
        #[inline(always)]
        pub fn set_linkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive completed interrupt enable"]
        #[inline(always)]
        pub const fn rxie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive completed interrupt enable"]
        #[inline(always)]
        pub fn set_rxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Ethernet interrupt enable"]
        #[inline(always)]
        pub const fn intie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet interrupt enable"]
        #[inline(always)]
        pub fn set_intie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Eie {
        #[inline(always)]
        fn default() -> Eie {
            Eie(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eir(pub u8);
    impl Eir {
        #[doc = "Receive error flag"]
        #[inline(always)]
        pub const fn rxerif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive error flag"]
        #[inline(always)]
        pub fn set_rxerif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Transmit error flag"]
        #[inline(always)]
        pub const fn txerif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit error flag"]
        #[inline(always)]
        pub fn set_txerif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Transmit completed flag"]
        #[inline(always)]
        pub const fn txif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit completed flag"]
        #[inline(always)]
        pub fn set_txif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Link change flag"]
        #[inline(always)]
        pub const fn linkif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Link change flag"]
        #[inline(always)]
        pub fn set_linkif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive completed flag"]
        #[inline(always)]
        pub const fn rxif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive completed flag"]
        #[inline(always)]
        pub fn set_rxif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Eir {
        #[inline(always)]
        fn default() -> Eir {
            Eir(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epaus(pub u16);
    impl Epaus {
        #[doc = "Flow control pause frame time"]
        #[inline(always)]
        pub const fn epaus(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Flow control pause frame time"]
        #[inline(always)]
        pub fn set_epaus(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Epaus {
        #[inline(always)]
        fn default() -> Epaus {
            Epaus(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Erxfcon(pub u8);
    impl Erxfcon {
        #[doc = "Broadcast packet matching filter settings"]
        #[inline(always)]
        pub const fn bcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Broadcast packet matching filter settings"]
        #[inline(always)]
        pub fn set_bcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Multicast packet matching filter settings"]
        #[inline(always)]
        pub const fn mcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Multicast packet matching filter settings"]
        #[inline(always)]
        pub fn set_mcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Hash table matching filter settings"]
        #[inline(always)]
        pub const fn hten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash table matching filter settings"]
        #[inline(always)]
        pub fn set_hten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Magic packet filter settings"]
        #[inline(always)]
        pub const fn mpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet filter settings"]
        #[inline(always)]
        pub fn set_mpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Receive filtering enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive filtering enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "CRC checksum filter settings"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CRC checksum filter settings"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Unicast match filter settings"]
        #[inline(always)]
        pub const fn ucen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast match filter settings"]
        #[inline(always)]
        pub fn set_ucen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Erxfcon {
        #[inline(always)]
        fn default() -> Erxfcon {
            Erxfcon(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Erxln(pub u16);
    impl Erxln {
        #[doc = "Reception length"]
        #[inline(always)]
        pub const fn erxln(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Reception length"]
        #[inline(always)]
        pub fn set_erxln(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Erxln {
        #[inline(always)]
        fn default() -> Erxln {
            Erxln(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Erxst(pub u16);
    impl Erxst {
        #[doc = "Receive DMA buffer start address"]
        #[inline(always)]
        pub const fn erxst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Receive DMA buffer start address"]
        #[inline(always)]
        pub fn set_erxst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Erxst {
        #[inline(always)]
        fn default() -> Erxst {
            Erxst(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Estat(pub u8);
    impl Estat {
        #[doc = "Transmission interrupted by MCU"]
        #[inline(always)]
        pub const fn txabrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission interrupted by MCU"]
        #[inline(always)]
        pub fn set_txabrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Packets receive in progress"]
        #[inline(always)]
        pub const fn rxbusy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Packets receive in progress"]
        #[inline(always)]
        pub fn set_rxbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Receive more than the set maximum packets"]
        #[inline(always)]
        pub const fn rxmore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive more than the set maximum packets"]
        #[inline(always)]
        pub fn set_rxmore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Receive nibble error / Receive more than the set maximum"]
        #[inline(always)]
        pub const fn rxnibble(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive nibble error / Receive more than the set maximum"]
        #[inline(always)]
        pub fn set_rxnibble(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive CRC error"]
        #[inline(always)]
        pub const fn rxcrcer(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive CRC error"]
        #[inline(always)]
        pub fn set_rxcrcer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Buffer error"]
        #[inline(always)]
        pub const fn bufer(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer error"]
        #[inline(always)]
        pub fn set_bufer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Interrupt"]
        #[inline(always)]
        pub const fn int(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt"]
        #[inline(always)]
        pub fn set_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Estat {
        #[inline(always)]
        fn default() -> Estat {
            Estat(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etxln(pub u16);
    impl Etxln {
        #[doc = "Transmission length"]
        #[inline(always)]
        pub const fn etxln(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Transmission length"]
        #[inline(always)]
        pub fn set_etxln(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Etxln {
        #[inline(always)]
        fn default() -> Etxln {
            Etxln(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etxst(pub u16);
    impl Etxst {
        #[doc = "Transmit DMA buffer start address"]
        #[inline(always)]
        pub const fn etxst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Transmit DMA buffer start address"]
        #[inline(always)]
        pub fn set_etxst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Etxst {
        #[inline(always)]
        fn default() -> Etxst {
            Etxst(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hth(pub u32);
    impl Hth {
        #[doc = "Hash Table byte 4"]
        #[inline(always)]
        pub const fn b4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 4"]
        #[inline(always)]
        pub fn set_b4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Hash Table byte 5"]
        #[inline(always)]
        pub const fn b5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 5"]
        #[inline(always)]
        pub fn set_b5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Hash Table byte 6"]
        #[inline(always)]
        pub const fn b6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 6"]
        #[inline(always)]
        pub fn set_b6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Hash Table byte 7"]
        #[inline(always)]
        pub const fn b7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 7"]
        #[inline(always)]
        pub fn set_b7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Hth {
        #[inline(always)]
        fn default() -> Hth {
            Hth(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htl(pub u32);
    impl Htl {
        #[doc = "Hash Table byte 0"]
        #[inline(always)]
        pub const fn b0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 0"]
        #[inline(always)]
        pub fn set_b0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Hash Table byte 1"]
        #[inline(always)]
        pub const fn b1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 1"]
        #[inline(always)]
        pub fn set_b1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Hash Table byte 2"]
        #[inline(always)]
        pub const fn b2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 2"]
        #[inline(always)]
        pub fn set_b2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Hash Table byte 3"]
        #[inline(always)]
        pub const fn b3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Hash Table byte 3"]
        #[inline(always)]
        pub fn set_b3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Htl {
        #[inline(always)]
        fn default() -> Htl {
            Htl(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr0(pub u8);
    impl Maadr0 {
        #[doc = "MAC Address byte 0"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 0"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr0 {
        #[inline(always)]
        fn default() -> Maadr0 {
            Maadr0(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr1(pub u8);
    impl Maadr1 {
        #[doc = "MAC Address byte 1"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 1"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr1 {
        #[inline(always)]
        fn default() -> Maadr1 {
            Maadr1(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr2(pub u8);
    impl Maadr2 {
        #[doc = "MAC Address byte 2"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 2"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr2 {
        #[inline(always)]
        fn default() -> Maadr2 {
            Maadr2(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr3(pub u8);
    impl Maadr3 {
        #[doc = "MAC Address byte 3"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 3"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr3 {
        #[inline(always)]
        fn default() -> Maadr3 {
            Maadr3(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr4(pub u8);
    impl Maadr4 {
        #[doc = "MAC Address byte 4"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 4"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr4 {
        #[inline(always)]
        fn default() -> Maadr4 {
            Maadr4(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maadr5(pub u8);
    impl Maadr5 {
        #[doc = "MAC Address byte 5"]
        #[inline(always)]
        pub const fn maadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MAC Address byte 5"]
        #[inline(always)]
        pub fn set_maadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Maadr5 {
        #[inline(always)]
        fn default() -> Maadr5 {
            Maadr5(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mabbipg(pub u8);
    impl Mabbipg {
        #[doc = "Minimum number of packet interval bytes"]
        #[inline(always)]
        pub const fn mabbipg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Minimum number of packet interval bytes"]
        #[inline(always)]
        pub fn set_mabbipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Mabbipg {
        #[inline(always)]
        fn default() -> Mabbipg {
            Mabbipg(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macon1(pub u8);
    impl Macon1 {
        #[doc = "MAC layer receive enable"]
        #[inline(always)]
        pub const fn marxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MAC layer receive enable"]
        #[inline(always)]
        pub fn set_marxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Control frame setting"]
        #[inline(always)]
        pub const fn passall(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Control frame setting"]
        #[inline(always)]
        pub fn set_passall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Receive pause frame enable"]
        #[inline(always)]
        pub const fn rxpaus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive pause frame enable"]
        #[inline(always)]
        pub fn set_rxpaus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Transmit pause frame enable control"]
        #[inline(always)]
        pub const fn txpaus(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit pause frame enable control"]
        #[inline(always)]
        pub fn set_txpaus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Pause frame setting. Active at full-duplex"]
        #[inline(always)]
        pub const fn fcen(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Pause frame setting. Active at full-duplex"]
        #[inline(always)]
        pub fn set_fcen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Macon1 {
        #[inline(always)]
        fn default() -> Macon1 {
            Macon1(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macon2(pub u8);
    impl Macon2 {
        #[doc = "Ethernet communication mode"]
        #[inline(always)]
        pub const fn fuldpx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet communication mode"]
        #[inline(always)]
        pub fn set_fuldpx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Jumbo frame received enable"]
        #[inline(always)]
        pub const fn hfrmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Jumbo frame received enable"]
        #[inline(always)]
        pub fn set_hfrmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Special 4 bytes are not involved in CRC."]
        #[inline(always)]
        pub const fn phdren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Special 4 bytes are not involved in CRC."]
        #[inline(always)]
        pub fn set_phdren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Transmit add CRC control"]
        #[inline(always)]
        pub const fn txcrcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit add CRC control"]
        #[inline(always)]
        pub fn set_txcrcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Short packet fill setting"]
        #[inline(always)]
        pub const fn padcfg(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Short packet fill setting"]
        #[inline(always)]
        pub fn set_padcfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
        }
    }
    impl Default for Macon2 {
        #[inline(always)]
        fn default() -> Macon2 {
            Macon2(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mamxfl(pub u16);
    impl Mamxfl {
        #[doc = "Maximum receive packet length"]
        #[inline(always)]
        pub const fn mamxfl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum receive packet length"]
        #[inline(always)]
        pub fn set_mamxfl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Mamxfl {
        #[inline(always)]
        fn default() -> Mamxfl {
            Mamxfl(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mird(pub u16);
    impl Mird {
        #[doc = "MII Read register"]
        #[inline(always)]
        pub const fn rd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MII Read register"]
        #[inline(always)]
        pub fn set_rd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Mird {
        #[inline(always)]
        fn default() -> Mird {
            Mird(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Miregadr(pub u8);
    impl Miregadr {
        #[doc = "MII Read register address"]
        #[inline(always)]
        pub const fn miregadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MII Read register address"]
        #[inline(always)]
        pub fn set_miregadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Miregadr {
        #[inline(always)]
        fn default() -> Miregadr {
            Miregadr(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Miwr(pub u32);
    impl Miwr {
        #[doc = "PHY register address"]
        #[inline(always)]
        pub const fn mirdl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY register address"]
        #[inline(always)]
        pub fn set_mirdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Write to MII register"]
        #[inline(always)]
        pub const fn write(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Write to MII register"]
        #[inline(always)]
        pub fn set_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MII write register"]
        #[inline(always)]
        pub const fn wr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "MII write register"]
        #[inline(always)]
        pub fn set_wr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Miwr {
        #[inline(always)]
        fn default() -> Miwr {
            Miwr(0)
        }
    }
}
