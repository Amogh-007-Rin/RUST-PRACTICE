use criterion::{black_box, Criterion};
use rand::Rng;

fn generate_large_csv(rows: usize) -> String {
    let mut rng = rand::thread_rng();
    let services = ["auth", "api", "db", "cache", "queue"];
    let mut csv = String::from("timestamp,service,latency_ms,status,bytes_sent\n");
    for i in 0..rows {
        let ts = format!("2024-01-01T00:{:02}:{:02}Z", (i / 60) % 60, i % 60);
        let svc = services[rng.gen_range(0..services.len())];
        let lat = rng.gen_range(1.0..1000.0);
        let status = if rng.gen_bool(0.1) {
            rng.gen_range(400..600)
        } else {
            200
        };
        let bytes = rng.gen_range(64..65536);
        csv.push_str(&format!("{ts},{svc},{lat:.1},{status},{bytes}\n"));
    }
    csv
}

pub fn bench_parsing(c: &mut Criterion) {
    let csv_data = generate_large_csv(10_000);
    c.bench_function("parse 10k rows", |b| {
        b.iter(|| {
            let processor =
                capstone_06_solution::DataProcessor::from_csv(black_box(&csv_data)).unwrap();
            black_box(processor)
        })
    });
}

pub fn bench_aggregation(c: &mut Criterion) {
    let csv_data = generate_large_csv(10_000);
    let processor = capstone_06_solution::DataProcessor::from_csv(&csv_data).unwrap();
    c.bench_function("aggregate 10k rows", |b| {
        b.iter(|| {
            let stats = processor.aggregate_stats();
            black_box(stats)
        })
    });
}

pub fn bench_filtering(c: &mut Criterion) {
    let csv_data = generate_large_csv(10_000);
    let processor = capstone_06_solution::DataProcessor::from_csv(&csv_data).unwrap();
    c.bench_function("filter 10k rows by service", |b| {
        b.iter(|| {
            let filtered = processor.filter_by_service(black_box("auth"));
            black_box(filtered)
        })
    });
}

criterion::criterion_group!(benches, bench_parsing, bench_aggregation, bench_filtering);
criterion::criterion_main!(benches);
