use futures::sink::SinkExt;
use std::error::Error;
use std::io::{self, BufRead};

use futures::StreamExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::{net::TcpStream, sync::broadcast, sync::mpsc};
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LengthDelimitedCodec, LinesCodec};

#[derive(Clone)]
struct Connection {
    addr: String,
    tx_ingress: mpsc::UnboundedSender<String>,
    rx_outgress: broadcast::Sender<String>,
}

impl Connection {
    fn new(addr: String, tx_socket: mpsc::UnboundedSender<String>) -> Connection {
        //Channel for incoming messages
        let (tx_outgress, rx_outgress) = broadcast::channel(100);
        Connection {
            addr,
            tx_ingress: tx_socket,
            rx_outgress: tx_outgress,
        }
    }

    async fn handle(&mut self) -> io::Result<()> {
        let stream = TcpStream::connect(self.addr.as_str()).await?;
        let (read, write) = stream.into_split();
        let read = FramedRead::new(read, tokio_util::codec::LinesCodec::new());
        let write_frame = FramedWrite::new(write, tokio_util::codec::LinesCodec::new());

        let mut rx = self.rx_outgress.subscribe();
        tokio::spawn(incoming_frames(read, self.tx_ingress.clone()));
        tokio::spawn(outgoing_frames(write_frame, rx));
        Ok(())
    }

    fn write(&self, msg: String) {
        self.rx_outgress.send(msg).unwrap();
    }
}

#[derive(Clone)]
struct ChatonClient {
    addr: String,
}

impl ChatonClient {
    fn new(addr: String) -> ChatonClient {
        ChatonClient { addr }
    }
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        let connection = Connection::new(self.addr.clone(), tx);
        let mut connection_handler = connection.clone();
        tokio::spawn(async move {
            connection_handler.handle().await.unwrap();
        });

        let mut self_clone = self.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                self_clone.process_incoming(msg);
                // println!("Received: {}", msg);
            }
        });

        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_line(&mut buffer)?;
            let processed = buffer.to_string().trim().to_string();
            connection.write(self.process_user_input(processed));
        }
    }

    fn process_user_input(&mut self, msg: String) -> String{
        msg
    }

    fn process_incoming(&mut self, msg: String) {
        println!("Received: {}", msg);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ChatonClient::new("127.0.0.1:6142".to_string());
    client.run()?;
    Ok(())
}

type ClientStream = FramedRead<OwnedReadHalf, LinesCodec>;
type ClientSink = FramedWrite<OwnedWriteHalf, LinesCodec>;

async fn incoming_frames(mut framed_read: ClientStream, tx: mpsc::UnboundedSender<String>) {
    while let Some(frame) = framed_read.next().await {
        let frame = frame.unwrap();
        tx.send(frame).unwrap();
    }
}

async fn outgoing_frames(
    mut stream: ClientSink,
    mut rx: broadcast::Receiver<String>,
) {
    loop {
        let frame = rx.recv().await.unwrap();
        // println!("Sending: {}", frame);
        stream.send(frame).await.unwrap();
    }
}
