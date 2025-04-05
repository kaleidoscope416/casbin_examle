use actix_casbin_auth::casbin::{DefaultModel, FileAdapter, Result};
use actix_casbin_auth::CasbinService;
use actix_web::{web, App, HttpResponse, HttpServer, Responder ,get , post};
use actix_casbin_auth::casbin::function_map::key_match2;
use actix_casbin_auth::casbin::CoreApi;
use middle::FakeAuth;
mod middle;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    
    let m = DefaultModel::from_file("model.conf")
        .await
        .unwrap();

    let a = FileAdapter::new("model.csv");

    let casbin_middleware = CasbinService::new(m, a).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        //.unwrap()
        .matching_fn(Some(key_match2), None);

    HttpServer::new(move || {
        App::new()
            .wrap(casbin_middleware.clone())
            .wrap(FakeAuth)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
