use module_054_solutions::first_slow_sample;

fn main() {
    let samples = [12, 25, 27, 41];
    let threshold = 25;
    let found = first_slow_sample(&samples, threshold);

    println!("samples={samples:?} threshold={threshold} result={found:?}");
    assert_eq!(found, Some((2, 27)));
}
