use openssl::ssl::{SslMethod, SslConnector, SslStream};
use std::net::TcpStream;
use inquire::Select;

fn main() {
    let items: [String; 2] = ["GET_SSL_CERTIFICATE".to_string(), "CLOSE".to_string()];
    let selected_item = menu(&items);

    if selected_item == "GET_SSL_CERTIFICATE" {

        let domain = "google.com:443";
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

        let stream = TcpStream::connect(domain).unwrap();
        let stream = connector.connect("google.com", stream).unwrap();
        extract_ssl_cert(stream);

    }
}

fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec())
        .prompt()
        .unwrap()
}

fn extract_ssl_cert(stream:SslStream<TcpStream>) {

    let cert = stream.ssl().peer_certificate();

    match cert {
        Some(cert) => {
            let subject_name = cert.subject_name();
            let issuer_name = cert.issuer_name();

            println!("Subject Name: {:?}", subject_name);
            println!("Issuer Name: {:?}", issuer_name);

            match cert.digest(openssl::hash::MessageDigest::sha256()) {
                Ok(fingerprint) => {
                    println!("Fingerprint (SHA-256): {:?}", fingerprint);
                }
                Err(error) => {
                    println!("Error extracting fingerprint: {}", error);
                }
            }
        }
        None => {
            println!("No certificate found.");
        }
    }
}