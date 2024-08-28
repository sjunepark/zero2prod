-- Add down migration script here
ALTER TABLE users
    RENAME password_hash TO password;
