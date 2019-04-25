use chip8::chipset::RandomByteGenerator;
use rand;

pub struct RandRandomByteGenerator {}
impl RandomByteGenerator for RandRandomByteGenerator {
    fn generate(&self) -> u8 {
        rand::random::<u8>()
    }
}
