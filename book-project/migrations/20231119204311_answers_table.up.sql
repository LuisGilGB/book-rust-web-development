-- Add up migration script here
CREATE TABLE answers
(
    id          SERIAL PRIMARY KEY,
    content     TEXT NOT NULL,
    question_id INTEGER REFERENCES questions (id)
);
