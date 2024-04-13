use std::process::Command;

use probe_rs::probe::list::Lister;

fn main() {
    // First get all valid probes.
    // Using probe-rs for this as it is also the backend of the embed tool.
    let probe_data = parse_probe_data();
    let lister = Lister::new();
    let all_probes = lister.list_all();

    // If no probes dont bother.
    if all_probes.is_empty() {
        println!("No probes found");
        return;
    }

    // Filter all probes for the ones we care about.
    // Return a reference to the probe info that we parsed.
    let matched_probes = all_probes
        .iter()
        .filter_map(|probe| {
            let probe_serial_id = probe.serial_number.clone().unwrap_or_default();
            let matched_probe = probe_data
                .iter()
                .find(|probe| probe.serial_id == probe_serial_id)?;
            Some(matched_probe)
        })
        .collect::<Vec<&Probe>>();

    println!("Flashing to {} probe(s)", matched_probes.len());

    // For each found probe run the embed command.
    // This tool expects the correct version to be install
    // This tool is also for personal use so its not worth the effort right now.

    // Other values like the VID and PID are hardcoded to my specific boards.
    // Again. No need to improve it till I have a need for it.

    // The embed command also expects to be in the dir that contains the project to flash.
    for probe in matched_probes {
        let mut command = Command::new(
            std::path::Path::new(&std::env::var("HOME").unwrap()).join(".cargo/bin/cargo-embed"),
        );
        command.args([
            "--chip",
            &probe.chip,
            "--probe-selector",
            &format!("0d28:0204:{}", probe.serial_id),
            "--target",
            &probe.target,
            "--release",
        ]);
        command.spawn().unwrap().wait().unwrap();
    }
}

#[derive(Debug)]
struct Probe {
    serial_id: String,
    target: String,
    chip: String,
}

// Simply parses the correct file and parses it.
fn parse_probe_data() -> Vec<Probe> {
    let probe_data_str = include_str!("../probe_data.txt");
    probe_data_str
        .lines()
        .map(|line| {
            let mut probe_data = line.split_whitespace();
            Probe {
                serial_id: probe_data
                    .next()
                    .expect("No serial id found in data")
                    .to_owned(),
                target: probe_data
                    .next()
                    .expect("No target found in data")
                    .to_owned(),
                chip: probe_data.next().expect("No chip found in data").to_owned(),
            }
        })
        .collect::<Vec<_>>()
}
