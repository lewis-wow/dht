use utils::zip;

#[test]
fn test_zip() {
    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c'];

    let result = zip(&a, &b);

    assert_eq!(result, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn test_zip_different_sizes() {
    let a = vec![1, 2];
    let b = vec!['a', 'b', 'c'];

    let result = zip(&a, &b);

    // Only the first two elements are zipped since `a` has fewer elements
    assert_eq!(result, vec![(1, 'a'), (2, 'b')]);
}
