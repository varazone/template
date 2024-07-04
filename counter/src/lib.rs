#![no_std]

use gstd::msg;

static mut COUNTER: i32 = 0;

#[no_mangle]
unsafe extern fn handle() {
    gstd::debug!("whoami: {:?}", gstd::exec::program_id());
    gstd::debug!("sender: {:?}", msg::source());

    let command = msg::load_bytes().expect("Invalid message");

    gstd::debug!("command: {}", gstd::String::from_utf8_lossy(&command));
    gstd::debug!("payload: {:x?}", &command);

    match gstd::dbg!(command.as_slice()) {
        b"inc" => COUNTER += 1,
        b"dec" => COUNTER -= 1,
        b"get" => (),
        _ => todo!(),
    };

    msg::reply(COUNTER, 0).expect("Unable to reply");
}

#[cfg(test)]
mod tests {
    use gtest::{Log, Program, System};

    #[test]
    fn my_counter_test() {
        let system = System::new();
        system.init_logger();
        let program = Program::current(&system);
        assert_eq!(program.id(), 1.into());

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
