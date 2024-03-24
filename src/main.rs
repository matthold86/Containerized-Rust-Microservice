use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    list: Mutex<Vec<String>>, // Using String as an example item type
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        list: Mutex::new(vec![]), // Initialize your list here
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(web::resource("/add").route(web::post().to(add_item)))
            .service(web::resource("/list").route(web::get().to(get_list)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn add_item(item: web::Json<String>, data: web::Data<AppState>) -> String {
    let mut list = data.list.lock().unwrap(); // Acquire the lock
    list.push(item.into_inner());
    "Item added successfully".to_string()
}

async fn get_list(data: web::Data<AppState>) -> web::Json<Vec<String>> {
    let list = data.list.lock().unwrap(); // Acquire the lock
    web::Json(list.clone())
}
