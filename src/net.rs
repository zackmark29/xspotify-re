use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, COOKIE, REFERER, USER_AGENT, ACCEPT_ENCODING, HOST};
use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error;
use std::str;
use crate::err;
#[macro_use]
use log::info;

const TOKEN_REQUEST_URL: &'static str =
    "https://open.spotify.com/get_access_token?reason=transport&productType=web_player";

const WEB_PLAYER_USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.88 Safari/537.36";

#[derive(Deserialize)]
struct Response {
    #[serde(rename(deserialize = "accessToken"))]
    access_token: String,
}

pub fn request_token() -> Result<String, impl Error> {
    let client = reqwest::blocking::Client::new();
    let req = client
        .get(TOKEN_REQUEST_URL)
        .header(ACCEPT_LANGUAGE, "de")
        .header("App-Platform", "WebPlayer")
        .header(USER_AGENT, WEB_PLAYER_USER_AGENT)
        .header(REFERER, "https://open.spotify.com/")
        .header("Spotify-App-Version", "1576851415")
        .header(ACCEPT, "application/json")
        .header(COOKIE, "'. ' \
            sp_t=8216320f1a1b55329cc788d3861bda2d; \
            sp_adid=e0c4b4b0-c2ce-47b1-91cd-9a6e500bd236; \
            _gcl_au=1.1.930311039.1568296866; \
            _hjid=3cfe7894-03cb-49b7-a008-59e9e02a58c6; \
            _fbp=fb.1.1568296866726.817501151; \
            open_env=php; \
            sp_ab=%7B%222019_04_premium_menu%22%3A%22control%22%7D; \
            sp_gaid=0088fcae70df1230ce6227ad4c27d9dfc16b52d53070edbb54dadd; \
            sp_phash=5b159869266f788cd98cb9ddbdfb3bcbeacac1cd; \
            spot=%7B%22t%22%3A1576582774%2C%22m%22%3A%22jp%22%2C%22p%22%3A%22open%22%7D; \
            optimizelyEndUserId=oeu1576847786549r0.3765501627206682; \
            optimizelySegments=%7B%226174980032%22%3A%22search%22%2C%226176630028%22%3A%22none%22%2C%226179250069%22%3A%22false%22%2C%226161020302%22%3A%22gc%22%7D; \
            optimizelyBuckets=%7B%7D; \
            sp_last_utm=%7B%22utm_campaign%22%3A%22your_account%22%2C%22utm_medium%22%3A%22menu%22%2C%22utm_source%22%3A%22spotify%22%7D; \
            __gads=ID=46eef32200ade146:T=1577188825:S=ALNI_MaSsNN-fkmKLUqUmpi87Lkk6BJz4Q; \
            _ga_0KW7E1R008=GS1.1.1577195047.3.1.1577195874.0; \
            _derived_epik=dj0yJnU9ZHhLV1NmV3hhb2VKOWhEcWdGUTRuN0RhbC1OdEFBMnMmbj1tOU5DS1E5UU11U1pQeWlLNTJMRW1RJm09NyZ0PUFBQUFBRjRDR1dJ; \
            _ga=GA1.2.1103680000.1568296854; \
            sp_landing=http%3A%2F%2Fopen.spotify.com%2F; \
            sss=1; \
            _gid=GA1.2.1895535078.1577587497; \
            sp_dc=AQAMQHYD7SGCY6Hu8aFyjDsPfTkv7s4LqxCm9fOMMsph1GR8yCxW_9cqOoeQH8TeFvcTrTEfcFcCiEZemGSWpCcSE9ESBTJVoW0X6ICQUg; \
            sp_key=99d88c42-2b0f-41c8-b3e3-e764398ce474; \
            _gat_gtag_UA_5784146_31=1");

    info!("Sending request {:?}", req);

    match req.send() {
        Ok(req) => {
            /* If the response is OK, there's a body, so .unwrap() is safe.
             * The second unwrap might break if Spotify returns 200 OK when it actually means 404
             * NOT FOUND, i.e. the endpoint moves. It does this. */
            let jr: Response = serde_json::from_str(&req.text().unwrap()).unwrap();

            Ok(jr.access_token)
        }
        Err(err) => Err(err),
    }
}

pub fn get_song(song_id: &'_ str, token: &'_ str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let req = client
        .get(&format!(
            "https://spclient.wg.spotify.com/storage-resolve/files/audio/interactive/{}?product=0",
            song_id
        ))
        .header(ACCEPT, "*/*")
        .header(ACCEPT_ENCODING, "identity")
        .header(HOST, "spclient.wg.spotify.com")
        .header(AUTHORIZATION, &format!("Bearer {}", token));

    info!("Sending request {:?}", req);

    match req.send() {
        Ok(req_body) => {
            let raw_response = req_body.text()?;
            
            info!("Response: {}", raw_response);

            //let raw_song_req = client.get(&raw_response[raw_response.find("http").unwrap()..raw_response.find("=").unwrap()]);
            let raw_song_req = client.get("https://audio4-fa.scdn.co/audio/e6db8ca5795084258db731f5219ed9303f4b5035?1585285677_vKYN1LNNavgmXLkGt-wVLlmpWtlLfGVov2bAsWkT8QY");

            info!("Sending request {:?}", raw_song_req);

            let raw_req_res = raw_song_req.send()?;
 
            match raw_req_res.status() {
                StatusCode::OK => Ok(raw_req_res.text()?),
                _ => Err(Box::new(err::BadResponseError {
                    code: raw_req_res.status().as_u16(),
                    response: raw_req_res.text()?,
                })),
            }
        }
        Err(err) => Err(Box::new(err)),
    }
}