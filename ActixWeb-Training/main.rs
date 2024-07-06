use actix_web::{get, post,web, web::scope, App, HttpResponse,HttpRequest , HttpServer, Responder};

struct AppState {
    app_name: String,
}

/**
* HTTPメソッドでデータを送信したり、受信したりするぞ
*/







// HTTP GET request to "/"
/* #[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} */

// HTTP POST request to "/echo"
/* #[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
 */
// HTTP GET request to "/hey"
/* async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
 */

async fn index() -> impl Responder {
    "Hello world!"
}

async fn app_name(data: web::Data<AppState>) -> impl Responder {
    format!("App Name: {}", &data.app_name)
}

// service関数でルーティングを設定する
// route関数でルーティングを設定する。
// どちらも同じことができるが、route関数はメソッドを指定できる

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix-web"),
            }))
            .service(
                scope("/html")
                    .route("/index.html", web::get().to(index))
                    // この場合、"/html/index.html"でアクセスする
            )
            /* .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) */
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}