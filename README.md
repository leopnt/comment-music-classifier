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

- put comment in music metadata tags: `<ADSR>,<CLARITY>,<GENRE>`.
Possible values (see `TagExtractor` class) are
    - ADSR: `attack, decay, release, sustain`
    - CLARITY: `dark, neutral, bright`
    - GENRE: `rock, disco, house, trance, techno, electro`
    So for example an audio file with tag comment `sustain,neutral,trance` will be moved to folder `SUSTAIN/NEUTRAL/TRANCE`

- move music files to a folder named `TCOTC` beside `main.py`. All music should be in `TCOTC` without subfolders.

- run script

```
python main.py
```

- Sorted music are in folder `TCOTC_SORTED/`
