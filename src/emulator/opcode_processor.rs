extern crate rand;

use emulator::memory::{Registers, Stack, Memory};
use emulator::display::TDisplay;

#[derive(Debug, PartialEq)]
pub struct OpCode {
    opcode: u16,
    nnn: u16,
    nn: u8,
    n: u8,
    x: u8,
    y: u8,
}

impl OpCode {
    pub fn from_data(data: u16) -> OpCode {
        OpCode {
            opcode: data,
            nnn: data & 0x0fff,
            nn: (data & 0x00ff) as u8,
            n: (data & 0x000f) as u8,
            x: ((data & 0x0f00) >> 8) as u8,
            y: ((data & 0x00f0) >> 4) as u8,
        }
    }

    pub fn get_raw(&self) -> u16 {
        self.opcode
    }

    pub fn get_address(&self) -> u16 {
        self.nnn
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_short_address(&self) -> u8 {
        self.nn
    }
}

pub trait TOpCodesProcessor {
    fn clear_screen(&self, &mut TDisplay);
    fn return_from_subroutine(&self, stack: &mut Stack, program_counter: &mut u16);
    fn jump_to_address(&self, program_counter: &mut u16, address: u16);
    fn call_subroutine(&self, program_counter: &mut u16, address: u16, stack: &mut Stack);
    fn cond_vx_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8);
    fn cond_vx_not_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8);
    fn cond_vx_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8);
    fn const_vx_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8);
    fn const_vx_plus_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8);
    fn assign_vx_equal_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vx_or_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vx_and_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vx_xor_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn math_vx_equal_vx_plus_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn math_vx_equal_vx_minus_vy(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vy_shr(&self, registers: &mut Registers, x: u8, y: u8);
    fn math_vx_equal_vy_minus_vx(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vy_shl(&self, registers: &mut Registers, x: u8, y: u8);
    fn cond_vx_not_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8);
    fn mem_i_equal_nnn(&self, address_register: &mut u16, nnn: u16);
    fn flow_pc_equal_v0_plus_nnn(&self, program_counter: &mut u16, nnn: u16);
    fn rand_vx_equal_rand_and_nn(&self, registers: &mut Registers, x: u8, nn: u8);
    fn draw_vx_vy_n(&self, x: u8, y: u8, n: u8, display: &mut TDisplay, memory: &Memory, address_register: &u16);
    fn mem_i_equal_i_plus_vx(&self, registers: &mut Registers, address_register: &mut u16, x: u8);
    fn mem_i_equal_sprite_addr_vx(&self, registers: &Registers, address_register: &mut u16, x: u8);
    fn mem_bcd(&self, registers: &Registers, address_register: &u16, memory: &mut Memory, x: u8);
    fn mem_reg_dump(&self, registers: &Registers, memory: &mut Memory, address_register: &mut u16, x: u8);
    fn mem_reg_load(&self, registers: &mut Registers, memory: &Memory, address_register: &mut u16, x: u8);
}

pub struct OpCodesProcessor {}

impl OpCodesProcessor {
    pub fn new() -> Self {
        OpCodesProcessor {}
    }
}

impl TOpCodesProcessor for OpCodesProcessor {
    fn clear_screen(&self, display: &mut TDisplay) {
        display.clear();
    }

    fn return_from_subroutine(&self, stack: &mut Stack, program_counter: &mut u16) {
        *program_counter = stack.pop();
    }

    fn jump_to_address(&self, program_counter: &mut u16, address: u16) {
        *program_counter = address;
    }

    fn call_subroutine(&self, program_counter: &mut u16, address: u16, stack: &mut Stack) {
        stack.push(*program_counter);
        *program_counter = address;
    }

    fn cond_vx_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8) {
        if registers.get_register_at(x as usize) != nn {
            return;
        }

        *program_counter += 2;
    }

    fn cond_vx_not_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8) {
        if registers.get_register_at(x as usize) == nn {
            return;
        }

        *program_counter += 2;
    }

    fn cond_vx_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
        if registers.get_register_at(x as usize) != registers.get_register_at(y as usize) {
            return;
        }

        *program_counter += 2;
    }
    
    fn const_vx_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
        registers.set_register_at(x as usize, nn);
    }

    fn const_vx_plus_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
        let old_x = registers.get_register_at(x as usize);
        registers.set_register_at(x as usize, old_x + nn);
    }

    fn assign_vx_equal_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vy = registers.get_register_at(y as usize);
        registers.set_register_at(x as usize, vy);
    }

    fn bitop_vx_equal_vx_or_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        registers.set_register_at(x as usize, vx | vy);
    }

    fn bitop_vx_equal_vx_and_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        registers.set_register_at(x as usize, vx & vy);
    }

    fn bitop_vx_equal_vx_xor_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        registers.set_register_at(x as usize, vx ^ vy);
    }

    fn math_vx_equal_vx_plus_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        let result = vx as u16 + vy as u16;
        if result > 0xff {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }
        registers.set_register_at(x as usize, result as u8);
    }

    fn math_vx_equal_vx_minus_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        let result = vx as i8 - vy as i8;
        if vx > vy {
            registers.set_register_at(0xf, 0x1);
            registers.set_register_at(x as usize, result as u8);
        } else {
            registers.set_register_at(0xf, 0x0);
            registers.set_register_at(x as usize, (result * -1) as u8);
        }
    }

    fn bitop_vx_equal_vy_shr(&self, registers: &mut Registers, x: u8, y: u8) {
        let vy = registers.get_register_at(y as usize);
        let result = vy >> 1;

        if vy & 0b00000001 == 0x1 {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }

        registers.set_register_at(x as usize, result);
        registers.set_register_at(y as usize, result);
    }

    fn math_vx_equal_vy_minus_vx(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        let result = vy as i8 - vx as i8;
        if vy > vx {
            registers.set_register_at(0xf, 0x1);
            registers.set_register_at(x as usize, result as u8);
        } else {
            registers.set_register_at(0xf, 0x0);
            registers.set_register_at(x as usize, (result * -1) as u8);
        }
    }

    fn bitop_vx_equal_vy_shl(&self, registers: &mut Registers, x: u8, y: u8) {
        let vy = registers.get_register_at(y as usize);
        let result = vy << 1;

        if vy & 0b10000000 == 0x80 {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }

        registers.set_register_at(x as usize, result);
        registers.set_register_at(y as usize, result);
    }

    fn cond_vx_not_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
        if registers.get_register_at(x as usize) == registers.get_register_at(y as usize) {
            return;
        }

        *program_counter += 2;
    }

    fn mem_i_equal_nnn(&self, address_register: &mut u16, nnn: u16) {
        *address_register = nnn;
    }

    fn flow_pc_equal_v0_plus_nnn(&self, program_counter: &mut u16, nnn: u16) {
        *program_counter = nnn;
    }

    fn rand_vx_equal_rand_and_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
        registers.set_register_at(x as usize, rand::random::<u8>() & nn);
    }

    fn draw_vx_vy_n(&self, x: u8, y: u8, n: u8, display: &mut TDisplay, memory: &Memory, address_register: &u16) {
        display.draw_sprite(x, y, n, address_register, memory);
    }

    fn mem_i_equal_i_plus_vx(&self, registers: &mut Registers, address_register: &mut u16, x: u8) {
        let vx = registers.get_register_at(x as usize);

        let result: u32 = *address_register as u32 + vx as u32;
        if result > 0xffff {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }

        *address_register = result as u16;
    }

    fn mem_i_equal_sprite_addr_vx(&self, registers: &Registers, address_register: &mut u16, x:u8) {
        let x = registers.get_register_at(x as usize);

        if x > 0xf {
            panic!(format!("Font cannot be greater than 0xf but {:x} given", x));
        }

        *address_register = (0x5 * x) as u16;
    }

    fn mem_bcd(&self, registers: &Registers, address_register: &u16, memory: &mut Memory, x: u8) {
        let x = registers.get_register_at(x as usize);

        let hundreds: u8 = ((x as f32) / 100.0).floor() as u8;
        let tens: u8 = ((x - hundreds * 100) as f32 / 10.0).floor() as u8;
        let ones: u8 = x - (hundreds * 100) - (tens * 10);

        memory.write(*address_register, hundreds);
        memory.write(*address_register + 0x1, tens);
        memory.write(*address_register + 0x2, ones);
    }

    fn mem_reg_dump(&self, registers: &Registers, memory: &mut Memory, address_register: &mut u16, x: u8) {
        for z in 0x0..x+0x1 {
            memory.write(*address_register, registers.get_register_at(z as usize));
            *address_register += 1;
        }
    }

    fn mem_reg_load(&self, registers: &mut Registers, memory: &Memory, address_register: &mut u16, x: u8) {
        for z in 0x0..x+0x1 {
            registers.set_register_at(z as usize, memory.read(*address_register));
            *address_register += 1;
        }
    }
}


#[cfg(test)]
mod test_opcode {
    use super::OpCode;

    #[test]
    fn test_get_raw() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x1456, opcode.get_raw());
    }

    #[test]
    fn test_get_address() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x0456, opcode.get_address());
    }

    #[test]
    fn test_get_short_address() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x56, opcode.get_short_address());
    }

    #[test]
    fn test_get_x() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x4, opcode.get_x());
    }

    #[test]
    fn test_get_y() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x5, opcode.get_y());
    }
}

#[cfg(test)]
mod test_opcodes_processor {
    use super::*;
    use emulator::memory::{Memory, Stack, Registers};
    use emulator::display::TDisplay;

    struct MockedDisplay {
        draw_sprite_called: bool,
        clear_called: bool,
    }

    impl MockedDisplay {
        fn new() -> Self {
            MockedDisplay {
                draw_sprite_called: false,
                clear_called: false,
            }
        }
    }

    impl TDisplay for MockedDisplay {
        fn draw_sprite(&mut self, x: u8, y: u8, rows: u8, address_register: &u16, memory: &Memory) -> bool {
            self.draw_sprite_called = true;

            true
        }

        fn clear(&mut self) {
            self.clear_called = true;
        }
    }

    #[test]
    fn test_clear_display() {
        let mut display = MockedDisplay::new();

        OpCodesProcessor::new().clear_screen(&mut display);

        assert!(display.clear_called);
    }

    #[test]
    fn test_return_from_subroutine() {
        let mut program_counter: u16 = 0x100;
        let mut stack = Stack::new();

        stack.push(program_counter);
        program_counter += 1;

        OpCodesProcessor::new().return_from_subroutine(&mut stack, &mut program_counter);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_jump_to_address() {
        let mut memory = Memory::new();
        let mut program_counter: u16 = 0;

        memory.write(0x100, 0x5);

        OpCodesProcessor::new().jump_to_address(&mut program_counter, 0x100);

        assert_eq!(0x5, memory.read(program_counter));
    }

    #[test]
    fn test_call_subroutine() {
        let mut stack = Stack::new();
        let mut program_counter = 0x100;

        OpCodesProcessor::new().call_subroutine(&mut program_counter, 0x150, &mut stack);

        assert_eq!(0x150, program_counter);
        assert_eq!(0x100, stack.pop());
    }

    #[test]
    fn test_cond_vx_equal_nn_true() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, nn);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_equal_nn_false() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xaa);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_cond_vx_not_equal_nn_true() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xaa);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_not_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_not_equal_nn_false() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, nn);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_not_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_cond_vx_equal_vy_true() {
        let x: u8 = 0x2;
        let y: u8 = 0x3;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0xff);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_equal_vy_false() {
        let x: u8 = 0x2;
        let y: u8 = 0x3;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0xaa);

        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().cond_vx_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_const_vx_equal_nn() {
        let x: u8 = 0x2;
        let nn:u8 = 0x10;

        let mut registers = Registers::new();
        
        OpCodesProcessor::new().const_vx_equal_nn(&mut registers, x, nn);

        assert_eq!(0x10, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_const_vx_plus_equal_nn_ok() {
        let x: u8 = 0x2;
        let nn: u8 = 0x1;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x5);

        OpCodesProcessor::new().const_vx_plus_equal_nn(&mut registers, x, nn);

        assert_eq!(0x6, registers.get_register_at(x as usize));
    }

    #[test]
    #[should_panic]
    fn test_const_vx_plus_equal_nn_will_panic_on_overflow() {
        let x: u8 = 0x2;
        let nn: u8 = 0x1;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);

        OpCodesProcessor::new().const_vx_plus_equal_nn(&mut registers, x, nn);
    }

    #[test]
    fn test_assign_vx_equal_vy() {
        let x: u8 = 0x2;
        let y: u8 = 0x3;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x1);
        registers.set_register_at(y as usize, 0x2);

        OpCodesProcessor::new().assign_vx_equal_vy(&mut registers, x, y);

        assert_eq!(0x2, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_or_vy() {
        let (mut registers, x, y) = setup_bitop();

        OpCodesProcessor::new().bitop_vx_equal_vx_or_vy(&mut registers, x, y);

        assert_eq!(0x5f, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_and_vy() {
        let (mut registers, x, y) = setup_bitop();

        OpCodesProcessor::new().bitop_vx_equal_vx_and_vy(&mut registers, x, y);

        assert_eq!(0x40, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_xor_vy() {
        let (mut registers, x, y) = setup_bitop();

        OpCodesProcessor::new().bitop_vx_equal_vx_xor_vy(&mut registers, x, y);

        assert_eq!(0x1f, registers.get_register_at(x as usize));
    }

    fn setup_bitop() -> (Registers, u8, u8) {
        let x: u8 = 0x2;
        let y: u8 = 0x3;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x5a);
        registers.set_register_at(y as usize, 0x45);

        (registers, x, y)
    }

    #[test]
    fn test_math_vx_equal_vx_plus_vy_without_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xf);
        registers.set_register_at(y as usize, 0xa);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().math_vx_equal_vx_plus_vy(&mut registers, x, y);

        assert_eq!(0x19, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));

    }

    #[test]
    fn test_math_vx_equal_vx_plus_vy_with_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0x2);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().math_vx_equal_vx_plus_vy(&mut registers, x, y);

        assert_eq!(0x1, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vx_minus_vy_without_undeflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0x2);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().math_vx_equal_vx_minus_vy(&mut registers, x, y);

        assert_eq!(0xfd, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vx_minus_vy_with_undeflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x2);
        registers.set_register_at(y as usize, 0x5);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().math_vx_equal_vx_minus_vy(&mut registers, x, y);

        assert_eq!(0x3, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_bitop_vx_equal_vy_shr_without_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(y as usize, 0x2);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().bitop_vx_equal_vy_shr(&mut registers, x, y);

        assert_eq!(0x1, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(y as usize));
        assert_eq!(0x0, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vy_shr_with_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(y as usize, 0x21);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().bitop_vx_equal_vy_shr(&mut registers, x, y);

        assert_eq!(0x10, registers.get_register_at(x as usize));
        assert_eq!(0x10, registers.get_register_at(y as usize));
        assert_eq!(0x1, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_math_vx_equal_vy_minus_vx_without_underflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x2);
        registers.set_register_at(y as usize, 0xff);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().math_vx_equal_vy_minus_vx(&mut registers, x, y);

        assert_eq!(0xfd, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vy_minus_vx_with_with() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0x2);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().math_vx_equal_vy_minus_vx(&mut registers, x, y);

        assert_eq!(0xfd, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_bitop_vx_equal_vy_shl_with_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(y as usize, 0x81);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().bitop_vx_equal_vy_shl(&mut registers, x, y);

        assert_eq!(0x2, registers.get_register_at(x as usize));
        assert_eq!(0x2, registers.get_register_at(y as usize));
        assert_eq!(0x1, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vy_shl_without_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(y as usize, 0x2);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().bitop_vx_equal_vy_shl(&mut registers, x, y);

        assert_eq!(0x4, registers.get_register_at(x as usize));
        assert_eq!(0x4, registers.get_register_at(y as usize));
        assert_eq!(0x0, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_cond_vx_not_equal_vy_true() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut program_counter: u16 = 0x100;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0x0f);

        OpCodesProcessor::new().cond_vx_not_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_not_equal_vy_false() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut program_counter: u16 = 0x100;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0xff);

        OpCodesProcessor::new().cond_vx_not_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_mem_i_equal_nnn() {
        let nnn: u16 = 0x200;
        let mut address_register: u16 = 0x100;

        OpCodesProcessor::new().mem_i_equal_nnn(&mut address_register, nnn);

        assert_eq!(0x200, address_register);
    }

    #[test]
    fn test_flow_pc_equal_v0_plus_nnn() {
        let nnn: u16 = 0x200;
        let mut program_counter: u16 = 0x100;

        OpCodesProcessor::new().flow_pc_equal_v0_plus_nnn(&mut program_counter, nnn);

        assert_eq!(0x200, program_counter);
    }

    #[test]
    fn test_rand_vx_equal_rand_and_nn() {
        let x: u8 = 0x1;
        let nn: u8 = 0xff;

        let mut registers = Registers::new();

        OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&mut registers, x, nn);
        let x_1 = registers.get_register_at(x as usize);

        OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&mut registers, x, nn);
        let x_2 = registers.get_register_at(x as usize);

        OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&mut registers, x, nn);
        let x_3 = registers.get_register_at(x as usize);

        assert_ne!(x_1, x_2);
        assert_ne!(x_1, x_3);
        assert_ne!(x_2, x_3);
    }

    #[test]
    fn test_mem_i_equal_i_plus_vx_without_overflow() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0xff;
        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0xf);
        registers.set_register_at(0xf, 0x1);

        OpCodesProcessor::new().mem_i_equal_i_plus_vx(&mut registers, &mut address_register, x);

        assert_eq!(0x10e, address_register);
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_mem_i_equal_i_plus_vx_with_overflow() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0xffff;
        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0xf);
        registers.set_register_at(0xf, 0x0);

        OpCodesProcessor::new().mem_i_equal_i_plus_vx(&mut registers, &mut address_register, x);

        assert_eq!(0xe, address_register);
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_mem_i_equal_sprite_addr_vx_ok() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0;

        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0x4);
        OpCodesProcessor::new().mem_i_equal_sprite_addr_vx(&mut registers, &mut address_register, x);

        assert_eq!(0x14, address_register);
    }

    #[test]
    #[should_panic]
    fn test_mem_i_equal_sprite_addr_vx_out_of_range() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0;

        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0xa1);
        OpCodesProcessor::new().mem_i_equal_sprite_addr_vx(&mut registers, &mut address_register, x);
    }

    #[test]
    fn test_mem_bcd() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0;
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 253);
        OpCodesProcessor::new().mem_bcd(&registers, &address_register, &mut memory, x);

        assert_eq!(2, memory.read(address_register));
        assert_eq!(5, memory.read(address_register + 1));
        assert_eq!(3, memory.read(address_register + 2));

        let mut memory = Memory::new();
        registers.set_register_at(x as usize, 49);
        OpCodesProcessor::new().mem_bcd(&registers, &address_register, &mut memory, x);

        assert_eq!(0, memory.read(address_register));
        assert_eq!(4, memory.read(address_register + 1));
        assert_eq!(9, memory.read(address_register + 2));

        let mut memory = Memory::new();
        registers.set_register_at(x as usize, 7);
        OpCodesProcessor::new().mem_bcd(&registers, &address_register, &mut memory, x);

        assert_eq!(0, memory.read(address_register));
        assert_eq!(0, memory.read(address_register + 1));
        assert_eq!(7, memory.read(address_register + 2));
    }

    #[test]
    fn test_mem_reg_dump() {
        let x: u8 = 0xf;
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        let mut address_register: u16 = 0x0;

        for z in 0x0..0xf+0x1 {
            registers.set_register_at(z as usize, 0xf - z);
        }

        OpCodesProcessor::new().mem_reg_dump(&registers, &mut memory, &mut address_register, x);

        for z in 0x0..0xf+0x1 {
            assert_eq!(0xf - z, memory.read(z as u16));
        }
    }

    #[test]
    fn test_mem_reg_load() {
        let x: u8 = 0xf;
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        let mut address_register: u16 = 0x0;

        for z in 0x0..x+0x1 {
            memory.write(z as u16, 0xf - z);
        }

        OpCodesProcessor::new().mem_reg_load(&mut registers, &memory, &mut address_register, x);

        for z in 0x0..x+0x1 {
            assert_eq!(0xf - z, registers.get_register_at(z as usize));
        }
    }

    #[test]
    fn test_draw_vx_vy_n() {
        let mut memory = Memory::new();
        let address_register: u16 = 0x0;
        let mut display = MockedDisplay::new();

        OpCodesProcessor::new().draw_vx_vy_n(0, 0, 3, &mut display, &mut memory, &address_register);

        assert!(display.draw_sprite_called);
    }
}