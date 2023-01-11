mod errors;
mod models;
mod word;

use errors::ErrorCatcher;
use word::{delete, edit, new, print, search, view};

use salvo::{affix, prelude::*, serve_static::StaticDir, Catcher};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    let tera = Tera::new("templates/**/*").unwrap();

    // Database connection pool
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Cannot create the db pool!");

    let router = Router::new()
        .push(Router::with_path("static/<**path>").get(StaticDir::new(["static/"])))
        .hoop(affix::inject(tera))
        .hoop(affix::inject(db_pool))
        .get(view::index)
        .push(
            Router::with_path("word")
                .push(Router::with_path("<id:num>").get(view::show_word_details))
                .push(
                    Router::with_path("new")
                        .get(new::new_word_page)
                        .post(new::new_word_api),
                )
                .push(
                    Router::with_path("edit/<id:num>")
                        .get(edit::edit_word_page) // Tera page
                        .post(edit::edit_word_api), // API
                )
                .push(
                    Router::with_path("delete/<id:num>").get(delete::delete_word),
                    // 这里是GET路由，因为这个是首页单词列表的一个按钮，点进去就会删除
                    // 你也可以写一个DELETE请求的API，通过JavaScript来调用
                )
                .push(Router::with_path("search").get(search::search_word)),
        )
        .push(Router::with_path("print").get(print::print)); // Print all the words

    // With catchers
    let catchers: Vec<Box<dyn Catcher>> = vec![Box::new(ErrorCatcher)];
    // New Service
    let service = Service::new(router).with_catchers(catchers);

    tracing::info!("Listening on http://127.0.0.1:3000");

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(service)
        .await;
}
