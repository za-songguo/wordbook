use salvo::{prelude::*, Writer};

use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NotFound(String),
    TeraError(String),
    ParseError(String),
    InternalServerError(String),
}

#[async_trait]
impl Writer for Error {
    async fn write(
        self,
        _req: &mut salvo::Request,
        depot: &mut salvo::Depot,
        res: &mut salvo::Response,
    ) {
        let status = match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseError(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_text = match self.clone() {
            Error::NotFound(m) => m,
            Error::TeraError(m) => m,
            Error::ParseError(m) => m,
            Error::InternalServerError(m) => m,
        };

        res.set_status_code(status);

        if let Error::TeraError(e) = self {
            // We cannot use Tera
            res.render(e);
        } else {
            let tera = depot.obtain::<tera::Tera>().unwrap();
            let mut context = tera::Context::new();

            context.insert("error", &error_text);
            res.render(Text::Html(
                tera.render("error.html", &context)
                    .unwrap_or_else(|_| "无法渲染错误页面，请联系网站管理员".into()),
            ));
        }
    }
}

impl From<tokio::io::Error> for Error {
    fn from(e: tokio::io::Error) -> Self {
        tracing::error!("{:?}", e);
        Self::InternalServerError("服务器发生内部错误，请联系网站管理员".into())
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        tracing::error!("{:?}", e);
        Self::TeraError("无法渲染网页模板，请联系网站管理员".into())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("{:?}", e);
        match e {
            // SQL query not found
            sqlx::Error::RowNotFound => {
                Self::NotFound("数据未找到，请检查地址是否指向有效资源".into())
            }
            _ => Self::InternalServerError("服务器发生内部错误，请联系网站管理员".into()),
        }
    }
}

impl From<salvo::http::ParseError> for Error {
    fn from(e: salvo::http::ParseError) -> Self {
        tracing::error!("{:?}", e);
        Self::ParseError("无法解析数据，请检查你的数据格式".into())
    }
}

// Error catcher
pub struct ErrorCatcher;
impl salvo::Catcher for ErrorCatcher {
    fn catch(&self, _req: &Request, _depot: &Depot, res: &mut Response) -> bool {
        let tera = tera::Tera::new("templates/**/*").unwrap();
        let mut context = tera::Context::new();

        if let Some(StatusCode::NOT_FOUND) = res.status_code() {
            context.insert("error", "找不到对应的页面或资源");

            res.render(Text::Html(
                tera.render("error.html", &context)
                    .unwrap_or_else(|_| "无法渲染错误页面，请联系网站管理员".into()),
            ));
        } else {
            context.insert("error", "未知错误，请联系网站管理员");

            res.render(Text::Html(
                tera.render("error.html", &context)
                    .unwrap_or_else(|_| "无法渲染错误页面，请联系网站管理员".into()),
            ));
        }
        true
    }
}
