use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ToDoItem {
    id: usize,
    title: String,
    completed: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let to_do_list = web::Data::new(Mutex::new(Vec::<ToDoItem>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(to_do_list.clone())
            .route("/todos", web::post().to(add_to_do_item))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn add_to_do_item(item: web::Json<ToDoItem>, to_do_list: web::Data<Mutex<Vec<ToDoItem>>>) -> impl Responder {
    let mut list = to_do_list.lock().unwrap();
    list.push(item.into_inner());
    HttpResponse::Ok().json(list)
}