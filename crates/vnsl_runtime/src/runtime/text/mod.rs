mod processor;

use processor::StringProcess;
use regex::Regex;

use crate::RunContext;

pub fn process_display_text(txt: String, ctx: &RunContext) -> String {
    StringProcess::new(txt, ctx)
        .chain(replace_runtime_val)
        .chain(string_cleanup)
        .string()
}

fn replace_runtime_val(txt: String, ctx: &RunContext) -> String {
    let var_template_regex = Regex::new(r#"\{\{([A-z0-9_ ]+)\}\}"#).unwrap();
    let mut result = txt.to_string();
    let iter = var_template_regex.captures_iter(&txt);

    for m in iter {
        let var_name = m.get(1).unwrap();
        if let Some(var_str) = ctx
            .lua_runtime
            .try_get_globals_val::<String>(var_name.as_str())
        {
            result = result.replace(m.get(0).unwrap().as_str(), &var_str);
        }
    }
    result
}

fn string_cleanup(txt: String, _ctx: &RunContext) -> String {
    txt.replace(r#"\""#, r#"""#)
}

#[cfg(test)]
mod test {
    use crate::RunContext;

    use super::replace_runtime_val;

    #[test]
    fn test_replace_val() {
        let ctx = RunContext::default();
        ctx.lua_runtime
            .set_globals_val("MY_CAPTURE", "CUNT!!".to_string())
            .unwrap();

        let result = replace_runtime_val("This is text {{MY_CAPTURE}} {{CAP_2}}".to_string(), &ctx);
        assert_eq!(result, "This is text CUNT!! {{CAP_2}}".to_string())
    }
}
