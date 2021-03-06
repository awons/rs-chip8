use crate::chipset::{RandomByteGenerator, INSTRUCTION_SIZE};
use crate::gpu::Gpu;
use crate::keyboard::{Key, Keyboard};
use crate::memory::{Memory, Registers, Stack};

use std::fmt;
use std::result;

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

    pub fn get_parts(&self) -> (u8, u8, u8, u8) {
        (((self.opcode & 0xf000) >> 12) as u8, self.x, self.y, self.n)
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

    pub fn get_n(&self) -> u8 {
        self.n
    }
}

impl fmt::LowerHex for OpCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        let string = format!("{:#06x}", self.opcode);
        formatter.write_str(&string)?;

        Ok(())
    }
}

pub trait OpCodesProcessor {
    fn clear_screen<G>(&self, _: &mut G)
    where
        G: Gpu;
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
    fn bitop_vx_equal_vx_shr(&self, registers: &mut Registers, x: u8);
    fn math_vx_equal_vy_minus_vx(&self, registers: &mut Registers, x: u8, y: u8);
    fn bitop_vx_equal_vx_shl(&self, registers: &mut Registers, x: u8);
    fn cond_vx_not_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8);
    fn mem_i_equal_nnn(&self, address_register: &mut u16, nnn: u16);
    fn flow_pc_equal_v0_plus_nnn(&self, program_counter: &mut u16, nnn: u16, registers: &Registers);
    fn rand_vx_equal_rand_and_nn(
        &self,
        generator: &RandomByteGenerator,
        registers: &mut Registers,
        x: u8,
        nn: u8,
    );
    fn draw_vx_vy_n<G>(
        &self,
        x: u8,
        y: u8,
        n: u8,
        display: &mut G,
        memory: &Memory,
        address_register: u16,
        registers: &mut Registers,
    ) where
        G: Gpu;
    fn mem_i_equal_i_plus_vx(&self, registers: &mut Registers, address_register: &mut u16, x: u8);
    fn mem_i_equal_sprite_addr_vx(&self, registers: &Registers, address_register: &mut u16, x: u8);
    fn mem_bcd(&self, registers: &Registers, address_register: u16, memory: &mut Memory, x: u8);
    fn mem_reg_dump(
        &self,
        registers: &Registers,
        memory: &mut Memory,
        address_register: u16,
        x: u8,
    );
    fn mem_reg_load(
        &self,
        registers: &mut Registers,
        memory: &Memory,
        address_register: u16,
        x: u8,
    );
    fn keyop_if_key_equal_vx<K>(
        &self,
        keyboard: &mut K,
        registers: &Registers,
        program_counter: &mut u16,
        x: u8,
    ) where
        K: Keyboard;
    fn keyop_if_key_not_equal_vx<K>(
        &self,
        keyboard: &mut K,
        registers: &Registers,
        program_counter: &mut u16,
        x: u8,
    ) where
        K: Keyboard;
    fn keyop_vx_equal_key<K>(
        &self,
        keyboard: &mut K,
        registers: &mut Registers,
        x: u8,
        program_counter: &mut u16,
    ) where
        K: Keyboard;
    fn timer_vx_equal_get_delay(&self, delay_timer: u8, registers: &mut Registers, x: u8);
    fn timer_delay_timer_equal_vx(&self, delay_timer: &mut u8, registers: &Registers, x: u8);
    fn sound_sound_timer_equal_vx(&self);
}

pub struct Chip8OpCodesProcessor {}

impl Chip8OpCodesProcessor {
    pub fn new() -> Self {
        Chip8OpCodesProcessor {}
    }
}

impl OpCodesProcessor for Chip8OpCodesProcessor {
    fn clear_screen<G>(&self, display: &mut G)
    where
        G: Gpu,
    {
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
        if registers.get_register_at(x as usize) == nn {
            *program_counter += INSTRUCTION_SIZE;
        }
    }

    fn cond_vx_not_equal_nn(
        &self,
        registers: &Registers,
        program_counter: &mut u16,
        x: u8,
        nn: u8,
    ) {
        if registers.get_register_at(x as usize) != nn {
            *program_counter += INSTRUCTION_SIZE;
        }
    }

    fn cond_vx_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
        if registers.get_register_at(x as usize) == registers.get_register_at(y as usize) {
            *program_counter += INSTRUCTION_SIZE;
        }
    }

    fn const_vx_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
        registers.set_register_at(x as usize, nn);
    }

    fn const_vx_plus_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
        let old_x = registers.get_register_at(x as usize);
        registers.set_register_at(x as usize, old_x.wrapping_add(nn));
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

        registers.set_register_at(x as usize, vx.wrapping_add(vy));

        if u16::from(vx) + u16::from(vy) > 0xff {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }
    }

    fn math_vx_equal_vx_minus_vy(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        registers.set_register_at(x as usize, vx.wrapping_sub(vy));

        if vx > vy {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }
    }

    fn bitop_vx_equal_vx_shr(&self, registers: &mut Registers, x: u8) {
        let vx = registers.get_register_at(x as usize);

        registers.set_register_at(0xf, vx & 0b0000_0001);
        registers.set_register_at(x as usize, vx >> 1);
    }

    fn math_vx_equal_vy_minus_vx(&self, registers: &mut Registers, x: u8, y: u8) {
        let vx = registers.get_register_at(x as usize);
        let vy = registers.get_register_at(y as usize);

        registers.set_register_at(x as usize, vy.wrapping_sub(vx));

        if vy > vx {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }
    }

    fn bitop_vx_equal_vx_shl(&self, registers: &mut Registers, x: u8) {
        let vx = registers.get_register_at(x as usize);

        if vx & 0b1000_0000 == 0b1000_0000 {
            registers.set_register_at(0xf, 0x1);
        } else {
            registers.set_register_at(0xf, 0x0);
        }

        registers.set_register_at(x as usize, vx << 1);
    }

    fn cond_vx_not_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
        if registers.get_register_at(x as usize) != registers.get_register_at(y as usize) {
            *program_counter += INSTRUCTION_SIZE;
        }
    }

    fn mem_i_equal_nnn(&self, address_register: &mut u16, nnn: u16) {
        *address_register = nnn;
    }

    fn flow_pc_equal_v0_plus_nnn(
        &self,
        program_counter: &mut u16,
        nnn: u16,
        registers: &Registers,
    ) {
        *program_counter = nnn + u16::from(registers.get_register_at(0));
    }

    fn rand_vx_equal_rand_and_nn(
        &self,
        generator: &RandomByteGenerator,
        registers: &mut Registers,
        x: u8,
        nn: u8,
    ) {
        registers.set_register_at(x as usize, generator.generate() & nn);
    }

    fn draw_vx_vy_n<G>(
        &self,
        vx: u8,
        vy: u8,
        n: u8,
        display: &mut G,
        memory: &Memory,
        address_register: u16,
        registers: &mut Registers,
    ) where
        G: Gpu,
    {
        let x = registers.get_register_at(vx as usize);
        let y = registers.get_register_at(vy as usize);
        let collision_detected = display.draw_sprite(x, y, n, address_register, memory);
        registers.set_register_at(0xf, collision_detected as u8);
    }

    fn mem_i_equal_i_plus_vx(&self, registers: &mut Registers, address_register: &mut u16, x: u8) {
        let vx = registers.get_register_at(x as usize);
        *address_register = *address_register + u16::from(vx);
    }

    fn mem_i_equal_sprite_addr_vx(&self, registers: &Registers, address_register: &mut u16, x: u8) {
        let x = registers.get_register_at(x as usize);

        if x > 0xf {
            panic!(format!("Font cannot be greater than 0xf but {:x} given", x));
        }

        *address_register = u16::from(0x5 * x);
    }

    fn mem_bcd(&self, registers: &Registers, address_register: u16, memory: &mut Memory, x: u8) {
        let x = registers.get_register_at(x as usize);

        let hundreds: u8 = (f32::from(x) / 100.0).floor() as u8;
        let tens: u8 = (f32::from(x - hundreds * 100) / 10.0).floor() as u8;
        let ones: u8 = x - (hundreds * 100) - (tens * 10);

        memory.write(address_register, hundreds);
        memory.write(address_register + 0x1, tens);
        memory.write(address_register + 0x2, ones);
    }

    fn mem_reg_dump(
        &self,
        registers: &Registers,
        memory: &mut Memory,
        address_register: u16,
        x: u8,
    ) {
        let mut counter = address_register;
        for z in 0x0..=x {
            memory.write(counter, registers.get_register_at(z as usize));
            counter += 1;
        }
    }

    fn mem_reg_load(
        &self,
        registers: &mut Registers,
        memory: &Memory,
        address_register: u16,
        x: u8,
    ) {
        let mut counter = address_register;
        for z in 0x0..=x {
            registers.set_register_at(z as usize, memory.read(counter));
            counter += 1;
        }
    }

    fn keyop_if_key_equal_vx<K>(
        &self,
        keyboard: &mut K,
        registers: &Registers,
        program_counter: &mut u16,
        x: u8,
    ) where
        K: Keyboard,
    {
        if let Some(key) = keyboard.get_pressed_key() {
            match key {
                Key::KeyESC => *program_counter = u16::max_value() - 2,
                key => {
                    if registers.get_register_at(x as usize) == key as u8 {
                        *program_counter += INSTRUCTION_SIZE;
                    }
                }
            }
        }
    }

    fn keyop_if_key_not_equal_vx<K>(
        &self,
        keyboard: &mut K,
        registers: &Registers,
        program_counter: &mut u16,
        x: u8,
    ) where
        K: Keyboard,
    {
        match keyboard.get_pressed_key() {
            Some(key) => match key {
                Key::KeyESC => *program_counter = u16::max_value() - 2,
                key => {
                    if registers.get_register_at(x as usize) != key as u8 {
                        *program_counter += INSTRUCTION_SIZE;
                    }
                }
            },
            None => {
                *program_counter += INSTRUCTION_SIZE;
            }
        }
    }

    fn keyop_vx_equal_key<K>(
        &self,
        keyboard: &mut K,
        registers: &mut Registers,
        x: u8,
        program_counter: &mut u16,
    ) where
        K: Keyboard,
    {
        match keyboard.wait_for_key_press() {
            Key::KeyESC => *program_counter = u16::max_value() - 2,
            key => registers.set_register_at(x as usize, key as u8),
        }
    }

    fn timer_vx_equal_get_delay(&self, delay_timer: u8, registers: &mut Registers, x: u8) {
        registers.set_register_at(x as usize, delay_timer);
    }

    fn timer_delay_timer_equal_vx(&self, delay_timer: &mut u8, registers: &Registers, x: u8) {
        *delay_timer = registers.get_register_at(x as usize);
    }

    fn sound_sound_timer_equal_vx(&self) {
        //TODO Implement
    }
}

#[cfg(test)]
mod test_opcode {
    use super::OpCode;

    #[test]
    fn test_get_parts() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!((0x1, 0x4, 0x5, 0x6), opcode.get_parts());
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

    #[test]
    fn test_get_n() {
        let opcode = OpCode::from_data(0x1456);

        assert_eq!(0x6, opcode.get_n());
    }
}

#[cfg(test)]
mod test_opcodes_processor {
    use super::*;
    use crate::gpu::{Gpu, GraphicMemory};
    use crate::keyboard::{Key, Keyboard};
    use crate::memory::{Memory, Registers, Stack};
    use rand;

    struct MockedGpu {
        draw_sprite_called: bool,
        clear_called: bool,
        graphic_memory: GraphicMemory,
    }

    impl MockedGpu {
        fn new() -> Self {
            MockedGpu {
                draw_sprite_called: false,
                clear_called: false,
                graphic_memory: GraphicMemory::new(),
            }
        }
    }

    impl Gpu for MockedGpu {
        fn draw_sprite(
            &mut self,
            x: u8,
            _y: u8,
            _rows: u8,
            _address_register: u16,
            _memory: &Memory,
        ) -> bool {
            self.draw_sprite_called = true;

            if x == 10 {
                return false;
            } else if x == 11 {
                return true;
            }

            panic!("Should never be here");
        }

        fn clear(&mut self) {
            self.clear_called = true;
        }

        fn get_memory<'a>(&'a self) -> &GraphicMemory {
            &self.graphic_memory
        }
    }

    struct MockedKeyboard;
    impl Keyboard for MockedKeyboard {
        fn wait_for_key_press(&mut self) -> Key {
            Key::Key5
        }

        fn get_pressed_key(&mut self) -> Option<Key> {
            Some(Key::Key4)
        }
    }

    struct TestRandomByteGenerator {}
    impl RandomByteGenerator for TestRandomByteGenerator {
        fn generate(&self) -> u8 {
            rand::random::<u8>()
        }
    }

    #[test]
    fn test_clear_display() {
        let mut display = MockedGpu::new();

        Chip8OpCodesProcessor::new().clear_screen(&mut display);

        assert!(display.clear_called);
    }

    #[test]
    fn test_return_from_subroutine() {
        let mut program_counter: u16 = 0x100;
        let mut stack = Stack::new();

        stack.push(program_counter);
        program_counter += 1;

        Chip8OpCodesProcessor::new().return_from_subroutine(&mut stack, &mut program_counter);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_jump_to_address() {
        let mut memory = Memory::new();
        let mut program_counter: u16 = 0;

        memory.write(0x100, 0x5);

        Chip8OpCodesProcessor::new().jump_to_address(&mut program_counter, 0x100);

        assert_eq!(0x5, memory.read(program_counter));
    }

    #[test]
    fn test_call_subroutine() {
        let mut stack = Stack::new();
        let mut program_counter = 0x100;

        Chip8OpCodesProcessor::new().call_subroutine(&mut program_counter, 0x150, &mut stack);

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

        Chip8OpCodesProcessor::new().cond_vx_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_equal_nn_false() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xaa);

        let mut program_counter: u16 = 0x100;

        Chip8OpCodesProcessor::new().cond_vx_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_cond_vx_not_equal_nn_true() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xaa);

        let mut program_counter: u16 = 0x100;

        Chip8OpCodesProcessor::new().cond_vx_not_equal_nn(&registers, &mut program_counter, x, nn);

        assert_eq!(0x102, program_counter);
    }

    #[test]
    fn test_cond_vx_not_equal_nn_false() {
        let x: u8 = 0x2;
        let nn: u8 = 0xab;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, nn);

        let mut program_counter: u16 = 0x100;

        Chip8OpCodesProcessor::new().cond_vx_not_equal_nn(&registers, &mut program_counter, x, nn);

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

        Chip8OpCodesProcessor::new().cond_vx_equal_vy(&registers, &mut program_counter, x, y);

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

        Chip8OpCodesProcessor::new().cond_vx_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_const_vx_equal_nn() {
        let x: u8 = 0x2;
        let nn: u8 = 0x10;

        let mut registers = Registers::new();

        Chip8OpCodesProcessor::new().const_vx_equal_nn(&mut registers, x, nn);

        assert_eq!(0x10, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_const_vx_plus_equal_nn_ok() {
        let x: u8 = 0x2;
        let nn: u8 = 0x1;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x5);

        Chip8OpCodesProcessor::new().const_vx_plus_equal_nn(&mut registers, x, nn);

        assert_eq!(0x6, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_const_vx_plus_equal_nn_will_wrap_on_overflow() {
        let x = 0x2;
        let vx = 0x5;
        let nn = 0xff;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, vx);

        Chip8OpCodesProcessor::new().const_vx_plus_equal_nn(&mut registers, x, nn);

        assert_eq!(
            (u16::from(vx) % 256 + u16::from(nn) % 256) as u8,
            registers.get_register_at(x as usize)
        );
    }

    #[test]
    fn test_assign_vx_equal_vy() {
        let x: u8 = 0x2;
        let y: u8 = 0x3;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x1);
        registers.set_register_at(y as usize, 0x2);

        Chip8OpCodesProcessor::new().assign_vx_equal_vy(&mut registers, x, y);

        assert_eq!(0x2, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_or_vy() {
        let (mut registers, x, y) = setup_bitop();

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_or_vy(&mut registers, x, y);

        assert_eq!(0x5f, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_and_vy() {
        let (mut registers, x, y) = setup_bitop();

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_and_vy(&mut registers, x, y);

        assert_eq!(0x40, registers.get_register_at(x as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_xor_vy() {
        let (mut registers, x, y) = setup_bitop();

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_xor_vy(&mut registers, x, y);

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

        Chip8OpCodesProcessor::new().math_vx_equal_vx_plus_vy(&mut registers, x, y);

        assert_eq!(0x19, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vx_plus_vy_with_overflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0xff);
        registers.set_register_at(y as usize, 0x1);
        registers.set_register_at(0xf, 0x0);

        Chip8OpCodesProcessor::new().math_vx_equal_vx_plus_vy(&mut registers, x, y);

        assert_eq!(0x0, registers.get_register_at(x as usize));
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

        Chip8OpCodesProcessor::new().math_vx_equal_vx_minus_vy(&mut registers, x, y);

        assert_eq!(0xfd, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vx_minus_vy_with_undeflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x0);
        registers.set_register_at(y as usize, 0x1);
        registers.set_register_at(0xf, 0x1);

        Chip8OpCodesProcessor::new().math_vx_equal_vx_minus_vy(&mut registers, x, y);

        assert_eq!(0xff, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_bitop_vx_equal_vx_shr_without_overflow() {
        let x: u8 = 0x1;
        let before = 0b0101_1110;
        let after = 0b0010_1111;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, before);
        registers.set_register_at(0xf, 0x1);

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_shr(&mut registers, x);

        assert_eq!(after, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_shr_with_overflow() {
        let x: u8 = 0x1;
        let before = 0b1010_1111;
        let after = 0b0101_0111;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, before);
        registers.set_register_at(0xf, 0x0);

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_shr(&mut registers, x);

        assert_eq!(after, registers.get_register_at(x as usize));
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

        Chip8OpCodesProcessor::new().math_vx_equal_vy_minus_vx(&mut registers, x, y);

        assert_eq!(0xfd, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_math_vx_equal_vy_minus_vx_with_underflow() {
        let x: u8 = 0x1;
        let y: u8 = 0x2;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, 0x1);
        registers.set_register_at(y as usize, 0x0);
        registers.set_register_at(0xf, 0x1);

        Chip8OpCodesProcessor::new().math_vx_equal_vy_minus_vx(&mut registers, x, y);

        assert_eq!(0xff, registers.get_register_at(x as usize));
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_bitop_vx_equal_vx_shl_with_overflow() {
        let x: u8 = 0x1;
        let before = 0b1010_1111;
        let after = 0b0101_1110;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, before);
        registers.set_register_at(0xf, 0x0);

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_shl(&mut registers, x);

        assert_eq!(after, registers.get_register_at(x as usize));
        assert_eq!(0x1, registers.get_register_at(0xf as usize));
    }

    #[test]
    fn test_bitop_vx_equal_vx_shl_without_overflow() {
        let x: u8 = 0x1;
        let before = 0b0010_1111;
        let after = 0b0101_1110;

        let mut registers = Registers::new();
        registers.set_register_at(x as usize, before);
        registers.set_register_at(0xf, 0x1);

        Chip8OpCodesProcessor::new().bitop_vx_equal_vx_shl(&mut registers, x);

        assert_eq!(after, registers.get_register_at(x as usize));
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

        Chip8OpCodesProcessor::new().cond_vx_not_equal_vy(&registers, &mut program_counter, x, y);

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

        Chip8OpCodesProcessor::new().cond_vx_not_equal_vy(&registers, &mut program_counter, x, y);

        assert_eq!(0x100, program_counter);
    }

    #[test]
    fn test_mem_i_equal_nnn() {
        let nnn: u16 = 0x200;
        let mut address_register: u16 = 0x100;

        Chip8OpCodesProcessor::new().mem_i_equal_nnn(&mut address_register, nnn);

        assert_eq!(0x200, address_register);
    }

    #[test]
    fn test_flow_pc_equal_v0_plus_nnn() {
        let nnn: u16 = 0x200;
        let mut program_counter: u16 = 0x100;

        let mut registers = Registers::new();
        registers.set_register_at(0, 0xff);

        Chip8OpCodesProcessor::new().flow_pc_equal_v0_plus_nnn(
            &mut program_counter,
            nnn,
            &registers,
        );

        assert_eq!(0x2ff, program_counter);
    }

    #[test]
    fn test_rand_vx_equal_rand_and_nn() {
        let x: u8 = 0x1;
        let nn: u8 = 0xff;

        let mut registers = Registers::new();

        let generator = TestRandomByteGenerator {};
        Chip8OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&generator, &mut registers, x, nn);
        let x_1 = registers.get_register_at(x as usize);

        Chip8OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&generator, &mut registers, x, nn);
        let x_2 = registers.get_register_at(x as usize);

        Chip8OpCodesProcessor::new().rand_vx_equal_rand_and_nn(&generator, &mut registers, x, nn);
        let x_3 = registers.get_register_at(x as usize);

        assert_ne!(x_1, x_2);
        assert_ne!(x_1, x_3);
        assert_ne!(x_2, x_3);
    }

    #[test]
    fn test_mem_i_equal_i_plus_vx() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0xff;
        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0xf);

        Chip8OpCodesProcessor::new().mem_i_equal_i_plus_vx(
            &mut registers,
            &mut address_register,
            x,
        );

        assert_eq!(0x10e, address_register);
    }

    #[test]
    fn test_mem_i_equal_sprite_addr_vx_ok() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0;

        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0x4);
        Chip8OpCodesProcessor::new().mem_i_equal_sprite_addr_vx(
            &mut registers,
            &mut address_register,
            x,
        );

        assert_eq!(0x14, address_register);
    }

    #[test]
    #[should_panic]
    fn test_mem_i_equal_sprite_addr_vx_out_of_range() {
        let x: u8 = 0x1;
        let mut address_register: u16 = 0;

        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 0xa1);
        Chip8OpCodesProcessor::new().mem_i_equal_sprite_addr_vx(
            &mut registers,
            &mut address_register,
            x,
        );
    }

    #[test]
    fn test_mem_bcd() {
        let x: u8 = 0x1;
        let address_register: u16 = 0;
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        registers.set_register_at(x as usize, 253);
        Chip8OpCodesProcessor::new().mem_bcd(&registers, address_register, &mut memory, x);

        assert_eq!(2, memory.read(address_register));
        assert_eq!(5, memory.read(address_register + 1));
        assert_eq!(3, memory.read(address_register + 2));

        let mut memory = Memory::new();
        registers.set_register_at(x as usize, 49);
        Chip8OpCodesProcessor::new().mem_bcd(&registers, address_register, &mut memory, x);

        assert_eq!(0, memory.read(address_register));
        assert_eq!(4, memory.read(address_register + 1));
        assert_eq!(9, memory.read(address_register + 2));

        let mut memory = Memory::new();
        registers.set_register_at(x as usize, 7);
        Chip8OpCodesProcessor::new().mem_bcd(&registers, address_register, &mut memory, x);

        assert_eq!(0, memory.read(address_register));
        assert_eq!(0, memory.read(address_register + 1));
        assert_eq!(7, memory.read(address_register + 2));
    }

    #[test]
    fn test_mem_reg_dump() {
        let x: u8 = 0xf;
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        let address_register: u16 = 0x200;

        let range = (0x0..=0xf).collect::<Vec<u8>>();

        for i in &range {
            registers.set_register_at(*i as usize, i + 5);
        }

        Chip8OpCodesProcessor::new().mem_reg_dump(&registers, &mut memory, address_register, x);

        for i in range {
            assert_eq!(i + 5, memory.read(address_register + u16::from(i)));
        }
    }

    #[test]
    fn test_mem_reg_load() {
        let x: u8 = 0xf;
        let mut memory = Memory::new();
        let mut registers = Registers::new();
        let address_register: u16 = 0x200;

        let range = (address_register..=(address_register + u16::from(x))).collect::<Vec<u16>>();

        for (i, address) in range.iter().enumerate() {
            memory.write(*address, i as u8);
        }

        Chip8OpCodesProcessor::new().mem_reg_load(&mut registers, &memory, address_register, x);

        for (i, _) in range.iter().enumerate() {
            assert_eq!(i as u8, registers.get_register_at(i));
        }
    }

    #[test]
    fn test_draw_vx_vy_n_without_collision() {
        let mut memory = Memory::new();
        let address_register: u16 = 0x0;
        let mut display = MockedGpu::new();
        let mut registers = Registers::new();

        registers.set_register_at(0, 10);

        Chip8OpCodesProcessor::new().draw_vx_vy_n(
            0,
            1,
            3,
            &mut display,
            &mut memory,
            address_register,
            &mut registers,
        );

        assert!(display.draw_sprite_called);
        assert_eq!(0x0, registers.get_register_at(0xf));
    }

    #[test]
    fn test_draw_vx_vy_n_with_collision() {
        let mut memory = Memory::new();
        let address_register: u16 = 0x0;
        let mut display = MockedGpu::new();
        let mut registers = Registers::new();

        registers.set_register_at(0, 11);

        Chip8OpCodesProcessor::new().draw_vx_vy_n(
            0,
            1,
            3,
            &mut display,
            &mut memory,
            address_register,
            &mut registers,
        );

        assert!(display.draw_sprite_called);
        assert_eq!(0x1, registers.get_register_at(0xf));
    }

    #[test]
    fn test_keyop_vx_equal_key() {
        let mut keyboard = MockedKeyboard {};
        let mut registers = Registers::new();
        let mut program_counter = 0;

        Chip8OpCodesProcessor::new().keyop_vx_equal_key(
            &mut keyboard,
            &mut registers,
            0x1,
            &mut program_counter,
        );

        assert_eq!(0x5, registers.get_register_at(0x1));
    }

    #[test]
    fn test_keyop_if_key_equal_vx() {
        let mut keyboard = MockedKeyboard {};
        let mut registers = Registers::new();
        let mut program_counter = 0x0;

        registers.set_register_at(0x1, 0x4);

        Chip8OpCodesProcessor::new().keyop_if_key_equal_vx(
            &mut keyboard,
            &mut registers,
            &mut program_counter,
            0x1,
        );

        assert_eq!(0x2, program_counter);
    }

    #[test]
    fn test_keyop_if_key_not_equal_vx() {
        let mut keyboard = MockedKeyboard {};
        let mut registers = Registers::new();
        let mut program_counter = 0x0;

        registers.set_register_at(0x1, 0x5);

        Chip8OpCodesProcessor::new().keyop_if_key_equal_vx(
            &mut keyboard,
            &mut registers,
            &mut program_counter,
            0x1,
        );

        assert_eq!(0x0, program_counter);
    }

    #[test]
    fn test_timer_vx_equal_get_delay() {
        let delay_timer = 0x20;
        let mut registers = Registers::new();

        Chip8OpCodesProcessor::new().timer_vx_equal_get_delay(delay_timer, &mut registers, 0xa);

        assert_eq!(0x20, registers.get_register_at(0xa));
    }

    #[test]
    fn test_timer_delay_timer_equal_vx() {
        let mut delay_timer = 0x20;
        let mut registers = Registers::new();

        registers.set_register_at(0xa, 0x30);

        Chip8OpCodesProcessor::new().timer_delay_timer_equal_vx(&mut delay_timer, &registers, 0xa);

        assert_eq!(0x30, delay_timer);
    }
}
