use crate::errors::Error;

use salvo::prelude::*;
use sqlx::{Pool, Postgres};
use tera::{Context, Tera};

#[handler]
// Delete a word
pub async fn delete_word(depot: &mut Depot, req: &mut Request) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap();
    let mut context = Context::new();

    let db_pool = depot.obtain::<Pool<Postgres>>().unwrap();

    let id = match req.param::<u32>("id") {
        Some(id) => id,
        None => return Err(Error::NotFound("请提供要删除的单词的ID".into())),
    };

    sqlx::query!("DELETE FROM words WHERE id = $1", id as i32)
        .execute(db_pool)
        .await?;

    context.insert("message", "删除单词成功！");

    let response = tera.render("success.html", &context)?;

    Ok(Text::Html(response))
}
