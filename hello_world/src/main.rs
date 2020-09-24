#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/hello/<_name>")]
fn hi(_name: String) -> &'static str {
    "Hello, worldz!"
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello, hi]).launch();
}
