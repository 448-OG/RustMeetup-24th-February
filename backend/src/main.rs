#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, serde::{Serialize, json::Json, }, http::Status
};
use lazy_static::lazy_static;
use sled::IVec;

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
async fn is_a_member(member: &str) -> Json<Bar> {
    match DB.get(member).unwrap() {
        Some(value) => {
                let when_registered: Bar = value.into();

            Json(when_registered)
        },
        None => {
            let error_type = format!("The member {} does not exist", member);
            Json(Bar { foo: error_type })
        }
    }

}



#[get("/<member>")]
async fn is_a_member_with_status(member: &str) -> Status {
    match DB.get(member).unwrap() {
        Some(_) => {
                Status::Ok
        },
        None => {
            Status::NotFound
        }
    }

}

#[post("/<member>")]
async fn register_member(member: &str) -> String {
    let foo = "Registered this afternoon";

    match DB.insert(member, Bar{foo: foo.to_string() }) {
        Ok(_) => format!("Registered {}", member),
        Err(error) => {
            eprintln!("Encountered error {:?}", error);

             "Couldn't register user".to_string()
        }
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
        .mount("/database", routes![is_a_member, which_database, register_member])
        .mount("/status", routes![is_a_member_with_status])
        .register("/", catchers![general_not_found])
}


#[catch(404)]
fn general_not_found() -> Json<Bar> {
    Json(Bar {foo: "404 error. Not found!".to_string()})
}


#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Bar{foo: String}

impl From<Bar> for IVec {
    fn from(value: Bar) -> Self {
        let mut foo = vec![2u8,];
        foo.extend_from_slice(value.foo.as_bytes());
            IVec::from(foo)
    }
}

impl From<IVec> for Bar {
    fn from(value: IVec) -> Self {
        Bar {
            foo: String::from_utf8(value[1..].to_vec()).unwrap()
        }
    }
}