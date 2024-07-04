#![no_std]

use controller_io::*;
use gstd::{msg, prelude::*, ActorId};

static mut OWNER: Option<ActorId> = None;
static mut CONTRACT: Option<ActorId> = None;

#[no_mangle]
unsafe extern fn init() {
    // TODO: 初始化用户地址与 Counter 合约地址
    let id = msg::load().expect("Failed to load payload");
    CONTRACT = Some(id);
    OWNER = Some(msg::source());
}

#[no_mangle]
unsafe extern fn handle() {
    // TODO: 向 Counter 地址发送 Inc/Dec/Get 消息
    let id = CONTRACT.as_mut().expect("State isn't initialized");

    let payload: Action = msg::load().expect("Failed to load payload");

    match payload {
        Action::Inc => msg::send(*id, b"inc", 0).expect("Failed to inc"),
        Action::Dec => msg::send(*id, b"dec", 0).expect("Failed to dec"),
        Action::Get => msg::send(*id, b"get", 0).expect("Failed to get"),
    };
}

#[no_mangle]
unsafe extern fn handle_reply() {
    // TODO: 接收消息的回复，并转发给用户
    let reply: i32 = msg::load().expect("Failed to load payload");

    let owner = OWNER.expect("Failed to get owner");

    msg::send(owner, reply, 0).expect("Failed to send reply");
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let info = unsafe {
        Info {
            owner: OWNER.take().expect("OWNER isn't initialized"),
            counter: CONTRACT.take().expect("CONTRACT isn't initialized"),
        }
    };
    msg::reply(info, 0).expect("Failed to reply from `state()`");
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtest::{Log, Program, System};

    #[test]
    fn my_controller_test() {
        let system = System::new();
        system.init_logger();
        let counter = Program::from_file(
            &system,
            "../target/wasm32-unknown-unknown/release/counter.opt.wasm",
        );
        assert_eq!(counter.id(), 1.into());
        let program = Program::current(&system);
        assert_eq!(program.id(), 2.into());

        let counter_init_result = counter.send_bytes(42, &[]);
        assert!(!counter_init_result.main_failed());

        let init_result = program.send(42, counter.id());
        assert!(!init_result.main_failed());

        let state: Info = program.read_state(b"").unwrap();
        let expected = Info{
            owner: 42.into(),
            counter: 1.into(),
        };
        assert_eq!(state, expected);

        let result = program.send(42, Action::Inc);
        let expected = Log::builder().payload(1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, Action::Dec);
        let expected = Log::builder().payload(0i32);
        assert!(result.contains(&expected));

        let result = program.send(42, Action::Get);
        let expected = Log::builder().payload(0i32);
        assert!(result.contains(&expected));

        let result = program.send(42, Action::Dec);
        let expected = Log::builder().payload(-1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, Action::Get);
        let expected = Log::builder().payload(-1i32);
        assert!(result.contains(&expected));

        let result = program.send(42, *b"ngmi");
        assert!(result.main_failed());
    }
}
