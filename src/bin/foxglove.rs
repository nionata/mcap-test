use foxglove::{Encode, log};
use serde::Serialize;

#[derive(Serialize)]
struct Metric {
    value: i64,
}

impl Encode for Metric {
    type Error = serde_json::Error;

    fn get_schema() -> Option<foxglove::Schema> {
        None
    }

    fn get_message_encoding() -> String {
        "json".to_string()
    }

    fn encode(&self, buf: &mut impl foxglove::bytes::BufMut) -> Result<(), Self::Error> {
        let value = serde_json::to_vec(self)?;

        buf.put(&value[..]);

        Ok(())
    }
}

fn main() {
    let server = foxglove::WebSocketServer::new();

    std::thread::spawn(|| {
        server.start_blocking().unwrap();
    });

    let mut value = 0;

    loop {
        log!("/metric", Metric { value });

        value += 1;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
