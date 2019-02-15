cfg_if! {
    if #[cfg(unix)] {
#[macro_export]
macro_rules! foo {
    ( $x:expr, $y:expr ) => {
        ($x + $y);
    };
}
    }
}

#[macro_export]
macro_rules! foo {
    ( $x:expr, $y:expr ) => {
        ($x + $y);
    };
}