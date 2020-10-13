use std::error;
#[macro_use]
use log::info;

mod err;
mod net;
mod crypto;

fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let song_input = "https://open.spotify.com/track/6LT5zGGT13lQqNt3VUqtuy";

    println!("Requesting a new token.");
    let token = net::request_token()?;

    info!("Received new token: {}", token);

    for song_cdn_id in parse_input(song_input)? {
        let song_data = net::get_song(&song_cdn_id, &token)?;

        println!("Downloaded encrypted song. Decrypting.");

        let unencrypted_song = crypto::decrypt(song_data)?;
    }

    Ok(())
}

fn parse_input(input: &'_ str) -> Result<Vec<String>, Box<dyn error::Error>> {
    if input.contains("/track/") {
        let track = &input[input.find("/track/").unwrap() + "/track/".len()
            ..input.find("?").unwrap_or(input.len())];
        Ok(vec![to_gid(track)])
    } else {
        Err(Box::new(err::BadInputError))
    }
}

fn to_gid(id: &'_ str) -> String {
    hex::encode(id)
}