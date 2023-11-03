use assembler;

struct MacroAssembler {
    asm: Box<Assembler>
}

impl MacroAssembler {

    pub fn new(buffer: *const u8, size: usize) -> MacroAssembler {
        MacroAssembler {
            asm: Box::new(Assembler::new())
        }
    }

}