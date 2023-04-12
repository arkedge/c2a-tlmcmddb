macro_rules! check_header {
    ($actual:expr, $expected:expr) => {
        anyhow::ensure!(
            $actual == $expected,
            "invalid header: expected: {}, but got: {}",
            $expected,
            $actual
        );
    };
}
pub(crate) use check_header;
