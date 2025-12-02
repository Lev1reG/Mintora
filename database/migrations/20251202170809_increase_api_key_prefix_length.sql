-- Migration: increase_api_key_prefix_length
-- Description: Increase key_prefix column size from 10 to 20 characters
-- Date: 2025-12-02

ALTER TABLE "api_keys" ALTER COLUMN "key_prefix" TYPE varchar(20);
