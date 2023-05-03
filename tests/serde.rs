#![cfg(all(feature = "serde", feature = "alloc"))]
#![allow(clippy::blacklisted_name)]

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Foo {
    #[serde(with = "emoji256")]
    bar: Vec<u8>,
}

#[test]
fn serialize() {
    let foo = Foo {
        bar: vec![1, 10, 100],
    };

    let ser = serde_json::to_string(&foo).expect("serialization failed");
    assert_eq!(ser, r#"{"bar":"🌈🌷💚"}"#);
}

#[test]
fn deserialize() {
    let foo = Foo {
        bar: vec![1, 10, 100],
    };

    let de: Foo = serde_json::from_str(r#"{"bar":"🌈🌷💚"}"#).expect("deserialization failed");
    assert_eq!(de, foo);
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Bar {
    #[serde(
        serialize_with = "emoji256::serialize",
        deserialize_with = "emoji256::deserialize"
    )]
    foo: Vec<u8>,
}

#[test]
fn serialize_upper() {
    let bar = Bar {
        foo: vec![1, 10, 100],
    };

    let ser = serde_json::to_string(&bar).expect("serialization failed");
    assert_eq!(ser, r#"{"foo":"🌈🌷💚"}"#);
}

#[test]
fn deserialize_upper() {
    let bar = Bar {
        foo: vec![1, 10, 100],
    };

    let de: Bar = serde_json::from_str(r#"{"foo":"🌈🌷💚"}"#).expect("deserialization failed");
    assert_eq!(de, bar);
}
