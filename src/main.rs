// Async chat server
// once compiled to interact with the code, open additional client terminals and
// use command - "telnet localhost 8080". Once connected you can type your chats
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

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

    // Enabling chats to post to all the clients - use broadcast channels of tokio
    // The function channel has to specify what type "T" will be broadcasted. In this case
    // we will broadcast "String"
    // The lines read in the below code will be broadcasted using this channel
    // Turbofish operator removed as compiler can figure out the Type from other code below
    let (tx, rx) = broadcast::channel(10);

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
        // of the client connecting
        // we can use the address variable addr to specifically deal with this particular 
        // client
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("The address of the client connected is {:?}", addr);

        // original tx has to be cloned for using inside the spawn function below
        // each time a client is accepted a new sender and receiver are cloned representing
        // channels to each client to tunnel messages through
        let tx = tx.clone();
        let mut rx = tx.subscribe();

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

                // tokio::select! macro can be used to concurrently run multiple asynchronous tasks without
                // each async task blocking the other async tasks like before. The macro takes several async
                // tasks as shown below and which ever resolves first is run among all the tasks. You have 
                // to provide the code block that needs to be run after it resolves. The await keyword is not
                // needed anymore, you get unwrap() the result obtained and get the result of the async task
                
                // The read_line implemented as a trait on AsyncBufReadExt
                tokio::select! {
                    // The below code to read a line from client
                    result = reader.read_line(&mut line) => {

                        // if no new lines are entered we break out of the loop
                        // The below happens when the client exits the connection
                        if result.unwrap() == 0 {
                            break;
                        }

                        // instead of sending the line read to all clients. Send it 
                        // only to this client from which we received the line from
                        // with the tx.send we are sending a tuple containing (string, address)
                        // The receiver on the other side is going to get this tuple 
                        tx.send((line.clone(), addr)).unwrap();
                        // clear the line so that a new line can be stored in the same variable
                        line.clear();
                    }

                    // receive the messages sent to this channel. Once messages
                    // are received write them to the client

                    result = rx.recv() => {
                        // receive the tuple of (string, address) from akk the senders
                        // since the client can see the text typed on the terminal, we can
                        // say that write to all clients whose address is not equal to current
                        // client address stored in the addr variable. This will eliminate the
                        // echo we see
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            socket_writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                        
                    }
                }
            }
        });
    }
}
