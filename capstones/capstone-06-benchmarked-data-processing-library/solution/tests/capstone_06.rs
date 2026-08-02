use capstone_06_solution::DataProcessor;

fn test_csv() -> &'static str {
    "timestamp,service,latency_ms,status,bytes_sent\n\
     2024-01-01T00:00:00Z,auth,12.5,200,1024\n\
     2024-01-01T00:00:01Z,auth,250.0,500,2048\n\
     2024-01-01T00:00:02Z,api,5.3,200,512\n\
     2024-01-01T00:00:03Z,api,150.7,404,256\n\
     2024-01-01T00:00:04Z,db,45.0,200,4096\n\
     2024-01-01T00:00:05Z,auth,300.5,503,128\n\
     2024-01-01T00:00:06Z,api,8.0,200,1024\n"
}

fn empty_csv() -> &'static str {
    "timestamp,service,latency_ms,status,bytes_sent\n"
}

#[test]
fn parse_csv_correctly() {
    let processor = DataProcessor::from_csv(test_csv()).expect("should parse CSV");
    let records = processor.records();
    assert_eq!(records.len(), 7);

    assert_eq!(records[0].timestamp, "2024-01-01T00:00:00Z");
    assert_eq!(records[0].service, "auth");
    assert_eq!(records[0].latency_ms, 12.5);
    assert_eq!(records[0].status, 200);
    assert_eq!(records[0].bytes_sent, 1024);

    assert_eq!(records[3].service, "api");
    assert_eq!(records[3].status, 404);
}

#[test]
fn filter_by_service() {
    let processor = DataProcessor::from_csv(test_csv()).expect("should parse CSV");
    let auth_records = processor.filter_by_service("auth");
    assert_eq!(auth_records.len(), 3);
    assert!(auth_records.iter().all(|r| r.service == "auth"));

    let api_records = processor.filter_by_service("api");
    assert_eq!(api_records.len(), 3);

    let db_records = processor.filter_by_service("db");
    assert_eq!(db_records.len(), 1);

    let none = processor.filter_by_service("nonexistent");
    assert!(none.is_empty());
}

#[test]
fn aggregate_stats() {
    let processor = DataProcessor::from_csv(test_csv()).expect("should parse CSV");
    let stats = processor.aggregate_stats();

    assert_eq!(stats.total_requests, 7);
    assert_eq!(stats.error_count, 3);

    let expected_avg = (12.5 + 250.0 + 5.3 + 150.7 + 45.0 + 300.5 + 8.0) / 7.0;
    assert!((stats.avg_latency_ms - expected_avg).abs() < 0.001);

    let expected_total_bytes: u64 = 1024 + 2048 + 512 + 256 + 4096 + 128 + 1024;
    assert_eq!(stats.total_bytes_sent, expected_total_bytes);

    let mut expected_errors: Vec<(u16, usize)> = vec![(404, 1), (500, 1), (503, 1)];
    expected_errors.sort();
    let mut actual_errors = stats.errors_by_status.clone();
    actual_errors.sort();
    assert_eq!(actual_errors, expected_errors);
}

#[test]
fn total_bytes_sent() {
    let processor = DataProcessor::from_csv(test_csv()).expect("should parse CSV");
    let total = processor.total_bytes_sent();
    assert_eq!(total, 1024 + 2048 + 512 + 256 + 4096 + 128 + 1024);
}

#[test]
fn percentile_calculations() {
    let processor = DataProcessor::from_csv(test_csv()).expect("should parse CSV");

    let p50 = processor.latency_percentile(50.0);
    let p95 = processor.latency_percentile(95.0);
    let p99 = processor.latency_percentile(99.0);

    assert!(p50 > 0.0);
    assert!(p95 > 0.0);
    assert!(p99 > 0.0);
    assert!(p99 >= p95);
    assert!(p95 >= p50);

    assert_eq!(p50, 45.0);
    assert_eq!(p95, 300.5);
    assert_eq!(p99, 300.5);
}

#[test]
fn empty_csv_handling() {
    let processor = DataProcessor::from_csv(empty_csv()).expect("should parse empty CSV");
    assert!(processor.records().is_empty());

    let stats = processor.aggregate_stats();
    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.avg_latency_ms, 0.0);
    assert!(stats.errors_by_status.is_empty());

    assert_eq!(processor.total_bytes_sent(), 0);
    assert_eq!(processor.latency_percentile(50.0), 0.0);
}

#[test]
fn error_status_counting() {
    let csv = "timestamp,service,latency_ms,status,bytes_sent\n\
               2024-01-01T00:00:00Z,auth,10.0,400,100\n\
               2024-01-01T00:00:01Z,auth,10.0,400,100\n\
               2024-01-01T00:00:02Z,auth,10.0,500,100\n\
               2024-01-01T00:00:03Z,auth,10.0,404,100\n\
               2024-01-01T00:00:04Z,auth,10.0,404,100\n\
               2024-01-01T00:00:05Z,auth,10.0,200,100\n";

    let processor = DataProcessor::from_csv(csv).expect("should parse CSV");
    let stats = processor.aggregate_stats();

    assert_eq!(stats.total_requests, 6);
    assert_eq!(stats.error_count, 5);

    let mut errors = stats.errors_by_status;
    errors.sort();
    assert_eq!(errors, vec![(400, 2), (404, 2), (500, 1)]);
}
