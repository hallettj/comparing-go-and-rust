
pub fn map<A, B, F>(callback: F, xs: &[A]) -> Vec<B>
    where F: Fn(&A) -> B {
    let mut result = Vec::new();
    for x in xs {
        result.push(callback(x));
    }
    result
}

#[test]
fn maps_ints() {
    let xs = vec![1, 2, 3, 4];
    let result = map(|x| x * 2, &xs);
    assert_eq!(vec![2, 4, 6, 8], result);
}
