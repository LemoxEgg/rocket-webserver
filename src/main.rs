use std::path::{Path, PathBuf};

use rocket::{
    fs::{FileServer, NamedFile},
    get, launch,
    response::Redirect,
    routes, uri,
};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![default, en])
}

#[get("/<uri..>")]
async fn en(uri: PathBuf) -> Option<NamedFile> {
    let uri = dbg!(uri);

    let uri = Path::new("/ssd/www").join(uri);
    if uri.is_dir() {
        NamedFile::open(uri.join("index.html")).await.ok()
    } else {
        NamedFile::open(uri).await.ok()
    }
}

#[get("/")]
fn default() -> Redirect {
    Redirect::to(uri!(en(Path::new("/en"))))
}
