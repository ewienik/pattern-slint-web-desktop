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
    use {
        backend::Backend,
        defs::Counter,
        proto::Message,
        tokio::{
            runtime::Runtime,
            sync::{mpsc, oneshot},
        },
    };

    mod proto {
        use {defs::Counter, tokio::sync::oneshot};

        #[derive(Debug)]
        pub(super) enum Message {
            Counter(Counter, oneshot::Sender<Counter>),
        }
    }

    pub struct Frontend {
        tx: mpsc::UnboundedSender<Message>,
        #[allow(dead_code)]
        rt: Runtime,
    }

    impl Frontend {
        pub fn new() -> Self {
            let rt = Runtime::new().unwrap();
            let (tx, mut rx) = mpsc::unbounded_channel();
            rt.spawn(async move {
                let be = Backend::new();
                while let Some(msg) = rx.recv().await {
                    match msg {
                        Message::Counter(counter, tx) => {
                            tx.send(be.process_counter(counter).await).unwrap();
                        }
                    }
                }
            });
            Self { tx, rt }
        }

        pub fn process_counter(&self, value: Counter, mut cont: impl FnMut(Counter)) {
            let (tx, rx) = oneshot::channel();
            self.tx.send(Message::Counter(value, tx)).unwrap();
            cont(rx.blocking_recv().unwrap());
        }
    }
}
