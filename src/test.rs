use crate::foo;

#[cfg(test)]
fn test_true() {
    assert_eq!(foo!(1, 2), 3);
}