-- Migration: change_clients_table
-- Description: Rename whatsapp_clients to clients and whatsapp_conversations to conversations
-- Purpose: Make the schema channel-agnostic to support future messaging platforms
-- Date: 2025-12-03

-- ============================================
-- Rename whatsapp_clients to clients
-- ============================================

-- Rename the table
ALTER TABLE "whatsapp_clients" RENAME TO "clients";

-- Rename constraints
ALTER TABLE "clients" RENAME CONSTRAINT "fk_whatsapp_clients_user" TO "fk_clients_user";

-- Rename indexes
ALTER INDEX "idx_whatsapp_clients_phone" RENAME TO "idx_clients_phone";
ALTER INDEX "idx_whatsapp_clients_user_id" RENAME TO "idx_clients_user_id";
ALTER INDEX "idx_whatsapp_clients_user_active" RENAME TO "idx_clients_user_active";

-- Add channel column to support multiple messaging platforms
ALTER TABLE "clients" ADD COLUMN "channel" varchar(20) NOT NULL DEFAULT 'whatsapp';

-- Add constraint to ensure only valid channels are allowed
ALTER TABLE "clients" ADD CONSTRAINT "clients_channel_check" CHECK (channel IN ('whatsapp', 'telegram', 'line', 'discord', 'slack'));

-- Create index for channel filtering
CREATE INDEX "idx_clients_channel" ON "clients" ("channel");

-- Update table comment to be channel-agnostic
COMMENT ON TABLE "clients" IS 'Messaging clients (WhatsApp, Telegram, etc.) linked to user accounts';
COMMENT ON COLUMN "clients"."phone_number" IS 'E.164 format: +628123456789 (or platform-specific identifier)';
COMMENT ON COLUMN "clients"."channel" IS 'Messaging platform: whatsapp, telegram, line, discord, slack';
COMMENT ON COLUMN "clients"."last_interaction_at" IS 'Last message timestamp from any channel';

-- ============================================
-- Rename whatsapp_conversations to conversations
-- ============================================

-- Rename the table
ALTER TABLE "whatsapp_conversations" RENAME TO "conversations";

-- Rename the foreign key column reference
ALTER TABLE "conversations" RENAME COLUMN "whatsapp_client_id" TO "client_id";

-- Rename constraints
ALTER TABLE "conversations" RENAME CONSTRAINT "fk_whatsapp_conversations_user" TO "fk_conversations_user";
ALTER TABLE "conversations" RENAME CONSTRAINT "fk_whatsapp_conversations_client" TO "fk_conversations_client";
ALTER TABLE "conversations" RENAME CONSTRAINT "fk_whatsapp_conversations_transaction" TO "fk_conversations_transaction";
ALTER TABLE "conversations" RENAME CONSTRAINT "whatsapp_conversations_direction_check" TO "conversations_direction_check";
ALTER TABLE "conversations" RENAME CONSTRAINT "whatsapp_conversations_confidence_check" TO "conversations_confidence_check";

-- Rename indexes
ALTER INDEX "idx_whatsapp_conversations_user_created" RENAME TO "idx_conversations_user_created";
ALTER INDEX "idx_whatsapp_conversations_transaction" RENAME TO "idx_conversations_transaction";
ALTER INDEX "idx_whatsapp_conversations_client" RENAME TO "idx_conversations_client";
ALTER INDEX "idx_whatsapp_conversations_intent" RENAME TO "idx_conversations_intent";

-- Update table and column comments to be channel-agnostic
COMMENT ON TABLE "conversations" IS 'Track messaging conversations across all channels for context, debugging, and AI improvement';
COMMENT ON COLUMN "conversations"."message_id" IS 'Message ID from the messaging platform';
COMMENT ON COLUMN "conversations"."direction" IS 'inbound or outbound';
COMMENT ON COLUMN "conversations"."intent" IS 'log_expense, query_balance, edit_transaction, etc.';
COMMENT ON COLUMN "conversations"."extracted_data" IS 'What AI extracted from message';
COMMENT ON COLUMN "conversations"."confidence_score" IS '0.00 to 1.00';
COMMENT ON COLUMN "conversations"."transaction_id" IS 'If transaction was created from this conversation';
