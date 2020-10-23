#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::content::Json;
use rocket::response::NamedFile;
use std::io;
use std::process::Command;

#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Request, Response};
use std::io::Cursor;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
#[get("/hammer/<threads>")]
fn hammer(threads: u64) -> Json<&'static str> {
    let _hammer = match Command::new("sh")
        .arg("-c")
        .arg(format!("stress_cpu_x64 {}", threads))
        .spawn()
    {
        Ok(_) => return Json(r#"{"status":"success"}"#),
        Err(_) => return Json(r#"{"status":"Error"}"#),
    };
}

#[get("/hammer/kill")]
fn kill() -> Json<&'static str> {
    let _kill = match Command::new("sh")
        .arg("-c")
        .arg("killall stress_cpu_x64")
        .output()
    {
        Ok(_) => return Json(r#"{"status":"killed"}"#),
        Err(_) => return Json(r#"{"status":"Failed"}"#),
    };
}
#[get("/app.js")]
fn load_js() -> io::Result<NamedFile> {
    return NamedFile::open("js/app.js");
}

fn main() {
    rocket::ignite()
        .attach(CORS())
        .mount("/api/v1", routes![hammer, kill])
        .mount("/js", routes![load_js])
        .launch();
}
