use bigdecimal::BigDecimal;

pub struct Tutor{
	pub id:uuid::Uuid,
	pub name:String,
	pub email:String,
	pub courses:i64,
	pub rating:BigDecimal,
}

impl Tutor{
	pub fn new(name:String,email:String)->Self{
	Tutor{
	id:uuid::Uuid::new_v4(),
	name,
	email,
	courses:0,
	rating:BigDecimal::from(0)
	}
	}
}
