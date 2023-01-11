use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

use crate::errors::Error;
use crate::models::WordDetail;

#[handler]
pub async fn print(depot: &Depot) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let mut context = Context::new();

    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let words = sqlx::query!("SELECT * FROM words")
        .fetch_all(db_pool)
        .await?;

    let mut words = words
        .iter()
        .map(|i| WordDetail {
            id: i.id as u32,
            word: i.word.clone(),
            meaning: i.meaning.clone(),
            usage: i.usage.clone(),
            date: i.date,
        })
        .collect::<Vec<WordDetail>>();
    // 按照单词的字母顺序排序
    words.sort_by_key(|i| i.word.clone());

    context.insert("words", &words);

    let response = tera.render("print.html", &context)?;

    Ok(Text::Html(response))
}
