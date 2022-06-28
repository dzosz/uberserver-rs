use futures::SinkExt;
use log::{debug, error, info};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tokio::time::{sleep, Duration, Instant};

use crate::client::Client;

const SOCKET_LIMIT: usize = 1024; // TODO rlimit::getrlimit_nofile() / 2;
const TCP_CHAR_LIMIT: usize = 1024;
const TIMEOUT: u64 = 60;
pub struct ChatServer {
    tls: bool,
    connected_clients: usize,
    // root
}

async fn process(stream: TcpStream,/* state: Arc<Mutex<ChatServer>>,*/ uid: usize) {
    let mut lines = Framed::new(stream, LinesCodec::new_with_max_length(TCP_CHAR_LIMIT));
    let timeout = sleep(Duration::from_secs(TIMEOUT));
    tokio::pin!(timeout); // Pinning the Sleep with tokio::pin! is necessary when the same Sleep is selected on multiple times.
    let mut client = Client::new();
    let mut lastdata = Instant::now();

    loop {
        tokio::select! {
            /*
            // A message was received from a peer. Send it to the current user.
            Some(msg) = peer.rx.recv() => {
                peer.lines.send(&msg).await?;
            }*/
            result = lines.next() => match result {
                // A message was received from the current user, we should
                // broadcast this message to the other users.
                Some(Ok(msg)) => {
                    debug!("received {}", msg);
                    //let mut state = state.lock().await;

                    if client.is_logged() {
                        lastdata = Instant::now();
                    }

                    client.Handle(&msg);

                    if !client.message_queue.is_empty() {
                        // FIXME check if we don't duplicate newline here because we use
                        // LinesCodec()
                        lines.send(&client.message_queue).await; 
                        client.message_queue.clear();
                    }

                    // TODO broadcast this message to other clients?
                    //let msg = format!("{}: {}", username, msg);
                    //state.broadcast(addr, &msg).await;

                    //self._root.session_manager.commit_guard()
                }
                Some(Err(e)) => {
                    error!("an error occurred while processing messages for {}; error = {:?}",
                        uid, e
                    );
                    break; // TODO should we exit from here or not?
                }
                // The stream has been exhausted.
                None => break,
            },
            _ = &mut timeout => {
                if lastdata < Instant::now() + Duration::from_secs(TIMEOUT) {
                    error!("client {} timed out", uid);
                    break
                }
                timeout.as_mut().reset(lastdata + Duration::from_secs(TIMEOUT));
            }
        }
    }
}

impl ChatServer {
    pub async fn start(port: u32) -> io::Result<()> {
        let state = Arc::new(Mutex::new(ChatServer {
            tls: false,
            connected_clients: 0,
        }));

        let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
        info!("Awaiting TCP messages on port {}", port);
        let listener = TcpListener::bind(addr).await?;

        let mut uid: usize = 0;
        loop {
            let (stream, addr) = listener.accept().await?;

            {
                let mut s = state.lock().await;

                /* TODO refactor to use methods
                if !s.connectionMade {
                } */

                if s.connected_clients >= SOCKET_LIMIT {
                    error!("too many connections: {} > {}", s.connected_clients, SOCKET_LIMIT);
                    let mut lines = Framed::new(stream, LinesCodec::new());
                    lines.send("DENIED too many connections, sorry!").await;
                    continue;
                }
                s.connected_clients += 1;
                uid += 1;
            }
            debug!("accepted connection {}", uid);

            let cloned_state = Arc::clone(&state);
            tokio::spawn(async move {
                process(stream, uid).await;
                let mut srv = cloned_state.lock().await;
                srv.connected_clients -= 1;
                debug!("closed connection {}", uid);
            });
        }
    }

    /*
    fn connectionMade() -> bool {
        false
    }

    fn connectionLost() {}

    fn removePWs() {
        // TODO impl
    }

    async fn dataReceived() {}
    fn timeoutConnection() {}

    fn Remove(self, reason='Quit') {}

    fn StartTLS(self) {}
    */
}
