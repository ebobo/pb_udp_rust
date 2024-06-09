use prost::bytes::BytesMut;
use prost::Message;
use rand::Rng;
use tokio::net::UdpSocket;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use proto::EchoMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random client ID
    let client_id: u32 = rand::thread_rng().gen();
    println!("Client ID: {}", client_id);

    // Bind to a random port
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    // Connect to the server on port 9080
    socket.connect("127.0.0.1:9080").await?;

    // Send a message to the server witch is an EchoMessage
    let message = EchoMessage {
        content: format!("Hello, server ! from client {}", client_id),
    };

    // Encode the message into a buffer
    let mut buf = BytesMut::with_capacity(message.encoded_len());
    // Serializing the message into a binary format
    message.encode(&mut buf)?;

    // Send the buffer to the server
    socket.send(&buf).await?;
    println!("Sent: {:?}", message);

    // Create a buffer to receive the response
    let mut response_buf = [0; 1024];
    // Receive the response from the server
    let len = socket.recv(&mut response_buf).await?;
    // Decode the response buffer into an EchoMessage
    let response_message = EchoMessage::decode(&response_buf[..len])?;

    println!("Received: {:?}", response_message);

    // Return Ok if everything went well
    Ok(())
}
