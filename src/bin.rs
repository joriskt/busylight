extern crate hidapi;

use std::ops::{Range, RangeInclusive};
use std::{thread, time};
use clap::{ArgMatches, ColorChoice, Parser};
use clap::builder::*;

use busylight::{Busylight, Color, Light};
use busylight::LightError::InvalidColor;

fn main() -> busylight::Result<()> {
    let args = Command::new("busylight")
        .author("Joris Klein Tijssink")
        .version("0.0.1")
        .about("Tiny command to address one or many compatible busylights.")
        .subcommand_required(true)

        .subcommand(
            Command::new("list")
                .about("List compatible busylights")
        )

        .subcommand(
            Command::new("solid")
                .about("Set selected lights to a solid color")
                .arg(Arg::new("color")
                    .required(true))
        )

        .subcommand(
            Command::new("fade")
                .about("Fade to the specified color in the specified time")
                .long_about("Fade from the current state to the provided <color>, taking <time> milliseconds from start to finish.")
                .arg(Arg::new("color")
                    .help("Color to fade to")
                    .required(true))
                .arg(Arg::new("time")
                    .help("Time to take (0 is short, 255 is long)")
                    .value_parser(ValueParser::from(RangeInclusive::new(0, 255)))
                    .required(true))
        )

        .subcommand(
            Command::new("off")
                .about("Turns busylights off")
        )

        .get_matches();

    match args.subcommand() {
        Some(("list", args)) => handle_list(args),
        Some(("off", args)) => handle_off(args),
        Some(("solid", args)) => handle_solid(args),
        Some(("fade", args)) => handle_fade(args), // TODO: impl handle_fade
        Some((cmd, _)) => panic!("command not implemented: {:?}", cmd),
        None => unreachable!() // Handled by clap due to .subcommand_required(true)
    }
}

/// Query HID devices and print a list of all recognized light devices.
fn handle_list(args: &ArgMatches) -> busylight::Result<()> {
    let busylight = Busylight::new()?;
    let lights = busylight.list_lights()?;

    for light in lights {
        let info = light.get_info();
        let interface = info.interface_number();
        let id = info.path();
        let product = light.get_info().product_string().unwrap_or("(unknown device)");
        let driver = light.get_type();
        println!("Found {driver:?} named {product}: ID {id:?} on {interface}");
    }

    Ok(())
}

fn handle_off(args: &ArgMatches) -> busylight::Result<()>  {
    let busylight = Busylight::new()?;
    let lights = busylight.list_lights()?;

    for light in lights {
        light.turn_off()?;
    }

    Ok(())
}

fn handle_solid(args: &ArgMatches) -> busylight::Result<()>  {
    let busylight = Busylight::new()?;
    let lights = busylight.list_lights()?;

    let color_raw = args.get_one("color").unwrap();
    let color = Color::parse(color_raw)
        .ok_or(InvalidColor(color_raw.clone()))?;

    for light in lights {
        light.set_solid_color(&color)?;
    }

    Ok(())
}

fn handle_fade(args: &ArgMatches) -> busylight::Result<()>  {
    let busylight = Busylight::new()?;
    let lights = busylight.list_lights()?;

    let color = Color::parse(args.get_one("color").unwrap()).unwrap();
    let time: &i64 = args.get_one("time").unwrap();

    for light in &lights {
        light.fade_to_color(&color, *time as u8)?;
    }

    // Wait before exiting to let the fade complete.
    let duration = time::Duration::from_millis((time * 40) as u64);
    thread::sleep(duration);

    // Only *after* sleeping do we drop all the instantiated HID devices.
    // THIS IS NECESSARY since Rust will drop them as soon as possible otherwise.
    for light in lights {
        drop(light);
    }

    Ok(())
}