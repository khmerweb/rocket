#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Vercel with Rocket!"
}

#[launch]
fn rocket() -> _ {
    // Vercel requires the app to listen on the port specified by the PORT environment variable
    // and the address to be 0.0.0.0.
    // Rocket automatically handles this if the ROCKET_PORT and ROCKET_ADDRESS env vars are set.
    rocket::build().mount("/", routes![index])
}