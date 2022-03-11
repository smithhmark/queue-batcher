use std::env;

use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    env_logger::init();

    let msg_count: usize = match env::var("MESSAGES") {
        Ok(x) => x.parse().expect("valid usize"),
        Err(_y) => 100,
    };

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    for ii in 0..msg_count {
        let message = format!("{}", (ii as f32).sin());
        exchange.publish(Publish::new(message.as_bytes(), "hello"))?;
    }

    connection.close()
}
