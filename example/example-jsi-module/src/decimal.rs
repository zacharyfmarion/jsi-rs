use std::marker::PhantomData;

use jsi::{host_object, FromValue, JsiValue, PropName, RuntimeHandle};
use rust_decimal::{prelude::ToPrimitive, Decimal};

use crate::helpers::call_obj_method;

/**
 * Implementation of the Decimal class in Rust, using the `rust_decimal` crate.
 */
pub struct FastDecimal<'rt> {
	value: Decimal,
	data: PhantomData<&'rt ()>,
}

impl FastDecimal<'_> {
	pub fn new(value: Decimal) -> Self {
		Self { value, data: PhantomData }
	}
}

// Read in a JS FastDecimal and parse it's value into a Rust FastDecimal struct
// TODO: Since we instantiate a new FastDecimal there might be some overhead here
impl<'rt> FromValue<'rt> for FastDecimal<'rt> {
	fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
		let string_decimal_value = call_obj_method(value, rt, "toString", []).ok()?;
		let string_decimal_value = String::from_value(&string_decimal_value, rt)?;
		let parsed_decimal = Decimal::from_str_exact(&string_decimal_value).ok()?;
		return Some(FastDecimal::new(parsed_decimal));
	}
}

#[host_object]
impl<'rt> FastDecimal<'rt> {
	/// Get the value of the Decimal as a string
	pub fn to_string(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<String> {
		Ok(self.value.to_string())
	}

	/// Get the value of the Decimal as a number
	pub fn to_number(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<f64> {
		self.value.to_f64().ok_or_else(|| anyhow::anyhow!("Unable to convert to number"))
	}

	/// Add two Decimals together
	pub fn add(&self, _rt: &mut RuntimeHandle<'rt>, other: FastDecimal) -> anyhow::Result<Self> {
		Ok(FastDecimal::new(self.value + other.value))
	}

	// /// Subtract one Decimal from another
	// pub fn subtract(&self, other: &Self) -> Self {
	// 	Self {
	// 		value: self.value - other.value,
	// 	}
	// }

	// /// Multiply two Decimals together
	// pub fn multiply(&self, other: &Self) -> Self {
	// 	Self {
	// 		value: self.value * other.value,
	// 	}
	// }

	// /// Divide one Decimal by another
	// pub fn divide(&self, other: &Self) -> Self {
	// 	Self {
	// 		value: self.value / other.value,
	// 	}
	// }
}