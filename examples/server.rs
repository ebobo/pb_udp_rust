use prost::bytes::BytesMut;
use prost::Message;
use tokio::net::UdpSocket;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use proto::EchoMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind to the server on port 9080
    let socket = UdpSocket::bind("127.0.0.1:9080").await?;
    println!("Server listening on 127.0.0.1:9080");

    // Create a buffer to receive data
    let mut buf = [0; 1024];

    loop {
        // Receive data from the socket
        let (len, addr) = socket.recv_from(&mut buf).await?;
        // Decode the received data into an EchoMessage
        let echo_message = EchoMessage::decode(&buf[..len])?;

        println!("Received from {}: {:?}", addr, echo_message);

        let response_message = EchoMessage {
            content: format!("{} echo from server", echo_message.content),
        };

        let mut response_buf = BytesMut::with_capacity(response_message.encoded_len());
        response_message.encode(&mut response_buf)?;

        socket.send_to(&response_buf, &addr).await?;
    }
}
