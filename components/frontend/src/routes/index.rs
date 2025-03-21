use crate::prelude::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    let index = IndexTemplate;
    Html(index.render().unwrap())
}
