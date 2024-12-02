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
use std::env;
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
struct Delegation {
    from_principal: Uuid,
    to_delegate: Uuid,
    topics: Vec<Topic>,
    end_date: Date,
}

// #[derive(Serialize, Deserialize, Clone)]
struct Voter {
    voter_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    birth_date: Date,
    status: VoterStatus,
    delegations_received: Vec<Delegation>,
    delegations_given: Vec<Delegation>,
}

#[derive(Serialize, Deserialize, Clone)]
struct VoteHistory {
    vote: String,
    score: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct VoteOption {
    vote: String,
    vote_description: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ScoredVote {
    vote: String,
    vote_description: String,
    score: u32,
}

// #[derive(Serialize, Deserialize, Clone)]
struct PollQuestion {
    poll_id: Uuid,
    question_id: Uuid,
    question_type: PollType,
    question_topic: Topic,
    question_description: String,
    vote_options: Vec<VoteOption>,
    votes: Vec<ScoredVote>,
}

// #[derive(Serialize, Deserialize, Clone)]
struct Poll {
    creator_user_id: Uuid,
    poll_id: Uuid,
    poll_opening_time: Time,
    poll_closing_time: Time,
    questions: Vec<PollQuestion>,
}

// #[derive(Serialize, Deserialize, Clone)]
struct PollResult {
    creator_user_id: Uuid,
    poll_id: Uuid,
    results: Vec<QuestionResult>,
}

// #[derive(Serialize, Deserialize, Clone)]
struct QuestionResult {
    poll_id: Uuid,
    question_id: Uuid,
    vote_results: Vec<u64>,
}

// #[derive(Serialize, Deserialize, Clone)]
struct RankedChoiceVote {
    user_id: Uuid,
    poll_id: Uuid,
    vote_time: Time,
    vote: Vec<ScoredVote>,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppState {
    pool: SqlitePool,
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
        delegations_received: Vec::new(),
        delegations_given: Vec::new(),
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

async fn set_user_status(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<SetUserStatusPayload>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        UPDATE voters
        SET status = ?1
        WHERE voter_id = ?2
        "#,
        payload.status.to_string(),
        user_id.to_string()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to update user status");

    (StatusCode::OK, "User status updated")
}

async fn check_user_status(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let status = sqlx::query!(
        r#"
        SELECT status
        FROM voters
        WHERE voter_id = ?1
        "#,
        user_id.to_string()
    )
    .fetch_one(&state.pool)
    .await
    .expect("Failed to fetch user status");

    (StatusCode::OK, Json(status.status))
}

async fn add_delegation(
    State(state): State<AppState>,
    Json(payload): Json<AddDelegationPayload>,
) -> impl IntoResponse {
    let delegation = Delegation {
        from_principal: payload.from,
        to_delegate: payload.to,
        topics: payload.topics,
        end_date: payload.end_date,
    };

    // Save delegation to the database
    sqlx::query!(
        r#"
        INSERT INTO delegations (from_principal, to_delegate, topics, end_date)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        delegation.from_principal.to_string(),
        delegation.to_delegate.to_string(),
        serde_json::to_string(&delegation.topics).unwrap(),
        delegation.end_date.to_string()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to save delegation");

    (StatusCode::OK, Json(delegation))
}

async fn remove_delegation(
    State(state): State<AppState>,
    Json(payload): Json<RemoveDelegationPayload>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        DELETE FROM delegations
        WHERE from_principal = ?1 AND to_delegate = ?2 AND topics = ?3
        "#,
        payload.from.to_string(),
        payload.to.to_string(),
        serde_json::to_string(&payload.topics).unwrap()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to remove delegation");

    (StatusCode::OK, "Delegation removed")
}

async fn add_to_vote_history(
    State(state): State<AppState>,
    Json(payload): Json<AddToVoteHistoryPayload>,
) -> impl IntoResponse {
    let vote_history = VoteHistory {
        vote: payload.vote,
        score: payload.score,
    };

    // Save vote history to the database
    sqlx::query!(
        r#"
        INSERT INTO vote_history (voter_id, vote, score)
        VALUES (?1, ?2, ?3)
        "#,
        payload.voter_id.to_string(),
        vote_history.vote,
        vote_history.score
    )
    .execute(&state.pool)
    .await
    .expect("Failed to save vote history");

    (StatusCode::OK, Json(vote_history))
}

async fn create_question(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuestionPayload>,
) -> impl IntoResponse {
    let question = PollQuestion {
        poll_id: payload.poll_id,
        question_id: payload.question_id,
        question_type: payload.question_type,
        question_topic: payload.question_topic,
        question_description: payload.question_description,
        vote_options: payload.vote_options,
        votes: Vec::new(),
    };

    // Save question to the database
    sqlx::query!(
        r#"
        INSERT INTO poll_questions (poll_id, question_id, question_type, question_topic, question_description, vote_options)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        question.poll_id.to_string(),
        question.question_id.to_string(),
        question.question_type.to_string(),
        question.question_topic.to_string(),
        question.question_description,
        serde_json::to_string(&question.vote_options).unwrap()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to save question");

    (StatusCode::OK, Json(question))
}

async fn create_poll(
    State(state): State<AppState>,
    Json(payload): Json<CreatePollPayload>,
) -> impl IntoResponse {
    let poll = Poll {
        creator_user_id: payload.creator_user_id,
        poll_id: payload.poll_id,
        poll_opening_time: payload.poll_opening_time,
        poll_closing_time: payload.poll_closing_time,
        questions: payload.questions,
    };

    // Save poll to the database
    sqlx::query!(
        r#"
        INSERT INTO polls (creator_user_id, poll_id, poll_opening_time, poll_closing_time, questions)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        poll.creator_user_id.to_string(),
        poll.poll_id.to_string(),
        poll.poll_opening_time.to_string(),
        poll.poll_closing_time.to_string(),
        serde_json::to_string(&poll.questions).unwrap()
    )
    .execute(&state.pool)
    .await
    .expect("Failed to save poll");

    (StatusCode::OK, Json(poll))
}

async fn get_poll_participation(
    State(state): State<AppState>,
    Path(poll_id): Path<Uuid>,
) -> impl IntoResponse {
    let participation = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM votes
        WHERE poll_id = ?1
        "#,
        poll_id.to_string()
    )
    .fetch_one(&state.pool)
    .await
    .expect("Failed to fetch poll participation");

    (StatusCode::OK, Json(participation.count.unwrap_or(0)))
}

async fn get_poll_results(
    State(state): State<AppState>,
    Path(poll_id): Path<Uuid>,
) -> impl IntoResponse {
    let results = sqlx::query!(
        r#"
        SELECT *
        FROM poll_results
        WHERE poll_id = ?1
        "#,
        poll_id.to_string()
    )
    .fetch_all(&state.pool)
    .await
    .expect("Failed to fetch poll results");

    (StatusCode::OK, Json(results))
}

async fn get_question_results(
    State(state): State<AppState>,
    Path(poll_question_id): Path<Uuid>,
) -> impl IntoResponse {
    let results = sqlx::query!(
        r#"
        SELECT *
        FROM question_results
        WHERE question_id = ?1
        "#,
        poll_question_id.to_string()
    )
    .fetch_all(&state.pool)
    .await
    .expect("Failed to fetch question results");

    (StatusCode::OK, Json(results))
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
        .route("/set_user_status/:user_id", post(set_user_status))
        .route("/check_user_status/:user_id", get(check_user_status))
        .route("/add_delegation", post(add_delegation))
        .route("/remove_delegation", post(remove_delegation))
        .route("/add_to_vote_history", post(add_to_vote_history))
        .route("/create_question", post(create_question))
        .route("/create_poll", post(create_poll))
        .route("/get_poll_participation/:poll_id", get(get_poll_participation))
        .route("/get_poll_results/:poll_id", get(get_poll_results))
        .route("/get_question_results/:poll_question_id", get(get_question_results))
        .route("/", get(|| async {
            let app = create_app(App);
            let html = app.render().await;
            (StatusCode::OK, html)
        }))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

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

// #[derive(Serialize, Deserialize)]
struct SetUserStatusPayload {
    status: VoterStatus,
}

// #[derive(Serialize, Deserialize)]
struct AddDelegationPayload {
    from: Uuid,
    to: Uuid,
    topics: Vec<Topic>,
    end_date: Date,
}

// #[derive(Serialize, Deserialize)]
struct RemoveDelegationPayload {
    from: Uuid,
    to: Uuid,
    topics: Vec<Topic>,
}

// #[derive(Serialize, Deserialize)]
struct AddToVoteHistoryPayload {
    voter_id: Uuid,
    vote: String,
    score: u32,
}

// #[derive(Serialize, Deserialize)]
struct CreateQuestionPayload {
    poll_id: Uuid,
    question_id: Uuid,
    question_type: PollType,
    question_topic: Topic,
    question_description: String,
    vote_options: Vec<VoteOption>,
}

// #[derive(Serialize, Deserialize)]
struct CreatePollPayload {
    creator_user_id: Uuid,
    poll_id: Uuid,
    poll_opening_time: Time,
    poll_closing_time: Time,
    questions: Vec<PollQuestion>,
}