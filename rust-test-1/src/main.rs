use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    counter: Mutex<i32>,


}

impl AppState {
    fn inc_counter(&self) -> i32{
        let mut c = self.counter.lock().unwrap();
        *c += 1;

        return *c;
    }
}

async fn index(app_state: web::Data<AppState>) -> impl Responder {
    app_state.inc_counter();

    let app_name = &app_state.app_name;
    HttpResponse::Ok().body(format!("Welcome to my web app {app_name}"))
}

#[get("/hello/{name}")]
async fn hello(path: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    app_state.inc_counter();
    
    let name = path.into_inner();
    HttpResponse::Ok().body(
        format!("Welcome {}", name)
    )
}

#[get("/counter")]
async fn counter(app_state: web::Data<AppState>) -> impl Responder {
    let c = app_state.inc_counter();

    HttpResponse::Ok().body(
        format!("Request count: {c}")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        app_name: String::from("jan-karl's test app"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(hello)
            .service(counter)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
