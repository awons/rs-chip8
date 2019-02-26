use crate::emulator::display::TDisplay;
use crate::emulator::keyboard::TKeyboard;
use crate::emulator::memory::{Memory, Registers, Stack, MEMORY_SIZE};
use crate::emulator::opcode_processor::{OpCode, TOpCodesProcessor};

pub const PROGRAM_COUNTER_BOUNDARY: u16 = 0x200;

pub trait Chipset {
    fn get_memory(&self) -> &Memory;
    fn tick(&mut self) -> Result<(), String>;
    fn current_opcode(&mut self) -> Option<OpCode>;
}

pub struct Chip8Chipset<O: TOpCodesProcessor, D: TDisplay, K: TKeyboard> {
    memory: Memory,
    registers: Registers,
    address_register: u16,
    program_counter: u16,
    stack: Stack,
    opcode_processor: O,
    display: D,
    keyboard: K,
    delay_timer: u8,
    sound_timer: u8,
}

impl<O: TOpCodesProcessor, D: TDisplay, K: TKeyboard> Chip8Chipset<O, D, K> {
    pub fn new(
        memory: Memory,
        stack: Stack,
        registers: Registers,
        opcode_processor: O,
        display: D,
        keyboard: K,
    ) -> Self {
        Self {
            memory,
            registers,
            address_register: 0,
            program_counter: PROGRAM_COUNTER_BOUNDARY,
            stack,
            opcode_processor,
            display,
            keyboard,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl<O: TOpCodesProcessor, D: TDisplay, K: TKeyboard> Chipset for Chip8Chipset<O, D, K> {
    fn get_memory(&self) -> &Memory {
        &self.memory
    }

    fn tick(&mut self) -> Result<(), String> {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        match self.current_opcode() {
            Some(opcode) => {
                match opcode.get_parts() {
                    (0x0, 0x0, 0xe, 0x0) => {
                        self.opcode_processor.clear_screen(&mut self.display);
                        self.program_counter += 2;
                    }
                    (0x0, 0x0, 0xe, 0xe) => {
                        self.opcode_processor
                            .return_from_subroutine(&mut self.stack, &mut self.program_counter);
                        self.program_counter += 2;
                    }
                    (0x1, _, _, _) => {
                        self.opcode_processor
                            .jump_to_address(&mut self.program_counter, opcode.get_address());
                    }
                    (0x2, _, _, _) => {
                        self.opcode_processor.call_subroutine(
                            &mut self.program_counter,
                            opcode.get_address(),
                            &mut self.stack,
                        );
                    }
                    (0x3, _, _, _) => {
                        self.opcode_processor.cond_vx_equal_nn(
                            &self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0x4, _, _, _) => {
                        self.opcode_processor.cond_vx_not_equal_nn(
                            &self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0x5, _, _, 0x0) => {
                        self.opcode_processor.cond_vx_equal_vy(
                            &self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0x6, _, _, _) => {
                        self.opcode_processor.const_vx_equal_nn(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0x7, _, _, _) => {
                        self.opcode_processor.const_vx_plus_equal_nn(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x0) => {
                        self.opcode_processor.assign_vx_equal_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x1) => {
                        self.opcode_processor.bitop_vx_equal_vx_or_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x2) => {
                        self.opcode_processor.bitop_vx_equal_vx_and_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x3) => {
                        self.opcode_processor.bitop_vx_equal_vx_xor_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x4) => {
                        self.opcode_processor.math_vx_equal_vx_plus_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x5) => {
                        self.opcode_processor.math_vx_equal_vx_minus_vy(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x6) => {
                        self.opcode_processor.bitop_vx_equal_vy_shr(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0x7) => {
                        self.opcode_processor.math_vx_equal_vy_minus_vx(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x8, _, _, 0xe) => {
                        self.opcode_processor.bitop_vx_equal_vy_shl(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0x9, _, _, 0x0) => {
                        self.opcode_processor.cond_vx_not_equal_vy(
                            &mut self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                            opcode.get_y(),
                        );
                        self.program_counter += 2;
                    }
                    (0xa, _, _, _) => {
                        self.opcode_processor
                            .mem_i_equal_nnn(&mut self.address_register, opcode.get_address());
                        self.program_counter += 2;
                    }
                    (0xb, _, _, _) => {
                        self.opcode_processor.flow_pc_equal_v0_plus_nnn(
                            &mut self.program_counter,
                            opcode.get_address(),
                            &self.registers,
                        );
                    }
                    (0xc, _, _, _) => {
                        self.opcode_processor.rand_vx_equal_rand_and_nn(
                            &mut self.registers,
                            opcode.get_x(),
                            opcode.get_short_address(),
                        );
                        self.program_counter += 2;
                    }
                    (0xd, _, _, _) => {
                        self.opcode_processor.draw_vx_vy_n(
                            opcode.get_x(),
                            opcode.get_y(),
                            opcode.get_n(),
                            &mut self.display,
                            &self.memory,
                            &self.address_register,
                            &mut self.registers,
                        );
                        self.program_counter += 2;
                    }
                    (0xe, _, 0x9, 0xe) => {
                        self.opcode_processor.keyop_if_key_equal_vx(
                            &mut self.keyboard,
                            &self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xe, _, 0xa, 0x1) => {
                        self.opcode_processor.keyop_if_key_not_equal_vx(
                            &mut self.keyboard,
                            &self.registers,
                            &mut self.program_counter,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x0, 0x7) => {
                        self.opcode_processor.timer_vx_equal_get_delay(
                            &self.delay_timer,
                            &mut self.registers,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x0, 0xa) => {
                        self.opcode_processor.keyop_vx_equal_key(
                            &mut self.keyboard,
                            &mut self.registers,
                            opcode.get_x(),
                            &mut self.program_counter,
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x1, 0x5) => {
                        self.opcode_processor.timer_delay_timer_equal_vx(
                            &mut self.delay_timer,
                            &self.registers,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x1, 0x8) => {
                        self.opcode_processor.sound_sound_timer_equal_vx();
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x1, 0xe) => {
                        self.opcode_processor.mem_i_equal_i_plus_vx(
                            &mut self.registers,
                            &mut self.address_register,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x2, 0x9) => {
                        self.opcode_processor.mem_i_equal_sprite_addr_vx(
                            &self.registers,
                            &mut self.address_register,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x3, 0x3) => {
                        self.opcode_processor.mem_bcd(
                            &self.registers,
                            &self.address_register,
                            &mut self.memory,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x5, 0x5) => {
                        self.opcode_processor.mem_reg_dump(
                            &self.registers,
                            &mut self.memory,
                            &mut self.address_register,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0xf, _, 0x6, 0x5) => {
                        self.opcode_processor.mem_reg_load(
                            &mut self.registers,
                            &self.memory,
                            &mut self.address_register,
                            opcode.get_x(),
                        );
                        self.program_counter += 2;
                    }
                    (0x0, 0x0, 0x0, 0x0) => {
                        return Err("No more opcodes".to_string());
                    }
                    _ => {
                        panic!("Unknown opcode {:#x}", opcode);
                    }
                }
                Ok(())
            }
            None => Err("No more opcodes".to_string()),
        }
    }

    fn current_opcode(&mut self) -> Option<OpCode> {
        if self.program_counter >= (MEMORY_SIZE as u16) {
            return None;
        }

        let data = ((self.memory.read(self.program_counter) as u16) << 8)
            + (self.memory.read(self.program_counter + 1) as u16);

        Some(OpCode::from_data(data))
    }
}

#[cfg(test)]
mod test_chipset {
    use super::*;
    use crate::emulator::display::Display;
    use crate::emulator::keyboard::Keyboard;
    use crate::emulator::memory::{Memory, Registers, Stack};
    use std::cell::Cell;

    impl<O: TOpCodesProcessor, D: TDisplay, K: TKeyboard> Chip8Chipset<O, D, K> {
        pub fn get_opcode_processor(&self) -> &O {
            &self.opcode_processor
        }
    }

    #[test]
    fn test_can_read_current_opcode() {
        let (mut memory, stack, registers) = create_memory();

        let program_data: [u8; 6] = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
        load_data_into_memory(&mut memory, &program_data);

        let mut chipset = Chip8Chipset::new(
            memory,
            stack,
            registers,
            MockedOpCodesProcessor::new(),
            Display::new(),
            Keyboard::new(),
        );

        let opcode = chipset.current_opcode().unwrap();
        assert_eq!(OpCode::from_data(0x102), opcode);
    }

    fn get_opcodes() -> Vec<(&'static str, u16)> {
        let mut opcodes = Vec::with_capacity(34);

        opcodes.push(("clear_screen", 0x00e0));
        opcodes.push(("return_from_subroutine", 0x00ee));
        opcodes.push(("jump_to_address", 0x1abc));
        opcodes.push(("call_subroutine", 0x2123));
        opcodes.push(("cond_vx_equal_nn", 0x3abc));
        opcodes.push(("cond_vx_not_equal_nn", 0x4abc));
        opcodes.push(("cond_vx_equal_vy", 0x5aa0));
        opcodes.push(("const_vx_equal_nn", 0x6210));
        opcodes.push(("const_vx_plus_equal_nn", 0x7210));
        opcodes.push(("assign_vx_equal_vy", 0x8210));
        opcodes.push(("bitop_vx_equal_vx_or_vy", 0x8211));
        opcodes.push(("bitop_vx_equal_vx_and_vy", 0x8212));
        opcodes.push(("bitop_vx_equal_vx_xor_vy", 0x8213));
        opcodes.push(("math_vx_equal_vx_plus_vy", 0x8214));
        opcodes.push(("math_vx_equal_vx_minus_vy", 0x8215));
        opcodes.push(("bitop_vx_equal_vy_shr", 0x8216));
        opcodes.push(("math_vx_equal_vy_minus_vx", 0x8217));
        opcodes.push(("bitop_vx_equal_vy_shl", 0x821e));
        opcodes.push(("cond_vx_not_equal_vy", 0x9120));
        opcodes.push(("mem_i_equal_nnn", 0xa123));
        opcodes.push(("flow_pc_equal_v0_plus_nnn", 0xb123));
        opcodes.push(("rand_vx_equal_rand_and_nn", 0xc123));
        opcodes.push(("draw_vx_vy_n", 0xd123));
        opcodes.push(("keyop_if_key_equal_vx", 0xe59e));
        opcodes.push(("keyop_if_key_not_equal_vx", 0xe5a1));
        opcodes.push(("timer_vx_equal_get_delay", 0xf507));
        opcodes.push(("keyop_vx_equal_key", 0xf50a));
        opcodes.push(("timer_delay_timer_equal_vx", 0xf515));
        opcodes.push(("sound_sound_timer_equal_vx", 0xf518));
        opcodes.push(("mem_i_equal_i_plus_vx", 0xf51e));
        opcodes.push(("mem_i_equal_sprite_addr_vx", 0xf529));
        opcodes.push(("mem_bcd", 0xf533));
        opcodes.push(("mem_reg_dump", 0xf555));
        opcodes.push(("mem_reg_load", 0xf565));

        opcodes
    }

    #[test]
    fn test_opcode_match() {
        for opcode_data in get_opcodes() {
            let (mut memory, stack, registers) = create_memory();
            let (method_name, opcode) = opcode_data;

            memory.write(PROGRAM_COUNTER_BOUNDARY, ((opcode & 0xff00) >> 8) as u8);
            memory.write(PROGRAM_COUNTER_BOUNDARY + 1, (opcode & 0x00ff) as u8);

            let mut chipset = Chip8Chipset::new(
                memory,
                stack,
                registers,
                MockedOpCodesProcessor::new(),
                Display::new(),
                Keyboard::new(),
            );

            let _ = chipset.tick();
            assert_eq!(
                method_name,
                chipset.get_opcode_processor().get_matched_method()
            );
        }
    }

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
    impl TOpCodesProcessor for MockedOpCodesProcessor {
        fn clear_screen(&self, _registers: &mut dyn TDisplay) {
            self.set_matched_method("clear_screen");
        }
        fn return_from_subroutine(&self, _stack: &mut Stack, _program_counter: &mut u16) {
            self.set_matched_method("return_from_subroutine");
        }
        fn jump_to_address(&self, _program_counter: &mut u16, _address: u16) {
            self.set_matched_method("jump_to_address");
        }
        fn call_subroutine(&self, _program_counter: &mut u16, _address: u16, _stack: &mut Stack) {
            self.set_matched_method("call_subroutine");
        }
        fn cond_vx_equal_nn(
            &self,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
            _nn: u8,
        ) {
            self.set_matched_method("cond_vx_equal_nn");
        }
        fn cond_vx_not_equal_nn(
            &self,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
            _nn: u8,
        ) {
            self.set_matched_method("cond_vx_not_equal_nn");
        }
        fn cond_vx_equal_vy(
            &self,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
            _y: u8,
        ) {
            self.set_matched_method("cond_vx_equal_vy");
        }
        fn const_vx_equal_nn(&self, _registers: &mut Registers, _x: u8, _nn: u8) {
            self.set_matched_method("const_vx_equal_nn");
        }
        fn const_vx_plus_equal_nn(&self, _registers: &mut Registers, _x: u8, _nn: u8) {
            self.set_matched_method("const_vx_plus_equal_nn");
        }
        fn assign_vx_equal_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("assign_vx_equal_vy");
        }
        fn bitop_vx_equal_vx_or_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_or_vy");
        }
        fn bitop_vx_equal_vx_and_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_and_vy");
        }
        fn bitop_vx_equal_vx_xor_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("bitop_vx_equal_vx_xor_vy");
        }
        fn math_vx_equal_vx_plus_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("math_vx_equal_vx_plus_vy");
        }
        fn math_vx_equal_vx_minus_vy(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("math_vx_equal_vx_minus_vy");
        }
        fn bitop_vx_equal_vy_shr(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("bitop_vx_equal_vy_shr");
        }
        fn math_vx_equal_vy_minus_vx(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("math_vx_equal_vy_minus_vx");
        }
        fn bitop_vx_equal_vy_shl(&self, _registers: &mut Registers, _x: u8, _y: u8) {
            self.set_matched_method("bitop_vx_equal_vy_shl");
        }
        fn cond_vx_not_equal_vy(
            &self,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
            _y: u8,
        ) {
            self.set_matched_method("cond_vx_not_equal_vy");
        }
        fn mem_i_equal_nnn(&self, _address_register: &mut u16, _nnn: u16) {
            self.set_matched_method("mem_i_equal_nnn");
        }
        fn flow_pc_equal_v0_plus_nnn(
            &self,
            _program_counter: &mut u16,
            _nnn: u16,
            _registers: &Registers,
        ) {
            self.set_matched_method("flow_pc_equal_v0_plus_nnn");
        }
        fn rand_vx_equal_rand_and_nn(&self, _registers: &mut Registers, _x: u8, _nn: u8) {
            self.set_matched_method("rand_vx_equal_rand_and_nn");
        }
        fn draw_vx_vy_n(
            &self,
            _x: u8,
            _y: u8,
            _n: u8,
            _display: &mut dyn TDisplay,
            _memory: &Memory,
            _address_register: &u16,
            _registers: &mut Registers,
        ) {
            self.set_matched_method("draw_vx_vy_n");
        }
        fn mem_i_equal_i_plus_vx(
            &self,
            _registers: &mut Registers,
            _address_register: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("mem_i_equal_i_plus_vx");
        }
        fn mem_i_equal_sprite_addr_vx(
            &self,
            _registers: &Registers,
            _address_register: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("mem_i_equal_sprite_addr_vx");
        }
        fn mem_bcd(
            &self,
            _registers: &Registers,
            _address_register: &u16,
            _memory: &mut Memory,
            _x: u8,
        ) {
            self.set_matched_method("mem_bcd");
        }
        fn mem_reg_dump(
            &self,
            _registers: &Registers,
            _memory: &mut Memory,
            _address_register: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("mem_reg_dump");
        }
        fn mem_reg_load(
            &self,
            _registers: &mut Registers,
            _memory: &Memory,
            _address_register: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("mem_reg_load");
        }
        fn keyop_if_key_equal_vx(
            &self,
            _keyboard: &mut dyn TKeyboard,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("keyop_if_key_equal_vx");
        }
        fn keyop_if_key_not_equal_vx(
            &self,
            _keyboard: &mut dyn TKeyboard,
            _registers: &Registers,
            _program_counter: &mut u16,
            _x: u8,
        ) {
            self.set_matched_method("keyop_if_key_not_equal_vx");
        }
        fn keyop_vx_equal_key(
            &self,
            _keyboard: &mut dyn TKeyboard,
            _registers: &mut Registers,
            _x: u8,
            _program_counter: &mut u16,
        ) {
            self.set_matched_method("keyop_vx_equal_key");
        }
        fn timer_vx_equal_get_delay(&self, _delay_timer: &u8, _registers: &mut Registers, _x: u8) {
            self.set_matched_method("timer_vx_equal_get_delay");
        }
        fn timer_delay_timer_equal_vx(
            &self,
            _delay_timer: &mut u8,
            _registers: &Registers,
            _x: u8,
        ) {
            self.set_matched_method("timer_delay_timer_equal_vx");
        }
        fn sound_sound_timer_equal_vx(&self) {
            self.set_matched_method("sound_sound_timer_equal_vx");
        }
    }
}
