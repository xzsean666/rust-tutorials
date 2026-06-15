//! 用 axum 构建一个最小 HTTP 服务。

use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn root() -> &'static str {
    "Rust Tutorial API"
}

/// 路径参数 `{name}` 被提取进 handler。
async fn greet(Path(name): Path<String>) -> Json<Greeting> {
    Json(Greeting {
        message: format!("Hello, {name}!"),
    })
}

/// 把路由组装成一个 Router,方便测试时复用。
fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/greet/{name}", get(greet))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("无法绑定端口");
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app()).await.expect("服务退出");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt; // 提供 oneshot

    #[tokio::test]
    async fn root_returns_text() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Rust Tutorial API");
    }

    #[tokio::test]
    async fn greet_returns_json() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/greet/Rust")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], br#"{"message":"Hello, Rust!"}"#);
    }

    #[tokio::test]
    async fn unknown_route_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/missing")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
