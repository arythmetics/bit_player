use std::{fs::File, io};
use std::io::{BufReader, Read};
use rodio::{Decoder, OutputStream, source::Source, Sink};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create the audio track queue with the Sink data type
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("crave_you.mp3").unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let mut action = String::new();
    println!("type 'play' if you want to play the song");
    io::stdin().read_line(&mut action).expect("unrecognized action");

    if action.trim_end() == "play" {
        // Add the source to the sink
        sink.append(source);

        loop {
            if action.trim_end() == "play" {
                action.clear();
                println!("type 'pause' to pause the song");
                io::stdin().read_line(&mut action).expect("unrecognized action");

                if action.trim_end() == "pause" {
                    sink.pause()
                }
            }
            if action.trim_end() == "pause" {
                action.clear();
                println!("type 'play' to resume the song");
                io::stdin().read_line(&mut action).expect("unrecognized action");

                if action.trim_end() == "play" {
                    sink.play()
                }
            }
            else {
                println!("unrecognized action, try again");
                io::stdin().read_line(&mut action).expect("unrecognized action");
            }
        }
    }
    sink.sleep_until_end();
}
