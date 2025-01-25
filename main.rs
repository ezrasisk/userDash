use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Widget {
    id: String,
    title: String,
    content: String,
}

struct AppState {
    widgets: Mutex<Vec<Widget>>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/index.html"))
}

#[get("/style.css")]
async fn css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/styles.css"))
}

#[get("/main.js")]
async fn js() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/main.js"))
}

#[get("/widgets")]
async fn get_widgets(data:web::Data<AppState>) -> impl Responder {
    let widgets = data.widgets.lock().unwrap().clone();
    HttpResponse::Ok().json(widgets)
}

#[post("/widgets")]
async fn create_widget(data:web::Data<AppState>, item: web::Json<Widget>) -> impl Responder {
    let mut widgets = data.widgets.lock().unwrap();
    widgets.push(item.into_inner());
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        widgets: Mutex::new(vec![Widget {
            id: "1".to_string(), title: "Weather".to_string(), content: "Sunny".to_string()
        }, Widget {
            id: "2".to_string(), title: "News".to_string(), content: "Breaking News".to_string()
        }, ])
    });

    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
        .service(index)
        .service(css)
        .service(js)
        .service(get_widgets)
        .service(create_widget)
    })

    .bind("127.0.0.1:8080")?
    .run()
    .await()
}