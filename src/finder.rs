use {HEADER, HEAD_SIZE, PORT_SIZE};
use std::net::SocketAddrV4;
use std::io::Write;
use std::str::FromStr;
use std;

pub struct Finder{
    name: String,
    socket: Option<SocketAddrV4>,
    beacon_receiving_socket: std::net::UdpSocket,
    last_beacon_time: std::time::Instant,
    //This is reused for reading in beacons
    buffer: Vec<u8>
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError{
    InvalidHeader,
    InvalidUtf8(std::str::Utf8Error)
}

impl std::error::Error for ParseError{
    fn description(&self) -> &str {
        match *self{
            ParseError::InvalidHeader => "Header didn't match for beacon packet",
            ParseError::InvalidUtf8(ref error) => error.description()
        }
        
    }
}

impl From<std::str::Utf8Error> for ParseError{
    fn from(error: std::str::Utf8Error) -> Self {
        ParseError::InvalidUtf8(error)
    }
}

impl std::fmt::Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!( f, "{}", std::error::Error::description(self) )
    }
}


pub enum Status{
    Found(SocketAddrV4),
    TimeSinceLastReceived(std::time::Duration)
}

pub fn finder(name: String) -> Result<Finder, std::io::Error>{
    let last_beacon_time = std::time::Instant::now();
    let beacon_receiving_socket = std::net::UdpSocket::bind("0.0.0.0:9092")?;
    beacon_receiving_socket.set_nonblocking(true);
    let buffer = vec!(0; name.len() + HEAD_SIZE + PORT_SIZE);
    Ok( Finder{name, socket: None, last_beacon_time, beacon_receiving_socket, buffer} )
}

fn extract_message(data: &[u8]) -> Result<(& str, u16), ParseError> {
    let (header, data) = data.split_at(HEAD_SIZE);
    if header != HEADER {
        return Err(ParseError::InvalidHeader);
    }
    let (port, name) = data.split_at(PORT_SIZE);
    let port: u16 = port[0] as u16 + ((port[1] as u16) << 8);
    let name = std::str::from_utf8(name)?;
    Ok( (name, port) )
}

fn check_socket(beacon_receiving_socket: & std::net::UdpSocket,
                buffer: &mut [u8]) -> Option<(std::net::SocketAddrV4, usize)>{

    let mut data_found = None;

    let mut latest_message = None;
    loop{
        match beacon_receiving_socket.recv_from(buffer){
            Ok(packet) => {
                latest_message = Some(packet);
            }
            Err(error) => {
                break;
            }
        }
    }
    if let Some( (num_bytes, address) ) = latest_message{
        let source_address = match address{
            std::net::SocketAddr::V4(src) => src,
            _ => panic!("V6 not supported")
        };
        data_found = Some( (source_address, num_bytes) );
    }
    data_found
}


impl Finder{
    //usize is length of message received



    pub fn poll_status(&mut self) -> Result<Status, ParseError>{
        let Finder { 
            ref mut buffer, 
            ref mut last_beacon_time, 
            ref beacon_receiving_socket,
            ref name,
            ref mut socket,
            ..
        } = *self; 
        let data_found = check_socket(beacon_receiving_socket, buffer);

        match data_found{
            Some( (source_address, length) ) => {
                let (target_name, port) = extract_message(& buffer[..length])?;

                if name != target_name{
                    let change_in_time = last_beacon_time.elapsed();
                    println!("different Names");
                    return Ok(Status::TimeSinceLastReceived(change_in_time));
                }

                let target_socket = SocketAddrV4::new(source_address.ip().clone(), port);

                let status = if *socket != Some( target_socket ) {
                    Status::Found(target_socket)
                } else {
                    Status::TimeSinceLastReceived( std::time::Duration::from_secs(0) )
                };
                *socket = Some(target_socket);
                *last_beacon_time = std::time::Instant::now();
                Ok(status)

            },
            None => {
                let change_in_time = last_beacon_time.elapsed();
                Ok(Status::TimeSinceLastReceived(change_in_time))
            }
        }
    }
}
