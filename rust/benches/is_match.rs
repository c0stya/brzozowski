use brzozowski::Expr;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_is_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_match");
    let inputs = vec![["(ba*n*(a*n)b*a)", "banana"]];
    for (idx, [input_regex, input_pattern]) in inputs.into_iter().enumerate() {
        let expr: Expr = input_regex.parse().unwrap();
        let compiled = regex::Regex::new(input_regex).unwrap();
        let id = format!("{idx:02}-{}", input_pattern);

        group.bench_with_input(
            BenchmarkId::new("Brzozowski", id.clone()),
            &(expr, input_pattern),
            |b, (expr, input_pattern)| b.iter(|| assert!(expr.is_match(black_box(input_pattern)))),
        );
        group.bench_with_input(
            BenchmarkId::new("Regex", id),
            &(compiled, input_pattern),
            |b, (compiled, input_pattern)| {
                b.iter(|| assert!(compiled.is_match(black_box(input_pattern))))
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_is_match);
criterion_main!(benches);
