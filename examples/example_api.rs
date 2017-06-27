extern crate socket_finder;

use socket_finder::Status;

fn main(){
    
    let mut finder = socket_finder::new("receiver_service".to_string()).unwrap();

    let mut target: Option<std::net::SocketAddrV4> = None;
    let socket = std::net::UdpSocket::bind("0.0.0.0:9090").unwrap();
    let mut data = b"test";

    // app loop
    loop {
        // send some data
        if let Some(target) = target {
            println!("ip: {}", target.ip() );
            socket.send_to(data, target).unwrap();
        }

        // Maintains a valid socket
        match finder.poll_status().unwrap(){
                Status::Found(addr) => target = Some(addr),
                Status::TimeSince(time_since) => println!("Time Since {:?}", time_since),
        }
        std::thread::sleep( std::time::Duration::from_secs(2) );
    }

}
