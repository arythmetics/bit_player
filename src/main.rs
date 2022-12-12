use bit_player::control_song;
use bit_player::song_controller::get_files;
use std::{fs::File, io};
use std::io::{BufReader, Read};
use rodio::{Decoder, OutputStream, source::Source, Sink};
use dotenv::dotenv;
use std::env;
use std::ops::Range;

fn main() {
    dotenv().ok();
    
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create the audio track queue with the Sink data type
    let sink = Sink::try_new(&stream_handle).unwrap();

    println!("Select the song number you would like to play");
    let songs = get_files();
    for i in &songs {
        println!("{}: {}", i.0, i.1)
    }

    let mut song_number_input = String::new();
    loop {
        let song_range = 0..songs.len();
        io::stdin().read_line(&mut song_number_input).expect("unrecognized action");

        if song_range.contains(&song_number_input.trim_end().parse().unwrap_or(usize::MAX)) {
            break
        } else {
            println!("Please select a number between {} and {}", 0, songs.len())
        }
        
    }
    let selected_song: u32 = song_number_input.trim_end().parse().unwrap();
    
    // Load a sound from a file, using a path relative to Cargo.toml
    let song_path = format!("{}/{}", env::var("SONGS_FOLDER").unwrap(), songs.get(&selected_song).unwrap());
    let file = BufReader::new(File::open(song_path).unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    
    let mut action = String::new();
    loop {
        println!("input action (play/pause/stop)");
        io::stdin().read_line(&mut action).expect("unrecognized action");
        if action.trim_end() == "stop" {
            break
        }
        control_song(&sink, action.trim_end());
        action.clear()
    }
}
