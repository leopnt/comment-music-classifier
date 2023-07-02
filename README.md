# Comment Music Classifier

Classify music in folders using comment tag in audio files metadata.
Allows fast live-searching of music for DJs.

## Instructions

- clone this repository

- make virtual env

```
python3 -m venv env
source env/bin/activate
```

- install requirements

```
pip install -r requirements
```

- put comment in music metadata tags: `<ENERGY>,<TAG1>;[TAG2];[TAG3]` using software of choice.
Possible values (see `TagExtractor` class) are
    - ENERGY: `1, 2, 3`
    - TAGS: e.g. `h, o, t` as per 'house' 'italo' 'techno'
    So for example an audio file with tag comment `2,h;d` will be moved to folders `HOUSE` and `DISCO` and renamed `2dh <title> <artist>.<file type>`. Note that tags are alphabetically re-ordered in filename. It matters to have similar tracks close to each other in tracklist when sorting by filename.

- move music files to a folder named `TCOTC` beside `main.py`. All music should be in `TCOTC` without subfolders.

- run script

```
python main.py > log.txt
```

- Sorted music are in folder `TCOTC_SORTED/`
