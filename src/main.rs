#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<org>/<repo>/<path>")]
fn handle(org: String, repo: String, path: String) -> String {
    return "woo".into();
}

fn main() {
    rocket::ignite().mount("/", routes![handle]).launch();
}
