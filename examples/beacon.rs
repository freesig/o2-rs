extern crate socket_finder;

use std::str::FromStr;

fn main(){
    

    let port: u16 = 9097;
    let ip = "0.0.0.0".parse().unwrap();
    let incoming_address = std::net::SocketAddrV4::new(ip, port);
    let socket = std::net::UdpSocket::bind(&incoming_address).expect("socket");
    socket.set_nonblocking(true).unwrap();

    let beacon = socket_finder::beacon("receiver_service", port);
    let mut data = [0u8; 4];

    // app loop
    loop {
        beacon.send().unwrap();

        while let Ok((amt, src)) = socket.recv_from(&mut data){
            let received_data = String::from_utf8_lossy(&data[..]);
            println!("Recieved data: {}", received_data);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));

    }

}
