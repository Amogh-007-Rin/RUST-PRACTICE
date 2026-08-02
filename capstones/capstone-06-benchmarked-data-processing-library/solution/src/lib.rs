use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Record {
    pub timestamp: String,
    pub service: String,
    pub latency_ms: f64,
    pub status: u16,
    pub bytes_sent: u64,
}

#[derive(Debug, PartialEq)]
pub struct AggregatedStats {
    pub total_requests: usize,
    pub error_count: usize,
    pub avg_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub total_bytes_sent: u64,
    pub errors_by_status: Vec<(u16, usize)>,
}

pub struct DataProcessor {
    records: Vec<Record>,
}

impl DataProcessor {
    pub fn from_csv(data: &str) -> Result<Self, csv::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(data.as_bytes());
        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: Record = result?;
            records.push(record);
        }
        Ok(DataProcessor { records })
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn filter_by_service(&self, service: &str) -> Vec<&Record> {
        self.records
            .iter()
            .filter(|r| r.service == service)
            .collect()
    }

    pub fn aggregate_stats(&self) -> AggregatedStats {
        let total_requests = self.records.len();

        if total_requests == 0 {
            return AggregatedStats {
                total_requests: 0,
                error_count: 0,
                avg_latency_ms: 0.0,
                p50_latency_ms: 0.0,
                p95_latency_ms: 0.0,
                p99_latency_ms: 0.0,
                total_bytes_sent: 0,
                errors_by_status: vec![],
            };
        }

        let mut error_count: usize = 0;
        let mut total_latency: f64 = 0.0;
        let mut total_bytes: u64 = 0;
        let mut error_status_counts: BTreeMap<u16, usize> = BTreeMap::new();
        let mut latencies: Vec<f64> = Vec::with_capacity(total_requests);

        for record in &self.records {
            total_latency += record.latency_ms;
            total_bytes += record.bytes_sent;
            latencies.push(record.latency_ms);

            if record.status >= 400 {
                error_count += 1;
                *error_status_counts.entry(record.status).or_insert(0) += 1;
            }
        }

        let avg_latency_ms = total_latency / total_requests as f64;

        let p50 = percentile_from_sorted(&mut latencies, 50.0);
        let p95 = percentile_from_sorted(&mut latencies, 95.0);
        let p99 = percentile_from_sorted(&mut latencies, 99.0);

        let errors_by_status: Vec<(u16, usize)> = error_status_counts.into_iter().collect();

        AggregatedStats {
            total_requests,
            error_count,
            avg_latency_ms,
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            total_bytes_sent: total_bytes,
            errors_by_status,
        }
    }

    pub fn total_bytes_sent(&self) -> u64 {
        const CHUNK_SIZE: usize = 8;
        let bytes: Vec<u64> = self.records.iter().map(|r| r.bytes_sent).collect();
        let mut sum: u64 = 0;
        let chunks = bytes.chunks_exact(CHUNK_SIZE);
        let remainder = chunks.remainder();
        for chunk in chunks {
            sum += chunk.iter().sum::<u64>();
        }
        sum += remainder.iter().sum::<u64>();
        sum
    }

    pub fn latency_percentile(&self, percentile: f64) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let mut latencies: Vec<f64> = self.records.iter().map(|r| r.latency_ms).collect();
        percentile_from_sorted(&mut latencies, percentile)
    }
}

fn percentile_from_sorted(data: &mut [f64], percentile: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let k = ((percentile / 100.0) * data.len() as f64).ceil() as usize;
    let idx = k.saturating_sub(1).min(data.len() - 1);
    let (_, mid, _) = data.select_nth_unstable_by(idx, |a, b| a.partial_cmp(b).unwrap());
    *mid
}
