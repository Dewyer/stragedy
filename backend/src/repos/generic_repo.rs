use mongodb::sync::{Collection,Cursor};
use std::error::Error;
use std::marker::PhantomData;
use bson::Document;
use crate::repos;
use lib::error::ApiError;
use crate::helpers;

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

	pub fn find_by_id(&self,id:bson::oid::ObjectId) -> Option<T>
	{
		let query_res:Result<Option<bson::Document>,mongodb::error::Error> = self.collection.find_one(helpers::query::QueryExp::new_by_id(&id).doc(),None);
		self.unwrap_query(query_res)
	}

	pub fn find_by_filter(&self, filter:Document) -> Option<T>
	{
		let query = self.collection.find_one(filter,None);
		self.unwrap_query(query)
	}

	pub fn find_all_like(&self,filter:Document) -> Result<Vec<T>,ApiError>
	{
		let query:Result<Cursor,mongodb::error::Error> = self.collection.find(filter,None);
		match query
		{
			Ok(cursor)=>
				{
					let mut res_vec:Vec<T> = Vec::new();
					for result in cursor
					{
						match result
						{
							Ok(doc) => {
								let modell_version = bson::from_bson::<T>(bson::Bson::Document(doc));
								if let Ok(modell) = modell_version
								{
									res_vec.push(modell);
								}
							},
							_ => {}
						}
					}

					Ok(res_vec)
				},
			Err(_)=>Err(ApiError::ServerError)
		}
	}

	pub fn get_all(&self) -> Result<Vec<T>,ApiError>
	{
		self.find_all_like(Document::new())
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
					Err(ApiError::ZeroModified)
				}
			},
			Err(er)=> Err(ApiError::UpdateFailed)
		}
	}
}
