use gateway::{Client, GatewayEventHandler};

mod error;
mod events;
mod gateway;
mod voice;

struct MyHandler {}

impl GatewayEventHandler for MyHandler {}

#[tokio::main]
async fn main() {
    Client::try_new(MyHandler {}, "NzY3Mzk1MDQ2Mzk5OTM0NDY0.X4xSVA.W-csLOq3w899--V_uC5GX3Rx8xI".to_string()).await.unwrap().run().await.unwrap();
}
