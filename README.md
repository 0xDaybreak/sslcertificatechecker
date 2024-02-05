# Overview
sslcerificatechecker is a lightweight SSL certificate checker developed in Rust as a school project. This endeavor was intended to deepen my understanding of cybersecurity and the Rust programming language.

# Getting Started

1. **Clone the repository:**

    ```bash
    git clone https://github.com/0xDaybreak/sslcertificatechecker.git
    cd RustSSLChecker
    ```

2. **Build and run the project:**

    ```bash
    cargo run
    ```

3. **Follow the prompts to either start the SSL certificate checker as a client or server.**

# Usage

## Client

- Choose "START AS CLIENT" from the menu.
- Enter the IP address and port to connect to.
- Select either "GET_SSL_CERTIFICATE" to retrieve an SSL certificate or "CLOSE" to exit.

## Server

- Choose "START AS SERVER" from the menu.
- Enter the address and port for the server to listen to.
- The server will wait for incoming connections and respond to SSL certificate requests.

# Dependencies

- [`tokio`](https://github.com/tokio-rs/tokio): Asynchronous runtime for Rust.
- [`inquire`](https://github.com/murarth/inquire): A Rust library for building interactive prompts.
- [`openssl`](https://github.com/sfackler/rust-openssl): Rust bindings for the OpenSSL library.

# Contribution

Feel free to contribute to this project by opening issues or pull requests. Your feedback and enhancements are highly appreciated!

# License

This project is licensed under the [MIT License](LICENSE).
