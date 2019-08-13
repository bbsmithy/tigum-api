-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE topics(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  date_created VARCHAR NOT NULL
);

CREATE TABLE resources (
  id SERIAL PRIMARY KEY,
  content_type TEXT,
  content TEXT,
  generated_by TEXT
);

CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  note_content INTEGER [] 
);