#![feature(result_option_map_or_default)]

mod events;
mod manifest;
pub use manifest::{Manifest, ManifestMap};

use lcf::{
    lmu::event::{Event as LcfEvent, page::EventPage},
    raw::lmu::event::commands::Commands,
};

pub fn build<S: std::hash::BuildHasher>(
    src: &str,
    codepage: &'static encoding_rs::Encoding,
    commands: &std::collections::HashMap<std::sync::Arc<str>, std::sync::Arc<Commands>, S>,
) -> impl Iterator<Item = LcfEvent> {
    ron::from_str::<Vec<events::Event>>(src)
        .unwrap()
        .into_iter()
        .scan(0, move |id, event| {
            Some(convert(id, codepage, commands, event))
        })
        .flatten()
}

fn convert<S: std::hash::BuildHasher>(
    id: &mut usize,
    codepage: &'static encoding_rs::Encoding,
    commands: &std::collections::HashMap<std::sync::Arc<str>, std::sync::Arc<Commands>, S>,
    event: events::Event,
) -> Vec<LcfEvent> {
    let x = position_to_range(&event.x);
    let y = position_to_range(&event.y);
    let name = event.name.map(|name| codepage.encode(&name).0.to_vec());

    itertools::iproduct!(x, y)
        .map(|(x, y)| {
            *id += 1;
            LcfEvent {
                id: *id as u32,
                name: name
                    .clone()
                    .unwrap_or_else(|| codepage.encode(&format!("EV{id:04}")).0.to_vec()),
                x,
                y,
                pages: event
                    .pages
                    .iter()
                    .map(|page| EventPage {
                        commands: commands
                            .get(page.file.as_str())
                            .map_or_default(|x| x.0.clone()),
                        graphic: (&page.graphic).into(),
                        ..Default::default()
                    })
                    .collect(),
            }
        })
        .collect()
}

fn position_to_range(pos: &events::Position) -> std::ops::RangeInclusive<u32> {
    match pos {
        events::Position::Constant(a) => *a..=*a,
        events::Position::Range(a, b) if a <= b => *a..=*b,
        events::Position::Range(a, b) => *b..=*a,
    }
}
