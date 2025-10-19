use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Out of memory during {phase}: used {used} bytes, limit {limit} bytes")]
    OutOfMemory {
        phase: &'static str,
        used: usize,
        limit: usize,
    },

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Type check error: {0}")]
    TypeError(String),

    #[error("Codegen error: {0}")]
    CodegenError(String),
}

pub struct MemoryStats {
    pub parse_bytes: usize,
    pub ir_bytes: usize,
    pub codegen_bytes: usize,
    pub total_bytes: usize,
}

impl MemoryStats {
    pub fn new(parse: usize, ir: usize, codegen: usize) -> Self {
        Self {
            parse_bytes: parse,
            ir_bytes: ir,
            codegen_bytes: codegen,
            total_bytes: parse + ir + codegen,
        }
    }

    pub fn format_human_readable(&self) -> String {
        format!(
            "Parse: {} | IR: {} | Codegen: {} | Total: {}",
            Self::format_bytes(self.parse_bytes),
            Self::format_bytes(self.ir_bytes),
            Self::format_bytes(self.codegen_bytes),
            Self::format_bytes(self.total_bytes)
        )
    }

    fn format_bytes(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}
