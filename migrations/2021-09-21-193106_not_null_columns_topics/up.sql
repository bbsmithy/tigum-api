-- Your SQL goes here

ALTER TABLE topics
    ALTER COLUMN date_created SET NOT NULL,
    ALTER COLUMN notes SET NOT NULL,
    ALTER COLUMN videos SET NOT NULL,
    ALTER COLUMN code SET NOT NULL,
    ALTER COLUMN article_snippets SET NOT NULL,
    ALTER COLUMN excercises SET NOT NULL,
    ALTER COLUMN images SET NOT NULL,
    ALTER COLUMN links SET NOT NULL,
    ALTER COLUMN published SET NOT NULL