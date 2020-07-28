#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};

use bytes::Bytes;

fn main() {
    println!("Hello, world!");
}
