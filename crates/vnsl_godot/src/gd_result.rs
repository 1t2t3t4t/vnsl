use godot::{
    builtin::Variant,
    global,
    meta::ToGodot,
    obj::Gd,
    prelude::{godot_api, GodotClass},
};

macro_rules! decl_result {
    ($name:ident, $type:ty) => {
        #[derive(Debug, GodotClass)]
        #[class(base=RefCounted, init)]
        pub struct $name {
            result: Option<$type>,
            err: Option<anyhow::Error>,
        }

        impl $name {
            pub fn wrap_gd_result(f: impl FnOnce() -> anyhow::Result<$type>) -> Gd<Self> {
                match f() {
                    Ok(res) => Self::ok(res),
                    Err(err) => Self::err(err),
                }
            }

            pub fn ok(result: $type) -> Gd<Self> {
                Gd::from_object(Self {
                    result: Some(result),
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
        impl $name {
            #[func]
            pub fn is_ok(&self) -> bool {
                self.result.is_some()
            }

            #[func]
            pub fn is_err(&self) -> bool {
                self.err.is_some()
            }

            #[func]
            pub fn result(&self) -> $type {
                self.result.clone().expect("should have result")
            }

            #[func]
            pub fn err_message(&self) -> String {
                match &self.err {
                    Some(err) => format!("{}", err),
                    None => "Ok".to_string(),
                }
            }

            #[func]
            pub fn print_err_if_available(&self) {
                if self.is_err() {
                    global::printerr(&[format!("Vnsl error: {}", self.err_message()).to_variant()]);
                }
            }
        }
    };
}

decl_result!(GdResultString, String);
decl_result!(GdResultBool, bool);
decl_result!(GdResult, ());
