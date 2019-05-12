pub const MEMORY_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 0xf;
const REGISTERS_COUNT: usize = 0x10;

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}

pub struct Stack {
    memory: [u16; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            memory: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;

        self.memory[self.stack_pointer]
    }

    pub fn push(&mut self, address: u16) {
        self.memory[self.stack_pointer] = address;
        self.stack_pointer += 1;
    }
}

pub struct Registers {
    registers: [u8; REGISTERS_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: [0; REGISTERS_COUNT],
        }
    }

    pub fn get_register_at(&self, index: usize) -> u8 {
        self.registers[index]
    }

    pub fn set_register_at(&mut self, index: usize, data: u8) {
        self.registers[index] = data;
    }
}

#[cfg(test)]
mod test_memory {
    use super::*;

    #[test]
    fn test_can_write_to_and_read_from_memory_at_given_address() {
        let mut memory = Memory::new();
        memory.write(0x100, 16);

        assert_eq!(16, memory.read(0x100));
    }

    #[test]
    #[should_panic]
    fn test_will_panic_when_trying_to_read_outside_of_available_memory() {
        let memory = Memory::new();
        memory.read(0x1000);
    }

    #[test]
    #[should_panic]
    fn test_will_panic_when_trying_to_write_outside_of_available_memory() {
        let mut memory = Memory::new();
        memory.write(0x1000, 1);
    }

    #[test]
    fn test_can_move_up_and_down_the_stack() {
        let mut stack = Stack::new();

        stack.push(0x100);
        stack.push(0x200);
        stack.push(0x300);

        assert_eq!(0x300, stack.pop());
        assert_eq!(0x200, stack.pop());
        assert_eq!(0x100, stack.pop());
    }

    #[test]
    fn test_can_write_to_and_read_from_registers() {
        let mut registers = Registers::new();

        assert_eq!(0, registers.get_register_at(0xe));
        registers.set_register_at(0xe, 1);
        assert_eq!(1, registers.get_register_at(0xe));
    }
}
