use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eth_vanity::{generator::Generator, wallet::Wallet};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate one wallet", |b| {
        b.iter(|| {
            let secp = secp256k1::Secp256k1::new();
            Wallet::generate(&secp);
        });
    });

    c.bench_function("generate wallets until one matches 'aaaa'", |b| {
        b.iter(|| {
            let generator = Generator::new(black_box("aaaa".into()));
            generator.run();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
