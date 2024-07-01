#![no_std]

use gstd::{msg, prelude::*, ActorId};
use controller_io::*;

static mut CONTRACT: Option<ActorId> = None;
static mut OWNER: Option<ActorId> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    // TODO: 初始化 Counter 合约地址
    let id = msg::load().expect("Failed to load payload");
    CONTRACT = Some(id);
    OWNER = Some(msg::source());
}

#[no_mangle]
unsafe extern "C" fn handle() {
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
unsafe extern "C" fn handle_reply() {
    // TODO: 接收 Get 消息的回复
    let reply: i32 = msg::load().expect("Failed to load payload");

    let owner = OWNER.expect("Failed to get owner");

    msg::send(owner, reply, 0).expect("Failed to send reply");
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let id = unsafe { CONTRACT.take().expect("State isn't initialized") };
    msg::reply(id, 0).expect("Failed to reply from `state()`");
}
