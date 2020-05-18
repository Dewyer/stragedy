use mongodb::sync::{Collection};
use std::error::Error;
use std::marker::PhantomData;
use bson::Document;
use crate::repos;
use crate::error::ApiError;

pub struct GenericRepo<T>
{
	collection:Collection,
	phantom:PhantomData<T>
}

impl<'de,T> GenericRepo<T>
where T : serde::Serialize + serde::Deserialize<'de>
{
	pub fn new(col:Collection) -> Self
	{
		GenericRepo
		{
			collection:col,
			phantom:PhantomData
		}
	}

	fn unwrap_query(&self,query:Result<Option<bson::Document>,mongodb::error::Error>) -> Option<T>
	{
		match query.ok()
		{
			Some(doc) =>
			match doc
			{
				Some(doc_val) => {
					match bson::from_bson::<T>(bson::Bson::from(doc_val)) {
						Ok(val)=> Some(val),
						Err(_)=>None
					}
				},
				None=> None
			},
			None => None,
		}
	}


	pub fn insert_model(&self,model:&T) -> Result<(),Box<dyn Error>>
	{
		let doc = bson::to_bson(model)?;
		let doc = doc.as_document().unwrap();
		self.collection.insert_one(doc.clone(),None)?;
		Ok(())
	}

	pub fn find_by_id(&self,id:&str) -> Option<T>
	{
		let query_res:Result<Option<bson::Document>,mongodb::error::Error> = self.collection.find_one(doc!{"_id":id},None);
		self.unwrap_query(query_res)
	}

	pub fn find_by_filter(&self, filter:Document) -> Option<T>
	{
		let query = self.collection.find_one(filter,None);
		self.unwrap_query(query)
	}

	pub fn update_by_doc(&self,filter:&Document,doc:&Document) -> Result<(),ApiError>
	{
		match self.collection.update_one(filter.clone(),doc.clone(),None)
		{
			Ok(update_res) => {
				if update_res.modified_count == 1
				{
					Ok(())
				}
				else
				{
					Err(ApiError::new("updated-0"))
				}
			},
			Err(er)=> Err(ApiError::new("update-err"))
		}
	}
}
