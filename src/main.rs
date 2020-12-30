use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";

    //HttpResponse::Ok() はステータスコード200 を持つ HttpResponseBuilder という構造体を返す
    // HttpResponseBuilder の body() という関数にレスポンスのボディを渡すと HttpResponse が返ってくる
    // 戻り値の方が　Result なので Ok で包む

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
