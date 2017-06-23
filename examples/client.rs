//  o2client.c - part of performance benchmark
//
//  see o2server.c for details


//#include "o2.h"
extern crate o2;


const max_msg_count: i32 = 50000;

fn client_test(o2_msg_data_ptr data, const char *types,
                 o2_arg_ptr *argv, int argc, void *user_data)
{
    msg_count++;
    // the value we send is arbitrary, but we've already sent
    // 1 message with value 1, so the 2nd message will have 2, etc...
    int32_t i = msg_count + 1;

    // server will shut down when it gets data == -1
    if (msg_count >= max_msg_count) {
        i = -1;
        running = FALSE;
    }
    o2_send(server_addresses[msg_count % N_ADDRS], 0, "i", i);
    if (msg_count % 10000 == 0) {
        printf("client received %d messages\n", msg_count);
    }
    if (msg_count < 100) {
        printf("client message %d is %d\n", msg_count, argv[0]->i32);
    }
    assert(msg_count == argv[0]->i32);
}

fn main(){
    let server_addresses: Vec<std::ffi::CString> = vec![];
    let client_addresses: Vec<std::ffi::CString> = vec![];
    let mut msg_count = 0;
    let mut running = true;

    if let Some(arg) = std::env::args().nth(1){
        o2::debug_flags();
    }

    let o2 = o2::new("test");
    o2.service_new("client");
   
    
    for i in 0..N_ADDRS{
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
            for i in 0..N_ADDRS{
                let mut cs = CString::new();
                write!(&mut cs, b"/client/benchmark/{}", i).unwrap();
                if event.path == cs{
                    if let OscType::Int(i) = event.ty{

                    }
                }
            }
        }
    }

    o2.finish();
    println("CLIENT DONE");
}
