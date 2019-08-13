-- Your SQL goes here
CREATE TABLE topics(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  date_created VARCHAR NOT NULL
)