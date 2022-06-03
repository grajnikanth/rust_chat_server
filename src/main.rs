// Async chat server
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Building an echo server which receives the clients message and sends it back to client


// The macro from tokio will make our main deal with async code
// Futures are tasks which happen in the future. like say a network request may come back
// in 100 milli seconds in the future
#[tokio::main]
async fn main() {

    // listens to incomming connect requests on the port
    // bind returns a impl Future. To get the Result inside the Future 
    // we need to use await keyword
    // await is Rust keyword which basically says, you are allowed to wait processing
    // until the code to the left of await is run
    let listener = TcpListener::bind(
        "localhost:8080").await.unwrap();

        // accept method - accepts a new connection and yields the socket and address
        // of the connection
        // addr will not be used so to eliminate unused errors we use "_" in front
    let (mut socket, _addr) = listener.accept().await.unwrap();

    //single message can be received and echoed back to the client with the following code
    // You can put the below code into an infinite loop to receive and echo multiple
    // messages

    // loop {
    //     // make a buffer which can hold upto 1024 bytes or 1KB
    //     let mut buffer = [0u8; 1024];

    //     // read the bytes in the socket and store it in the buffer. 
    //     // bytes_read will store how many actual bytes were read by the socket
    //     let bytes_read = socket.read(&mut buffer).await.unwrap();

    //     // take all the bytes and write it back to the socket 
    //     // with the ..bytes_read we are saying send the bytes truncated by the 
    //     // the number of bytes correspoding to bytes_read
    //     socket.write_all(&buffer[..bytes_read]).await.unwrap();
    //     // after writing the message back there is nothing else and the connection is stopped

    // }
    
    let mut reader = 10;

}
