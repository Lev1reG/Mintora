-- Migration: create_refresh_tokens
-- Description: Create refresh_tokens table for JWT refresh token rotation
-- Date: 2025-12-02

CREATE TABLE "refresh_tokens" (
  "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "token_hash" varchar(255) NOT NULL UNIQUE,
  "device_info" varchar(255),
  "ip_address" varchar(45),
  "user_agent" text,
  "expires_at" timestamp NOT NULL,
  "revoked_at" timestamp,
  "created_at" timestamp NOT NULL DEFAULT now(),
  "last_used_at" timestamp,

  CONSTRAINT fk_refresh_tokens_user FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

COMMENT ON TABLE "refresh_tokens" IS 'JWT refresh tokens for secure token rotation and session management';
COMMENT ON COLUMN "refresh_tokens"."token_hash" IS 'SHA-256 hash of the refresh token';
COMMENT ON COLUMN "refresh_tokens"."device_info" IS 'Optional device identifier for multi-device tracking';
COMMENT ON COLUMN "refresh_tokens"."revoked_at" IS 'NULL = active, NOT NULL = revoked/invalidated';

-- Indexes
CREATE INDEX idx_refresh_tokens_user_id ON "refresh_tokens" ("user_id");
CREATE INDEX idx_refresh_tokens_token_hash ON "refresh_tokens" ("token_hash");
CREATE INDEX idx_refresh_tokens_expires_at ON "refresh_tokens" ("expires_at");
CREATE INDEX idx_refresh_tokens_active ON "refresh_tokens" ("user_id", "expires_at")
  WHERE revoked_at IS NULL;
