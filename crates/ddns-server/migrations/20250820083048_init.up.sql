-- -- Add up migration script here
PRAGMA foreign_keys = ON;
-- PRAGMA journal_mode = WAL;
-- PRAGMA synchronous = NORMAL;
-- PRAGMA busy_timeout = 5000;

-- ----------------------------
-- Users
-- ----------------------------
CREATE TABLE IF NOT EXISTS users (
  id            TEXT PRIMARY KEY,      -- UUID
  username      TEXT NOT NULL UNIQUE,
  password_phc  TEXT NOT NULL,          -- argon2 PHC string
  created_at    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER IF NOT EXISTS trg_users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
  UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TABLE IF NOT EXISTS devices (
  id          TEXT PRIMARY KEY, -- UUID
  user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  label       TEXT,             -- deviceName from client
  created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_seen_at TEXT NULL
);

CREATE INDEX IF NOT EXISTS idx_devices_user ON devices(user_id);

CREATE TABLE IF NOT EXISTS refresh_tokens (
  id               INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id          TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  device_id        TEXT NULL REFERENCES devices(id) ON DELETE SET NULL,

  token_hash       BLOB NOT NULL UNIQUE,              -- 32 bytes SHA-256(raw)
  issued_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at       TEXT NOT NULL,

  rotated_from     INTEGER NULL REFERENCES refresh_tokens(id) ON DELETE SET NULL,

  revoked_at       TEXT NULL,
  revoked_reason   TEXT NULL,

  last_used_at     TEXT NULL,
  last_ip          TEXT NULL,                          -- store as text IPv4/IPv6
  user_agent       TEXT NULL,

  CHECK (length(token_hash) = 32)
);

CREATE INDEX IF NOT EXISTS idx_refresh_user       ON refresh_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_device     ON refresh_tokens(device_id);
CREATE INDEX IF NOT EXISTS idx_refresh_expires    ON refresh_tokens(expires_at);
CREATE INDEX IF NOT EXISTS idx_refresh_active     ON refresh_tokens(user_id, revoked_at, expires_at);
CREATE INDEX IF NOT EXISTS idx_refresh_last_used  ON refresh_tokens(last_used_at);

CREATE VIEW IF NOT EXISTS v_active_refresh AS
SELECT *
FROM refresh_tokens
WHERE revoked_at IS NULL
  AND expires_at > CURRENT_TIMESTAMP;

-- ----------------------------
-- Zones (Cloudflare zones that a user can manage)
-- If you don't want zone-level control, you can skip this and store zone_id on records only.
-- ----------------------------
-- CREATE TABLE IF NOT EXISTS zones (
--   id            TEXT PRIMARY KEY,     -- internal UUID
--   owner_user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--   cf_zone_id    TEXT NOT NULL,         -- cloudflare zone id
--   name          TEXT NOT NULL,         -- e.g. example.com
--   created_at    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
--   UNIQUE(owner_user_id, cf_zone_id)
-- );

-- CREATE INDEX IF NOT EXISTS idx_zones_owner ON zones(owner_user_id);
-- CREATE UNIQUE INDEX IF NOT EXISTS uq_zones_cf_zone_id_per_owner ON zones(owner_user_id, cf_zone_id);

-- ----------------------------
-- DNS Records (what client can list/update)
-- recordId in API = records.id (internal UUID)
-- cf_record_id: Cloudflare DNS record id
-- ----------------------------
CREATE TABLE IF NOT EXISTS records (
  id             TEXT PRIMARY KEY, -- UUID (API recordId)
  owner_user_id  TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  -- zone_id        TEXT NOT NULL,     -- cloudflare zone id (keep as-is, no FK required)
  providers_record_id   TEXT NOT NULL,     -- cloudflare dns record id

  name           TEXT NOT NULL,     -- fqdn
  type           TEXT NOT NULL CHECK (type IN ('A', 'AAAA')),

  proxied        INTEGER NULL CHECK (proxied IN (0,1)),
  ttl            INTEGER NULL CHECK (ttl IS NULL OR (ttl >= 1 AND ttl <= 86400)),

  allow_update   INTEGER NOT NULL DEFAULT 1 CHECK (allow_update IN (0,1)),

  -- last known content (optional cache)
  last_content   TEXT NULL,
  last_updated_at TEXT NULL,

  created_at     TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at     TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

  UNIQUE(owner_user_id, providers_record_id)
);

CREATE TRIGGER IF NOT EXISTS trg_records_updated_at
AFTER UPDATE ON records
FOR EACH ROW
BEGIN
  UPDATE records SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE INDEX IF NOT EXISTS idx_records_owner     ON records(owner_user_id);
-- CREATE INDEX IF NOT EXISTS idx_records_zone      ON records(zone_id);
CREATE INDEX IF NOT EXISTS idx_records_name      ON records(name);
CREATE INDEX IF NOT EXISTS idx_records_allow_upd ON records(owner_user_id, allow_update);

-- ----------------------------
-- Record update audit log
-- stores every attempt (success/fail), good for security and debugging
-- ----------------------------
CREATE TABLE IF NOT EXISTS record_updates (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  record_id     TEXT NOT NULL REFERENCES records(id) ON DELETE CASCADE,
  user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  old_content   TEXT NULL,
  new_content   TEXT NOT NULL,
  detected_by   TEXT NULL CHECK (detected_by IS NULL OR detected_by IN ('ifconfig','stun','http','cloudflare-trace')),
  source_ip     TEXT NULL,
  user_agent    TEXT NULL,
  providers_request_id TEXT NULL,
  success       INTEGER NOT NULL CHECK (success IN (0,1)),
  error_code    TEXT NULL,
  error_message TEXT NULL,
  created_at    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_updates_record_time ON record_updates(record_id, created_at);
CREATE INDEX IF NOT EXISTS idx_updates_user_time   ON record_updates(user_id, created_at);

-- ----------------------------
-- Optional: housekeeping helper views
-- ----------------------------
CREATE VIEW IF NOT EXISTS v_records_list AS
SELECT
  r.id             AS recordId,
  -- r.zone_id        AS zoneId,
  r.name,
  r.type,
  r.proxied,
  r.ttl,
  r.allow_update   AS allowUpdate,
  r.last_content   AS lastContent,
  r.last_updated_at AS lastUpdatedAt
FROM records r;
