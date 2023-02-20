mod repos;
mod base;
use rocket::fs::{FileServer, NamedFile};
use rocket::{launch, get, routes};
use crate::repos::build_index;

// Current release number
pub(crate) fn current() -> u64{
    1
}

#[get("/repo/latest")]
async fn get_latest() -> String{
    format!("{}", current())
}

#[get("/")]
async fn index() -> Option<NamedFile>{
    match build_index().await.ok(){
        Some(()) => (),
        None => return None,
    }
    NamedFile::open("index.html").await.ok()
}

#[launch]
fn rocket() -> _{
    rocket::build()
        .mount("/", routes![index, get_latest])
        .mount("/assets", FileServer::from("static"))
}
