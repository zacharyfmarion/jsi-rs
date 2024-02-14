use jsi::{
    FromObject, FromValue, IntoValue, JsiFn, JsiObject, JsiString, JsiValue, PropName,
    RuntimeHandle,
};

/// Calls a method on an object from Rust, properly handling errors and ensuring
/// that "this" is set correctly.
///
/// ```no_run
/// // If you have a JS object like { methodName() { return 34 } }
/// call_obj_method(&jsi_value, &mut rt, "methodName");
/// ```
pub fn call_obj_method<'rt, T: IntoIterator<Item = JsiValue<'rt>>>(
    value: &JsiValue<'rt>,
    rt: &mut RuntimeHandle<'rt>,
    field_name: &str,
    args: T,
) -> anyhow::Result<JsiValue<'rt>> {
    if value.is_undefined() {
        return Err(anyhow::anyhow!("Cannot access property of undefined"));
    }

    if !value.is_object() {
        return Err(anyhow::anyhow!("Property access on non-object value"));
    }

    let obj = JsiObject::from_value(value, rt)
        .ok_or(anyhow::anyhow!("Failed to convert value to object"))?;
    let method = get_object_field_as_fn(&obj, rt, field_name);

    match method {
        Some(m) => {
            return m
                .call_with_this(&obj, args, rt)
                .map_err(|_| anyhow::anyhow!("Failed to call method"));
        }
        None => {
            return Err(anyhow::anyhow!(
                "Cannot call method {} on object",
                field_name
            ));
        }
    }
}

// Get a field from an object. Note that JsiValue could be undefined, and needs to be checked is is_undefined()
pub fn get_object_field<'rt>(
    obj: &JsiObject<'rt>,
    rt: &mut RuntimeHandle<'rt>,
    field_name: &str,
) -> JsiValue<'rt> {
    let field = PropName::new(field_name, rt);
    return obj.get(field, rt);
}

/// Get a field from an object and convert it to a function
pub fn get_object_field_as_fn<'rt>(
    obj: &JsiObject<'rt>,
    rt: &mut RuntimeHandle<'rt>,
    field_name: &str,
) -> Option<JsiFn<'rt>> {
    let object_value = get_object_field(obj, rt, field_name);
    let object_value = JsiObject::from_value(&object_value, rt)?;
    return JsiFn::from_object(&object_value, rt);
}

pub fn get_global_value<'rt>(
    rt: &mut RuntimeHandle<'rt>,
    field_name: &str,
) -> Option<JsiValue<'rt>> {
    let prop_name = PropName::new(field_name, rt);
    Some(rt.global().get(prop_name, rt))
}

/// Log a message to the console
pub fn log_to_console<'rt>(rt: &mut RuntimeHandle<'rt>, message: &str) -> anyhow::Result<JsiValue<'rt>> {
    let console_log = get_global_value(rt, "console").ok_or(anyhow::anyhow!(
        "Failed to get console object"
    ))?;

    let args = [JsiString::new(message, rt).into_value(rt)];

    return call_obj_method(
        &console_log,
        rt,
        "log",
        args,
    )
}

/// Set a global value on the JSI runtime
pub fn set_global_value<'rt>(rt: &mut RuntimeHandle<'rt>, name: &str, value: &JsiValue<'rt>) {
    rt.global().set(PropName::new(name, rt), value, rt);
}
