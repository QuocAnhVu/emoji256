use criterion::{criterion_group, criterion_main, Criterion};
use rustc_hex::{FromHex, ToHex};

const DATA: &[u8] = include_bytes!("../src/lib.rs");

fn bench_encode(c: &mut Criterion) {
    c.bench_function("emoji256_encode", |b| b.iter(|| emoji256::encode(DATA)));

    c.bench_function("emoji256_encode_to_slice", |b| {
        b.iter(|| {
            let mut buf = [0u8; DATA.len() * 4];
            emoji256::encode_to_slice(DATA, &mut buf).unwrap()
        })
    });

    c.bench_function("rustc_hex_encode", |b| b.iter(|| DATA.to_hex::<String>()));
}

fn bench_decode(c: &mut Criterion) {
    c.bench_function("emoji256_decode", |b| {
        let emoji256 = emoji256::encode(DATA);
        b.iter(|| emoji256::decode(&emoji256).unwrap())
    });

    c.bench_function("emoji256_decode_to_slice", |b| {
        let emoji256 = emoji256::encode(DATA);
        b.iter(|| {
            let mut buf = [0u8; DATA.len()];
            emoji256::decode_to_slice(&emoji256, &mut buf).unwrap()
        })
    });

    c.bench_function("rustc_hex_decode", |b| {
        let hex = DATA.to_hex::<String>();
        b.iter(|| hex.from_hex::<Vec<u8>>().unwrap())
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
