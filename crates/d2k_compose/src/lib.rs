#![feature(result_option_map_or_default)]

mod compose;
mod events;
mod manifest;

pub use compose::compose;

use lcf::{
    lmu::event::{Event as LcfEvent, page::EventPage},
    raw::lmu::event::commands::Commands,
};
