#[derive(Debug)]
pub struct Counter(i32);

impl From<i32> for Counter {
    fn from(value: i32) -> Self {
        Counter(value)
    }
}

impl From<Counter> for i32 {
    fn from(value: Counter) -> Self {
        value.0
    }
}
