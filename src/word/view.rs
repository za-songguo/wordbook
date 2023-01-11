use crate::errors::Error;
use crate::models::{Word, WordDetail};

use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

#[handler]
pub async fn index(depot: &mut Depot) -> Result<Text<String>, Error> {
    let mut tera = depot.obtain::<Tera>().unwrap().to_owned(); // .clone()
    tera.full_reload()?;

    let mut context = Context::new();

    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let words = sqlx::query!("SELECT id, word FROM words")
        .fetch_all(db_pool)
        .await?;

    let words = words
        .iter()
        .map(|i| Word {
            id: i.id as u32,
            word: i.word.clone(),
        })
        .collect::<Vec<Word>>();

    context.insert("words", &words);

    let response = tera.render("index.html", &context)?;

    Ok(Text::Html(response))
}

#[handler]
pub async fn show_word_details(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<Text<String>, Error> {
    // req.query::<T>(name) => /word?id=xxx
    let id = match req.param::<u32>("id") {
        Some(id) => id,
        None => {
            return Err(Error::NotFound(
                "数据未找到，请检查地址是否指向有效资源".into(),
            ))
        }
    };

    let tera = depot.obtain::<Tera>().unwrap();
    let mut context = Context::new();

    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let word = sqlx::query!("SELECT * FROM words WHERE id = $1", id as i32)
        .fetch_one(db_pool)
        .await?;

    let word = WordDetail {
        id: word.id as u32,
        word: word.word,
        meaning: word.meaning,
        usage: word.usage,
        date: word.date,
    };

    let related_words = sqlx::query!(
        "SELECT id, word FROM words WHERE usage LIKE $1 AND word != $2",
        format!("%{}%", word.word),
        word.word
    )
    .fetch_all(db_pool)
    .await?;

    let related_words = if related_words.is_empty() {
        None
    } else {
        Some(
            related_words
                .iter()
                .map(|i| Word {
                    id: i.id as u32,
                    word: i.word.clone(),
                })
                .collect::<Vec<Word>>(),
        )
    };

    context.insert("related_words", &related_words);
    context.insert("word", &word);

    let response = tera.render("word/word.html", &context)?;

    Ok(Text::Html(response))
}
