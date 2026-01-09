// Standalone test to compare serialization formats
// Run with: rustc test_serialization.rs && ./test_serialization

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

fn main() {
    println!("Comparing bincode 1.3.3 vs postcard serialization formats\n");

    let test_values = vec![
        ("Null", Value::Null),
        ("Integer(42)", Value::Integer(42)),
        ("Integer(MAX)", Value::Integer(i64::MAX)),
        ("Integer(MIN)", Value::Integer(i64::MIN)),
        ("Real(3.14159)", Value::Real(3.14159)),
        ("Text(empty)", Value::Text(String::new())),
        ("Text(Hello)", Value::Text("Hello".to_string())),
        ("Blob([1,2,3])", Value::Blob(vec![1, 2, 3])),
    ];

    for (name, value) in test_values {
        // Serialize with bincode 1.3.3
        let bincode_bytes = bincode::serialize(&value).expect("bincode serialization failed");

        // Serialize with postcard
        let postcard_bytes = postcard::to_stdvec(&value).expect("postcard serialization failed");

        println!("{:20} | bincode: {:?} ({} bytes)",
            name, bincode_bytes, bincode_bytes.len());
        println!("{:20} | postcard: {:?} ({} bytes)",
            "", postcard_bytes, postcard_bytes.len());

        // Check if formats match
        if bincode_bytes == postcard_bytes {
            println!("{:20} | ✅ FORMATS MATCH", "");
        } else {
            println!("{:20} | ❌ FORMATS DIFFER", "");
        }
        println!();
    }
}
