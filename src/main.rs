mod client;
mod server;

use std::net::TcpStream;
use inquire::Select;
use openssl::ssl::SslStream;


fn main() {
    let items: [String; 2] = ["START AS CLIENT".to_string(), "START AS SERVER".to_string()];
    let selected_item = menu(&items);
    match selected_item.as_str() {
        "START AS CLIENT" => client::start_client(),
        "START AS SERVER" => server::start_server(),
        _ => Ok(())
    };
}


fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec()).prompt().unwrap()
}
