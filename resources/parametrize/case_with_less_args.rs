use rstest::rstest_parametrize;

#[cfg(test)]
#[rstest_parametrize(a, b, case(42), case(1, 2), case(43))]
fn error_too_much_arguments(a: u32, b: u32) {}
