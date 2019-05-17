use chip8::chipset::RandomByteGenerator;

pub struct RandRandomByteGenerator {}

impl RandRandomByteGenerator {
    pub fn new() -> RandRandomByteGenerator {
        RandRandomByteGenerator {}
    }
}

impl RandomByteGenerator for RandRandomByteGenerator {
    fn generate(&self) -> u8 {
        (js_sys::Math::random() * 255.0) as u8
    }
}
