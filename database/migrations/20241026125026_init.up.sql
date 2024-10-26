-- Add up migration script here
-- Enable the pgcrypto extension for generating UUIDs
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Define the log_type enum
CREATE TYPE "log_type" AS ENUM (
    'MESSAGE_DELETE'
);

-- Create the guilds table with default values for timestamps and UUIDs
CREATE TABLE "guilds" (
      "id" UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
      "guild_id" TEXT NOT NULL,
      "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
      "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
      PRIMARY KEY("id")
);

-- Create the log_types table with default values for timestamps and UUIDs
CREATE TABLE "log_types" (
     "id" UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
     "guild_id" UUID NOT NULL,
     "channel_id" TEXT,
     "type" LOG_TYPE NOT NULL,
     "enabled" BOOLEAN NOT NULL,
     "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
     "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
     PRIMARY KEY("id")
);

-- Create the guild_configs table with default values for timestamps and UUIDs
CREATE TABLE "guild_configs" (
     "id" UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
     "guild_id" UUID NOT NULL,
     "default_channel_id" TEXT,
     "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
     "updated_at" TIMESTAMP NOT NULL DEFAULT NOW(),
     PRIMARY KEY("id")
);

-- Set up foreign key constraints
ALTER TABLE "guild_configs"
    ADD FOREIGN KEY("guild_id") REFERENCES "guilds"("id")
        ON UPDATE NO ACTION ON DELETE CASCADE;

ALTER TABLE "log_types"
    ADD FOREIGN KEY("guild_id") REFERENCES "guilds"("id")
        ON UPDATE NO ACTION ON DELETE CASCADE;
