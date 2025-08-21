pub Enum CourseType{
	FREE,
	PAID
}
#[derive(Debug,sql::FromRow)]
pub struct Course{
	pub id:Uuid,
	pub tutor_id:Uuid,
	pub name:String,
	pub rating:Option<String>,
	pub enrolled:i64,
	pub enrolled_limit:i64
}


