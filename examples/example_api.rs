extern crate socket_finder;

use socket_finder::Event;

fn main(){
    
    let finder = socket_finder::new("name".to_string());

    let mut target = None;
    let socket = std::net::UdpSocket::bind("0.0.0.0:9090").unwrap();
    let mut data = [1u8; 4];

    // app loop
    loop {
        // send some data
        if let Some(target) = target {
            socket.send_to(&data, target).unwrap();
        }

        // Maintains a valid socket
        for event in finder.poll_events(){
            match event {
                Event::Found(addr) => target = Some(addr),
                Event::Lost => target = None,
            }
        }
    }

}
