use prost::bytes::BytesMut;
use prost::Message;
use tokio::net::UdpSocket;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use proto::EchoMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:9080").await?;

    let message = EchoMessage {
        content: String::from("Hello, server!"),
    };

    let mut buf = BytesMut::with_capacity(message.encoded_len());
    message.encode(&mut buf)?;

    socket.send(&buf).await?;
    println!("Sent: {:?}", message);

    let mut response_buf = [0; 1024];
    let len = socket.recv(&mut response_buf).await?;
    let response_message = EchoMessage::decode(&response_buf[..len])?;

    println!("Received: {:?}", response_message);

    Ok(())
}
