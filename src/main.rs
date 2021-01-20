mod phone {
    tonic::include_proto!("grpc.testing");
}

use flume::{Receiver, Sender};
use tonic::{transport::Channel, Request, Response};

type PhoneService = phone::phone_client::PhoneClient<Channel>;

#[tokio::main]
async fn main() {
    let mut client = PhoneService::connect("http://localhost:50051")
        .await
        .expect("aaa");
    let (tx, rx) = flume::unbounded();

    // Uncommenting this will make it work fine
    /*tx.send(phone::StreamCallRequest {
        phone_number: "555".to_string(),
    })
    .expect("aa");*/

    println!("before stream");
    let mut calls = client
        .stream_call(rx.into_stream())
        .await
        .expect("aa")
        .into_inner();
    println!("after stream");

    tx.send(phone::StreamCallRequest {
        phone_number: "444".to_string(),
    })
    .expect("aa");

    let _ = calls.message().await;
}
