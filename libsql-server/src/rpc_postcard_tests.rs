#![cfg(test)]

//! Minimal smoke tests for postcard serialisation.
//! These verify the bincode → postcard migration works correctly.

use crate::query::Value;

/// Verify roundtrip works for all Value variants.
#[test]
fn test_value_roundtrip_all_variants() {
    let values = vec![
        Value::Null,
        Value::Integer(0),
        Value::Integer(i64::MAX),
        Value::Integer(i64::MIN),
        Value::Real(std::f64::consts::PI),
        Value::Real(f64::MAX),
        Value::Text(String::new()),
        Value::Text("hello 🦀".to_string()),
        Value::Blob(vec![]),
        Value::Blob(vec![0u8, 255u8]),
    ];

    for value in &values {
        let encoded = postcard::to_stdvec(value).expect("encode failed");
        let decoded: Value = postcard::from_bytes(&encoded).expect("decode failed");
        assert_eq!(format!("{:?}", value), format!("{:?}", decoded));
    }
}

/// Verify large blobs don't hit size limits.
#[test]
fn test_large_blob_roundtrip() {
    let value = Value::Blob(vec![42u8; 1_000_000]);
    let encoded = postcard::to_stdvec(&value).expect("encode failed");
    let decoded: Value = postcard::from_bytes(&encoded).expect("decode failed");
    assert!(matches!(decoded, Value::Blob(b) if b.len() == 1_000_000));
}

/// Verify postcard errors convert correctly to our error type.
#[test]
fn test_postcard_error_conversion() {
    let garbage = vec![0xff, 0xfe];
    let result: Result<Value, postcard::Error> = postcard::from_bytes(&garbage);

    if let Err(e) = result {
        let crate_error = crate::error::Error::from(e);
        assert!(
            matches!(crate_error, crate::error::Error::Internal(msg) if msg.contains("Unexpected"))
        );
    }
}
