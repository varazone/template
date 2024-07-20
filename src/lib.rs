#![no_std]

use automation_io::*;
use gstd::{actor_id, exec, msg, prelude::*, ReservationId, Reservations};

// 延迟消息
// 1. send vs send_delayed

// 单次 Gas 预留
// 1. reserve 限额
// 2. 超时自动 unreserve
// 3. 手动 unreserve
// 4. send_from_reservation, 同一 rid 不可多次用
static mut RESERVED: ReservationId = ReservationId::zero();

// 多次 Gas 预留
// 1. 多次 reserve 限额
// 2. 定时任务, self-execution
static mut MANAGER: Reservations = Reservations::new();

fn self_execution() {
    // 可选: 向 owner 地址发送消息，观察 mailbox 内容
    let reservation = unsafe { MANAGER.try_take_reservation(100_000_000) };
    if let Some(reservation) = reservation {
        msg::send_bytes_from_reservation(
            reservation.id(),
            actor_id!("0x7453a73e8398c970a2b17319e3084874e47b737bd0b5f1a1f405a382e6b05458"),
            format!("remaining: {}", unsafe { MANAGER.count_valid() }),
            0,
        )
        .expect("Failed to send message from reservation");
    }

    // 通过向自身发送延迟消息，触发下一次执行
    let reservation = unsafe { MANAGER.try_take_reservation(100_000_000) };
    if let Some(reservation) = reservation {
        msg::send_bytes_delayed_from_reservation(
            reservation.id(),
            exec::program_id(),
            "send_bytes_delayed_from_reservation",
            0,
            1,
        )
        .expect("Failed to send message from reservation");
    }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    if msg::source() == exec::program_id() {
        self_execution();
        return;
    }

    let payload = msg::load().expect("Failed to load payload");
    let mut rid = unsafe { RESERVED };

    let reply = match payload {
        Action::Reserve { gas, blocks } => {
            unsafe {
                rid = exec::reserve_gas(gas, blocks).expect("Failed to reserve");
                RESERVED = rid;
            }
            Event::Reserved(gas, blocks, rid)
        }
        Action::ReserveMany { gas, blocks, times } => {
            unsafe {
                for _ in 0..times {
                    MANAGER
                        .reserve(gas, blocks)
                        .expect("Failed to reserve many");
                }
            }
            Event::ReservedMany(gas, blocks, times)
        }
        Action::Unreserve => {
            let amount = rid.unreserve().expect("Failed to unreserve");
            Event::Unreserved(amount)
        }
        Action::Send { to, payload, value } => {
            msg::send_bytes(to, payload.clone(), value).expect("Failed to send");
            Event::Sent(to, payload, value)
        }
        Action::SendDelayed {
            to,
            payload,
            value,
            delay,
        } => {
            msg::send_bytes_delayed(to, payload.clone(), value, delay).expect("Failed to send");
            Event::SentDelayed(to, payload, value, delay)
        }
        Action::SendFromReservation { to, payload, value } => {
            msg::send_bytes_from_reservation(rid, to, payload.clone(), value)
                .expect("Failed to send");
            Event::SentFromReservation(to, payload, value)
        }
    };
    let _ = msg::reply(reply, 0).expect("Failed to reply");
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    let rid = unsafe { RESERVED };
    let _ = msg::reply(rid, 0);
}
