#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, ReservationId};

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = ();
    /// I/O types for the `handle()` entry point.
    type Handle = InOut<Action, Event>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    type State = Out<State>;
}

pub type State = ReservationId;

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    Reserve {
        gas: u64,
        blocks: u32,
    },
    ReserveMany {
        gas: u64,
        blocks: u32,
        times: u32,
    },
    Unreserve,
    Send {
        to: ActorId,
        payload: Vec<u8>,
        value: u128,
    },
    SendDelayed {
        to: ActorId,
        payload: Vec<u8>,
        value: u128,
        delay: u32,
    },
    SendFromReservation {
        to: ActorId,
        payload: Vec<u8>,
        value: u128,
    },
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    Reserved(u64, u32, ReservationId),
    ReservedMany(u64, u32, u32),
    Unreserved(u64),
    Sent(ActorId, Vec<u8>, u128),
    SentDelayed(ActorId, Vec<u8>, u128, u32),
    SentFromReservation(ActorId, Vec<u8>, u128),
}
