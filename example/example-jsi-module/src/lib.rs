use decimal::FastDecimal;
use jsi::{FromValue, IntoValue, JsiFn, PropName};
use rust_decimal::Decimal;

mod helpers;
mod decimal;

#[cfg(target_os = "android")]
mod android;

pub fn init(rt: *mut jsi::sys::Runtime, call_invoker: cxx::SharedPtr<jsi::sys::CallInvoker>) {
    let (mut rt, _) = jsi::init(rt, call_invoker);
    helpers::log_to_console(&mut rt, "hello from Rust");
    // TODO: I want this struct to be instantiated with a value (as a rust object)...so actually
    // I want a function, that takes a value, and returns a rust object, and then I want to call

    let decimal_from_string_name = "__FastDecimal";

    // Create a FastDecimal instance from a string
    let decimal_from_string = JsiFn::from_host_fn(
        &PropName::new(&decimal_from_string_name, &mut rt), 
        1, 
        Box::new(move |_this, args, rt| {
            // TODO: Support number as well (Or should this validation happen in JS?)
            if args.len() != 1 {
                // Kind of weird that I am using the Ok variant here but I don't
                // want a runtime crash...maybe this is handled above but I want 
                // the error to happen in JS
                return Err(anyhow::anyhow!("Invalid args"));
            }

            let string = &args[0];
            let string = String::from_value(string, rt);
            match string {
                Some(s) => {
                    let decimal = Decimal::from_str_exact(&s)?;
                    Ok(FastDecimal::new(decimal).into_value(rt))
                }
                None => Err(anyhow::anyhow!("Invalid string")),
            }
        }), 
        &mut rt
    );

    let host_fn = decimal_from_string.into_value(&mut rt);
    helpers::set_global_value(&mut rt, &decimal_from_string_name, &host_fn);
}