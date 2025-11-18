use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, get, launch, response::Redirect, routes, uri};

const DEFAULT_PATH: &str = "/ssd/site-portfolio/www";

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![default, files, server_en, server_fr, server_tok],
    )
}

#[get("/<uri..>")]
async fn files(uri: PathBuf) -> Option<NamedFile> {
    let uri = dbg!(uri);

    let uri = Path::new(DEFAULT_PATH).join(uri);
    if uri.is_dir() {
        NamedFile::open(uri.join("index.html")).await.ok()
    } else {
        NamedFile::open(uri).await.ok()
    }
}

#[get("/en/<file>")]
async fn server_en(file: String) -> Option<NamedFile> {
    NamedFile::open(dbg!(
        Path::new(DEFAULT_PATH).join("en").join(file + ".html")
    ))
    .await
    .ok()
}
#[get("/fr/<file>")]
async fn server_fr(file: String) -> Option<NamedFile> {
    NamedFile::open(dbg!(
        Path::new(DEFAULT_PATH).join("fr").join(file + ".html")
    ))
    .await
    .ok()
}
#[get("/tok/<file>")]
async fn server_tok(file: String) -> Option<NamedFile> {
    NamedFile::open(dbg!(
        Path::new(DEFAULT_PATH).join("tok").join(file + ".html")
    ))
    .await
    .ok()
}

#[get("/")]
fn default() -> Redirect {
    Redirect::to(uri!(files(Path::new("/en"))))
}
