use crate::{errors::Error, models::NewWord};

use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

#[handler]
// Only returns the page
pub async fn new_word_page(depot: &mut Depot) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let response = tera.render("word/new.html", &Context::new())?;

    Ok(Text::Html(response))
}

#[handler]
pub async fn new_word_api(depot: &mut Depot, req: &mut Request) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let new_word: NewWord = req.parse_form().await?; // 解析表单数据到struct

    sqlx::query!(
        "INSERT INTO words (word, meaning, usage) VALUES ($1, $2, $3)",
        new_word.word,
        new_word.meaning,
        new_word.usage
    )
    .execute(db_pool)
    .await?;

    let mut context = Context::new();
    context.insert("message", "新增单词成功！");

    let response = tera.render("success.html", &context)?;

    Ok(Text::Html(response))
}
