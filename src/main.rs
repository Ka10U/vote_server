use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Sqlite};
use dotenv::dotenv;
use std::env;

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

async fn save_vote(vote: web::Json<Vote>, pool: web::Data<SqlitePool>) -> impl Responder {
    let query_result = sqlx::query!(
        r#"
        INSERT INTO votes (user_id, vote)
        VALUES (?1, ?2)
        "#,
        vote.user_id,
        vote.vote
    )
    .execute(pool.get_ref())
    .await;

    match query_result {
        Ok(_) => web::Json(VoteResponse {
            id: 0, // You might want to retrieve the actual ID from the database
            user_id: vote.user_id.clone(),
            vote: vote.vote.clone(),
        }),
        Err(e) => {
            eprintln!("Failed to save vote: {}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/vote", web::post().to(save_vote))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}