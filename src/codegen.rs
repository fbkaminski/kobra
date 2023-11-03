trait CodeStub {
   fn generate(&mut self, assembler: MacroAssembler);
}

/*
 * RuntimeStub - calling entry points on the Runtime
 */

struct RuntimeStub {
    id: RuntimeId
}

impl RuntimeStub {

    pub fn new(id: RuntimeId) -> RuntimeStub {
        RuntimeStub {
            id: id 
        }
    }

}

impl CodeStub for RuntimeStub {

    fn generate(&mut self, assembler: MacroAssembler) {

    }

}

/*
 * CEntryStub - native code stub
 */

 struct CEntryStub {

 }
 
 impl CEntryStub {
 
 }
 
 impl CodeStub for CEntryStub {
 
    fn generate(&mut self, assembler: MacroAssembler) {

    }
 
 }
