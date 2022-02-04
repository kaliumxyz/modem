use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rodio::{source::SineWave, source::Source, Decoder, Device, Devices, OutputStream};
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate dasp;
use dasp_signal::{self as signal, Signal};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: Option<String>,

    files: Vec<String>,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for item in args.files {
        println!("{:?}!", item);
        parse_file(item);
        // play_file(item);
    }
}

#[allow(dead_code)]
struct Ops {
    sin: String,
    saw: String,
}

fn parse_file(item: String) {
    let stack: Vec<u16> = Vec::new();
    let file = BufReader::new(File::open(item).expect("open failed"));
    println!("{:?}!", file);
    for line in file.lines() {
        let mut is_comment = 0;
        for ch in line.expect("Unable to read line").chars() {
            match ch {
                '/' => is_comment += 1,
                's' => sin(),
                // 0..9 => stack.push(ch),
                _ => is_comment = 0,
            }
            if is_comment == 2 {
                break;
            } else if is_comment > 0 {
                continue;
            }
            println!("Character: {}", ch);
        }
    }
}

#[allow(dead_code)]
fn sin() {
    let mut devices = cpal::default_host().output_devices().unwrap();
    let mut device = devices.last().unwrap();
    let mut config = device.default_output_config().unwrap().config();
    config.channels = 2;
    // Generates a saw wave signal at 1hz to be sampled 4 times per second.
    let mut signal = signal::rate(44100.0).const_hz(200.0).saw();
    let mut stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
                for frame in data.chunks_mut(1) {
                    let value = cpal::Sample::from::<f32>(&(signal.next() as f32));
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            move |err| {
                // react to errors here.
            },
        )
        .unwrap();

    // let (_stream, stream_handle) = OutputStream::try_from_device(&device).ok().unwrap();
    while true {}
}

#[allow(dead_code)]
fn play_file(item: String) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(item).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
