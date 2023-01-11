use crate::{errors::Error, models::NewWord};

use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

#[handler]
// Edit word page
pub async fn edit_word_page(depot: &mut Depot, req: &mut Request) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let mut context = Context::new();

    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let id = match req.param::<u32>("id") {
        Some(id) => id,
        None => return Err(Error::NotFound("请提供要修改的单词的ID".into())),
    };

    // 通过URL里的ID来获取到之前的单词信息，渲染到模板上
    let word_before_edit = sqlx::query!(
        "SELECT word, meaning, usage FROM words WHERE id = $1",
        id as i32
    )
    .fetch_one(db_pool)
    .await?;
    let word_before_edit = NewWord {
        word: word_before_edit.word,
        meaning: word_before_edit.meaning,
        usage: word_before_edit.usage,
    };

    context.insert("word_before_edit", &word_before_edit);
    // 把ID也传进去，方便表单submit的跳转到POST的API
    context.insert("word_id", &id);

    let response = tera.render("word/edit.html", &context)?;
    Ok(Text::Html(response))
}

#[handler]
// Edit word API (POST)
pub async fn edit_word_api(req: &mut Request, depot: &mut Depot) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let new_word: NewWord = req.parse_form().await?; // 解析表单数据到struct
    let id = match req.param::<u32>("id") {
        Some(id) => id,
        None => return Err(Error::NotFound("请提供要修改的单词的ID".into())),
    };

    // 更新数据
    sqlx::query!(
        "UPDATE words SET word = $1, meaning = $2, usage = $3 WHERE id = $4",
        new_word.word,
        new_word.meaning,
        new_word.usage,
        id as i32,
    )
    .execute(db_pool)
    .await?;

    let mut context = Context::new();
    context.insert("message", "修改单词成功！");

    let response = tera.render("success.html", &context)?;

    Ok(Text::Html(response))
}
