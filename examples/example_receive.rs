extern crate socket_finder;

use std::str::FromStr;

fn main(){
    

    let incoming_address = std::net::SocketAddrV4::from_str("0.0.0.0:9090").unwrap();
    let socket = std::net::UdpSocket::bind(&incoming_address).unwrap();

    let beacon = socket_finder::beacon("name".to_string(), &incoming_address);
    let mut data = [0u8; 4];

    // app loop
    loop {
        beacon.send();
        let (size, addr) = socket.recv_from(&mut data).unwrap();

    }

}
