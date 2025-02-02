use godot::{
    builtin::Variant,
    meta::ToGodot,
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

#[derive(Debug, GodotClass)]
#[class(base=RefCounted, init)]
pub struct GdResult {
    result: Option<Variant>,
    err: Option<anyhow::Error>,
}

impl GdResult {
    pub fn ok(result: impl ToGodot) -> Gd<Self> {
        Gd::from_object(Self {
            result: Some(result.to_variant()),
            err: None,
        })
    }

    pub fn err(err: impl Into<anyhow::Error>) -> Gd<Self> {
        Gd::from_object(Self {
            result: None,
            err: Some(err.into()),
        })
    }
}

#[godot_api]
impl GdResult {
    #[func]
    pub fn is_ok(&self) -> bool {
        self.result.is_some()
    }

    #[func]
    pub fn is_err(&self) -> bool {
        self.err.is_some()
    }

    #[func]
    pub fn result(&self) -> Variant {
        self.result.clone().expect("should have result")
    }

    #[func]
    pub fn err_message(&self) -> String {
        match &self.err {
            Some(err) => format!("{}", err),
            None => "Ok".to_string(),
        }
    }
}

pub fn wrap_gd_result<T>(f: impl FnOnce() -> anyhow::Result<T>) -> Gd<GdResult>
where
    T: ToGodot,
{
    match f() {
        Ok(res) => GdResult::ok(res.to_variant()),
        Err(err) => GdResult::err(err),
    }
}
