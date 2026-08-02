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
    #[allow(dead_code)]
    records: Vec<Record>,
}

impl DataProcessor {
    pub fn from_csv(_data: &str) -> Result<Self, csv::Error> {
        todo!("implement from_csv")
    }

    pub fn records(&self) -> &[Record] {
        todo!("implement records")
    }

    pub fn filter_by_service(&self, _service: &str) -> Vec<&Record> {
        todo!("implement filter_by_service")
    }

    pub fn aggregate_stats(&self) -> AggregatedStats {
        todo!("implement aggregate_stats")
    }

    pub fn total_bytes_sent(&self) -> u64 {
        todo!("implement total_bytes_sent")
    }

    pub fn latency_percentile(&self, _percentile: f64) -> f64 {
        todo!("implement latency_percentile")
    }
}
