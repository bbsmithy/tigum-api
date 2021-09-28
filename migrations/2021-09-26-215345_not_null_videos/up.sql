-- Your SQL goes here
ALTER TABLE videos
    ALTER COLUMN date_created SET NOT NULL,
    ALTER COLUMN topic_id SET NOT NULL,
    ALTER COLUMN user_id SET NOT NULL,
    ALTER COLUMN date_updated SET NOT NULL,
    ALTER COLUMN published SET NOT NULL,
    ALTER COLUMN title SET NOT NULL