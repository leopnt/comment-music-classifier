import mutagen # audio metadata reading

import os
import shutil
import re
from collections.abc import Iterator

class TagExtractor:
    ALLOWED_ADSR = ["ATTACK", "DECAY", "SUSTAIN", "RELEASE"]
    ALLOWED_CLARITY = ["DARK", "NEUTRAL", "BRIGHT"]
    ALLOWED_GENRE = ["TECHNO", "TRANCE", "HOUSE", "ELECTRO", "DISCO", "ROCK"]

    class ParsingException(Exception):
        pass

    class UnknownClassifierException(Exception):
        pass

    class FileException(Exception):
        pass

    class TitleException(Exception):
        pass

    def __init__(self, file_path: str):
        self.file_path = file_path
        self.audio_file = mutagen.File(file_path)
        if not self.audio_file:
            raise TagExtractor.FileException(file_path)

    def title(self):
        tags = self.audio_file.tags

        # MP4
        if isinstance(tags, mutagen.mp4.MP4Tags):
            # Check mp4 tag has a title
            if '\xa9nam' in tags.keys():
                return tags['\xa9nam'][0]

        if 'TIT2' not in self.audio_file.tags.keys():
            raise TagExtractor.TitleException(
                    "Title not found for {}".format(self.file_path))

        return self.audio_file.tags['TIT2']

    def artist(self):
        tags = self.audio_file.tags

        # MP4
        if isinstance(tags, mutagen.mp4.MP4Tags):
            # Check mp4 tag has a title
            if '\xa9ART' in tags.keys():
                return tags['\xa9ART'][0]

        return self.audio_file.tags['TPE1']
        
    def comment(self):
        tags = self.audio_file.tags

        comment = ''

        # MP4
        if isinstance(tags, mutagen.mp4.MP4Tags):
            # Check mp4 tag has a comment
            if '\xa9cmt' in tags.keys():
                comment = tags['\xa9cmt'][0]

        # MP3 and AIFF
        else:
            comments = tags.getall('COMM')
            for comm in comments:
                comm_text = comm.text[0]
                if self.assert_classifiers(comm_text.upper(), raise_exception=False):
                    comment = comm_text

        self.assert_classifiers(comment.upper())

        return comment.upper()

    def assert_classifiers(self, comment: str, raise_exception: bool = True) -> bool:
        if not re.match(r'[a-zA-Z]+,[a-zA-Z]+,[a-zA-Z]+', comment):
            if raise_exception:
                raise TagExtractor.ParsingException(
                    "Incorrect comment '{}' for file: {}".format(comment, self.file_path))
            return False

        classifiers = comment.split(',')

        adsr = classifiers[0] in TagExtractor.ALLOWED_ADSR
        clarity = classifiers[1] in TagExtractor.ALLOWED_CLARITY
        genre = classifiers[2] in TagExtractor.ALLOWED_GENRE

        if not (adsr and clarity and genre):
            if raise_exception:
                raise TagExtractor.UnknownClassifierException(
                    "'Unknown classifier in {}' for file: {}".format(comment, self.file_path))
            return False
        
        return True

class AudioFile:
    def __init__(self, path: str):
        self.path = path
        self.tag_extractor = TagExtractor(self.path)
        self.comment = self.tag_extractor.comment()
        self.artist = self.tag_extractor.artist()
        try:
            self.title = self.tag_extractor.title()
        except TagExtractor.TitleException:
            base = os.path.basename(path)
            title, _ = os.path.splitext(base)
            self.title = title
    
    def get_classifiers(self, comment: str) -> list[str]:
        return self.comment.split(',')

    def target_dir(self) -> str:
        return "/".join(self.get_classifiers(self.comment))

def get_files(dir_path: str) -> Iterator[AudioFile]:
    # iterate directory
    for file_name in os.listdir(dir_path):
        file_path = os.path.join(dir_path, file_name)

        is_file = os.path.isfile(file_path)
        if is_file:
            try:
                yield AudioFile(file_path)

            except TagExtractor.FileException as e:
                print(e)
            except TagExtractor.ParsingException as e:
                print(e)
            except TagExtractor.UnknownClassifierException as e:
                print(e)

def main():
    dir_path = r'./TCOTC/'
    dir_path_target = r'./TCOTC_SORTED/'

    shutil.rmtree(dir_path_target, ignore_errors=True)

    for file in get_files(dir_path):

        dst_dir = os.path.join(dir_path_target, file.target_dir())
        os.makedirs(dst_dir, exist_ok=True)

        src = file.path

        _, file_extension = os.path.splitext(file.path)
        file_target_path = "{} - {}".format(file.title, file.artist)

        # replace strange characters with '?'
        file_target_path = file_target_path.encode('ascii', 'replace').decode('utf-8')
        # replace not allowed characters
        file_target_path = file_target_path.replace("<", "!")
        file_target_path = file_target_path.replace(">", "!")
        file_target_path = file_target_path.replace(":", "!")
        file_target_path = file_target_path.replace('"', "!")
        file_target_path = file_target_path.replace('/', "!")
        file_target_path = file_target_path.replace('|', "!")
        file_target_path = file_target_path.replace('\\', "!")
        file_target_path = file_target_path.replace('?', "!")
        file_target_path = file_target_path.replace('*', "!")
        # crop file be below maximum allowed length by OS
        file_target_path = file_target_path[:250]
        # add extension
        file_target_path += file_extension

        dst = os.path.join(dst_dir, file_target_path)

        print("copy: ", src, dst)
        shutil.copy(src, dst)

if __name__ == "__main__":
    main()
