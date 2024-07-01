#![no_std]

use gstd::{msg, prelude::*, ActorId, MessageId};
use gstd::exec::{wait, wake};
use controller_io::*;

static mut CONTRACT: Option<ActorId> = None;
static mut OWNER: Option<ActorId> = None;
static mut STATUS: Option<Status> = None;

pub enum Status {
    Idle,
    MessageSent(MessageId, MessageId),
    MessageReceived(i32),
}

#[no_mangle]
unsafe extern "C" fn init() {
    // TODO: 初始化 Counter 合约地址
    let id = msg::load().expect("Failed to load payload");
    CONTRACT = Some(id);
    OWNER = Some(msg::source());
    STATUS = Some(Status::Idle);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    // TODO: 向 Counter 地址发送 Inc/Dec/Get 消息
    let id = CONTRACT.as_mut().expect("State isn't initialized");

    let payload: Action = msg::load().expect("Failed to load payload");

    let status = STATUS.as_mut().expect("Status isn't initialized");

    match *status {
        Status::Idle => {
            let out_msg_id = match payload {
                Action::Inc => msg::send(*id, b"inc", 0).expect("Failed to inc"),
                Action::Dec => msg::send(*id, b"dec", 0).expect("Failed to dec"),
                Action::Get => msg::send(*id, b"get", 0).expect("Failed to get"),
            };
            *status = Status::MessageSent(msg::id(), out_msg_id);
            wait();
        }
        Status::MessageSent(_, _) => {
            msg::reply(Reply::NotReady, 0).expect("Failed to reply");
        }
        Status::MessageReceived(n) => {
            *status = Status::Idle;
            msg::reply(Reply::Counter(n), 0).expect("Failed to reply");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn handle_reply() {
    // TODO: 接收 Get 消息的回复
    let reply: i32 = msg::load().expect("Failed to load payload");

    let status = STATUS.as_mut().expect("Status isn't initialized");

    let reply_to = msg::reply_to().unwrap();

    /*
    if let Status::MessageSent(in_msg_id, out_msg_id) = status && out_msg_id == reply_to {
        status = Status::MessageReceived(reply);
        wake(in_msg_id);
    }
    */

    if let Status::MessageSent(in_msg_id, out_msg_id) = *status {
        if out_msg_id == reply_to {
            *status = Status::MessageReceived(reply);
            wake(in_msg_id).expect("Failed to wake message");
        }
    }

    // let owner = OWNER.expect("Failed to get owner");

    // msg::send(owner, reply, 0).expect("Failed to send reply");
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let id = unsafe { CONTRACT.take().expect("State isn't initialized") };
    msg::reply(id, 0).expect("Failed to reply from `state()`");
}
