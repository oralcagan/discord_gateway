use gateway::GatewayEventHandler;

mod gateway;
mod error;
mod events;
mod voice;

struct MyHandler {

}

impl GatewayEventHandler for MyHandler {}

#[tokio::main]
async fn main() {
    gateway::Client::<MyHandler>::run(MyHandler {}, String::from("")).await.unwrap();
}