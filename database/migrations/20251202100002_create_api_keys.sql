-- Migration: create_api_keys
-- Description: Create api_keys table for AI Agent authentication
-- Date: 2025-12-02

CREATE TABLE "api_keys" (
  "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "name" varchar(100) NOT NULL,
  "key_prefix" varchar(10) NOT NULL,
  "key_hash" varchar(255) NOT NULL UNIQUE,
  "scopes" jsonb NOT NULL DEFAULT '["transactions:read", "transactions:write"]',
  "last_used_at" timestamp,
  "expires_at" timestamp,
  "revoked_at" timestamp,
  "created_at" timestamp NOT NULL DEFAULT now(),
  "updated_at" timestamp NOT NULL DEFAULT now(),

  CONSTRAINT fk_api_keys_user FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

COMMENT ON TABLE "api_keys" IS 'API keys for AI Agent and programmatic access';
COMMENT ON COLUMN "api_keys"."name" IS 'Human-readable name (e.g., "AI Agent Production")';
COMMENT ON COLUMN "api_keys"."key_prefix" IS 'First 8-16 chars of key for identification (e.g., "mnt_live_abc123")';
COMMENT ON COLUMN "api_keys"."key_hash" IS 'SHA-256 hash of full API key';
COMMENT ON COLUMN "api_keys"."scopes" IS 'JSON array of permissions (future: fine-grained access control)';
COMMENT ON COLUMN "api_keys"."revoked_at" IS 'NULL = active, NOT NULL = revoked';

-- Indexes
CREATE INDEX idx_api_keys_user_id ON "api_keys" ("user_id");
CREATE INDEX idx_api_keys_key_hash ON "api_keys" ("key_hash");
CREATE INDEX idx_api_keys_active ON "api_keys" ("user_id")
  WHERE revoked_at IS NULL;
CREATE INDEX idx_api_keys_prefix ON "api_keys" ("key_prefix");

-- Unique constraint: user cannot have duplicate API key names
CREATE UNIQUE INDEX idx_api_keys_user_name ON "api_keys" ("user_id", "name")
  WHERE revoked_at IS NULL;
