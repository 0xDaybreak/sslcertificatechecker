use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use inquire::Select;
use tokio::net::TcpStream;
#[tokio::main]
pub(crate) async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    let items: [String; 2] = ["GET_SSL_CERTIFICATE".to_string(), "CLOSE".to_string()];

    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buf:[u8;1024] = [0;1024];
    loop {
        let selected_item = menu(&items);
        match selected_item.as_str() {
            "GET_SSL_CERTIFICATE" => {
                buf[0] = 49;
                socket.write_all(&buf[0..1]).await?;

                let n = socket.read(&mut buf).await?;
                if n > 0 {
                    let ssl_reply = std::str::from_utf8(&buf[0..n]).unwrap();
                    println!("Received SSL certificate:\n{}", ssl_reply);
                }
            }
            "CLOSE" => break,
            _ => {}
        }
    }

    Ok(())
}
fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec()).prompt().unwrap()
}
