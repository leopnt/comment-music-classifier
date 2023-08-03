# Comment Music Classifier

Classify music in folders using comment tag in audio files metadata.
Allows fast live-searching of music for DJs.

## Why

Music is hard to classify. But it's useful to take the time to do it properly
when mixing music live. Indeed, it makes it easier to stay in a more coherent DJ
set while still having a dense Music library of various genres. Since some gears
can display only 3 tracks at a time on the screen (e.g. CDJ 850), it can be very
long to scroll. Moreover, it sometimes doesn't display the cover, which helps to
remember the track's mood. We don't have that time when doing live performance
as we have to deal with many other time-consuming challenges such as identifying
the crowd's mood, beatmatching, making a soft selection for the next DJ and so
on.

You can tag your tracks in well known DJ pieces of software, but it's
proprietary and not garantied to work on every gear. Plus if you want to move to
another software in the future, you are stuck on the software because of the
time you spent on those tags. Yet since we use the comment section of ID3 tags
here, `comment-music-classifier` is compatible with any DJ software by sorting
tracks by `comment`.

## Solution

Tag your tracks in the comment section. It will copy it in associated folders
(see instructions for details). The copied track file name is renamed with the
tag at the beggining. So similar tracks are closed to each other when scrolling
(thanks to sorting by file name).

## Drawbacks

- If you put 3 tags per track, it will take 3-times more disk space. (as tracks
are copied in corresponding folders)
- Only works with `wav`, `mp3` and `aiff`
- Music is not analyzed. So BPM will be approximated on DJ gear.

## Instructions

- Add custom tags to the comment section of ID3 tag in your songs.
E.g: setting the comment to `2,b;a` for `MyMusic/my_track.mp3` will copy this
music to
  - `MyTargetFolder/a/2ab My Track Title - My Track Artist.mp3`
  - `MyTargetFolder/b/2ab My Track Title - My Track Artist.mp3`

- Install `cargo`

- Run

```shell
cargo run -- -s path/to/folder/of/tracks -t path/to/target/folder
```

- On the DJ gear, browse USB folder where you copied the tracks
