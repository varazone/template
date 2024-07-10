#![no_std]

use controller_io::*;
use gstd::{msg, prelude::*, ActorId};

static mut CONTRACT: Option<ActorId> = None;
static mut OWNER: Option<ActorId> = None;

#[gstd::async_init]
async fn init() {
    // TODO: 初始化 Counter 合约地址
    let id = msg::load().expect("Failed to load payload");
    let admin = msg::source();
    unsafe {
        CONTRACT = Some(id);
        OWNER = Some(admin);
    }
}

#[gstd::async_main]
async fn main() {
    // TODO: 向 Counter 地址发送 Inc/Dec/Get 消息
    let id = unsafe { CONTRACT.as_mut().expect("State isn't initialized") };
    let action: Action = msg::load().expect("Failed to load payload");

    let future: gstd::msg::CodecMessageFuture<i32> = match action {
        Action::Inc => msg::send_for_reply_as(*id, b"inc", 0, 0).expect("Failed to send inc"),
        Action::Dec => msg::send_for_reply_as(*id, b"dec", 0, 0).expect("Failed to send dec"),
        Action::Get => msg::send_for_reply_as(*id, b"get", 0, 0).expect("Failed to send get"),
    };
    let n = future.await.expect("Unable to get reply");
    msg::reply(n, 0).expect("Failed to reply");
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
        let expected = Info {
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
