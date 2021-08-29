use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Code {
	pub type_: String,
	pub code: String,
	pub name: String,
	pub market: i32,
	pub hsgt: i32,
	pub bk: String,
	pub roe: f32,
	pub zgb: i128,
	pub ltgb: i128,
	pub ltsz: i128,
	pub zsz: i128,
	pub ssdate: String,
}

impl Code {
	pub fn to_stock(&mut self) {
		self.type_ = "stock".to_string();
	}
	pub fn to_fund(&mut self) {
		self.type_ = "fund".to_string();
	}
	pub fn is_stock(&self) -> bool {
		self.type_ == "stock".to_string()
	}
	pub fn is_fund(&self) -> bool {
		self.type_ == "fund".to_string()
	}
}
