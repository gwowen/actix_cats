// based on code from actix examples
#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result
};

use bytes::Bytes;

// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/* handlers for various cats */
#[get("/firstcat")]
async fn firstcat(_session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/firstcat.html")))
}

#[get("/secondcat")]
async fn secondcat(_session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/secondcat.html")))
}

#[get("/thirdcat")]
async fn thirdcat(_session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/thirdcat.html")))
}

#[get("/ceilingcat")]
async fn ceilingcat(_session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/firstcat.html")))
}

/* end handlers for cats */

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        // create a new app
        App::new()
        // cookie session middleware
        .wrap(CookieSession::signed(&[0;32]).secure(false))
        // enable logger - apparently we need to register the actix-web logger last?
        .wrap(middleware::Logger::default())
        // register simple route, handle all methods
        .service(welcome)
        // register other routes
        .service(firstcat)
        .service(secondcat)
        .service(thirdcat)
        .service(ceilingcat)
        // with path parameters
        .service(web::resource("/user/{name}").route(web::get().to(with_param)))
        // async response body
        .service(
            web::resource("/async-body/{name}").route(web::get().to(response_body)),
        )
        .service(
            web::resource("/test").to(|req: HttpRequest| match *req.method(){
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound()
            }),
        )
        .service(web::resource("/error").to(|| async {
            error::InternalError::new(
                io::Error::new(io::ErrorKind::Other, "test"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }))
        // static files
        .service(fs::Files::new("/static", "static").show_files_listing())
        //redirect
        .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
            println!("{:?}", req);
            HttpResponse::Found()
            .header(header::LOCATION, "static/welcome.html")
            .finish()
        })))
        // default
        .default_service(
            // 404 for GET request
            web::resource("")
                .route(web::get().to(p404))
                // all requests that are not GET
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
        )
    })
    // want to bind to 0.0.0.0 for docker to work
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
