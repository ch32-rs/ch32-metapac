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
    #[doc = "SFR_STATUS_REG."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "PIOC indirect address register."]
    #[inline(always)]
    pub const fn indir_addr(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PIOC timer0 count register."]
    #[inline(always)]
    pub const fn tmr0_count(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "PIOC timer0 control register."]
    #[inline(always)]
    pub const fn tmr0_ctrl(self) -> crate::common::Reg<regs::Tmr0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "PIOC timer0 initial value register."]
    #[inline(always)]
    pub const fn tmr0_init(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "PIOC bit cycle register."]
    #[inline(always)]
    pub const fn bit_cycle(self) -> crate::common::Reg<regs::BitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PIOC indirect address register2."]
    #[inline(always)]
    pub const fn indir_addr2(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "PIOC port direction register."]
    #[inline(always)]
    pub const fn port_dir(self) -> crate::common::Reg<regs::PortDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "PIOC port input/output register."]
    #[inline(always)]
    pub const fn port_io(self) -> crate::common::Reg<regs::PortIo, crate::common::RW> {
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
        #[doc = "CYC_CNT3."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_cnt3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CYC_CNT3."]
        #[inline(always)]
        pub const fn set_cyc_cnt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "CYC_CNT4."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_cnt4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CYC_CNT4."]
        #[inline(always)]
        pub const fn set_cyc_cnt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "CYC_CNT5."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_cnt5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CYC_CNT5."]
        #[inline(always)]
        pub const fn set_cyc_cnt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "CYC_CNT6."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_cnt6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CYC_CNT6."]
        #[inline(always)]
        pub const fn set_cyc_cnt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "CYC_TAIL."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_tail(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CYC_TAIL."]
        #[inline(always)]
        pub const fn set_cyc_tail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "IN_EDGE, 引脚输入电平采样时点选择."]
        #[must_use]
        #[inline(always)]
        pub const fn in_edge(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IN_EDGE, 引脚输入电平采样时点选择."]
        #[inline(always)]
        pub const fn set_in_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "CODE_MOD, 编码位的调制方式."]
        #[must_use]
        #[inline(always)]
        pub const fn code_mod(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CODE_MOD, 编码位的调制方式."]
        #[inline(always)]
        pub const fn set_code_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "TX_EN, 编码位的发送使能."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX_EN, 编码位的发送使能."]
        #[inline(always)]
        pub const fn set_tx_en(&mut self, val: bool) {
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
                .field("cyc_cnt3", &self.cyc_cnt3())
                .field("cyc_cnt4", &self.cyc_cnt4())
                .field("cyc_cnt5", &self.cyc_cnt5())
                .field("cyc_cnt6", &self.cyc_cnt6())
                .field("cyc_tail", &self.cyc_tail())
                .field("in_edge", &self.in_edge())
                .field("code_mod", &self.code_mod())
                .field("tx_en", &self.tx_en())
                .finish()
        }
    }
    #[doc = "PIOC bit cycle register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BitCycle(pub u8);
    impl BitCycle {
        #[doc = "CYCLE."]
        #[must_use]
        #[inline(always)]
        pub const fn cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "CYCLE."]
        #[inline(always)]
        pub const fn set_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
        }
        #[doc = "TX_O0."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_o0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX_O0."]
        #[inline(always)]
        pub const fn set_tx_o0(&mut self, val: bool) {
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
                .field("cycle", &self.cycle())
                .field("tx_o0", &self.tx_o0())
                .finish()
        }
    }
    #[doc = "PIOC port direction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortDir(pub u8);
    impl PortDir {
        #[doc = "DIR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dir0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DIR0."]
        #[inline(always)]
        pub const fn set_dir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DIR1."]
        #[must_use]
        #[inline(always)]
        pub const fn dir1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DIR1."]
        #[inline(always)]
        pub const fn set_dir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PU0."]
        #[must_use]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PU0."]
        #[inline(always)]
        pub const fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PU1."]
        #[must_use]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PU1."]
        #[inline(always)]
        pub const fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "MOD0."]
        #[must_use]
        #[inline(always)]
        pub const fn mod0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MOD0."]
        #[inline(always)]
        pub const fn set_mod0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "MOD1."]
        #[must_use]
        #[inline(always)]
        pub const fn mod1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MOD1."]
        #[inline(always)]
        pub const fn set_mod1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "MOD2."]
        #[must_use]
        #[inline(always)]
        pub const fn mod2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MOD2."]
        #[inline(always)]
        pub const fn set_mod2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "MOD3."]
        #[must_use]
        #[inline(always)]
        pub const fn mod3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MOD3."]
        #[inline(always)]
        pub const fn set_mod3(&mut self, val: bool) {
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
                .field("dir0", &self.dir0())
                .field("dir1", &self.dir1())
                .field("pu0", &self.pu0())
                .field("pu1", &self.pu1())
                .field("mod0", &self.mod0())
                .field("mod1", &self.mod1())
                .field("mod2", &self.mod2())
                .field("mod3", &self.mod3())
                .finish()
        }
    }
    #[doc = "PIOC port input/output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortIo(pub u8);
    impl PortIo {
        #[doc = "OUT0."]
        #[must_use]
        #[inline(always)]
        pub const fn out0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OUT0."]
        #[inline(always)]
        pub const fn set_out0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "OUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn out1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OUT1."]
        #[inline(always)]
        pub const fn set_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "XOR0."]
        #[must_use]
        #[inline(always)]
        pub const fn xor0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "XOR0."]
        #[inline(always)]
        pub const fn set_xor0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "XOR1."]
        #[must_use]
        #[inline(always)]
        pub const fn xor1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "XOR1."]
        #[inline(always)]
        pub const fn set_xor1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "IN0."]
        #[must_use]
        #[inline(always)]
        pub const fn in0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IN0."]
        #[inline(always)]
        pub const fn set_in0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "IN1."]
        #[must_use]
        #[inline(always)]
        pub const fn in1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IN1."]
        #[inline(always)]
        pub const fn set_in1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "BIT_RX_IO."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_rx_io(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "BIT_RX_IO."]
        #[inline(always)]
        pub const fn set_bit_rx_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "IN_XOR."]
        #[must_use]
        #[inline(always)]
        pub const fn in_xor(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IN_XOR."]
        #[inline(always)]
        pub const fn set_in_xor(&mut self, val: bool) {
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
                .field("out0", &self.out0())
                .field("out1", &self.out1())
                .field("xor0", &self.xor0())
                .field("xor1", &self.xor1())
                .field("in0", &self.in0())
                .field("in1", &self.in1())
                .field("bit_rx_io", &self.bit_rx_io())
                .field("in_xor", &self.in_xor())
                .finish()
        }
    }
    #[doc = "PIOC system configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysCfg(pub u8);
    impl SysCfg {
        #[doc = "CLK_GATE, eMCU CLK gate enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_gate(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLK_GATE, eMCU CLK gate enable."]
        #[inline(always)]
        pub const fn set_clk_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RESET."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "IO_EN0."]
        #[must_use]
        #[inline(always)]
        pub const fn io_en0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO_EN0."]
        #[inline(always)]
        pub const fn set_io_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "IO_EN1."]
        #[must_use]
        #[inline(always)]
        pub const fn io_en1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO_EN1."]
        #[inline(always)]
        pub const fn set_io_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "CFG_USER, user defined."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_user(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CFG_USER, user defined."]
        #[inline(always)]
        pub const fn set_cfg_user(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "DATA_MW_SR, master wating slave eMCU to read."]
        #[must_use]
        #[inline(always)]
        pub const fn data_mw_sr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DATA_MW_SR, master wating slave eMCU to read."]
        #[inline(always)]
        pub const fn set_data_mw_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "DATA_SW_MR, slave eMCU wating master to read."]
        #[must_use]
        #[inline(always)]
        pub const fn data_sw_mr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DATA_SW_MR, slave eMCU wating master to read."]
        #[inline(always)]
        pub const fn set_data_sw_mr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "INT_REQ, int request."]
        #[must_use]
        #[inline(always)]
        pub const fn int_req(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "INT_REQ, int request."]
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
                .field("clk_gate", &self.clk_gate())
                .field("reset", &self.reset())
                .field("io_en0", &self.io_en0())
                .field("io_en1", &self.io_en1())
                .field("cfg_user", &self.cfg_user())
                .field("data_mw_sr", &self.data_mw_sr())
                .field("data_sw_mr", &self.data_sw_mr())
                .field("int_req", &self.int_req())
                .finish()
        }
    }
    #[doc = "PIOC timer0 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr0Ctrl(pub u8);
    impl Tmr0Ctrl {
        #[doc = "FREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn freq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "FREQ."]
        #[inline(always)]
        pub const fn set_freq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "OUT_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn out_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OUT_EN."]
        #[inline(always)]
        pub const fn set_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "EN_LEVEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn en_level0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EN_LEVEL0."]
        #[inline(always)]
        pub const fn set_en_level0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "EN_LEVEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn en_level1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EN_LEVEL1."]
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
                .field("freq", &self.freq())
                .field("mode", &self.mode())
                .field("out_en", &self.out_en())
                .field("en", &self.en())
                .field("en_level0", &self.en_level0())
                .field("en_level1", &self.en_level1())
                .finish()
        }
    }
}
