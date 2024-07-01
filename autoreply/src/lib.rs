#![no_std]

use gstd::msg;
use gstd::ActorId;

static mut COUNTER: i32 = 256;

#[no_mangle]
unsafe extern "C" fn handle() {
    let command = msg::load_bytes().expect("Invalid message");

    // msg::send_bytes(ActorId::zero(), command.clone(), 0).expect("nothing");
    // msg::send(ActorId::zero(), COUNTER, 0).expect("nothing");
    // let mut counter = unsafe { COUNTER };
    
    match command.as_slice() {
        b"inc" => COUNTER += 1,
        b"dec" => COUNTER -= 1,
        b"get" => (),
        _ => (),
    };

    // msg::reply(COUNTER, 0).expect("Unable to reply");
}
