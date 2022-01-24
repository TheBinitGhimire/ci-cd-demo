#[macro_use] extern crate rocket;
use rocket::{build, response::content::Html};

#[get("/")]
fn index() -> Html<&'static str> {
    Html("<h1 align=\"center\">\
        Hello, there!\
    </h1>\
    <h2 align=\"center\">\
        This is the Third Deployment.\
    </h2>")
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index])
}