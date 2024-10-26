-- Add down migration script here

-- Remove foreign key constraints
ALTER TABLE "guild_configs" DROP CONSTRAINT IF EXISTS "guild_configs_guild_id_fkey";
ALTER TABLE "log_types" DROP CONSTRAINT IF EXISTS "log_types_guild_id_fkey";

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS "guild_configs";
DROP TABLE IF EXISTS "log_types";
DROP TABLE IF EXISTS "guilds";

-- Drop the log_type enum type
DROP TYPE IF EXISTS "log_type";

-- Drop the pgcrypto extension if it's no longer needed
DROP EXTENSION IF EXISTS "pgcrypto";