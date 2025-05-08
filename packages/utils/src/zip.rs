pub fn zip<T: Clone, U: Clone>(a: &[T], b: &[U]) -> Vec<(T, U)> {
    a.iter()
        .cloned()
        .zip(b.iter().cloned())
        .collect()
}
