-- Add up migration script here
BEGIN;
-- Back fill `status` for historical entries
UPDATE newsletter.public.subscriptions
SET status = 'confirmed'
WHERE status IS NULL;
-- Make `status` column mandatory
ALTER TABLE newsletter.public.subscriptions
    ALTER COLUMN status SET NOT NULL;
COMMIT;