pub fn compile(script: impl Into<String>) -> anyhow::Result<()> {
    vnsl_parser::parse(&script.into())
}
