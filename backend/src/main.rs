use backend;
use backend::repos::Repo;

fn main()
{
	let mode = std::env::args().nth(1);
	match mode
	{
		Some(val)=>{
			match &val[..]
			{
				"clean" => {
					let repo = Repo::new().expect("Couldn't connect to the db!");
					println!("Wiping all collections!");
					repo.planet_repo.delete_by_doc(bson::Document::new()).expect("Error wiping planets");
					repo.player_repo.delete_by_doc(bson::Document::new()).expect("Error wiping players");
					println!("Done!");
				},
				_=>{}
			}
		}
		_=>
		{
			backend::rocket(true).launch();
		}
	}
}
