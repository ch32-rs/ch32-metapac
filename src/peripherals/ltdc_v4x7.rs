#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LCD-TFT Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltdc {
    ptr: *mut u8,
}
unsafe impl Send for Ltdc {}
unsafe impl Sync for Ltdc {}
impl Ltdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Synchronization Size Configuration Register."]
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::Sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Back Porch Configuration Register."]
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::Bpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Active Width Configuration Register."]
    #[inline(always)]
    pub const fn awcr(self) -> crate::common::Reg<regs::Awcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Total Width Configuration Register."]
    #[inline(always)]
    pub const fn twcr(self) -> crate::common::Reg<regs::Twcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Global Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Shadow Reload Configuration Register."]
    #[inline(always)]
    pub const fn srcr(self) -> crate::common::Reg<regs::Srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Background Color Configuration Register."]
    #[inline(always)]
    pub const fn bccr(self) -> crate::common::Reg<regs::Bccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Line Interrupt Position Configuration Register."]
    #[inline(always)]
    pub const fn lipcr(self) -> crate::common::Reg<regs::Lipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Current Position Status Register."]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Current Display Status Register."]
    #[inline(always)]
    pub const fn cdsr(self) -> crate::common::Reg<regs::Cdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Layerx Control Register."]
    #[inline(always)]
    pub const fn l1cr(self) -> crate::common::Reg<regs::L1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register."]
    #[inline(always)]
    pub const fn lwhpcr(self) -> crate::common::Reg<regs::Lwhpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register."]
    #[inline(always)]
    pub const fn lwvpcr(self) -> crate::common::Reg<regs::Lwvpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Layerx Color Keying Configuration Register."]
    #[inline(always)]
    pub const fn lckcr(self) -> crate::common::Reg<regs::Lckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Layerx Pixel Format Configuration Register."]
    #[inline(always)]
    pub const fn lpfcr(self) -> crate::common::Reg<regs::Lpfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Layerx Default Color Configuration Register."]
    #[inline(always)]
    pub const fn ldccr(self) -> crate::common::Reg<regs::Ldccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Address Register."]
    #[inline(always)]
    pub const fn lcfbar(self) -> crate::common::Reg<regs::Lcfbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Length Register."]
    #[inline(always)]
    pub const fn lcfblr(self) -> crate::common::Reg<regs::Lcfblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Layerx ColorFrame Buffer Line Number Register."]
    #[inline(always)]
    pub const fn lcfblnr(self) -> crate::common::Reg<regs::Lcfblnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Layerx CLUT Write Register."]
    #[inline(always)]
    pub const fn lclutwr(self) -> crate::common::Reg<regs::Lclutwr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "Active Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awcr(pub u32);
    impl Awcr {
        #[doc = "Accumulated Active Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub const fn aah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Active Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub fn set_aah(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)."]
        #[inline(always)]
        pub const fn aaw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)."]
        #[inline(always)]
        pub fn set_aaw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awcr {
        #[inline(always)]
        fn default() -> Awcr {
            Awcr(0)
        }
    }
    #[doc = "Background Color Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bccr(pub u32);
    impl Bccr {
        #[doc = "Background Color Blue value."]
        #[inline(always)]
        pub const fn bcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Blue value."]
        #[inline(always)]
        pub fn set_bcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Background Color Green value."]
        #[inline(always)]
        pub const fn bcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Green value."]
        #[inline(always)]
        pub fn set_bcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Background Color Red value."]
        #[inline(always)]
        pub const fn bcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Background Color Red value."]
        #[inline(always)]
        pub fn set_bcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bccr {
        #[inline(always)]
        fn default() -> Bccr {
            Bccr(0)
        }
    }
    #[doc = "Back Porch Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcr(pub u32);
    impl Bpcr {
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)."]
        #[inline(always)]
        pub const fn avbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)."]
        #[inline(always)]
        pub fn set_avbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)."]
        #[inline(always)]
        pub const fn ahbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)."]
        #[inline(always)]
        pub fn set_ahbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Bpcr {
        #[inline(always)]
        fn default() -> Bpcr {
            Bpcr(0)
        }
    }
    #[doc = "Current Display Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdsr(pub u32);
    impl Cdsr {
        #[doc = "Vertical Data Enable display Status."]
        #[inline(always)]
        pub const fn vdes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Data Enable display Status."]
        #[inline(always)]
        pub fn set_vdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal Data Enable display Status."]
        #[inline(always)]
        pub const fn hdes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Data Enable display Status."]
        #[inline(always)]
        pub fn set_hdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical Synchronization display Status."]
        #[inline(always)]
        pub const fn vsyncs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Synchronization display Status."]
        #[inline(always)]
        pub fn set_vsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Horizontal Synchronization display Status."]
        #[inline(always)]
        pub const fn hsyncs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Synchronization display Status."]
        #[inline(always)]
        pub fn set_hsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cdsr {
        #[inline(always)]
        fn default() -> Cdsr {
            Cdsr(0)
        }
    }
    #[doc = "Current Position Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpsr(pub u32);
    impl Cpsr {
        #[doc = "Current Y Position."]
        #[inline(always)]
        pub const fn cypos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current Y Position."]
        #[inline(always)]
        pub fn set_cypos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current X Position."]
        #[inline(always)]
        pub const fn cxpos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current X Position."]
        #[inline(always)]
        pub fn set_cxpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cpsr {
        #[inline(always)]
        fn default() -> Cpsr {
            Cpsr(0)
        }
    }
    #[doc = "Global Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "LCD-TFT controller enable bit."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD-TFT controller enable bit."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Pixel Clock Polarity."]
        #[inline(always)]
        pub const fn pcpol(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel Clock Polarity."]
        #[inline(always)]
        pub fn set_pcpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Data Enable Polarity."]
        #[inline(always)]
        pub const fn depol(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Data Enable Polarity."]
        #[inline(always)]
        pub fn set_depol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Vertical Synchronization Polarity."]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Synchronization Polarity."]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Horizontal Synchronization Polarity."]
        #[inline(always)]
        pub const fn hspol(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Synchronization Polarity."]
        #[inline(always)]
        pub fn set_hspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clears the Line Interrupt Flag."]
        #[inline(always)]
        pub const fn clif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the Line Interrupt Flag."]
        #[inline(always)]
        pub fn set_clif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag."]
        #[inline(always)]
        pub const fn cfuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag."]
        #[inline(always)]
        pub fn set_cfuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn crrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clears Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Enable."]
        #[inline(always)]
        pub fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt Enable."]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt Enable."]
        #[inline(always)]
        pub fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Register Reload interrupt enable."]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload interrupt enable."]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Line Interrupt flag."]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt flag."]
        #[inline(always)]
        pub fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt flag."]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt flag."]
        #[inline(always)]
        pub fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Flag."]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "Layerx Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L1cr(pub u32);
    impl L1cr {
        #[doc = "Layer Enable."]
        #[inline(always)]
        pub const fn len(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer Enable."]
        #[inline(always)]
        pub fn set_len(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Keying Enable."]
        #[inline(always)]
        pub const fn colken(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color Keying Enable."]
        #[inline(always)]
        pub fn set_colken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Color Look-Up Table Enable."]
        #[inline(always)]
        pub const fn cluten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Color Look-Up Table Enable."]
        #[inline(always)]
        pub fn set_cluten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for L1cr {
        #[inline(always)]
        fn default() -> L1cr {
            L1cr(0)
        }
    }
    #[doc = "Layerx Color Frame Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcfbar(pub u32);
    impl Lcfbar {
        #[doc = "Color Frame Buffer Start Address."]
        #[inline(always)]
        pub const fn cfbadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Color Frame Buffer Start Address."]
        #[inline(always)]
        pub fn set_cfbadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lcfbar {
        #[inline(always)]
        fn default() -> Lcfbar {
            Lcfbar(0)
        }
    }
    #[doc = "Layerx ColorFrame Buffer Line Number Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcfblnr(pub u32);
    impl Lcfblnr {
        #[doc = "Frame Buffer Line Number."]
        #[inline(always)]
        pub const fn cfblnbr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frame Buffer Line Number."]
        #[inline(always)]
        pub fn set_cfblnbr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Lcfblnr {
        #[inline(always)]
        fn default() -> Lcfblnr {
            Lcfblnr(0)
        }
    }
    #[doc = "Layerx Color Frame Buffer Length Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcfblr(pub u32);
    impl Lcfblr {
        #[doc = "Color Frame Buffer Line Length."]
        #[inline(always)]
        pub const fn cfbll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Line Length."]
        #[inline(always)]
        pub fn set_cfbll(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Color Frame Buffer Pitch in bytes."]
        #[inline(always)]
        pub const fn cfbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Pitch in bytes."]
        #[inline(always)]
        pub fn set_cfbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Lcfblr {
        #[inline(always)]
        fn default() -> Lcfblr {
            Lcfblr(0)
        }
    }
    #[doc = "Layerx Color Keying Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckcr(pub u32);
    impl Lckcr {
        #[doc = "Color Key Blue value."]
        #[inline(always)]
        pub const fn ckblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Blue value."]
        #[inline(always)]
        pub fn set_ckblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Color Key Green value."]
        #[inline(always)]
        pub const fn ckgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Green value."]
        #[inline(always)]
        pub fn set_ckgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Color Key Red value."]
        #[inline(always)]
        pub const fn ckred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Red value."]
        #[inline(always)]
        pub fn set_ckred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lckcr {
        #[inline(always)]
        fn default() -> Lckcr {
            Lckcr(0)
        }
    }
    #[doc = "Layerx CLUT Write Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lclutwr(pub u32);
    impl Lclutwr {
        #[doc = "Blue value."]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue value."]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green value."]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green value."]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red value."]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red value."]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "CLUT Address."]
        #[inline(always)]
        pub const fn clutadd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT Address."]
        #[inline(always)]
        pub fn set_clutadd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Lclutwr {
        #[inline(always)]
        fn default() -> Lclutwr {
            Lclutwr(0)
        }
    }
    #[doc = "Layerx Default Color Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ldccr(pub u32);
    impl Ldccr {
        #[doc = "Default Color Blue."]
        #[inline(always)]
        pub const fn dcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Blue."]
        #[inline(always)]
        pub fn set_dcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Default Color Green."]
        #[inline(always)]
        pub const fn dcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Green."]
        #[inline(always)]
        pub fn set_dcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Default Color Red."]
        #[inline(always)]
        pub const fn dcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Red."]
        #[inline(always)]
        pub fn set_dcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ldccr {
        #[inline(always)]
        fn default() -> Ldccr {
            Ldccr(0)
        }
    }
    #[doc = "Line Interrupt Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lipcr(pub u32);
    impl Lipcr {
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Line Interrupt Position."]
        #[inline(always)]
        pub fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Lipcr {
        #[inline(always)]
        fn default() -> Lipcr {
            Lipcr(0)
        }
    }
    #[doc = "Layerx Pixel Format Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpfcr(pub u32);
    impl Lpfcr {
        #[doc = "Pixel Format."]
        #[inline(always)]
        pub const fn pf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Pixel Format."]
        #[inline(always)]
        pub fn set_pf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Lpfcr {
        #[inline(always)]
        fn default() -> Lpfcr {
            Lpfcr(0)
        }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lwhpcr(pub u32);
    impl Lwhpcr {
        #[doc = "Window Horizontal Start Position."]
        #[inline(always)]
        pub const fn whstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Start Position."]
        #[inline(always)]
        pub fn set_whstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Window Horizontal Stop Position."]
        #[inline(always)]
        pub const fn whsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Stop Position."]
        #[inline(always)]
        pub fn set_whsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Lwhpcr {
        #[inline(always)]
        fn default() -> Lwhpcr {
            Lwhpcr(0)
        }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lwvpcr(pub u32);
    impl Lwvpcr {
        #[doc = "Window Vertical Start Position."]
        #[inline(always)]
        pub const fn wvstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Start Position."]
        #[inline(always)]
        pub fn set_wvstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Window Vertical Stop Position."]
        #[inline(always)]
        pub const fn wvsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Stop Position."]
        #[inline(always)]
        pub fn set_wvsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Lwvpcr {
        #[inline(always)]
        fn default() -> Lwvpcr {
            Lwvpcr(0)
        }
    }
    #[doc = "Shadow Reload Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcr(pub u32);
    impl Srcr {
        #[doc = "Immediate Reload."]
        #[inline(always)]
        pub const fn imr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Immediate Reload."]
        #[inline(always)]
        pub fn set_imr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vertical Blanking Reload."]
        #[inline(always)]
        pub const fn vbr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Blanking Reload."]
        #[inline(always)]
        pub fn set_vbr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Srcr {
        #[inline(always)]
        fn default() -> Srcr {
            Srcr(0)
        }
    }
    #[doc = "Synchronization Size Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscr(pub u32);
    impl Sscr {
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub const fn vsh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub fn set_vsh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)."]
        #[inline(always)]
        pub const fn hsw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)."]
        #[inline(always)]
        pub fn set_hsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Sscr {
        #[inline(always)]
        fn default() -> Sscr {
            Sscr(0)
        }
    }
    #[doc = "Total Width Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twcr(pub u32);
    impl Twcr {
        #[doc = "Total Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub const fn totalh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Total Height (in units of horizontal scan line)."]
        #[inline(always)]
        pub fn set_totalh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Total Width (in units of pixel clock period)."]
        #[inline(always)]
        pub const fn totalw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Total Width (in units of pixel clock period)."]
        #[inline(always)]
        pub fn set_totalw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Twcr {
        #[inline(always)]
        fn default() -> Twcr {
            Twcr(0)
        }
    }
}
