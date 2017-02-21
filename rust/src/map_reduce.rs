// This import brings the `par_iter` method into scope.
// `par_iter` is a method on a Rayon trait, and Rayon provides an implementation of that trait for
// standard slice types.
use rayon::prelude::*;

pub struct LogEntry {
    start_time: usize, // milliseconds since epoch
    end_time:   usize, // in milliseconds since epoch
}

pub fn average_response_time(logs: &[LogEntry]) -> usize {
    let total = logs.par_iter()
        .map(|ref entry| entry.end_time - entry.start_time)
        .sum();
    total / logs.len()
}

#[test]
fn computes_average_response_time() {
    let logs = vec![
        LogEntry { start_time: 1487555845895, end_time: 1487555846000 },
        LogEntry { start_time: 1487555845897, end_time: 1487555845947 },
    ];
    assert_eq!(77, average_response_time(&logs));
}
