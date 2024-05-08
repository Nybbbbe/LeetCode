extern crate two_sum;
use two_sum::two_sum;
use bencher::{benchmark_group, benchmark_main, Bencher};

// Benchmark test
fn benchmark_two_sum(b: &mut Bencher) {
    let nums = (1..1_000_000).collect::<Vec<_>>();
    b.iter(|| {
        let _result = two_sum(nums.clone(), 444_999);  // Assuming this pair exists
    });
}

benchmark_group!(benches, benchmark_two_sum);
benchmark_main!(benches);