//! XML generation for S3 responses

use data_plane::storage::{BucketMetadata, ListObjectsResult};

/// Generate ListAllMyBucketsResult XML
pub fn list_buckets_xml(buckets: &[BucketMetadata], owner_id: &str) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Owner>
    <ID>"#);
    xml.push_str(owner_id);
    xml.push_str(r#"</ID>
    <DisplayName>cloudemu</DisplayName>
  </Owner>
  <Buckets>"#);
    
    for bucket in buckets {
        xml.push_str("\n    <Bucket>\n      <Name>");
        xml.push_str(&escape_xml(&bucket.name));
        xml.push_str("</Name>\n      <CreationDate>");
        // Convert to ISO 8601 format
        xml.push_str(&bucket.created_at);
        xml.push_str("</CreationDate>\n    </Bucket>");
    }
    
    xml.push_str("\n  </Buckets>\n</ListAllMyBucketsResult>");
    xml
}

// Note: create_bucket_xml removed - CreateBucket returns 200 OK with empty body in most regions

/// Generate ListBucketResult XML (ListObjectsV2)
pub fn list_objects_v2_xml(result: &ListObjectsResult) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">"#);
    
    xml.push_str("\n  <Name>");
    xml.push_str(&escape_xml(&result.name));
    xml.push_str("</Name>");
    
    if let Some(ref prefix) = result.prefix {
        xml.push_str("\n  <Prefix>");
        xml.push_str(&escape_xml(prefix));
        xml.push_str("</Prefix>");
    } else {
        xml.push_str("\n  <Prefix></Prefix>");
    }
    
    xml.push_str("\n  <KeyCount>");
    xml.push_str(&result.contents.len().to_string());
    xml.push_str("</KeyCount>");
    
    xml.push_str("\n  <MaxKeys>");
    xml.push_str(&result.max_keys.to_string());
    xml.push_str("</MaxKeys>");
    
    if let Some(ref delim) = result.delimiter {
        xml.push_str("\n  <Delimiter>");
        xml.push_str(&escape_xml(delim));
        xml.push_str("</Delimiter>");
    }
    
    xml.push_str("\n  <IsTruncated>");
    xml.push_str(if result.is_truncated { "true" } else { "false" });
    xml.push_str("</IsTruncated>");
    
    if let Some(ref token) = result.continuation_token {
        xml.push_str("\n  <ContinuationToken>");
        xml.push_str(&escape_xml(token));
        xml.push_str("</ContinuationToken>");
    }
    
    if let Some(ref token) = result.next_continuation_token {
        xml.push_str("\n  <NextContinuationToken>");
        xml.push_str(&escape_xml(token));
        xml.push_str("</NextContinuationToken>");
    }
    
    for obj in &result.contents {
        xml.push_str("\n  <Contents>");
        xml.push_str("\n    <Key>");
        xml.push_str(&escape_xml(&obj.key));
        xml.push_str("</Key>");
        xml.push_str("\n    <LastModified>");
        xml.push_str(&obj.last_modified);
        xml.push_str("</LastModified>");
        xml.push_str("\n    <ETag>");
        xml.push_str(&obj.etag);
        xml.push_str("</ETag>");
        xml.push_str("\n    <Size>");
        xml.push_str(&obj.size.to_string());
        xml.push_str("</Size>");
        xml.push_str("\n    <StorageClass>");
        xml.push_str(&obj.storage_class);
        xml.push_str("</StorageClass>");
        xml.push_str("\n  </Contents>");
    }
    
    for prefix in &result.common_prefixes {
        xml.push_str("\n  <CommonPrefixes>");
        xml.push_str("\n    <Prefix>");
        xml.push_str(&escape_xml(prefix));
        xml.push_str("</Prefix>");
        xml.push_str("\n  </CommonPrefixes>");
    }
    
    xml.push_str("\n</ListBucketResult>");
    xml
}

/// Generate GetBucketVersioning response
pub fn get_bucket_versioning_xml(status: &str) -> String {
    if status == "Disabled" {
        // When versioning has never been enabled, return empty element
        return r#"<?xml version="1.0" encoding="UTF-8"?>
<VersioningConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/"/>"#.to_string();
    }
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<VersioningConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Status>{}</Status>
</VersioningConfiguration>"#,
        status
    )
}

/// Generate GetBucketLocation response
pub fn get_bucket_location_xml(region: &str) -> String {
    if region == "us-east-1" {
        // us-east-1 returns null/empty LocationConstraint
        return r#"<?xml version="1.0" encoding="UTF-8"?>
<LocationConstraint xmlns="http://s3.amazonaws.com/doc/2006-03-01/"/>"#.to_string();
    }
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<LocationConstraint xmlns="http://s3.amazonaws.com/doc/2006-03-01/">{}</LocationConstraint>"#,
        region
    )
}

/// Generate CopyObjectResult XML
pub fn copy_object_xml(etag: &str, last_modified: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<CopyObjectResult>
  <ETag>{}</ETag>
  <LastModified>{}</LastModified>
</CopyObjectResult>"#,
        etag, last_modified
    )
}

// TODO: Implement batch delete operations (DeleteObjects)
// pub fn delete_objects_xml(deleted: &[String], errors: &[(String, String, String)]) -> String

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Generate CreateMultipartUpload response XML
pub fn create_multipart_upload_xml(bucket: &str, key: &str, upload_id: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<InitiateMultipartUploadResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    <Bucket>{}</Bucket>
    <Key>{}</Key>
    <UploadId>{}</UploadId>
</InitiateMultipartUploadResult>"#,
        escape_xml(bucket), escape_xml(key), upload_id
    )
}

/// Generate CompleteMultipartUpload response XML
pub fn complete_multipart_upload_xml(bucket: &str, key: &str, etag: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<CompleteMultipartUploadResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    <Location>http://{}.s3.amazonaws.com/{}</Location>
    <Bucket>{}</Bucket>
    <Key>{}</Key>
    <ETag>{}</ETag>
</CompleteMultipartUploadResult>"#,
        escape_xml(bucket), escape_xml(key), escape_xml(bucket), escape_xml(key), etag
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("<foo>"), "&lt;foo&gt;");
        assert_eq!(escape_xml("&\"'"), "&amp;&quot;&apos;");
    }

    #[test]
    fn test_get_bucket_location_xml() {
        // us-east-1 should be empty
        let xml = get_bucket_location_xml("us-east-1");
        assert!(xml.contains("<LocationConstraint"));
        assert!(!xml.contains(">us-east-1<"));
        
        // other regions should have content
        let xml = get_bucket_location_xml("us-west-2");
        assert!(xml.contains(">us-west-2<"));
    }
    
    #[test]
    fn test_get_bucket_versioning_xml() {
        let disabled = get_bucket_versioning_xml("Disabled");
        assert!(disabled.contains("VersioningConfiguration"));
        assert!(!disabled.contains("<Status>")); // Should be empty self-closing or empty content
        
        let enabled = get_bucket_versioning_xml("Enabled");
        assert!(enabled.contains("<Status>Enabled</Status>"));
    }
}
