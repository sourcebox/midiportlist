#![doc = include_str!("../README.md")]

use midir::{MidiInput, MidiOutput};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    println!("Input ports:");
    let midi_input = MidiInput::new(env!("CARGO_PKG_NAME"))?;
    for (i, p) in midi_input.ports().iter().enumerate() {
        println!("    ({}) {}", i, midi_input.port_name(p)?);
    }

    println!();

    println!("Output ports:");
    let midi_output = MidiOutput::new(env!("CARGO_PKG_NAME"))?;
    for (i, p) in midi_output.ports().iter().enumerate() {
        println!("    ({}) {}", i, midi_output.port_name(p)?);
    }

    Ok(())
}
