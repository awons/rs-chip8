use emulator::memory::{Memory, Stack, Registers, MEMORY_SIZE};
use emulator::opcode_processor::{OpCode, TOpCodesProcessor};

const REGISTERS_NUMBER: usize = 16;
pub const REGISTER_VF: usize = 0xf;

pub const UPPER_FONT_BOUNDARY: u16 = 0x4f;
pub const PROGRAM_COUNTER_BOUNDARY: u16 = 0x200;

pub trait Chipset {
    fn get_memory(&self) -> &Memory;
    fn tick(&mut self);
    fn next_opcode(&mut self) -> Option<OpCode>;
}

pub struct Chip8Chipset<T:TOpCodesProcessor> {
    memory: Memory,
    registers: Registers,
    address_register: u16,
    program_counter: u16,
    stack: Stack,
    opcode_processor: T,
}

impl <T:TOpCodesProcessor> Chip8Chipset<T> {
    pub fn new(memory: Memory, stack: Stack, registers: Registers, opcode_processor: T) -> Self {
        Self {
            memory,
            registers,
            address_register: 0,
            program_counter: PROGRAM_COUNTER_BOUNDARY,
            stack,
            opcode_processor,
        }
    }

    pub fn get_opcode_processor(&self) -> &T {
        &self.opcode_processor
    }
}

impl <T:TOpCodesProcessor> Chipset for Chip8Chipset<T> {
    fn get_memory(&self) -> &Memory {
        &self.memory
    }

    fn tick(&mut self) {
        if let Some(opcode) = self.next_opcode() {
            match opcode.get_raw() {
                x if x == 0x00e0 => {
                    self.opcode_processor.clear_screen(&mut self.registers);
                }
                x if x == 0x00ee => {
                    self.opcode_processor.return_from_subroutine(&mut self.stack, &mut self.program_counter);
                }
                x if (x & 0xf000) == 0x1000 => {
                    self.opcode_processor.jump_to_address(&mut self.program_counter, opcode.get_address());
                }
                x if (x & 0xf000) == 0x2000 => {
                    self.opcode_processor.call_subroutine(&mut self.program_counter, opcode.get_address(), &mut self.stack);
                }
                x if (x & 0xf000) == 0x3000 => {
                    self.opcode_processor.cond_vx_equal_nn(&self.registers, &mut self.program_counter, opcode.get_x(), opcode.get_short_address());
                }
                x if (x & 0xf000) == 0x4000 => {
                    self.opcode_processor.cond_vx_not_equal_nn(&self.registers, &mut self.program_counter, opcode.get_x(), opcode.get_short_address());
                }
                x if (x & 0xf00f) == 0x5000 => {
                    self.opcode_processor.cond_vx_equal_vy(&self.registers, &mut self.program_counter, opcode.get_x(), opcode.get_short_address());
                }
                x if (x & 0xf000) == 0x6000 => {
                    self.opcode_processor.const_vx_equal_nn(&mut self.registers, opcode.get_x(), opcode.get_short_address());
                }
                //Add missing matches
                _ => {}/*panic!(println!("Unknown opcode: {}", x))*/
            }
        };
    }

    fn next_opcode(&mut self) -> Option<OpCode> {
        if self.program_counter >= (MEMORY_SIZE as u16) {
            return None;
        }

        let data = (self.memory.read(self.program_counter) as u16) << 8
            | (self.memory.read(self.program_counter + 1) as u16);
        self.program_counter += 2;

        Some(OpCode::from_data(data))
    }
}

#[cfg(test)]
mod test_chipset {
    use super::*;
    use emulator::memory::{Memory, Stack, Registers};
    use std::cell::Cell;

    #[test]
    fn test_can_read_next_opcode() {
        let (mut memory, stack, registers) = create_memory();

        let program_data: [u8; 6] = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
        load_data_into_memory(&mut memory, &program_data);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        let mut opcode = chipset.next_opcode().unwrap();
        assert_eq!(OpCode::from_data(0x102), opcode);

        opcode = chipset.next_opcode().unwrap();
        assert_eq!(OpCode::from_data(0x304), opcode);

        opcode = chipset.next_opcode().unwrap();
        assert_eq!(OpCode::from_data(0x506), opcode);
    }

    #[test]
    fn test_match_clear_screen_opcode() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xe0);
        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("clear_screen", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_return_from_subroutine() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xee);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("return_from_subroutine", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_jump_to_address() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x1a);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xbc);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("jump_to_address", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_call_subroutine() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x21);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0x23);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("call_subroutine", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_cond_vx_equal_nn() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x3a);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xbc);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("cond_vx_equal_nn", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_cond_vx_not_equal_nn() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x4a);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xbc);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("cond_vx_not_equal_nn", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_cond_vx_equal_vy() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x5a);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0xa0);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("cond_vx_equal_vy", chipset.get_opcode_processor().get_matched_method());
    }

    #[test]
    fn test_match_const_vx_equal_nn() {
        let (mut memory, stack, registers) = create_memory();
        memory.write(PROGRAM_COUNTER_BOUNDARY, 0x62);
        memory.write(PROGRAM_COUNTER_BOUNDARY + 1, 0x10);

        let mut chipset = Chip8Chipset::new(memory, stack, registers, MockedOpCodesProcessor::new());

        chipset.tick();
        assert_eq!("const_vx_equal_nn", chipset.get_opcode_processor().get_matched_method());
    }

    //Add missing match tests

    fn create_memory() -> (Memory, Stack, Registers) {
        (Memory::new(), Stack::new(), Registers::new())
    }

    fn load_data_into_memory(memory: &mut Memory, data: &[u8]) {
        let mut address = PROGRAM_COUNTER_BOUNDARY;
        for byte in data.iter() {
            memory.write(address, *byte);
            address += 1;
        }
    }

    struct MockedOpCodesProcessor {
        matched_method: Cell<String>,
    }
    impl MockedOpCodesProcessor {
        pub fn new() -> Self {
            Self {
                matched_method: Cell::new(String::new()),
            }
        }

        pub fn set_matched_method(&self, matched_method: &str) {
            self.matched_method.set(matched_method.to_owned());
        }

        pub fn get_matched_method(&self) -> String {
            self.matched_method.take()
        }
    }
    impl TOpCodesProcessor for MockedOpCodesProcessor{
        fn clear_screen(&self, registers: &mut Registers) {
            self.set_matched_method("clear_screen");
        }
        fn return_from_subroutine(&self, stack: &mut Stack, program_counter: &mut u16) {
            self.set_matched_method("return_from_subroutine");
        }
        fn jump_to_address(&self, program_counter: &mut u16, address: u16) {
            self.set_matched_method("jump_to_address");
        }
        fn call_subroutine(&self, program_counter: &mut u16, address: u16, stack: &mut Stack) {
            self.set_matched_method("call_subroutine");
        }
        fn cond_vx_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8) {
            self.set_matched_method("cond_vx_equal_nn");
        }
        fn cond_vx_not_equal_nn(&self, registers: &Registers, program_counter: &mut u16, x: u8, nn: u8) {
            self.set_matched_method("cond_vx_not_equal_nn");
        }
        fn cond_vx_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
            self.set_matched_method("cond_vx_equal_vy");
        }
        fn const_vx_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
            self.set_matched_method("const_vx_equal_nn");
        }
        fn const_vx_plus_equal_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
            self.set_matched_method("const_vx_plus_equal_nn");
        }
        fn assign_vx_equal_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("assign_vx_equal_vy");
        }
        fn bitop_vx_equal_vx_or_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_or_vy");
        }
        fn bitop_vx_equal_vx_and_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_and_vy");
        }
        fn bitop_vx_equal_vx_xor_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_xor_vy");
        }
        fn math_vx_equal_vx_plus_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("math_vx_equal_vx_plus_vy");
        }
        fn math_vx_equal_vx_minus_vy(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("math_vx_equal_vx_minus_vy");
        }
        fn bitop_vx_equal_vy_shr(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("bitop_vx_equal_vy_shr");
        }
        fn math_vx_equal_vy_minus_vx(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("math_vx_equal_vy_minus_vx");
        }
        fn bitop_vx_equal_vy_shl(&self, registers: &mut Registers, x: u8, y: u8) {
            self.set_matched_method("bitop_vx_equal_vy_shl");
        }
        fn cond_vx_not_equal_vy(&self, registers: &Registers, program_counter: &mut u16, x: u8, y: u8) {
            self.set_matched_method("cond_vx_not_equal_vy");
        }
        fn mem_i_equal_nnn(&self, address_register: &mut u16, nnn: u16) {
            self.set_matched_method("mem_i_equal_nnn");
        }
        fn flow_pc_equal_v0_plus_nnn(&self, program_counter: &mut u16, nnn: u16) {
            self.set_matched_method("flow_pc_equal_v0_plus_nnn");
        }
        fn rand_vx_equal_rand_and_nn(&self, registers: &mut Registers, x: u8, nn: u8) {
            self.set_matched_method("rand_vx_equal_rand_and_nn");
        }
        fn draw_vx_vy_n(&self, x: u8, y: u8, n: u8) {
            self.set_matched_method("draw_vx_vy_n");
        }
        fn mem_i_equal_i_plus_vx(&self, registers: &mut Registers, address_register: &mut u16, x: u8) {
            self.set_matched_method("mem_i_equal_i_plus_vx");
        }
        fn mem_i_equal_sprite_addr_vx(&self, registers: &Registers, address_register: &mut u16, x:u8) {
            self.set_matched_method("mem_i_equal_sprite_addr_vx");
        }
        fn mem_bcd(&self, registers: &Registers, address_register: &u16, memory: &mut Memory, x: u8) {
            self.set_matched_method("mem_bcd");
        }
        fn mem_reg_dump(&self, registers: &Registers, memory: &mut Memory, address_register: &mut u16, x: u8) {
            self.set_matched_method("mem_reg_dump");
        }
        fn mem_reg_load(&self, registers: &mut Registers, memory: &Memory, address_register: &mut u16, x: u8) {
            self.set_matched_method("mem_reg_load");
        }
    }
}