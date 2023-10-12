use std::net::TcpStream;
use std::ops::Add;
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use openssl::x509::X509NameRef;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub(crate) async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Received connection from client with address {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err={:?}", e);
                        return;
                    }
                };

                if buf[0] == 49 {
                        socket.read(&mut buf).await;
                        let domain = std::str::from_utf8(&buf).unwrap();
                        let trimmed_domain = domain.trim_matches(char::from(0));
                        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
                        let stream = TcpStream::connect((trimmed_domain, 443)).unwrap();
                        let stream = connector.connect(trimmed_domain, stream).unwrap();
                        let ssl_reply = extract_ssl_cert(stream);
                        println!("{:?}", ssl_reply);

                        let mut index = 0;
                        for entry in ssl_reply.to_vec() {
                            let ssl_reply_bytes = entry.as_bytes();
                            let remaining_space = buf.len() - index;
                            let copy_length = std::cmp::min(remaining_space, ssl_reply_bytes.len());
                            buf[index..index + copy_length].copy_from_slice(&ssl_reply_bytes[0..copy_length]);
                            index += copy_length;
                        }
                            socket.write_all(&mut buf).await.unwrap();
                        }
                }
        });
    }
}
fn convert_x509_to_string(x_509ref:&X509NameRef) -> String {
    let mut res = String::new();
    for entry in x_509ref.entries() {
        let obj = entry.object();
        let value = entry.data().as_utf8().unwrap();
        res.push_str(&format!("{}: {}\n", obj.to_string(), value));
    }
    res
}
fn extract_ssl_cert(stream: SslStream<TcpStream>) -> Vec<String> {
    let cert = stream.ssl().peer_certificate();
    let mut res:Vec<String> = vec![];
    match cert {
        Some(cert) => {
            let subject_name = convert_x509_to_string(cert.subject_name());
            let issuer_name = convert_x509_to_string(cert.issuer_name());
            res.push(subject_name);
            res.push(issuer_name);

            match cert.digest(openssl::hash::MessageDigest::sha256()) {
                Ok(fingerprint) => {
                    let mut fp = String::new();
                    fp.push_str("fingerprint: ");
                    for i in fingerprint.to_vec() {
                        fp.push_str(i.to_string().as_str());
                    }
                    res.push(fp);
                }
                Err(error) => {
                    //println!("Error extracting fingerprint: {}", error);
                }
            }
        }
        None => {
            println!("No certificate found.");
        }
    }
    res
}