// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use anyhow::Context;
use std::io::Write;
use std::sync::Arc;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let mut buffer = Vec::new();
        write!(&mut buffer, "{}", reply).context("Failed to format reply data");
        
        let shared_data = Arc::new(buffer);
        let data_for_first = shared_data.clone();
        let data_for_second = shared_data.clone();
    
        let first_handler = tokio::spawn(async move {
            run_echo_loop(first, data_for_first).await
        });
    
        let second_handler = tokio::spawn(async move {
            run_echo_loop(second, data_for_second).await
        });
    
        tokio::select! {
            res = first_handler => res.context("First listener task failed."),
            res = second_handler => res.context("Second listener task failed."),
        };
    
        ()
}

async fn run_echo_loop(listener: TcpListener, data : Arc<Vec<u8>>) -> Result<(), anyhow::Error> {
    
    loop {
            match listener.accept().await {
                Ok((mut socket, _addr)) => {
                    let data_to_send = data.clone();
                    
                    tokio::spawn(async move {
                        let (_, mut writer) = socket.split();
                        if let Err(e) = writer.write_all(&data_to_send).await {
                            eprintln!("Error sending response: {}", e);
                        }
                    });
                },
                Err(e) => return Err(anyhow::anyhow!("Error accepting connection: {}", e)),
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
