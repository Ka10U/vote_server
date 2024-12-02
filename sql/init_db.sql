-- Créer la table des utilisateurs
CREATE TABLE voters (
    voter_id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    birth_date TEXT NOT NULL,
    status TEXT NOT NULL
);

-- Créer la table des délégations
CREATE TABLE delegations (
    from_principal TEXT NOT NULL,
    to_delegate TEXT NOT NULL,
    topics TEXT NOT NULL,
    end_date TEXT NOT NULL,
    FOREIGN KEY (from_principal) REFERENCES voters(voter_id),
    FOREIGN KEY (to_delegate) REFERENCES voters(voter_id)
);

-- Créer la table des questions de sondage
CREATE TABLE poll_questions (
    poll_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    question_type TEXT NOT NULL,
    question_topic TEXT NOT NULL,
    question_description TEXT NOT NULL,
    vote_options TEXT NOT NULL,
    votes TEXT NOT NULL,
    FOREIGN KEY (poll_id) REFERENCES polls(poll_id)
);

-- Créer la table des sondages
CREATE TABLE polls (
    creator_user_id TEXT NOT NULL,
    poll_id TEXT PRIMARY KEY,
    poll_opening_time TEXT NOT NULL,
    poll_closing_time TEXT NOT NULL,
    questions TEXT NOT NULL,
    FOREIGN KEY (creator_user_id) REFERENCES voters(voter_id)
);

-- Créer la table des résultats de sondage
CREATE TABLE poll_results (
    creator_user_id TEXT NOT NULL,
    poll_id TEXT NOT NULL,
    results TEXT NOT NULL,
    FOREIGN KEY (creator_user_id) REFERENCES voters(voter_id),
    FOREIGN KEY (poll_id) REFERENCES polls(poll_id)
);

-- Créer la table des résultats de questions
CREATE TABLE question_results (
    poll_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    vote_results TEXT NOT NULL,
    FOREIGN KEY (poll_id) REFERENCES polls(poll_id),
    FOREIGN KEY (question_id) REFERENCES poll_questions(question_id)
);

-- Créer la table des historiques de votes
CREATE TABLE vote_history (
    voter_id TEXT NOT NULL,
    vote TEXT NOT NULL,
    score INTEGER NOT NULL,
    FOREIGN KEY (voter_id) REFERENCES voters(voter_id)
);