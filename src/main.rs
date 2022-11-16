use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rodio::{source::SineWave, source::Source, Decoder, Device, Devices, OutputStream, Sink};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

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
        println!("file: {:?}\nmuted: {:?}\n", path, args.mute);
        let vm = State{ compile_nestings: 0, mute: true};
        parse_file(path, vm);
        // play_file(item);
    }
}

pub struct State {
    compile_nestings: u8,
    mute: bool,
}

#[derive(Debug, PartialEq)]
enum Words {
    Sine,
    Default,
    Saw,
    Swap,
    Loop,
    Sin,
    Tan,
    Pi,
    Compile,
    CompileEnd,
    Comment,
    Number(u16),
    Undefined,
}

/// this gets the file path, reads the file, and passes it to the tokenizer
fn parse_file(path: String, mut state: State) {

    let stack: Vec<u16> = Vec::new();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open(path).expect("open failed"));
    // println!("{:?}!", file);
    // TODO: add index for error printing / prettier error printing
    for line in file.lines() {
        for word in line.expect("Unable to read line").split_whitespace() {
            // add unwrapping string and detecting number here. TODO find out if unwrap can ever fail on a string, how???

            let res = match word {
                "//" => Words::Comment,
                "sine" => Words::Sine,
                "sin" => Words::Sin,
                "default" => Words::Default,
                "saw" => Words::Saw,
                "loop" => Words::Loop,
                "swap" => Words::Swap,
                "tan" => Words::Tan,
                "pi" => Words::Pi,
                ":" => Words::Compile,
                ";" => Words::CompileEnd,
                _ => Words::Undefined,
            };

            match res {
                Words::Compile => {
                    // add some way to start a compiling mode for defining new words
                    state.compile_nestings += 1;
                    // TODO: line goes to recursive compilation
                    break;
                },
                Words::Swap => {
                    break;
                },
                Words::Comment => {
                    break;
                },
                Words::Sine => {
                    // Add a dummy source of the sake of the example.
                    let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
                    sink.append(source);
                },
                // Words::Number => {
                //     stack.push(word);
                // }
                Words::Undefined => {
                    panic!("Undefined word: {:?}", word);
                //     // match x {
                //     //     0..=9 => Words::Number(word.parse::<u16>().unwrap()),
                //     // };
                },
                _ => {},
            }
            // if res == Words::comment {
            //     break;
            // }
            println!("Word: {}, {:?}", word, res);
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
