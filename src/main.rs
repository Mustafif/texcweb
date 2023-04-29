mod base;
mod repos;
mod blog;

use crate::repos::build_index;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::{get, launch, post, routes};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::future::Future;
use std::path::PathBuf;
use texc_v3_web::{Mode, WebConfig};

// Current release number
pub(crate) fn current() -> u64 {
    3
}

#[get("/repo/latest")]
async fn get_latest() -> String {
    format!("{}", current())
}

#[get("/sh")]
async fn sh() -> Option<NamedFile> {
    let s = r#"
    #!/bin/sh
    echo "Enter password"
    sudo -s 
    echo "deb [arch=amd64, trusted=yes] https://mkproj.github.io/texcreate-deb texcreate-deb main" >> /etc/apt/sources.list
    apt-get update && apt-get upgrade
    echo "Installing TexCreate..."
    apt-get install texcreate -y
    echo "Setting up autocomplete..."
    apt-get install bash-completion -y
    texcreate gen-complete --shell bash
    mv texcreate.bash /usr/share/bash-completion/completions/
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

#[get("/blog/<title>")]
async fn blog_(title: String) -> Option<NamedFile>{
    let file_name = format!("{title}.html");
    let path = PathBuf::from("blog_views").join(&file_name);
    NamedFile::open(path).await.ok()
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
        .mount("/", routes![index, sh, get_latest, build_texc, create_proj, blog_])
        .mount("/assets", FileServer::from("static"))
}
