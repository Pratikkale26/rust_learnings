#[route("GET")]
fn home() {
    println!("Welcome to the home page!");
}

#[route("POST")]
fn create_post() {
    println!("Creating a new post!");
}