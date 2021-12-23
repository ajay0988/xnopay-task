use mongodb::{Client, Collection};
use lazy_static::lazy_static;

// TODO: possibly remove lazy static reference to simplify
lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    let mongo_connection_string = "mongodb+srv://ajay:ajay@cluster0.wzxln.mongodb.net/myFirstDatabase?retryWrites=true&w=majority";
    Client::with_uri_str(&mongo_connection_string)
        .expect("Failed to initialize standalone mongo client.")
}


pub fn collection(coll_name: &str) -> Collection {
    MONGO.database("xnopay").collection(coll_name)
}