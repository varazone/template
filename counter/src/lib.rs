#![no_std]

use gstd::msg;

static mut COUNTER: i32 = 0;

#[no_mangle]
unsafe extern fn init() {
    gstd::debug!("whoami: {:?}", gstd::exec::program_id()); // 合约地址
}

#[no_mangle]
unsafe extern fn handle() {
    gstd::debug!("sender: {:?}", msg::source()); // 用户地址

    let command = msg::load_bytes().expect("Invalid message");

    gstd::debug!("payload: {:x?}", &command); // 消息 bytes
    gstd::debug!("command: {}", gstd::String::from_utf8_lossy(&command)); // 消息 str

    match command.as_slice() {
        b"inc" /* 0x696e63 */ => COUNTER += 1,
        b"dec" /* 0x646563 */ => COUNTER -= 1,
        b"get" /* 0x676574 */ => (),
        _ => todo!(),
    };

    gstd::debug!("counter: {}", COUNTER); // 更新后的状态
    msg::reply(COUNTER, 0).expect("Unable to reply");
}

#[cfg(test)]
mod tests {
    use gtest::{Log, Program, System};

    #[test]
    fn my_counter_test() {
        // create test environment
        let system = System::new();
        system.init_logger();

        // create program instance
        let program = Program::current(&system);
        assert_eq!(program.id(), 1.into());

        // initialize program
        let init_result = program.send_bytes(42, &[]);
        assert!(!init_result.main_failed());

        let result = program.send(42, *b"inc");
        let expected = Log::builder().payload(1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"dec");
        let expected = Log::builder().payload(0i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"get");
        let expected = Log::builder().payload(0i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"dec");
        let expected = Log::builder().payload(-1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"get");
        let expected = Log::builder().payload(-1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"ngmi");
        assert!(result.main_failed());
    }
}
