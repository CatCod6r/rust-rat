use tokio::net::TcpStream;

pub struct Instance {
    ip: u32,
    hostname: String,
    uuid: String,
    public_key: String,
    private_key: String,
    active_instance: ActiveInstance,
}
struct ActiveInstance {
    stream: TcpStream,
    port: u32,
}
