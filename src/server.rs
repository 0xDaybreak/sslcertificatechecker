use openssl::ssl::{SslConnector, SslMethod, SslStream};
use openssl::x509::{X509, X509NameRef};
use std::net::TcpStream;
use std::ops::Add;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub(crate) async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Received connection from client with address {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 8192];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err={:?}", e);
                        return;
                    }
                };

                match buf[0]  {
                    49 => {
                        socket.read(&mut buf).await;
                        let domain = std::str::from_utf8(&buf).unwrap();
                        let trimmed_domain = domain.trim_matches(char::from(0));
                        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
                        let stream = TcpStream::connect((trimmed_domain, 443)).unwrap();
                        let stream = connector.connect(trimmed_domain, stream).unwrap();
                        buf.iter_mut().for_each(|m| *m = 0);
                        let ssl_reply = extract_ssl_cert(stream).unwrap();
                        if ssl_reply.len() <= buf.len() {
                            buf[0..ssl_reply.len()].copy_from_slice(&ssl_reply);
                        }
                        println!("Sent to {} the following data {:?}", addr, buf);

                        socket.write_all(&mut buf).await.unwrap();
                    }
                    50 => {

                    }

                    _ => {}
                }
            }
        });
    }
}
fn extract_ssl_cert(stream: SslStream<TcpStream>) -> Option<Vec<u8>> {
    let cert = stream.ssl().peer_certificate();
    match cert {
        Some(cert) => {
            let text = cert.to_text().unwrap();
            Some(text)
        }
        None => None,
    }
}
