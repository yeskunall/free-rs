extern crate cfonts;

use std::error::Error;

use cfonts::{Align, BgColors, Colors, Env, Fonts, Options, say};
use clap::Parser;
use free::{MemMetrics, WithError, get_mem_usage};
use serde::Serialize;
use size::Size;

#[derive(Debug, Default, Serialize)]
pub struct Metrics {
    pub memory: MemMetrics,
}

#[derive(Parser, Debug)]
#[command(
    about,
    long_about = "A friendlier vm_stat(1) written in Rust.\n\n\
    free displays the total amount of free and used physical and\n\
    swap memory in the system, as well as the buffers and caches used by the\n\
    kernel.",
    version
)]
struct Args {}

pub fn get_metrics() -> WithError<Metrics> {
    let mut metrics = Metrics::default();

    metrics.memory = get_mem_usage()?;

    Ok(metrics)
}

fn main() -> Result<(), Box<dyn Error>> {
    say(Options {
        text: String::from("Free"),
        font: Fonts::FontTiny,
        colors: vec![Colors::Candy],
        background: BgColors::Transparent,
        align: Align::Left,
        letter_spacing: 1,
        line_height: 1,
        spaceless: false,
        max_length: 0,
        gradient: Vec::new(),
        independent_gradient: false,
        transition_gradient: false,
        raw_mode: false,
        env: Env::Cli,
        ..Options::default()
    });

    let _args = Args::parse();

    let ram_total = get_metrics()?.memory.ram_total;
    let ram_usage = get_metrics()?.memory.ram_usage;
    let swap_total = get_metrics()?.memory.swap_total;
    let swap_usage = get_metrics()?.memory.swap_usage;

    println!("Mem total: {}", Size::from_bytes(ram_total));
    println!("Mem usage: {}", Size::from_bytes(ram_usage));

    println!("Swap total: {}", swap_total);
    println!("Swap usage: {}", swap_usage);

    Ok(())
}
