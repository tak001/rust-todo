use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

// エラーハンドリングの定義
// エラーをまとめる enum を定義
// actix_web::ResponseError として使うために derive マクロで Debug を付与している必要があります
#[derive(Error, Debug)]
enum MyError {}

// actix_web::ResponseError を MyError に実装する
// 今回はデフォルトの実装をそのまま使うので、あたらに実装するものはない
impl ResponseError for MyError {}

// MyError は actix_web::ResponseError を実装しているので index の戻り値に MyError を使うことができる
// path
#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";

    //HttpResponse::Ok() はステータスコード200 を持つ HttpResponseBuilder という構造体を返す
    // HttpResponseBuilder の body() という関数にレスポンスのボディを渡すと HttpResponse が返ってくる
    // 戻り値の方が　Result なので Ok で包む

    Ok(HttpResponse::Ok().body(response_body))
}

// サーバーの起動
#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}


