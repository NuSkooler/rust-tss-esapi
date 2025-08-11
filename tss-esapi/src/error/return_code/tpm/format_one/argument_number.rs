// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use bitfield::bitfield;

/// Enum representing the type of argument with which
/// the format one tpm response code is associated with.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArgumentNumber {
    Parameter(u8),
    Handle(u8),
    Session(u8),
}

impl From<u8> for ArgumentNumber {
    fn from(value: u8) -> Self {
        let argument_number_structure = ArgumentNumberStructure(value);
        if argument_number_structure.is_parameter() {
            ArgumentNumber::Parameter(argument_number_structure.parameter_number())
        } else if argument_number_structure.is_session() {
            ArgumentNumber::Session(argument_number_structure.session_number())
        } else {
            ArgumentNumber::Handle(argument_number_structure.handle_number())
        }
    }
}

impl From<ArgumentNumber> for u8 {
    fn from(argument_number: ArgumentNumber) -> u8 {
        let mut structure = ArgumentNumberStructure(0);
        match argument_number {
            ArgumentNumber::Parameter(number) => {
                structure.set_is_parameter(true);
                structure.set_parameter_number(number);
            }
            ArgumentNumber::Session(number) => {
                structure.set_is_parameter(false);
                structure.set_is_session(true);
                structure.set_session_number(number);
            }
            ArgumentNumber::Handle(number) => {
                structure.set_is_parameter(false);
                structure.set_is_session(false);
                structure.set_handle_number(number);
            }
        }
        structure.0
    }
}

impl std::fmt::Display for ArgumentNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentNumber::Parameter(number) => {
                write!(f, "associated with parameter number {number}")
            }
            ArgumentNumber::Session(number) => {
                write!(f, "associated with session number {number}")
            }
            ArgumentNumber::Handle(number) => {
                write!(f, "associated with handle number {number}")
            }
        }
    }
}

bitfield! {
    /// A struct representing the the argument in format one
    /// TPM return code.
    #[derive(PartialEq, Copy, Clone)]
    struct ArgumentNumberStructure(u8);
    impl Debug;
    is_parameter, set_is_parameter: 0;
    u8, parameter_number, set_parameter_number: 5, 2;
    is_session, set_is_session: 5;
    u8, session_number, set_session_number: 4, 2;
    u8, handle_number, set_handle_number: 4, 2;
}
