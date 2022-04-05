use rustls::internal::msgs::message::Message;
use tokio::{task, time};

use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use std::{time::Duration, io::Bytes};

use futures::StreamExt;
use log::info;
use tmq::{subscribe, Context, Result, TmqError, Multipart};

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    // color_backtrace::install();

    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    let mut socket = subscribe(&Context::new())
    .connect("tcp://127.0.0.1:7899")?
    .subscribe(b"topic")?;

    loop {
        let event = eventloop.poll().await;
        println!("{:?}", event.unwrap());
        if let Some(msg) = socket.next().await{
            println!("ZEROMQ MSG -> {:?}",msg);
            publish_message(client, );
        }

    }
}

async fn publish_message(client : AsyncClient, mut data : Multipart){
    if let Some(my_data) = data.pop_front(){

        client.publish_bytes("Ola", QoS::AtLeastOnce,false , "ola".into());     
    }

}


async fn requests(client: AsyncClient) {
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();

    for i in 1..=10 {
        client
            .publish("hello/world", QoS::ExactlyOnce, false, "Enviando publicação mqtt via rust")
            .await
            .unwrap();

        time::sleep(Duration::from_secs(1)).await;
    }

    time::sleep(Duration::from_secs(120)).await;
}
