use {HEADER, HEAD_SIZE, PORT_SIZE};
use std;

pub struct Beacon{
    // This is the beacon
    // HEADER: b"soc#"
    // PORT: u16 ("The port you want to receive on")
    // NAME: str ("The name connection")
    data: Vec<u8>
}
pub fn beacon(name: &str, address: u16) -> Beacon{
    let mut data = Vec::with_capacity(name.len() + HEAD_SIZE + PORT_SIZE);
    data.extend(HEADER);
    data.push( address as u8 );
    data.push( (address >> 8) as u8 );
    data.extend( name.as_bytes() );

    Beacon{data}
}


impl Beacon{
    pub fn send(&self) -> Result<(), std::io::Error> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:9091")?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.data[..], "255.255.255.255:9092")?;
        Ok(())
    }
}
