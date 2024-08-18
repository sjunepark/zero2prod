-- Add down migration script here
BEGIN;
-- Make `status` column nullable
ALTER TABLE newsletter.public.subscriptions
    ALTER COLUMN status DROP NOT NULL;
COMMIT;

