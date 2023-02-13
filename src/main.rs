use socket::{Socket, AF_INET, SOCK_DGRAM, SOL_SOCKET, SO_REUSEADDR};

fn main() {
    let socket = Socket::new(AF_INET, SOCK_DGRAM, 0).unwrap();
    socket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1).unwrap();
    socket.bind("0.0.0.0:5353").unwrap();
    socket.listen(1).unwrap();
    socket.accept().unwrap();

    loop {
        println!("{:?}", socket.recv(1024, 0).unwrap());
    }
}
