use crate::model::{CompileMode, CompileOptions};

mod compile;

pub fn handle_compile(mode: CompileMode, options: CompileOptions) {
    match mode {
        CompileMode::Json => compile::compile_json(options),
        CompileMode::Csv => compile::compile_csv(options),
    }
}
