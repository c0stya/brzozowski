use brzozowski::{augment, augment_imperative};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{DistString, Standard};

fn random_string(len: usize) -> String {
    let alpha = Standard;
    let mut rng = rand::thread_rng();
    alpha.sample_string(&mut rng, len)
}

fn bench_augment(c: &mut Criterion) {
    let mut group = c.benchmark_group("Augment");
    let inputs = vec![10, 100, 1000, 10_000, 100_000]
        .into_iter()
        .map(random_string)
        .collect::<Vec<_>>();
    for input in inputs {
        let input = input.chars().collect::<Vec<char>>();
        let id = input.len();
        group.bench_with_input(BenchmarkId::new("Imperative", id), &input, |b, i| {
            b.iter(|| augment_imperative(&i.to_vec()))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", id), &input, |b, i| {
            b.iter(|| augment(i.iter().copied()).collect::<String>())
        });
    }
    group.finish();
}

criterion_group!(benches, bench_augment);
criterion_main!(benches);
