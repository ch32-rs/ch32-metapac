#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PIOC registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pioc {
    ptr: *mut u8,
}
unsafe impl Send for Pioc {}
unsafe impl Sync for Pioc {}
impl Pioc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Indirectly addressed data read/write port."]
    #[inline(always)]
    pub const fn indir_port(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Indirectly addressed data read/write port 2."]
    #[inline(always)]
    pub const fn indir_port2(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Low byte of the program counter."]
    #[inline(always)]
    pub const fn prg_count(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "SFR_STATUS_REG."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "PIOC indirect address register."]
    #[inline(always)]
    pub const fn indir_addr(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PIOC timer0 count register."]
    #[inline(always)]
    pub const fn tmr0_count(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "PIOC timer0 control register."]
    #[inline(always)]
    pub const fn tmr0_ctrl(self) -> crate::common::Reg<regs::Tmr0Ctrl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "PIOC timer0 initial value register."]
    #[inline(always)]
    pub const fn tmr0_init(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "PIOC bit cycle register."]
    #[inline(always)]
    pub const fn bit_cycle(self) -> crate::common::Reg<regs::BitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PIOC indirect address register2."]
    #[inline(always)]
    pub const fn indir_addr2(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "PIOC port direction register."]
    #[inline(always)]
    pub const fn port_dir(self) -> crate::common::Reg<regs::PortDir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "PIOC port input/output register."]
    #[inline(always)]
    pub const fn port_io(self) -> crate::common::Reg<regs::PortIo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "PIOC bit configuration register."]
    #[inline(always)]
    pub const fn bit_config(self) -> crate::common::Reg<regs::BitConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PIOC system configuration register."]
    #[inline(always)]
    pub const fn sys_cfg(self) -> crate::common::Reg<regs::SysCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "PIOC control read register."]
    #[inline(always)]
    pub const fn ctrl_rd(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "PIOC control write register. SFR_CTRL_WR. master read-write, host read-only."]
    #[inline(always)]
    pub const fn ctrl_wr(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "PIOC data exchange register."]
    #[inline(always)]
    pub const fn data_exch(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "PIOC data register%s."]
    #[inline(always)]
    pub const fn data_reg(self, n: usize) -> crate::common::Reg<u8, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 1usize) as _) }
    }
}
pub mod regs {
    #[doc = "PIOC bit configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BitConfig(pub u8);
    impl BitConfig {
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cyc_cnt3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_bit_cyc_cnt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cyc_cnt4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_bit_cyc_cnt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cyc_cnt5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_bit_cyc_cnt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cyc_cnt6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The periodic timing state of the encoded bits."]
        #[inline(always)]
        pub const fn set_bit_cyc_cnt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Periodic state of the encoded bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cyc_tail(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Periodic state of the encoded bits."]
        #[inline(always)]
        pub const fn set_bit_cyc_tail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Pin input level sampling point selection."]
        #[must_use]
        #[inline(always)]
        pub const fn port_in_edge(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pin input level sampling point selection."]
        #[inline(always)]
        pub const fn set_port_in_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "How the coded bits are modulated."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_code_mod(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "How the coded bits are modulated."]
        #[inline(always)]
        pub const fn set_bit_code_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The transmission of the coded bits is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_tx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The transmission of the coded bits is enabled."]
        #[inline(always)]
        pub const fn set_bit_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for BitConfig {
        #[inline(always)]
        fn default() -> BitConfig {
            BitConfig(0)
        }
    }
    impl core::fmt::Debug for BitConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BitConfig")
                .field("bit_cyc_cnt3", &self.bit_cyc_cnt3())
                .field("bit_cyc_cnt4", &self.bit_cyc_cnt4())
                .field("bit_cyc_cnt5", &self.bit_cyc_cnt5())
                .field("bit_cyc_cnt6", &self.bit_cyc_cnt6())
                .field("bit_cyc_tail", &self.bit_cyc_tail())
                .field("port_in_edge", &self.port_in_edge())
                .field("bit_code_mod", &self.bit_code_mod())
                .field("bit_tx_en", &self.bit_tx_en())
                .finish()
        }
    }
    #[doc = "The transmission of the coded bits is enabled."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BitCycle(pub u8);
    impl BitCycle {
        #[doc = "BIT_CYCLE, coded-bit period in clocks minus one."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "BIT_CYCLE, coded-bit period in clocks minus one."]
        #[inline(always)]
        pub const fn set_bit_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
        #[doc = "The raw bit data of the encoded bit to be sent, which is a double-buffered structure."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_tx_o0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The raw bit data of the encoded bit to be sent, which is a double-buffered structure."]
        #[inline(always)]
        pub const fn set_bit_tx_o0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for BitCycle {
        #[inline(always)]
        fn default() -> BitCycle {
            BitCycle(0)
        }
    }
    impl core::fmt::Debug for BitCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BitCycle")
                .field("bit_cycle", &self.bit_cycle())
                .field("bit_tx_o0", &self.bit_tx_o0())
                .finish()
        }
    }
    #[doc = "The raw bit data of the encoded bit to be sent, which is a double-buffered structure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortDir(pub u8);
    impl PortDir {
        #[doc = "IO0 pin direction control."]
        #[must_use]
        #[inline(always)]
        pub const fn port_dir0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 pin direction control."]
        #[inline(always)]
        pub const fn set_port_dir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "IO1 pin direction control."]
        #[must_use]
        #[inline(always)]
        pub const fn port_dir1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 pin direction control."]
        #[inline(always)]
        pub const fn set_port_dir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "IO0 pin pull-up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_pu0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 pin pull-up enable."]
        #[inline(always)]
        pub const fn set_port_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IO1 pin pull-up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_pu1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 pin pull-up enable."]
        #[inline(always)]
        pub const fn set_port_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn port_mod0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_port_mod0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn port_mod1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_port_mod1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn port_mod2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_port_mod2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn port_mod3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Pin mode control, host side defines the purpose."]
        #[inline(always)]
        pub const fn set_port_mod3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for PortDir {
        #[inline(always)]
        fn default() -> PortDir {
            PortDir(0)
        }
    }
    impl core::fmt::Debug for PortDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortDir")
                .field("port_dir0", &self.port_dir0())
                .field("port_dir1", &self.port_dir1())
                .field("port_pu0", &self.port_pu0())
                .field("port_pu1", &self.port_pu1())
                .field("port_mod0", &self.port_mod0())
                .field("port_mod1", &self.port_mod1())
                .field("port_mod2", &self.port_mod2())
                .field("port_mod3", &self.port_mod3())
                .finish()
        }
    }
    #[doc = "Pin mode control, host side defines the purpose."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortIo(pub u8);
    impl PortIo {
        #[doc = "IO0 output data; Timer0 output overrides it when enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_out0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 output data; Timer0 output overrides it when enabled."]
        #[inline(always)]
        pub const fn set_port_out0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "IO1 output data."]
        #[must_use]
        #[inline(always)]
        pub const fn port_out1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 output data."]
        #[inline(always)]
        pub const fn set_port_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "IO0 input XOR PORT_OUT0, used for change detection."]
        #[must_use]
        #[inline(always)]
        pub const fn port_xor0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 input XOR PORT_OUT0, used for change detection."]
        #[inline(always)]
        pub const fn set_port_xor0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IO1 input XOR PORT_OUT1, used for change detection."]
        #[must_use]
        #[inline(always)]
        pub const fn port_xor1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 input XOR PORT_OUT1, used for change detection."]
        #[inline(always)]
        pub const fn set_port_xor1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Resolved IO0 input level."]
        #[must_use]
        #[inline(always)]
        pub const fn port_in0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Resolved IO0 input level."]
        #[inline(always)]
        pub const fn set_port_in0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Resolved IO1 input level."]
        #[must_use]
        #[inline(always)]
        pub const fn port_in1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Resolved IO1 input level."]
        #[inline(always)]
        pub const fn set_port_in1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Bit decoded from IO0 by the hardware codec."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_rx_i0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Bit decoded from IO0 by the hardware codec."]
        #[inline(always)]
        pub const fn set_bit_rx_i0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "IO1 input XOR IO0 input."]
        #[must_use]
        #[inline(always)]
        pub const fn port_in_xor(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 input XOR IO0 input."]
        #[inline(always)]
        pub const fn set_port_in_xor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for PortIo {
        #[inline(always)]
        fn default() -> PortIo {
            PortIo(0)
        }
    }
    impl core::fmt::Debug for PortIo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortIo")
                .field("port_out0", &self.port_out0())
                .field("port_out1", &self.port_out1())
                .field("port_xor0", &self.port_xor0())
                .field("port_xor1", &self.port_xor1())
                .field("port_in0", &self.port_in0())
                .field("port_in1", &self.port_in1())
                .field("bit_rx_i0", &self.bit_rx_i0())
                .field("port_in_xor", &self.port_in_xor())
                .finish()
        }
    }
    #[doc = "IO1 level change can wake or request an interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "ALU carry flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_c(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ALU carry flag."]
        #[inline(always)]
        pub const fn set_flag_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Software general-purpose bit X."]
        #[must_use]
        #[inline(always)]
        pub const fn gp_bit_x(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software general-purpose bit X."]
        #[inline(always)]
        pub const fn set_gp_bit_x(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ALU zero flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_z(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ALU zero flag."]
        #[inline(always)]
        pub const fn set_flag_z(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Software general-purpose bit Y."]
        #[must_use]
        #[inline(always)]
        pub const fn gp_bit_y(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Software general-purpose bit Y."]
        #[inline(always)]
        pub const fn set_gp_bit_y(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Timer0 timeout reset enable while waiting."]
        #[must_use]
        #[inline(always)]
        pub const fn en_tout_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer0 timeout reset enable while waiting."]
        #[inline(always)]
        pub const fn set_en_tout_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Return stack is non-empty."]
        #[must_use]
        #[inline(always)]
        pub const fn stack_used(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Return stack is non-empty."]
        #[inline(always)]
        pub const fn set_stack_used(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("flag_c", &self.flag_c())
                .field("gp_bit_x", &self.gp_bit_x())
                .field("flag_z", &self.flag_z())
                .field("gp_bit_y", &self.gp_bit_y())
                .field("en_tout_rst", &self.en_tout_rst())
                .field("stack_used", &self.stack_used())
                .finish()
        }
    }
    #[doc = "IO1 input XOR IO0 input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysCfg(pub u8);
    impl SysCfg {
        #[doc = "Global clock control for the eMCU."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_clk_gate(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global clock control for the eMCU."]
        #[inline(always)]
        pub const fn set_mst_clk_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Force an eMCU reset."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force an eMCU reset."]
        #[inline(always)]
        pub const fn set_mst_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Mode and output control switch for IO0."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_io_en0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mode and output control switch for IO0."]
        #[inline(always)]
        pub const fn set_mst_io_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Mode and output control switch for IO1."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_io_en1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Mode and output control switch for IO1."]
        #[inline(always)]
        pub const fn set_mst_io_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Configure information bit, software-defined."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_cfg_b4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Configure information bit, software-defined."]
        #[inline(always)]
        pub const fn set_mst_cfg_b4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SFR_CTRL_WR waiting to be read."]
        #[must_use]
        #[inline(always)]
        pub const fn data_mw_sr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SFR_CTRL_WR waiting to be read."]
        #[inline(always)]
        pub const fn set_data_mw_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SFR_CTRL_RD waiting to be read."]
        #[must_use]
        #[inline(always)]
        pub const fn data_sw_mr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SFR_CTRL_RD waiting to be read."]
        #[inline(always)]
        pub const fn set_data_sw_mr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Interrupt request activation."]
        #[must_use]
        #[inline(always)]
        pub const fn int_req(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt request activation."]
        #[inline(always)]
        pub const fn set_int_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SysCfg {
        #[inline(always)]
        fn default() -> SysCfg {
            SysCfg(0)
        }
    }
    impl core::fmt::Debug for SysCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysCfg")
                .field("mst_clk_gate", &self.mst_clk_gate())
                .field("mst_reset", &self.mst_reset())
                .field("mst_io_en0", &self.mst_io_en0())
                .field("mst_io_en1", &self.mst_io_en1())
                .field("mst_cfg_b4", &self.mst_cfg_b4())
                .field("data_mw_sr", &self.data_mw_sr())
                .field("data_sw_mr", &self.data_sw_mr())
                .field("int_req", &self.int_req())
                .finish()
        }
    }
    #[doc = "Interrupt request activation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr0Ctrl(pub u8);
    impl Tmr0Ctrl {
        #[doc = "Timer0 clock divider; 000=/1024, 001=/256, 010=/64, 011=/16, 100=/8, 101=/4, 110=/2, 111=/1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_freq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Timer0 clock divider; 000=/1024, 001=/256, 010=/64, 011=/16, 100=/8, 101=/4, 110=/2, 111=/1."]
        #[inline(always)]
        pub const fn set_tmr0_freq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "0=timer, 1=free-running PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0=timer, 1=free-running PWM."]
        #[inline(always)]
        pub const fn set_tmr0_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Host-visible general-purpose bit X; the eMCU sees SB_TMR0_OUT_EN on this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn gp_bit_x(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Host-visible general-purpose bit X; the eMCU sees SB_TMR0_OUT_EN on this bit."]
        #[inline(always)]
        pub const fn set_gp_bit_x(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Host-visible general-purpose bit Y; the eMCU sees SB_TMR0_ENABLE on this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn gp_bit_y(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Host-visible general-purpose bit Y; the eMCU sees SB_TMR0_ENABLE on this bit."]
        #[inline(always)]
        pub const fn set_gp_bit_y(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "IO0 level change can wake or request an interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn en_level0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO0 level change can wake or request an interrupt."]
        #[inline(always)]
        pub const fn set_en_level0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "IO1 level change can wake or request an interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn en_level1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO1 level change can wake or request an interrupt."]
        #[inline(always)]
        pub const fn set_en_level1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tmr0Ctrl {
        #[inline(always)]
        fn default() -> Tmr0Ctrl {
            Tmr0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Tmr0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmr0Ctrl")
                .field("tmr0_freq", &self.tmr0_freq())
                .field("tmr0_mode", &self.tmr0_mode())
                .field("gp_bit_x", &self.gp_bit_x())
                .field("gp_bit_y", &self.gp_bit_y())
                .field("en_level0", &self.en_level0())
                .field("en_level1", &self.en_level1())
                .finish()
        }
    }
}
