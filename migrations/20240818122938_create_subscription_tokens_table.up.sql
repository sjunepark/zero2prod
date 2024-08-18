-- Add up migration script here
CREATE TABLE subscription_tokens
(
    subscription_token TEXT NOT NULL,
    subscription_id    UUID NOT NULL REFERENCES subscriptions (id),
    PRIMARY KEY (subscription_token)
);
