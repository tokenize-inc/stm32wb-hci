//! Events and return parameters whose decoders are disabled by cargo features.
//!
//! Run with `cargo test --no-default-features --features event-command-complete --test unparsed`.
#![cfg(all(
    feature = "event-command-complete",
    not(feature = "event-hardware-error"),
    not(feature = "event-le-long-term-key-request"),
    not(feature = "event-gatt-procedure-timeout"),
    not(feature = "return-reset"),
    not(feature = "return-gatt-init"),
))]

extern crate stm32wb_hci as hci;

use hci::event::command::ReturnParameters;
use hci::event::*;
use hci::vendor::event::response::VendorReturnParameters;
use hci::vendor::event::VendorEvent;

#[test]
fn disabled_event() {
    let buffer = [0x10, 1, 0x01];
    match Event::new(Packet(&buffer)) {
        Ok(Event::Unparsed(0x10)) => (),
        other => panic!("Did not get unparsed event: {:?}", other),
    }
}

#[test]
fn disabled_le_meta_event() {
    let buffer = [
        0x3E, 13, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(Event::UnparsedLeMeta(0x05)) => (),
        other => panic!("Did not get unparsed LE meta event: {:?}", other),
    }
}

#[test]
fn disabled_vendor_event() {
    let buffer = [0xFF, 4, 0x02, 0x0C, 0x01, 0x02];
    match Event::new(Packet(&buffer)) {
        Ok(Event::Vendor(VendorEvent::Unparsed(0x0C02))) => (),
        other => panic!("Did not get unparsed vendor event: {:?}", other),
    }
}

#[test]
fn disabled_return_parameters() {
    let buffer = [0x0E, 4, 1, 0x03, 0x0C, 0x00];
    match Event::new(Packet(&buffer)) {
        Ok(Event::CommandComplete(command::CommandComplete {
            return_params: ReturnParameters::Unparsed(params),
            ..
        })) => {
            assert_eq!(params.opcode, hci::Opcode(0x0C03));
            assert_eq!(params.status, hci::Status::Success);
        }
        other => panic!("Did not get unparsed return parameters: {:?}", other),
    }
}

#[test]
fn disabled_vendor_return_parameters() {
    let buffer = [0x0E, 4, 1, 0x01, 0xFD, 0x0C];
    match Event::new(Packet(&buffer)) {
        Ok(Event::CommandComplete(command::CommandComplete {
            return_params: ReturnParameters::Vendor(VendorReturnParameters::Unparsed(params)),
            ..
        })) => {
            assert_eq!(params.opcode, hci::Opcode(0xFD01));
            assert_eq!(params.status, hci::Status::CommandDisallowed);
        }
        other => panic!("Did not get unparsed return parameters: {:?}", other),
    }
}

#[test]
fn unknown_opcode_is_still_an_error() {
    let buffer = [0x0E, 3, 1, 0x67, 0x43];
    match Event::new(Packet(&buffer)) {
        Err(Error::UnknownOpcode(opcode)) => assert_eq!(opcode.0, 0x4367),
        other => panic!("Did not get unknown opcode: {:?}", other),
    }
}
