mod repos;
mod base;

use std::future::Future;
use rocket::fs::{FileServer, NamedFile};
use rocket::{launch, get, routes, post};
use rocket::form::Form;
use texc_v3_web::{Mode, WebConfig};
use crate::repos::build_index;

// Current release number
pub(crate) fn current() -> u64{
    2
}

#[get("/repo/latest")]
async fn get_latest() -> String{
    format!("{}", current())
}

#[get("/create")]
async fn build_texc() -> Option<NamedFile>{
    match texc_v3_web::build_index(Some("texcreate.html".to_string()), Some(Mode::Light)).await.ok(){
       Some(()) => (),
        None => return None
    };
    NamedFile::open("texcreate.html").await.ok()
}

#[post("/", data = "<input>")]
async fn create_proj(input: Form<WebConfig>) -> Option<NamedFile>{
    let input = input.into_inner();
    let file = input.zip().await.ok();
    match file{
        Some(name) => NamedFile::open(&name).await.ok(),
        None => None
    }
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
        .mount("/", routes![index, get_latest, build_texc, create_proj])
        .mount("/assets", FileServer::from("static"))
}
