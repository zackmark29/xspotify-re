# xspotify-re

Learning about XSpotify and Spotify in the process.
This is historical and unlikely to ever be touched.

## Notes

- EncryptedSong

- std::string Downloads::GetRawSongData()
    SongID is EncryptedSong.fileID (looks like this is the songs id, like what youd get from copying a link to a song, but in hex - main::to_gid())
    rawHostData is the response of a request to
        http://spclient.wg.spotify.com/storage-resolve/files/audio/interactive_prefetch/{SongID}?product=0
        with the header Authorization: Bearer {access_token}
    
    parsedHost is whatever string is between "https://", plus 8 chars, and "/audio/".
    parsedparam is whatever is between "/audio/", and "=".
    
    the value returned is the value of a request to
        http://{parsedHost}/{parsedParam}
        with no authentication.


- void Downloads::DownloadFileProcess()
    calls decrypt() on GetRawSongData(), gives it Key and IVKEY
    output of decrypt goes to plaintext
    plaintext goes through DeleteOGGHeader
    thats the raw song data, as an .ogg apparently but its absolutely not

- int decrypt
    returns the length of the decrypted data.
    cipertext is the first argument, which is always the result of GetRawSongData()
    plaintext is the last argument, a buffer that the raw song data is written to.

    it calls EVP_CIPHER_CTX_new() - this is aes128 or something (important !!)

    it calls EVP_DecryptInit_ex(ctx, plaintext, &len, ciphertext, ciphertext_len)
    it calls EVP_DecryptFinal_ex(ctx, plaintext + len, &len)

- DeleteOGGHeader
    finds the string (the ogg magic?) ÿÿÿÿOggS, goes 4 chars past that, and returns the rest.


EncryptedSong.parsedHost = (EncryptedSong.rawHostData.substr(
    EncryptedSong.rawHostData.find("https://") + 8)
    ).erase(
        EncryptedSong.rawHostData.substr(
            EncryptedSong.rawHostData.find("https://") + 8
        ).find("/audio/")
    );



    
EncryptedSong.parsedParam = EncryptedSong.rawHostData.substr(EncryptedSong.rawHostData.find("/audio/")).erase(EncryptedSong.rawHostData.substr(EncryptedSong.rawHostData.find("/audio/")).find("="));


==== WGET ====

GET /storage-resolve/files/audio/interactive/e6db8ca5795084258db731f5219ed9303f4b5035?product=0 HTTP/1.1
User-Agent: Wget/1.20.3 (linux-gnu)
Accept: \*/\*
Accept-Encoding: identity
Host: spclient.wg.spotify.com
Connection: Keep-Alive
Authorization: Bearer \<token\>

=== REQWEST ====

{
    method: GET, 
    url: "https://spclient.wg.spotify.com/storage-resolve/files/audio/interactive/e6db8ca5795084258db731f5219ed9303f4b5035?product=0",
    headers: {
        "user-agent": "Wget/1.20.3 (linux-gnu)",
        "accept": "\*/\*",
        "accept-encoding": "identity",
        "connection": "Keep-Alive",
        "host": "spclient.wg.spotify.com",
        "authorization": "Bearer \<token\>"
    }
}