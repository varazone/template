#![no_std]

use gstd::msg;

static mut COUNTER: i32 = 0;

#[no_mangle]
unsafe extern fn handle() {
    let command = msg::load_bytes().expect("Invalid message");

    match command.as_slice() {
        b"inc" => COUNTER += 1,
        b"dec" => COUNTER -= 1,
        b"get" => (),
        _ => todo!(),
    };

    msg::reply(COUNTER, 0).expect("Unable to reply");
}
