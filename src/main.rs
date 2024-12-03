use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Sqlite};
use dotenv::dotenv;
use std::{env, fmt::Display};
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use uuid::Uuid;
use time::{Date, Time};

#[derive(Serialize, Deserialize, Clone)]
enum PollType {
    Referendum,
    OptionalRankedChoice,
    ForcedRankedChoice,
    QuantifiedAnswers,
}

#[derive(Serialize, Deserialize, Clone)]
enum ReferendumOption {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone)]
enum VoterStatus {
    Public,
    Private,
}

#[derive(Serialize, Deserialize, Clone)]
enum Topic {
    Geopolitics,
    Defense,
    Work,
    Industry,
    Family,
    Finances,
    Education,
    Research,
    Judicial,
    LawEnforcement,
    Environment,
    Energy,
    Medical,
    Culture,
    Technology,
    Sports
}

// #[derive(Serialize, Deserialize, Clone)]
struct Voter {
    voter_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    birth_date: Date,
    status: VoterStatus,
}


async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let user = Voter {
        voter_id: Uuid::new_v4(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        birth_date: payload.birth_date,
        status: payload.status,
    };

    // Save user to the database
    sqlx::query!(
        r#"
        INSERT INTO voters (voter_id, first_name, last_name, email, birth_date, status)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        user.voter_id.to_string(),
        user.first_name,
        user.last_name,
        user.email,
        user.birth_date.to_string(),
        user.status.to_string()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to save user");

    (StatusCode::OK, Json(user))
}

#[component]
fn App() -> impl IntoView {
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

    let state = AppState { pool };

    let app = Router::new()
        .route("/create_user", post(create_user))
        .route("/", get(|| async {
            let app = create_app(App);
            let html = app.render().await;
            (StatusCode::OK, html)
        }))
        .with_state(state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();

    Ok(())
}

// #[derive(Serialize, Deserialize)]
struct CreateUserPayload {
    first_name: String,
    last_name: String,
    email: String,
    birth_date: Date,
    status: VoterStatus,
}
