Actix Cats
===================================

Actix Cats, the web app that takes you to the Planet Catulon to meet the weird and wonderful alien cats of this beauitful world!

Actually, it's just an example app I created to mess around with Rust's [Actix](http://actix.rs) framework to see how it serves static files, as well as how to build a slimmed down Docker container using the Rust Docker build image. But if you want to see interesting pictures of cats, you came to the right place, too.

## Build instructions
If you just want to see what the app does, just run

`cargo run`

in the directory root, and then navigate to the [Catulon Embassy homepage](http://0.0.0.0:8080/welcome) to click around the app. 

To build the docker image, run

`docker build -t actix_cats .`

and then run 

`docker run -it --rm -p 8080:8080 --name actix_cats_running actix_cats`

to start up a Docker container with the app.
