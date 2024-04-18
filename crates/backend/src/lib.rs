use defs::Counter;

#[derive(Clone)]
pub struct Backend;

impl Backend {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_counter(&self, counter: Counter) -> Counter {
        Counter::from(i32::from(counter) + 1)
    }
}
