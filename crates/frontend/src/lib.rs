pub use {defs::Counter, implementation::Frontend};

#[cfg(target_arch = "wasm32")]
mod implementation {
    use {
        defs::Counter,
        std::future::Future,
        wasm_bindgen::prelude::*,
        wasm_bindgen_futures::JsFuture,
        web_sys::{Request, RequestInit, RequestMode, Response},
    };

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
            counter: Counter,
            cont: impl FnOnce(Counter) + 'static,
        ) -> impl Future<Output = ()> + 'static {
            async move {
                let mut opts = RequestInit::new();
                opts.method("GET");
                opts.mode(RequestMode::Cors);

                let window = web_sys::window().unwrap();
                let url = format!(
                    "{}counter/{}",
                    window.location().href().unwrap(),
                    i32::from(counter),
                );

                let request = Request::new_with_str_and_init(&url, &opts).unwrap();

                let resp = JsFuture::from(window.fetch_with_request(&request))
                    .await
                    .unwrap();
                let resp: Response = resp.dyn_into().unwrap();
                let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

                cont((json.as_f64().unwrap() as i32).into());
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
