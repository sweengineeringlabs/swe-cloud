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

-- Secrets table
CREATE TABLE IF NOT EXISTS secrets (
    arn TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    kms_key_id TEXT,
    created_at TEXT NOT NULL,
    last_changed_date TEXT,
    last_accessed_date TEXT,
    tags TEXT,
    deleted_date TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_secrets_name ON secrets(name);

-- Secret Versions table
CREATE TABLE IF NOT EXISTS secret_versions (
    secret_arn TEXT NOT NULL,
    version_id TEXT NOT NULL,
    version_stages TEXT,
    secret_string TEXT,
    secret_binary BLOB,
    created_date TEXT NOT NULL,
    
    PRIMARY KEY (secret_arn, version_id),
    FOREIGN KEY (secret_arn) REFERENCES secrets(arn) ON DELETE CASCADE
);

-- KMS Keys table
CREATE TABLE IF NOT EXISTS kms_keys (
    id TEXT PRIMARY KEY,
    arn TEXT NOT NULL,
    description TEXT,
    key_usage TEXT DEFAULT 'ENCRYPT_DECRYPT',
    key_spec TEXT DEFAULT 'SYMMETRIC_DEFAULT',
    key_state TEXT DEFAULT 'Enabled',
    created_at TEXT NOT NULL,
    deletion_date TEXT,
    tags TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_kms_keys_arn ON kms_keys(arn);

-- Event Buses
CREATE TABLE IF NOT EXISTS event_buses (
    name TEXT PRIMARY KEY,
    arn TEXT NOT NULL,
    policy TEXT
);

-- Event Rules
CREATE TABLE IF NOT EXISTS event_rules (
    name TEXT NOT NULL,
    event_bus_name TEXT NOT NULL,
    arn TEXT NOT NULL,
    event_pattern TEXT,
    state TEXT DEFAULT 'ENABLED',
    description TEXT,
    schedule_expression TEXT,
    created_at TEXT NOT NULL,
    
    PRIMARY KEY (event_bus_name, name),
    FOREIGN KEY (event_bus_name) REFERENCES event_buses(name) ON DELETE CASCADE
);

-- Event Targets
CREATE TABLE IF NOT EXISTS event_targets (
    id TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    event_bus_name TEXT NOT NULL,
    arn TEXT NOT NULL,
    input TEXT,
    input_path TEXT,
    
    PRIMARY KEY (event_bus_name, rule_name, id),
    FOREIGN KEY (event_bus_name, rule_name) REFERENCES event_rules(event_bus_name, name) ON DELETE CASCADE
);

-- Event History
CREATE TABLE IF NOT EXISTS event_history (
    id TEXT PRIMARY KEY,
    event_bus_name TEXT NOT NULL,
    source TEXT,
    detail_type TEXT,
    detail TEXT,
    time TEXT,
    resources TEXT,
    matched_rules TEXT
);
"#;
