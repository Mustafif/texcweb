mod base;
mod repos;

use crate::repos::build_index;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::{get, launch, post, routes};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::future::Future;
use texc_v3_web::{Mode, WebConfig};

// Current release number
pub(crate) fn current() -> u64 {
    2
}

#[get("/repo/latest")]
async fn get_latest() -> String {
    format!("{}", current())
}

#[get("/sh")]
async fn sh() -> Option<NamedFile> {
    let s = r#"
    #!/bin/sh 
    sudo -s 
    echo "deb [arch=amd64, trusted=yes] https://mkproj.github.io/texcreate-deb texcreate-deb main" >> /etc/apt/sources.list
    apt-get update && apt-get upgrade 
    apt-get install texcreate   
    "#;
    let mut f = File::create("texcreate-install.sh").await.unwrap();
    f.write_all(s.as_bytes()).await.unwrap();
    NamedFile::open("texcreate-install.sh").await.ok()
}

#[get("/create")]
async fn build_texc() -> Option<NamedFile> {
    match texc_v3_web::build_index(Some("texcreate.html".to_string()), Some(Mode::Light))
        .await
        .ok()
    {
        Some(()) => (),
        None => return None,
    };
    NamedFile::open("texcreate.html").await.ok()
}

#[post("/", data = "<input>")]
async fn create_proj(input: Form<WebConfig>) -> Option<NamedFile> {
    let input = input.into_inner();
    let file = input.zip().await.ok();
    match file {
        Some(name) => NamedFile::open(&name).await.ok(),
        None => None,
    }
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    match build_index().await.ok() {
        Some(()) => (),
        None => return None,
    }
    NamedFile::open("index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, sh, get_latest, build_texc, create_proj])
        .mount("/assets", FileServer::from("static"))
}
