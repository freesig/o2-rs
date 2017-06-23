extern crate socket_finder;

fn main(){
    
    let sender_service = socket_finder::sender("name");

    let mut target = None;
    let socket = std::net::UdpSocket::bind("0.0.0.0:9090").unwrap();

    // app loop
    loop {
        // send some data
        if let Some(socket) = socket {
            socket.send_to(&data, target).unwrap();
        }

        // Maintains a valid socket
        match sender_service.poll_events() {
            Event::Found(addr) => target = Some(addr),
            Event::Lost => target = None,
        }
    }

}
