mod processor;

use processor::StringProcess;

use crate::RunContext;

pub fn process_display_text(txt: String, ctx: &RunContext) -> String {
    StringProcess::new(txt, ctx).chain(no_ops).string()
}

fn no_ops(txt: String, _ctx: &RunContext) -> String {
    txt
}
