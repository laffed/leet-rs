#[cfg(test)]
pub struct TestCase<I, E> {
    pub input: I,
    pub expected: E,
}

#[cfg(test)]
impl<I, E> TestCase<I, E> {
    pub fn new(input: I, expected: E) -> Self {
        TestCase { input, expected }
    }
}
