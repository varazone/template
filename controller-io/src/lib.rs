#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = In<ActorId>;
    /// I/O types for the `handle()` entry point.
    ///
    /// Here the [`PingPong`] type is used for both incoming and outgoing messages.
    type Handle = InOut<Action, Reply>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    ///
    /// You can also specify just an output ([`Out`]) or input ([`In`](gmeta::In)) type, if both
    /// ([`InOut`]) are expected like here.
    type State = Out<ActorId>;
}



/// Replies with [`Pong`](PingPong::Pong) if received [`Ping`](PingPong::Ping).
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    Inc,
    Dec,
    Get,
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Reply {
    NotReady,
    Counter(i32),
}
