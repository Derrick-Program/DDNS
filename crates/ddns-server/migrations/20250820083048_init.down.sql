-- Add down migration script here
PRAGMA foreign_keys = OFF;
DROP VIEW IF EXISTS v_records_list;
DROP VIEW IF EXISTS v_active_refresh;
DROP TRIGGER IF EXISTS trg_records_updated_at;
DROP TRIGGER IF EXISTS trg_users_updated_at;
DROP TABLE IF EXISTS record_updates;
DROP TABLE IF EXISTS records;
DROP TABLE IF EXISTS refresh_tokens;
DROP TABLE IF EXISTS devices;
DROP TABLE IF EXISTS users;
-- DROP TABLE IF EXISTS zones;
PRAGMA foreign_keys = ON;
