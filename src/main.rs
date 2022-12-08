use bit_player::control_song;
use bit_player::song_controller::get_files;
use std::{fs::File, io};
use std::io::{BufReader, Read};
use rodio::{Decoder, OutputStream, source::Source, Sink};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create the audio track queue with the Sink data type
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("Wordy.mp3").unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let mut action = String::new();
    println!("type 'play' if you want to play the song");
    io::stdin().read_line(&mut action).expect("unrecognized action");

    if action.trim_end() == "play" {
        // Add the source to the sink
        sink.append(source);
        action.clear();

        loop {
            println!("input action (play/pause)");
            io::stdin().read_line(&mut action).expect("unrecognized action");
            control_song(&sink, action.trim_end());
            action.clear()
        }
    }
    sink.sleep_until_end();
}
