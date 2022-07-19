-- Your SQL goes here
ALTER TABLE users 
ADD COLUMN profile_pic_url TEXT DEFAULT '' NOT NULL,
ADD COLUMN subdomain TEXT DEFAULT '' NOT NULL;