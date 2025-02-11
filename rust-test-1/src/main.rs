use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;
use prometheus::{Counter, Opts, Registry};

struct AppState {
    app_name: String,
    mem_counter: Mutex<i32>,

    //registry: Registry,
    counter: Mutex<Counter>,
}

impl AppState {
    fn inc_counter(&self) -> (i32, f64) {
        let mut mc = self.mem_counter.lock().unwrap();
        *mc += 1;

        let c = self.counter.lock().unwrap();
        c.inc();

        return (*mc, c.get());
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

#[get("/metrics")]
async fn metrics(app_state: web::Data<AppState>) -> impl Responder {
    let (mc, c) = app_state.inc_counter();

    HttpResponse::Ok().body(
        format!("Memory counter: {mc}\nPrometheus counter: {c}")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Prometheus counter and registry
    let counter_opts = Opts::new("counter", "total number of API requests");
    let counter = Counter::with_opts(counter_opts).unwrap();
    let registry = Registry::new();
    registry.register(Box::new(counter.clone())).unwrap();

    let state = web::Data::new(AppState {
        app_name: String::from("jan-karl's test app"),
        mem_counter: Mutex::new(0),
        //registry: registry,
        counter: Mutex::new(counter),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .service(hello)
            .service(metrics)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
