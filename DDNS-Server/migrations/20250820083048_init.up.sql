-- Add up migration script here
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA busy_timeout = 5000;

-- 使用者表（示意）
CREATE TABLE IF NOT EXISTS users
(
    id           TEXT PRIMARY KEY,    -- UUID string
    username     TEXT UNIQUE NOT NULL,
    password_phc TEXT        NOT NULL -- argon2 PHC 字串
);

-- 追蹤裝置（可選，用於統計/稽核）
CREATE TABLE IF NOT EXISTS devices
(
    id         TEXT PRIMARY KEY, -- UUID string
    user_id    TEXT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    label      TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Refresh Token（只存 hash）
CREATE TABLE IF NOT EXISTS refresh_tokens
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id        TEXT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    device_id      TEXT      NOT NULL REFERENCES devices (id) ON DELETE CASCADE,
    token_hash     BLOB      NOT NULL UNIQUE, -- 32 bytes SHA-256(raw)
    issued_at      TIMESTAMP NOT NULL,
    expires_at     TIMESTAMP NOT NULL,
    rotated_from   INTEGER   NULL REFERENCES refresh_tokens (id) ON DELETE SET NULL,
    revoked_at     TIMESTAMP NULL,
    revoked_reason TEXT      NULL,
    last_used_at   TIMESTAMP NULL,
    user_agent     TEXT      NULL,
    ip_net         TEXT      NULL
);

-- 方便常用查詢的索引
CREATE INDEX IF NOT EXISTS idx_refresh_user ON refresh_tokens (user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_device ON refresh_tokens (device_id);
CREATE INDEX IF NOT EXISTS idx_refresh_expires ON refresh_tokens (expires_at);
CREATE INDEX IF NOT EXISTS idx_refresh_active ON refresh_tokens (user_id, revoked_at, expires_at);

-- 僅列出“有效”的 refresh token（沒過期、未撤銷）
CREATE VIEW IF NOT EXISTS v_active_refresh AS
SELECT *
FROM refresh_tokens
WHERE revoked_at IS NULL
  AND expires_at > CURRENT_TIMESTAMP;

