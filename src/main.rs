use actix_web::{post, web, App, HttpResponse, Responder, HttpServer};
use serde::Deserialize;

// Serdeはデータの通信やプログラム間の通信で使用するためインストール
// CalcParamsという名前のデータ構造を定義し、Serdeの一部であるDeserializeトレイトを実装する
// Deserializeトレイトとは外部のデータ形式をRustに復元するためのもの、これによりJSONなどから読み込み可能となる
#[derive(Deserialize)]
pub struct CalcParams {
    a: i32, // パラメータ 'a'：整数型
    b: i32, // パラメータ 'b'：整数型
}

// '/add'エンドポイントのハンドラ関数
#[post("/add")]
async fn add(form: web::Form<CalcParams>) -> impl Responder {
    HttpResponse::Ok().body((form.a + form.b).to_string()) // 'a'と'b'を足して文字列に変換して返す
}

// '/sub'エンドポイントのハンドラ関数
#[post("/sub")]
async fn sub(form: web::Form<CalcParams>) -> impl Responder {
    HttpResponse::Ok().body((form.a - form.b).to_string()) // 'a'から'b'を引いて文字列に変換して返す
}

// '/mul'エンドポイントのハンドラ関数
#[post("/mul")]
async fn mul(form: web::Form<CalcParams>) -> impl Responder {
    HttpResponse::Ok().body((form.a * form.b).to_string()) // 'a'と'b'を掛けて文字列に変換して返す
}

// '/div'エンドポイントのハンドラ関数
#[post("/div")]
async fn div(form: web::Form<CalcParams>) -> impl Responder {
    if form.b == 0 {
        return HttpResponse::BadRequest().body("Cannot divide by zero"); // ゼロで割ることはできない場合、BadRequestを返す
    }
    HttpResponse::Ok().body((form.a / form.b).to_string()) // 'a'を'b'で割って文字列に変換して返す
}

// アプリケーションのエントリーポイント
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Actix HTTPサーバーを構築し、指定したエンドポイントをサービスとして登録する
    HttpServer::new(|| {
        App::new()
            .service(add) // 'add'エンドポイントのハンドラを登録
            .service(sub) // 'sub'エンドポイントのハンドラを登録
            .service(mul) // 'mul'エンドポイントのハンドラを登録
            .service(div) // 'div'エンドポイントのハンドラを登録
    })
    .bind("127.0.0.1:8080")? // サーバーを指定のアドレスとポートにバインド
    .run() // サーバーを実行
    .await // イベントループの終了を待機
}
