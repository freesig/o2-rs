pub struct Finder{
}

pub struct Beacon{
}

pub enum Event{
    Found(std::net::SocketAddrV4),
    Lost
}

pub fn new(name: String) -> Finder{
    Finder{}
}

pub fn beacon(name: String, address: &std::net::SocketAddrV4) -> Beacon{
    Beacon{}
}

impl Finder{
    pub fn poll_events(&self) -> Vec<Event>{
        unimplemented!();
    }
}

impl Beacon{
    pub fn send(&self) -> Result<(), std::io::Error> {
        let data = b"worked";
        let socket = std::net::UdpSocket::bind("0.0.0.0:9091")?;
        socket.set_broadcast(true)?;
        socket.send_to(data, "255.255.255.255:9092")?;
        Ok(())
    }
}
