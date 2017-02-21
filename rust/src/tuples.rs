use std::cmp;

// Returns a tuple
pub fn min_and_max(xs: &[isize]) -> (isize, isize) {
    let init = (xs[0], xs[0]);
    xs.iter()
        .fold(init, |(min, max), &x| (cmp::min(min, x), cmp::max(max, x)))
}

pub fn consume_tuple() {
    let xs = vec![1, 2, 3, 4, 5];
    let (min, max) = min_and_max(&xs); // unpack tuple with destructuring assignment
    assert_eq!(1, min);
    assert_eq!(5, max);
}

#[test]
fn computes_min_and_max() {
    consume_tuple();
}
