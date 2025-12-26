//! SQLite schema for the emulator

/// SQL to create all tables
pub const SCHEMA: &str = r#"
-- Buckets table
CREATE TABLE IF NOT EXISTS buckets (
    name TEXT PRIMARY KEY,
    region TEXT NOT NULL DEFAULT 'us-east-1',
    created_at TEXT NOT NULL,
    owner_id TEXT NOT NULL DEFAULT '000000000000',
    
    -- Versioning: Disabled, Enabled, Suspended
    versioning TEXT DEFAULT 'Disabled',
    
    -- JSON fields
    acl TEXT,
    policy TEXT,
    lifecycle_rules TEXT,
    cors_rules TEXT,
    notification_config TEXT,
    public_access_block TEXT,
    tags TEXT,
    
    -- Flags
    object_lock_enabled INTEGER DEFAULT 0
);

-- Objects table
CREATE TABLE IF NOT EXISTS objects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    
    -- Version info
    version_id TEXT,
    is_latest INTEGER DEFAULT 1,
    is_delete_marker INTEGER DEFAULT 0,
    
    -- Content info
    content_hash TEXT NOT NULL,
    content_length INTEGER NOT NULL,
    content_type TEXT DEFAULT 'application/octet-stream',
    content_encoding TEXT,
    cache_control TEXT,
    content_disposition TEXT,
    
    -- Metadata
    etag TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    metadata TEXT,
    storage_class TEXT DEFAULT 'STANDARD',
    
    -- Constraints
    FOREIGN KEY (bucket) REFERENCES buckets(name) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_objects_bucket_key ON objects(bucket, key);
CREATE INDEX IF NOT EXISTS idx_objects_bucket_latest ON objects(bucket, is_latest);
CREATE INDEX IF NOT EXISTS idx_objects_bucket_key_version ON objects(bucket, key, version_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_objects_unique_version ON objects(bucket, key, version_id);

-- Multipart uploads table
CREATE TABLE IF NOT EXISTS multipart_uploads (
    upload_id TEXT PRIMARY KEY,
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    initiated TEXT NOT NULL,
    metadata TEXT,
    
    FOREIGN KEY (bucket) REFERENCES buckets(name) ON DELETE CASCADE
);

-- Multipart parts table
CREATE TABLE IF NOT EXISTS multipart_parts (
    upload_id TEXT NOT NULL,
    part_number INTEGER NOT NULL,
    content_hash TEXT NOT NULL,
    size INTEGER NOT NULL,
    etag TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    
    PRIMARY KEY (upload_id, part_number),
    FOREIGN KEY (upload_id) REFERENCES multipart_uploads(upload_id) ON DELETE CASCADE
);

-- Request log (CloudTrail-like)
CREATE TABLE IF NOT EXISTS request_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    service TEXT NOT NULL,
    operation TEXT NOT NULL,
    bucket TEXT,
    key TEXT,
    status_code INTEGER,
    error_code TEXT,
    request_id TEXT NOT NULL,
    user_agent TEXT,
    source_ip TEXT
);

CREATE INDEX IF NOT EXISTS idx_request_log_timestamp ON request_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_request_log_bucket ON request_log(bucket);
"#;
