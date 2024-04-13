use std::process::Command;

use probe_rs::probe::list::Lister;

fn main() {
    let microbit_data = parse_microbit_data();
    let lister = Lister::new();
    let probes = lister.list_all();

    if probes.is_empty() {
        println!("No probes found");
        return;
    }

    let microbits = probes
        .iter()
        .filter_map(|probe| {
            let probe_serial_id = probe.serial_number.clone().unwrap_or_default();
            let microbit = microbit_data
                .iter()
                .find(|microbit| microbit.serial_id == probe_serial_id)?;
            Some(microbit)
        })
        .collect::<Vec<&Microbit>>();

    println!("Flashing to {} microbit(s)", microbits.len());

    for microbit in microbits {
        let mut command = Command::new(
            std::path::Path::new(&std::env::var("HOME").unwrap()).join(".cargo/bin/cargo-embed"),
        );
        command.args([
            "--chip",
            &microbit.chip,
            "--probe-selector",
            &format!("0d28:0204:{}", microbit.serial_id),
            "--target",
            &microbit.target,
            "--release",
        ]);
        println!("{command:?}");
        command.spawn().unwrap().wait().unwrap();
    }
}

#[derive(Debug)]
struct Microbit {
    serial_id: String,
    target: String,
    chip: String,
}

fn parse_microbit_data() -> Vec<Microbit> {
    let microbit_datia_str = include_str!("../microbit_data.txt");
    microbit_datia_str
        .lines()
        .map(|line| {
            let mut microbit_info = line.split_whitespace();
            Microbit {
                serial_id: microbit_info.next().unwrap().to_owned(),
                target: microbit_info.next().unwrap().to_owned(),
                chip: microbit_info.next().unwrap().to_owned(),
            }
        })
        .collect::<Vec<_>>()
}
