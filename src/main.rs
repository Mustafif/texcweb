use rocket::response::Redirect;
use rocket::fs::NamedFile;
use rocket::{launch, get, routes};
use texcreate_repo::Repo;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Current release number
fn current() -> u64{
    0
}

#[get("/repo/latest")]
async fn get_latest() -> String{
    format!("{}", current())
}

#[get("/")]
fn index() -> String{
    "Under construction".to_string()
}

#[launch]
fn rocket() -> _{
    rocket::build()
        .mount("/", routes![index, get_latest])
}
