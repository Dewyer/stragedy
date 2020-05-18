use bson::Document;
use bson::oid::ObjectId;

pub struct QueryExp
{
	doc:Document
}

impl QueryExp
{
	pub fn new() -> Self
	{
		QueryExp
		{
			doc:Document::new()
		}
	}

	pub fn new_by_id(id:&ObjectId) -> Self
	{
		let mut dc = Document::new();
		dc.insert("_id",id);
		QueryExp
		{
			doc:dc
		}
	}

	pub fn doc(&self) -> Document
	{
		self.doc.clone()
	}
}
