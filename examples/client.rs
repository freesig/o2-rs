//  o2client.c - part of performance benchmark
//
//  see o2server.c for details


//#include "o2.h"
extern crate o2;


const max_msg_count: i32 = 50000;


fn main(){
    let server_addresses: Vec<std::ffi::CString> = vec![];
    let mut msg_count = 0;
    let mut running = true;

    if let Some(arg) = std::env::args().nth(1){
        o2::debug_flags();
    }

    let o2 = o2::new("test");
    o2.service_new("client");
   
    
    for i in 0..N_ADDRS{
        let mut cs = CString::new();
        write!(&mut cs, b"server/benchmark/{}", i).unwrap();
        server_addresses.push(cs);
    }

    while o2.status("server") < O2_REMOTE {
        o2.poll();
        std::thread::sleep( std::time::Duration::from_millis(2) ); // 2ms
    }
    println!("We discovered the server.\ntime is {}.\n", o2.time_get());
   
    // Demonstrate delay 1 second
    let now = o2.time_get();
    while (o2.time_get() < now + 1) {
        o2.poll();
        std::thread::sleep( std::time::Duration::from_millis(2) );
    }
    
    println!("Here we go! ...\ntime is {}.\n", o2_time_get());
    
    o2.send("!server/benchmark/0", 0, "i", 1);
    
    while running {
        o2.poll();
        //usleep(2000); // 2ms // as fast as possible
        for event in o2.poll_events() {
            match (event.path, event.ty) {
                ("/client/benchmark/foo", OscType::Int(i)) => {
                },
                ("/client/bar/baz", OscType::String(s)) => {
                },
            }
        }
    }

    o2.finish();
    println("CLIENT DONE");
}
