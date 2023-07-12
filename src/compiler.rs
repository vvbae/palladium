#[derive(Default)]
pub struct Compiler {
    pub free_reg: Vec<u8>,
    pub used_reg: Vec<u8>,
    pub assembly: Vec<String>, // assembly code of the program
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_reg: (0..31).rev().collect::<Vec<u8>>(),
            used_reg: Vec::new(),
            assembly: Vec::new(),
        }
    }
}
