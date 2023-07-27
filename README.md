# Comment Music Classifier

Classify music in folders using comment tag in audio files metadata.
Allows fast live-searching of music for DJs.

## Instructions

- Add custom tags to the comment section of ID3 tag in your songs.
E.g: setting the comment to `2,b;a` for `MyMusic/my_track.mp3` will copy this music to
    - `MyTargetFolder/a/2ab My Track Title - My Track Artist.mp3`
    - `MyTargetFolder/b/2ab My Track Title - My Track Artist.mp3`

- Install `cargo`

- Run

```shell
cargo run -- -s path/to/folder/of/tracks -t path/to/target/folder
```
