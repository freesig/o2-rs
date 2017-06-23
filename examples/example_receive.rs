extern crate socket_finder;

use std::str::FromStr;

fn main(){
    

    let incoming_address = std::net::SocketAddrV4::from_str("0.0.0.0:9090");
    let socket = std::net::UdpSocket::bind(&incoming_address).unwrap();

    let receive_service = socket_finder::receiver("name", &incoming_address);

    // app loop
    loop {
        receive_service.send_beacon();
        let (size, addr) = socket.recv_from(&mut data).unwrap();

    }

}
