pub use {defs::Counter, implementation::Frontend};

#[cfg(target_arch = "wasm32")]
mod implementation {
    use {defs::Counter, std::future::Future};

    pub struct Frontend {}

    impl Frontend {
        pub fn new() -> Self {
            Self {}
        }

        pub(super) fn spawn(&self, future: impl Future<Output = ()> + 'static) {
            wasm_bindgen_futures::spawn_local(future);
        }

        pub fn process_counter_async(
            &self,
            value: Counter,
            cont: impl FnOnce(Counter) + Send + 'static,
        ) -> impl Future<Output = ()> + Send + 'static {
            async move {
                cont((i32::from(value) + 1).into());
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod implementation {
    use {
        backend::Backend,
        defs::Counter,
        std::{future::Future, sync::Arc},
        tokio::runtime::Runtime,
    };

    pub struct Frontend {
        be: Arc<Backend>,
        rt: Runtime,
    }

    impl Frontend {
        pub fn new() -> Self {
            Self {
                be: Arc::new(Backend::new()),
                rt: Runtime::new().unwrap(),
            }
        }

        pub(super) fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
            self.rt.spawn(future);
        }

        pub(super) fn process_counter_async(
            &self,
            counter: Counter,
            cont: impl FnOnce(Counter) + Send + 'static,
        ) -> impl Future<Output = ()> + Send + 'static {
            let be = Arc::clone(&self.be);
            async move {
                cont(be.process_counter(counter).await);
            }
        }
    }
}

impl Frontend {
    pub fn process_counter(&self, counter: Counter, cont: impl FnOnce(Counter) + Send + 'static) {
        self.spawn(self.process_counter_async(counter, cont));
    }
}
