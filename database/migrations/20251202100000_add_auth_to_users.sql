-- Migration: add_auth_to_users
-- Description: Add password_hash column to users table for email/password authentication
-- Date: 2025-12-02

-- Add password_hash column (nullable for backward compatibility with WhatsApp-only users)
ALTER TABLE "users"
ADD COLUMN "password_hash" varchar(255);

-- Add comment
COMMENT ON COLUMN "users"."password_hash" IS 'Argon2id hash - NULL for WhatsApp-only users without dashboard access';

-- Update constraint: users with email must have password_hash (dashboard access)
-- Users without email (WhatsApp-only) should not have password_hash
ALTER TABLE "users"
ADD CONSTRAINT users_email_password_check
CHECK (
  (email IS NOT NULL AND password_hash IS NOT NULL) OR
  (email IS NULL AND password_hash IS NULL)
);
