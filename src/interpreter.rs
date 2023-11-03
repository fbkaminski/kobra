struct Interpreter {
    assembler: Assembler
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            assembler: Assembler::new()
        }
    }
}