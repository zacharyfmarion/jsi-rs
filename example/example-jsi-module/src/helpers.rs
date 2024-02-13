use jsi::{
	FromObject, FromValue, IntoValue, JsiFn, JsiObject, JsiString, JsiValue, PropName, RuntimeHandle
};

/// Log a message to the console
pub fn log_to_console<'rt>(rt: &mut RuntimeHandle<'rt>, message: &str) {
    let console = PropName::new("console", rt);
    let console = rt.global().get(console, rt);
    let console = JsiObject::from_value(&console, rt).unwrap();

    let console_log = console.get(PropName::new("log", rt), rt);
    let console_log = JsiObject::from_value(&console_log, rt).unwrap();
    let console_log = JsiFn::from_object(&console_log, rt).unwrap();
    console_log
        .call(
            [JsiString::new(message, rt).into_value(rt)],
            rt,
        )
        .unwrap();
}

/// Set a global value on the JSI runtime
pub fn set_global_value<'rt>(rt: &mut RuntimeHandle<'rt>, name: &str, value: &JsiValue<'rt>) {
    rt.global().set(PropName::new(name, rt), value, rt);
}