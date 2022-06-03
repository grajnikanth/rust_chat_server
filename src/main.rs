// Async chat server
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

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

    // This outer loop is required so that we can check if multiple clients are connecting
    // to the listener. If they are connecting, once the connection is accepted
    // a thread is spawned to  deal with that particular client
    // Then a block of code is sent to the tokio spawned thread to executed in that thread
    // This will let multiple clients to connect and interact with the server
    loop {
        

        // Note that the individual smaller tasks are blocking type of code that is synchronous
        // the threads are asynchronous where one thread does not block the next thread
        // but inside each client execution the code block is executed one step at a time

        //For example in the accept() function step below, The first client is accepted and then
        // that client data stream is spawned into a thread to execute. Now the loop continues
        // but the code will stop at the accept() function, the next time in the loop. Now if
        // a second client comes in and connects, the accept().await is executed and then
        // a new thread is spawned.

        // accept method - accepts a new connection and yields the socket and address
        // of the connection
        // addr will not be used so to eliminate unused errors we use "_" in front
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn( async move {
            // instead of using socket.read() we will use a BufReader which can read lines as 
            // byte streams. This will eliminate us haveing to deal with the number of bytes
            // we have to store in our buffer etc.

            // first we need to split the socket to distinguish between message receiver and
            // message sender to client
            // 
            let (socket_reader, mut socket_writer) = socket.split();

            // BufReader will do the heavy lifting of tracking number of bytes read etc. 
            // We only have to worry about reading one line at a time
            let mut reader = BufReader::new(socket_reader);

            // Each line read will be stored in the variable line
            let mut line = String::new();

            //single message can be received and echoed back to the client with the following code
            // You can put the below code into an infinite loop to receive and echo multiple
            // messages

            loop {
                // The read_line implemented as a trait on AsyncBufReadExt

                let bytes_read = reader.read_line(&mut line).await.unwrap();  

                // if no new lines are entered we break out of the loop
                // The below happens when the client exits the connection
                if bytes_read == 0 {
                    break;
                }

                socket_writer.write_all(line.as_bytes()).await.unwrap();

                // clear the contents stored. As the read_line function appends to the contents
                // of the line variable. We want to clear it so that each new line is echoed back
                // rather than all the lines entered so far
                line.clear();

            }
        });
    }
}
