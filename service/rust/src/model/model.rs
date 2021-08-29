use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Code {
	pub type_: String,
	pub data: SourceCode,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SourceCode {
	pub code: String,
	pub name: String,
	pub market: i32,
	pub hsgt: i32,
	pub bk: String,
	pub roe: f32,
	pub zgb: f64,
	pub ltgb: f64,
	pub ltsz: f64,
	pub zsz: f64,
	pub ssdate: String,
}

impl SourceCode {
	pub fn to_code(self, type_: String) -> Code {
		Code {
			type_,
			data: self.into(),
		}
	}
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
