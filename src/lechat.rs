use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Sqlite};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use mod voting;

#[derive(Serialize, Deserialize)]
struct Vote {
    user_id: String,
    vote: String,
}

#[derive(Serialize)]
struct VoteResponse {
    id: i64,
    user_id: String,
    vote: String,
}

async fn save_vote(Json(vote): Json<Vote>, pool: SqlitePool) -> impl IntoResponse {
    let query_result = sqlx::query!(
        r#"
        INSERT INTO votes (user_id, vote)
        VALUES (?1, ?2)
        "#,
        vote.user_id,
        vote.vote
    )
    .execute(&pool)
    .await;

    match query_result {
        Ok(_) => (StatusCode::OK, Json(VoteResponse {
            id: 0, // You might want to retrieve the actual ID from the database
            user_id: vote.user_id.clone(),
            vote: vote.vote.clone(),
        })),
        Err(e) => {
            eprintln!("Failed to save vote: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        }
    }
}

#[component]
fn app() -> impl IntoView {
    view! {
        <div>
            <h1>"Vote Server"</h1>
            <form on:submit=|event| {
                event.prevent_default();
                // Handle form submission
            }>
                <input type="text" placeholder="User ID" />
                <input type="text" placeholder="Vote" />
                <button type="submit">"Submit"</button>
            </form>
        </div>
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app = Router::new()
        .route("/vote", post(save_vote))
        .route("/", get(|| async {
            let app = create_app(app);
            let html = app.render().await;
            (StatusCode::OK, html)
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
