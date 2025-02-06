use crate::RunContext;

pub trait Processor {
    fn process_text(&self, txt: String, ctx: &RunContext) -> String;
}

impl<F> Processor for F
where
    F: Fn(String, &RunContext) -> String,
{
    fn process_text(&self, txt: String, ctx: &RunContext) -> String {
        (self)(txt, ctx)
    }
}

#[derive(Debug)]
pub struct StringProcess<'a>(String, &'a RunContext);

impl<'a> StringProcess<'a> {
    pub const fn new(txt: String, ctx: &'a RunContext) -> Self {
        Self(txt, ctx)
    }

    pub fn chain(mut self, processor: impl Processor) -> Self {
        self.0 = processor.process_text(self.0, &self.1);
        self
    }

    pub fn string(self) -> String {
        self.0
    }
}
