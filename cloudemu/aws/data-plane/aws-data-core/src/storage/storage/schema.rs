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

-- CloudWatch Metrics
CREATE TABLE IF NOT EXISTS cw_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    namespace TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    dimensions TEXT,
    value REAL NOT NULL,
    unit TEXT,
    timestamp TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cw_metrics_name ON cw_metrics(namespace, metric_name);
CREATE INDEX IF NOT EXISTS idx_cw_metrics_time ON cw_metrics(timestamp);

-- CloudWatch Log Groups
CREATE TABLE IF NOT EXISTS cw_log_groups (
    name TEXT PRIMARY KEY,
    arn TEXT NOT NULL,
    retention_days INTEGER,
    created_at TEXT NOT NULL
);

-- CloudWatch Log Streams
CREATE TABLE IF NOT EXISTS cw_log_streams (
    name TEXT NOT NULL,
    log_group_name TEXT NOT NULL,
    arn TEXT NOT NULL,
    created_at TEXT NOT NULL,
    
    PRIMARY KEY (log_group_name, name),
    FOREIGN KEY (log_group_name) REFERENCES cw_log_groups(name) ON DELETE CASCADE
);

-- CloudWatch Log Events
CREATE TABLE IF NOT EXISTS cw_log_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    log_group_name TEXT NOT NULL,
    log_stream_name TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    message TEXT NOT NULL,
    
    FOREIGN KEY (log_group_name, log_stream_name) REFERENCES cw_log_streams(log_group_name, name) ON DELETE CASCADE
);

-- Cognito User Pools
CREATE TABLE IF NOT EXISTS cognito_user_pools (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    arn TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Cognito Groups
CREATE TABLE IF NOT EXISTS cognito_groups (
    user_pool_id TEXT NOT NULL,
    group_name TEXT NOT NULL,
    description TEXT,
    precedence INTEGER,
    created_at TEXT NOT NULL,
    
    PRIMARY KEY (user_pool_id, group_name),
    FOREIGN KEY (user_pool_id) REFERENCES cognito_user_pools(id) ON DELETE CASCADE
);

-- Cognito Users
CREATE TABLE IF NOT EXISTS cognito_users (
    user_pool_id TEXT NOT NULL,
    username TEXT NOT NULL,
    email TEXT,
    status TEXT DEFAULT 'CONFIRMED',
    enabled BOOLEAN DEFAULT 1,
    created_at TEXT NOT NULL,
    
    PRIMARY KEY (user_pool_id, username),
    FOREIGN KEY (user_pool_id) REFERENCES cognito_user_pools(id) ON DELETE CASCADE
);

-- Cognito User Attributes
CREATE TABLE IF NOT EXISTS cognito_user_attributes (
    user_pool_id TEXT NOT NULL,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    value TEXT,
    
    PRIMARY KEY (user_pool_id, username, name),
    FOREIGN KEY (user_pool_id, username) REFERENCES cognito_users(user_pool_id, username) ON DELETE CASCADE
);

-- Step Functions State Machines
CREATE TABLE IF NOT EXISTS sf_state_machines (
    arn TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    definition TEXT NOT NULL,
    role_arn TEXT NOT NULL,
    type TEXT DEFAULT 'STANDARD',
    created_at TEXT NOT NULL
);

-- Step Functions Executions
CREATE TABLE IF NOT EXISTS sf_executions (
    arn TEXT PRIMARY KEY,
    state_machine_arn TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT DEFAULT 'RUNNING',
    input TEXT,
    output TEXT,
    start_date TEXT NOT NULL,
    stop_date TEXT,
    
    FOREIGN KEY (state_machine_arn) REFERENCES sf_state_machines(arn) ON DELETE CASCADE
);

-- SQS Queues
CREATE TABLE IF NOT EXISTS sqs_queues (
    name TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    arn TEXT NOT NULL,
    created_at TEXT NOT NULL,
    visibility_timeout INTEGER DEFAULT 30,
    message_retention_period INTEGER DEFAULT 345600,
    delay_seconds INTEGER DEFAULT 0,
    receive_message_wait_time_seconds INTEGER DEFAULT 0,
    policy TEXT,
    tags TEXT
);

-- SQS Messages
CREATE TABLE IF NOT EXISTS sqs_messages (
    id TEXT PRIMARY KEY,
    queue_name TEXT NOT NULL,
    body TEXT NOT NULL,
    attributes TEXT,
    message_attributes TEXT,
    md5_body TEXT,
    sent_at TEXT NOT NULL,
    visible_at TEXT NOT NULL,
    receipt_handle TEXT,
    receive_count INTEGER DEFAULT 0,
    
    FOREIGN KEY (queue_name) REFERENCES sqs_queues(name) ON DELETE CASCADE
);

-- DynamoDB Tables
CREATE TABLE IF NOT EXISTS ddb_tables (
    name TEXT PRIMARY KEY,
    arn TEXT NOT NULL,
    status TEXT DEFAULT 'ACTIVE',
    attribute_definitions TEXT NOT NULL,
    key_schema TEXT NOT NULL,
    provisioned_throughput TEXT,
    billing_mode TEXT DEFAULT 'PAY_PER_REQUEST',
    created_at TEXT NOT NULL,
    item_count INTEGER DEFAULT 0,
    table_size_bytes INTEGER DEFAULT 0
);

-- DynamoDB Items
CREATE TABLE IF NOT EXISTS ddb_items (
    table_name TEXT NOT NULL,
    partition_key TEXT NOT NULL,
    sort_key TEXT,
    item_json TEXT NOT NULL,
    
    PRIMARY KEY (table_name, partition_key, sort_key),
    FOREIGN KEY (table_name) REFERENCES ddb_tables(name) ON DELETE CASCADE
);

-- SNS Topics
CREATE TABLE IF NOT EXISTS sns_topics (
    name TEXT PRIMARY KEY,
    arn TEXT NOT NULL UNIQUE,
    display_name TEXT,
    policy TEXT,
    tags TEXT,
    created_at TEXT NOT NULL
);

-- SNS Subscriptions
CREATE TABLE IF NOT EXISTS sns_subscriptions (
    arn TEXT PRIMARY KEY,
    topic_arn TEXT NOT NULL,
    protocol TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    owner TEXT,
    subscription_attributes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (topic_arn) REFERENCES sns_topics(arn) ON DELETE CASCADE
);

-- Lambda Functions
CREATE TABLE IF NOT EXISTS lambda_functions (
    name TEXT PRIMARY KEY,
    arn TEXT NOT NULL,
    runtime TEXT NOT NULL,
    role TEXT NOT NULL,
    handler TEXT NOT NULL,
    code_hash TEXT NOT NULL,
    description TEXT,
    timeout INTEGER DEFAULT 3,
    memory_size INTEGER DEFAULT 128,
    environment_variables TEXT,
    last_modified TEXT NOT NULL
);

-- VPCs table
CREATE TABLE IF NOT EXISTS vpc_vpcs (
    id TEXT PRIMARY KEY,
    cidr_block TEXT NOT NULL,
    state TEXT DEFAULT 'available',
    is_default INTEGER DEFAULT 0,
    tags TEXT
);

-- Subnets table
CREATE TABLE IF NOT EXISTS vpc_subnets (
    id TEXT PRIMARY KEY,
    vpc_id TEXT NOT NULL,
    cidr_block TEXT NOT NULL,
    availability_zone TEXT NOT NULL,
    state TEXT DEFAULT 'available',
    map_public_ip_on_launch INTEGER DEFAULT 0,
    tags TEXT,
    FOREIGN KEY (vpc_id) REFERENCES vpc_vpcs(id) ON DELETE CASCADE
);

-- Security Groups table
CREATE TABLE IF NOT EXISTS vpc_security_groups (
    id TEXT PRIMARY KEY,
    group_name TEXT NOT NULL,
    description TEXT,
    vpc_id TEXT NOT NULL,
    tags TEXT,
    FOREIGN KEY (vpc_id) REFERENCES vpc_vpcs(id) ON DELETE CASCADE
);

-- EC2 Instances table
CREATE TABLE IF NOT EXISTS ec2_instances (
    id TEXT PRIMARY KEY,
    image_id TEXT NOT NULL,
    instance_type TEXT NOT NULL,
    key_name TEXT,
    state TEXT DEFAULT 'running',
    private_ip TEXT,
    public_ip TEXT,
    vpc_id TEXT,
    subnet_id TEXT,
    security_groups TEXT, -- JSON array of IDs
    launch_time TEXT NOT NULL,
    tags TEXT,
    FOREIGN KEY (vpc_id) REFERENCES vpc_vpcs(id) ON DELETE SET NULL,
    FOREIGN KEY (subnet_id) REFERENCES vpc_subnets(id) ON DELETE SET NULL
);

-- EC2 Key Pairs table
CREATE TABLE IF NOT EXISTS ec2_key_pairs (
    key_name TEXT PRIMARY KEY,
    key_fingerprint TEXT NOT NULL,
    key_material TEXT,
    tags TEXT
);
"#;
