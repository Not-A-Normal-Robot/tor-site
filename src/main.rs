use std::os::unix::net::UnixListener;

use actix_web::{App, HttpResponseBuilder, HttpServer, Responder, get, http::StatusCode};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK).body(
        "Forms FORM-29827281-12:\n\
        Test Assessment Report\n\n\n\
        This was a triumph.\n\
        I'm making a note here:\n\
        HUGE SUCCESS.\n\
        It's hard to overstate\n\
        my satisfaction.\n\
        Aperture Science\n\
        We do what we must\n
        because we can.\n\
        For the good of all uf us.\n\
        Except the ones who are dead.\n\n\
        But there's no sense crying\n\
        over every mistake.\n\
        You just keep on trying\n\
        till you run out of cake.\n\
        And the Science gets done.\n\
        And you make a neat gun.\n\
        For the people who are\n\
        still alive.",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if uzers::get_effective_uid() == 0 {
        eprintln!("refusing to run as euid 0 (root)");
        std::process::exit(1);
    } else if uzers::get_effective_gid() == 0 {
        eprintln!("refusing to run with egid 0 (root group)");
        std::process::exit(1);
    }

    let Some(uds_path) = tor_site::env::get_uds_path() else {
        eprintln!(
            "environment variable {} must be set to where you want to listen to",
            tor_site::env::VAR_NAME_UDS_PATH
        );
        std::process::exit(1);
    };

    let uds = match UnixListener::bind(uds_path) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("error trying to bind to uds: {e}");
            std::process::exit(1);
        }
    };

    HttpServer::new(|| App::new().service(greet))
        .listen_uds(uds)?
        .run()
        .await
}
