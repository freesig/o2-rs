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
    pub fn send(&self){
        unimplemented!();
    }
}
