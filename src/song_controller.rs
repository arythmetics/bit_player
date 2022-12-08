use std::{fs, env, collections::HashMap};

pub fn get_files() -> HashMap<u32, String> {
    let songs_folder = env::var("SONGS_FOLDER").expect("Cannot find folder");
    let songs_list: Vec<String> = fs::read_dir(songs_folder).unwrap()
        .filter_map(|x| {
            let dir_entry = x.ok()?;
            let path_buf = dir_entry.path();
            let file_name = path_buf.file_name()?;
            let string = file_name.to_str()?;
            Some(string.to_string())
        })
        .collect();
    
    let songs_hash_map: HashMap<u32, String> = songs_list
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u32, x.to_string()))
        .collect();

    return songs_hash_map
}