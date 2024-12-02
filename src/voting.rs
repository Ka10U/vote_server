use serde::{Deserialize, Serialize};
use time::{Date, Time};
use uuid::{self, Uuid};

enum PollType {
    Referendum,
    OptionalRankedChoice,
    ForcedRankedChoice,
    QuantifiedAnswers,
}

enum ReferendumOption {
    Yes,
    No,
}

enum VoterStatus {
    Public,
    Private,
}

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
}

#[derive(Serialize, Deserialize)]
struct Delegation {
    from_principal: Voter::voter_id,
    to_delegate: Voter::voter_id,
    topics: Vec<Topic>,
    end_date: Date,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct VoteHistory {
    vote: String,
    score: u32,
}

#[derive(Serialize, Deserialize)]
struct VoteOption {
    vote: String,
    vote_description: String,
}

#[derive(Serialize, Deserialize)]
struct ScoredVote {
    vote: String,
    vote_description: String,
    score: u32,
}

#[derive(Serialize, Deserialize)]
struct PollQuestion {
    poll_id: Uuid,
    question_id: Uuid,
    question_type: PollType,
    question_topic: Topic,
    question_description: String,
    vote_options: Vec<VoteOption>,
    votes: Vec<ScoredVote>,
}

#[derive(Serialize, Deserialize)]
struct Poll {
    creator_user_id: Voter::voter_id,
    poll_id: Uuid,
    poll_opening_time: Time,
    poll_closing_time: Time,
    questions: Vec<PollQuestion>,
}

#[derive(Serialize, Deserialize)]
struct PollResult {
    creator_user_id: Voter::voter_id,
    poll_id: Uuid,
    results: Vec<QuestionResult>,
}

#[derive(Serialize, Deserialize)]
struct QuestionResult {
    poll_id: Uuid,
    question_id: Uuid,
    vote_results: Vec<u64> 
}

#[derive(Serialize, Deserialize)]
struct RankedChoiceVote {
    user_id: Voter::voter_id,
    poll_id: Poll::poll_id,
    vote_time: Time,
    vote: Vec<ScoredVote>,
}

fn create_user(
    first_name: String,
    last_name: String,
    email: String,
    birth_date: Date,
    status: VoterStatus,
) -> Voter {
    Voter {
        voter_id: Uuid::new_v4(),
        first_name,
        last_name,
        email,
        birth_date,
        status,
        delegations_received: Vec!(),
        delegations_given: Vec!(),
    }
}

fn set_user_status(user_id: Uuid, status: VoterStatus) -> void {
    todo!("update user status with new status value");
}

fn check_user_status(user_id: Uuid) -> VoterStatus {
    todo!("return Voter[voter_id].status");
}

fn add_delegation(from: Uuid, to: Uuid, topics: Vec<Topic>) -> Delegation {
    todo!("push delegation to User[from].delegations_given");
    todo!("push delegation to User[to].delegations_received");
}

fn remove_delegation(from: Uuid, to: Uuid, topics: Vec<Topic>) -> void {
    todo!("remove delegation to User[from].delegations_given");
    todo!("remove delegation to User[to].delegations_received");
}

fn add_to_vote_history(voter: Uuid, ) -> void {
    todo!("when a vote is submitted, add it to the voter history");
}

fn create_question(
    poll_id: Uuid,
    question_id: Uuid,
    question_type: PollType,
    question_topic: Topic,
    question_description: String,
    vote_options: Vec<VoteOption>,
) -> PollQuestion {
    PollQuestion {
        poll_id,
        question_id,
        question_description,
        question_type,
        question_topic,
        vote_options,
        votes: Vec!(),
    }
}

fn create_poll(
    creator_user_id: Uuid,
    poll_id: Uuid,
    poll_opening_time: Time,
    poll_closing_time: Time,
    questions: Vec<PollQuestion>,
) -> Poll {
    Poll {
        creator_user_id,
        poll_id,
        poll_opening_time,
        poll_closing_time,
        questions,
    }
}

fn get_poll_participation(poll_id: Uuid) -> u64 {
    todo!("get number of votes recorded for that poll");
}

fn get_poll_results(poll_id: Uuid) -> PollResult {
    todo!("set results 'pending' if poll is not ended yet");
    todo!("return results for each question of the poll");
}

fn get_question_results(poll_question_id: Uuid) -> QuestionResult {}