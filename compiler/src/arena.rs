use bumpalo::Bump;

pub struct CompilerArenas {
    pub parse_arena: Bump,
    pub ir_arena: Bump,
    pub codegen_arena: Bump,
}

impl CompilerArenas {
    pub fn new(total_limit: usize) -> Self {
        let chunk_size = total_limit / 3;
        Self {
            parse_arena: Bump::with_capacity(chunk_size),
            ir_arena: Bump::with_capacity(chunk_size),
            codegen_arena: Bump::with_capacity(chunk_size),
        }
    }

    pub fn new_default() -> Self {
        Self::new(100 * 1024 * 1024)
    }

    pub fn reset(&mut self) {
        self.parse_arena.reset();
        self.ir_arena.reset();
        self.codegen_arena.reset();
    }

    pub fn total_allocated(&self) -> usize {
        self.parse_arena.allocated_bytes()
            + self.ir_arena.allocated_bytes()
            + self.codegen_arena.allocated_bytes()
    }

    pub fn parse_allocated(&self) -> usize {
        self.parse_arena.allocated_bytes()
    }

    pub fn ir_allocated(&self) -> usize {
        self.ir_arena.allocated_bytes()
    }

    pub fn codegen_allocated(&self) -> usize {
        self.codegen_arena.allocated_bytes()
    }
}

impl Default for CompilerArenas {
    fn default() -> Self {
        Self::new_default()
    }
}
