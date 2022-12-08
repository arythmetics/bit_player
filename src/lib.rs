// Internal Modules
pub mod song_controller;

//External Modules
use rodio::Sink;

pub fn control_song(queue: &Sink, action: &str) {
    if action == "play" {
        queue.play()
    }
    if action == "pause" {
        queue.pause()
    }
    else {
        println!("unrecognized action")
    }
}