use axum::response::Html;

pub async fn root() -> Html<String> {
    let html = tokio::fs::read_to_string("frontend/dist/index.html")
        .await
        .expect("index.html not found");
    Html(html)
}
