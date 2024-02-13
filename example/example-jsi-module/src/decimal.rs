use jsi::{host_object, PropName, RuntimeHandle};
use rust_decimal::{prelude::ToPrimitive, Decimal};

/**
 * Implementation of the Decimal class in Rust, using the `rust_decimal` crate.
 */

pub struct FastDecimal {
	value: Decimal,
}

impl FastDecimal {
	pub fn new(value: Decimal) -> Self {
		Self { value }
	}
}

#[host_object]
impl FastDecimal {
	/// Get the value of the Decimal as a string
	pub fn to_string(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<String> {
		Ok(self.value.to_string())
	}

	/// Get the value of the Decimal as a number
	pub fn to_number(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<f64> {
		match self.value.to_f64() {
			Some(n) => {
				Ok(n)
			}
			None => Err(anyhow::anyhow!("Invalid number")),
		}
	}

	// /// Add two Decimals together
	// pub fn add(&self, other: &Self) -> Self {
	// 	Self {
	// 		value: self.value + other.value,
	// 	}
	// }

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