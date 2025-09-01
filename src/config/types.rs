use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::input::Input; // Assuming Input is in super::input

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum WorkingMode {
    Cmd,
    Serve,
    Repl,
}

impl WorkingMode {
    pub fn is_serve(&self) -> bool {
        matches!(self, WorkingMode::Serve)
    }
    pub fn is_repl(&self) -> bool {
        matches!(self, WorkingMode::Repl)
    }
    pub fn is_cmd(&self) -> bool {
        matches!(self, WorkingMode::Cmd)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LastMessage {
    pub input: Input,
    pub output: String,
    pub continuous: bool,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct StateFlags: u32 {
        const SESSION_EMPTY = 0b00000001;
        const SESSION = 0b00000010;
        const ROLE = 0b00000100;
        const AGENT = 0b0001000;
        const RAG = 0b0010000;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AssertState {
    Pass,
    Bare,
    False(StateFlags),
    True(StateFlags),
    TrueFalse(StateFlags, StateFlags),
}

impl AssertState {
    pub fn pass() -> Self {
        Self::Pass
    }

    pub fn bare() -> Self {
        Self::Bare
    }

    pub fn assert(&self, flags: StateFlags) -> bool {
        match self {
            AssertState::Pass => true,
            AssertState::Bare => flags.is_empty(),
            AssertState::False(assert_flags) => !flags.intersects(*assert_flags),
            AssertState::True(assert_flags) => flags.contains(*assert_flags),
            AssertState::TrueFalse(true_flags, false_flags) => {
                flags.contains(*true_flags) && !flags.intersects(*false_flags)
            }
        }
    }
}
