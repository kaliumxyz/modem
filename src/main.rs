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
    files: Vec<String>,

    #[clap(short, long)]
    mute: bool,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for path in args.files {
        println!("{:?}, {:?}", path, args.mute);
        parse_file(path, !args.mute);
        // play_file(item);
    }
}

#[allow(dead_code)]
struct Words {
    sin: String,
    saw: String,
}

/// this gets the file path, reads the file, and passes it to the tokenizer
fn parse_file(path: String, mute: bool) {
    let stack: Vec<u16> = Vec::new();
    let file = BufReader::new(File::open(path).expect("open failed"));
    println!("{:?}!", file);
    for line in file.lines() {
        for word in line.expect("Unable to read line").split_whitespace() {
            if mute {
                let res = match word {
                    "//" => break,
                    "sin" => "sin",
                    // 0..9 => stack.push(ch),
                    _ => continue,
                };
                println!("Word: {}", res);
            } else {
                match word {
                    "//" => break,
                    "sin" => {},
                    // 0..9 => stack.push(ch),
                    _ => continue,
                }
                println!("Word: {}", word);
            }
        }
    }
}


#[allow(dead_code)]
fn sin() {
    let devices = cpal::default_host().output_devices().unwrap();
    let device = devices.last().unwrap();
    let mut config = device.default_output_config().unwrap().config();
    config.channels = 2;
    // Generates a saw wave signal at 1hz to be sampled 4 times per second.
    let mut signal = signal::rate(44100.0).const_hz(44100.0).saw();
    let stream = device
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
