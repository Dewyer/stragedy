use bson::Document;

pub struct UpdateExp
{
	doc:Document
}


impl UpdateExp
{
	pub fn new() -> Self
	{
		UpdateExp
		{
			doc:Document::new()
		}
	}

	pub fn set<S>(&mut self,what:&str,to:S) -> &mut Self
		where S : serde::Serialize
	{
		if self.doc.contains_key("$set")
		{
			let mut old = self.doc.get("$set").unwrap().as_document().unwrap().clone();
			self.doc.remove("$set");
			old.insert(what,bson::to_bson(&to).unwrap())
			self.doc.insert("$set",old);
		}
		else {
			let mut doc = bson::Document::new();
			doc.insert(what, bson::to_bson(&to).unwrap());
			self.doc.insert("$set", doc);
		}
		self
	}

	pub fn doc(&self) -> Document
	{
		self.doc.clone()
	}
}
