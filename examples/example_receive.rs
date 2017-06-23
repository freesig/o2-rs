extern crate socket_finder;

use std::str::FromStr;

fn main(){
    

    let incoming_address = std::net::SocketAddrV4::from_str("0.0.0.0:9090").expect("incoming");
    let socket = std::net::UdpSocket::bind(&incoming_address).expect("socket");

    let beacon = socket_finder::beacon("name".to_string(), &incoming_address);
    let mut data = [0u8; 4];

    // app loop
    loop {
        beacon.send().unwrap();
        //let (size, addr) = socket.recv_from(&mut data).expect("recv");

        std::thread::sleep(std::time::Duration::from_secs(1));

    }

}
