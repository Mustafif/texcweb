use rocket::response::Redirect;
use rocket::fs::NamedFile;
use rocket::{launch, get, routes};
use texcreate_repo::Repo;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Current release number
static CURR: u64 = 0;

#[get("/repo/latest")]
async fn get_latest() -> String{
    format!("{CURR}")
}

#[launch]
fn rocket() -> _{
    rocket::build()
        .mount("/", routes![get_latest])
}
