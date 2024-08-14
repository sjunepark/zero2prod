use std::net::TcpListener;

/// Spin up an instance of our application
/// and return its address (i.e. http://localhost:XXXX)
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
