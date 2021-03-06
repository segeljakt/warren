#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    PutStructure(usize, usize, usize), // Ident, Arity, XReg
    SetVariable(usize),                // XReg
    SetValue(usize),                   // XReg
    GetStructure(usize, usize, usize), // Ident, Arity, XReg
    UnifyVariable(usize),              // XReg
    UnifyValue(usize),                 // XReg
}

impl Operation {
    /// Number of machine words (usize, program indicies) to be advanced
    /// after this operation execution. Normally it is instruction length,
    /// except of jump instructions which doesn't advance
    ///
    /// OpCode is always first item, and then every `usize` value
    /// is additional item in program
    pub fn advance(&self) -> usize {
        match self {
            Self::PutStructure(_, _, _) |
            Self::SetVariable(_) |
            Self::SetValue(_) |
            Self::GetStructure(_, _, _) |
            Self::UnifyVariable(_) |
            Self::UnifyValue(_) => self.size(),
        }
    }

    /// Instruction size
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::PutStructure(_, _, _) => 4,
            Self::SetVariable(_) => 2,
            Self::SetValue(_) => 2,
            Self::GetStructure(_, _, _) => 4,
            Self::UnifyVariable(_) => 2,
            Self::UnifyValue(_) => 2,
        }
    }
}
