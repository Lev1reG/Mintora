-- SQL dump generated using DBML (dbml.dbdiagram.io)
-- Database: PostgreSQL
-- Generated at: 2025-12-02T07:22:48.209Z

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "email" varchar(255) UNIQUE,
  "username" varchar(255) UNIQUE,
  "full_name" varchar(255) NOT NULL,
  "role" varchar(20) NOT NULL DEFAULT 'user',
  "status" varchar(20) NOT NULL DEFAULT 'active',
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "deleted_at" timestamp
);

CREATE TABLE "whatsapp_clients" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid NOT NULL,
  "phone_number" varchar(20) UNIQUE NOT NULL,
  "country_code" varchar(5) NOT NULL DEFAULT '+62',
  "is_verified" boolean NOT NULL DEFAULT false,
  "is_active" boolean NOT NULL DEFAULT true,
  "last_interaction_at" timestamp,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "transactions" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid NOT NULL,
  "type" varchar(20) NOT NULL,
  "amount" decimal(15,2) NOT NULL,
  "currency" varchar(3) NOT NULL DEFAULT 'IDR',
  "category_id" uuid,
  "payment_method_id" uuid,
  "merchant_name" varchar(255),
  "location" varchar(255),
  "description" text,
  "transaction_date" date NOT NULL,
  "source" varchar(20) NOT NULL,
  "source_message_id" varchar(255),
  "attachment_urls" jsonb,
  "metadata" jsonb,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "deleted_at" timestamp
);

CREATE TABLE "categories" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid,
  "name" varchar(100) NOT NULL,
  "type" varchar(20) NOT NULL,
  "icon" varchar(50),
  "color" varchar(7),
  "parent_category_id" uuid,
  "is_system" boolean NOT NULL DEFAULT false,
  "is_active" boolean NOT NULL DEFAULT true,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "category_aliases" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "category_id" uuid NOT NULL,
  "alias" varchar(100) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "payment_methods" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid,
  "name" varchar(100) NOT NULL,
  "type" varchar(30) NOT NULL,
  "last_4_digits" varchar(4),
  "is_system" boolean NOT NULL DEFAULT false,
  "is_active" boolean NOT NULL DEFAULT true,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "merchants" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid,
  "name" varchar(255) NOT NULL,
  "default_category_id" uuid,
  "location" varchar(255),
  "tags" jsonb,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "tags" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid NOT NULL,
  "name" varchar(50) NOT NULL,
  "color" varchar(7),
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "transaction_tags" (
  "transaction_id" uuid NOT NULL,
  "tag_id" uuid NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  PRIMARY KEY ("transaction_id", "tag_id")
);

CREATE TABLE "whatsapp_conversations" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid NOT NULL,
  "whatsapp_client_id" uuid NOT NULL,
  "message_id" varchar(255),
  "direction" varchar(20) NOT NULL,
  "message_text" text NOT NULL,
  "intent" varchar(100),
  "extracted_data" jsonb,
  "confidence_score" decimal(3,2),
  "transaction_id" uuid,
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "audit_logs" (
  "id" uuid PRIMARY KEY DEFAULT (gen_random_uuid()),
  "user_id" uuid,
  "action" varchar(100) NOT NULL,
  "entity_type" varchar(50) NOT NULL,
  "entity_id" uuid NOT NULL,
  "old_values" jsonb,
  "new_values" jsonb,
  "ip_address" varchar(45),
  "user_agent" text,
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX ON "users" ("role");

CREATE INDEX ON "users" ("status");

CREATE INDEX ON "users" ("created_at");

CREATE INDEX ON "whatsapp_clients" ("phone_number");

CREATE INDEX ON "whatsapp_clients" ("user_id");

CREATE INDEX ON "whatsapp_clients" ("user_id", "is_active");

CREATE INDEX "idx_user_date" ON "transactions" ("user_id", "transaction_date");

CREATE INDEX "idx_user_type" ON "transactions" ("user_id", "type");

CREATE INDEX "idx_user_type_date" ON "transactions" ("user_id", "type", "transaction_date");

CREATE INDEX ON "transactions" ("category_id");

CREATE INDEX ON "transactions" ("created_at");

CREATE INDEX ON "categories" ("user_id");

CREATE INDEX ON "categories" ("type");

CREATE INDEX ON "categories" ("user_id", "type");

CREATE INDEX ON "categories" ("is_system");

CREATE INDEX ON "category_aliases" ("alias");

CREATE INDEX ON "category_aliases" ("category_id");

CREATE UNIQUE INDEX ON "category_aliases" ("category_id", "alias");

CREATE INDEX ON "payment_methods" ("user_id");

CREATE INDEX ON "payment_methods" ("type");

CREATE INDEX ON "payment_methods" ("user_id", "is_active");

CREATE INDEX ON "merchants" ("user_id");

CREATE INDEX ON "merchants" ("name");

CREATE INDEX ON "merchants" ("user_id", "name");

CREATE INDEX ON "tags" ("user_id");

CREATE UNIQUE INDEX ON "tags" ("user_id", "name");

CREATE INDEX ON "transaction_tags" ("transaction_id");

CREATE INDEX ON "transaction_tags" ("tag_id");

CREATE INDEX "idx_user_created" ON "whatsapp_conversations" ("user_id", "created_at");

CREATE INDEX ON "whatsapp_conversations" ("transaction_id");

CREATE INDEX ON "whatsapp_conversations" ("whatsapp_client_id");

CREATE INDEX ON "whatsapp_conversations" ("intent");

CREATE INDEX "idx_user_created" ON "audit_logs" ("user_id", "created_at");

CREATE INDEX "idx_entity" ON "audit_logs" ("entity_type", "entity_id");

CREATE INDEX ON "audit_logs" ("action");

CREATE INDEX ON "audit_logs" ("created_at");

COMMENT ON TABLE "users" IS 'Core user accounts for web dashboard and WhatsApp integration';

COMMENT ON COLUMN "users"."email" IS 'Optional for WhatsApp-only users';

COMMENT ON COLUMN "users"."role" IS 'admin or user';

COMMENT ON COLUMN "users"."status" IS 'active, suspended, or deleted';

COMMENT ON COLUMN "users"."deleted_at" IS 'Soft delete timestamp';

COMMENT ON TABLE "whatsapp_clients" IS 'Whitelisted WhatsApp numbers linked to user accounts';

COMMENT ON COLUMN "whatsapp_clients"."phone_number" IS 'E.164 format: +628123456789';

COMMENT ON COLUMN "whatsapp_clients"."last_interaction_at" IS 'Last WhatsApp message timestamp';

COMMENT ON TABLE "transactions" IS 'Core financial transaction records (income & expenses)';

COMMENT ON COLUMN "transactions"."type" IS 'income or expense';

COMMENT ON COLUMN "transactions"."amount" IS 'Amount in IDR';

COMMENT ON COLUMN "transactions"."merchant_name" IS 'Free-text merchant name';

COMMENT ON COLUMN "transactions"."description" IS 'User notes or AI-extracted description';

COMMENT ON COLUMN "transactions"."transaction_date" IS 'When transaction actually occurred';

COMMENT ON COLUMN "transactions"."source" IS 'whatsapp or web';

COMMENT ON COLUMN "transactions"."source_message_id" IS 'WhatsApp message ID for reference';

COMMENT ON COLUMN "transactions"."attachment_urls" IS 'Array of receipt image URLs';

COMMENT ON COLUMN "transactions"."metadata" IS 'Flexible field for future use';

COMMENT ON COLUMN "transactions"."deleted_at" IS 'Soft delete timestamp';

COMMENT ON TABLE "categories" IS 'Expense and income categories with hierarchical support';

COMMENT ON COLUMN "categories"."user_id" IS 'NULL for system categories';

COMMENT ON COLUMN "categories"."type" IS 'income, expense, or both';

COMMENT ON COLUMN "categories"."icon" IS 'Emoji or icon identifier';

COMMENT ON COLUMN "categories"."color" IS 'Hex color code: #FF5733';

COMMENT ON COLUMN "categories"."parent_category_id" IS 'For subcategories';

COMMENT ON COLUMN "categories"."is_system" IS 'System vs user-defined';

COMMENT ON TABLE "category_aliases" IS 'Help AI agent map variations to categories (e.g., "supermarket" → "Groceries")';

COMMENT ON COLUMN "category_aliases"."alias" IS 'Alternative names for AI matching';

COMMENT ON TABLE "payment_methods" IS 'Payment methods (Cash, GoPay, OVO, Cards, etc.)';

COMMENT ON COLUMN "payment_methods"."user_id" IS 'NULL for system defaults';

COMMENT ON COLUMN "payment_methods"."type" IS 'cash, card, bank_transfer, digital_wallet, other';

COMMENT ON COLUMN "payment_methods"."last_4_digits" IS 'Last 4 digits for cards';

COMMENT ON TABLE "merchants" IS 'Store vendor/merchant information for auto-categorization';

COMMENT ON COLUMN "merchants"."user_id" IS 'NULL for shared merchants';

COMMENT ON COLUMN "merchants"."default_category_id" IS 'AI learns merchant → category mapping';

COMMENT ON COLUMN "merchants"."tags" IS 'Flexible tags array';

COMMENT ON TABLE "tags" IS 'Flexible tagging system (e.g., business, personal, tax-deductible)';

COMMENT ON COLUMN "tags"."color" IS 'Hex color code';

COMMENT ON TABLE "transaction_tags" IS 'Many-to-many relationship between transactions and tags';

COMMENT ON TABLE "whatsapp_conversations" IS 'Track WhatsApp chat sessions for context, debugging, and AI improvement';

COMMENT ON COLUMN "whatsapp_conversations"."message_id" IS 'WhatsApp message ID';

COMMENT ON COLUMN "whatsapp_conversations"."direction" IS 'inbound or outbound';

COMMENT ON COLUMN "whatsapp_conversations"."intent" IS 'log_expense, query_balance, edit_transaction, etc.';

COMMENT ON COLUMN "whatsapp_conversations"."extracted_data" IS 'What AI extracted from message';

COMMENT ON COLUMN "whatsapp_conversations"."confidence_score" IS '0.00 to 1.00';

COMMENT ON COLUMN "whatsapp_conversations"."transaction_id" IS 'If transaction was created';

COMMENT ON TABLE "audit_logs" IS 'Track all important actions for security, debugging, and compliance';

COMMENT ON COLUMN "audit_logs"."user_id" IS 'NULL for system actions';

COMMENT ON COLUMN "audit_logs"."action" IS 'create_transaction, delete_user, update_budget, etc.';

COMMENT ON COLUMN "audit_logs"."entity_type" IS 'transaction, user, category, etc.';

COMMENT ON COLUMN "audit_logs"."old_values" IS 'Before state';

COMMENT ON COLUMN "audit_logs"."new_values" IS 'After state';

ALTER TABLE "whatsapp_clients" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("category_id") REFERENCES "categories" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("payment_method_id") REFERENCES "payment_methods" ("id");

ALTER TABLE "categories" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "categories" ADD FOREIGN KEY ("parent_category_id") REFERENCES "categories" ("id");

ALTER TABLE "category_aliases" ADD FOREIGN KEY ("category_id") REFERENCES "categories" ("id");

ALTER TABLE "payment_methods" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "merchants" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "merchants" ADD FOREIGN KEY ("default_category_id") REFERENCES "categories" ("id");

ALTER TABLE "tags" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "transaction_tags" ADD FOREIGN KEY ("transaction_id") REFERENCES "transactions" ("id");

ALTER TABLE "transaction_tags" ADD FOREIGN KEY ("tag_id") REFERENCES "tags" ("id");

ALTER TABLE "whatsapp_conversations" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "whatsapp_conversations" ADD FOREIGN KEY ("whatsapp_client_id") REFERENCES "whatsapp_clients" ("id");

ALTER TABLE "whatsapp_conversations" ADD FOREIGN KEY ("transaction_id") REFERENCES "transactions" ("id");

ALTER TABLE "audit_logs" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
