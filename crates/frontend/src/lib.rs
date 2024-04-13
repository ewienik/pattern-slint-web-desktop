pub use implementation::*;

#[cfg(target_arch = "wasm32")]
mod implementation {
    use defs::Counter;

    pub struct Frontend {}

    impl Frontend {
        pub fn new() -> Self {
            Self {}
        }
        pub fn process_counter(&self, value: Counter, mut cont: impl FnMut(Counter)) {
            cont((i32::from(value) + 1).into());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    use {backend::Backend, defs::Counter};

    pub struct Frontend {
        be: Backend,
    }

    impl Frontend {
        pub fn new() -> Self {
            Self { be: Backend::new() }
        }
        pub fn process_counter(&self, value: Counter, cont: impl FnMut(Counter)) {
            self.be.process_counter(value, cont);
        }
    }
}
