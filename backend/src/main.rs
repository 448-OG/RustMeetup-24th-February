#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref DB:  sled::Db = {
        match sled::open("members.db") {
            Ok(db) => db,
            Err(error) => {
                eprintln!("Unable to Open database because of error {:?}", error);

                std::process::exit(0)
            }
        }
    };
}


#[get("/<member>")]
async fn is_a_member(member: &str) -> String {
    match DB.get(member).unwrap() {
        Some(value) => String::from_utf8(value.to_vec()).unwrap(),
        None => format!("The member {} does not exist", member)
    }

}

#[get("/what")]
async fn which_database() -> &'static str {
    "My database is called `sled`"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("assets/"))
        .mount("/database", routes![is_a_member, which_database])
}


fn handle_db() -> bool{
    match sled::open("members.db") {
        Ok(_) => true,
        Err(_) => false
    }

}