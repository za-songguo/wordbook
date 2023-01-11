use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

use crate::{errors::Error, models::Word};

#[handler]
pub async fn search_word(req: &mut Request, depot: &Depot) -> Result<Text<String>, Error> {
    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();
    let tera = depot.obtain::<Tera>().unwrap();
    let mut context = Context::new();

    let query = req.query::<&str>("query").unwrap_or_default();

    let result = sqlx::query!(
        "SELECT id, word FROM words WHERE word LIKE $1 OR meaning LIKE $1 OR usage LIKE $1",
        format!("%{}%", query)
    )
    .fetch_all(db_pool)
    .await?;

    let result = if result.is_empty() {
        None
    } else {
        Some(
            result
                .iter()
                .map(|i| Word {
                    id: i.id as u32,
                    word: i.word.clone(),
                })
                .collect::<Vec<Word>>(),
        )
    };

    context.insert("result", &result);

    Ok(Text::Html(tera.render("search.html", &context)?))
}
