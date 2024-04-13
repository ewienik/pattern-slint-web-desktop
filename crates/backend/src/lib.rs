use {
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

pub struct Backend {
    tx: mpsc::UnboundedSender<Message>,
    #[allow(dead_code)]
    rt: Runtime,
}

impl Backend {
    pub fn new() -> Self {
        let rt = Runtime::new().unwrap();
        let (tx, mut rx) = mpsc::unbounded_channel();
        rt.spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    Message::Counter(counter, tx) => {
                        tx.send(Counter::from(i32::from(counter) + 1)).unwrap();
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
