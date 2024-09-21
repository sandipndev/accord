How do we calculate the Pitch?
Pitch = 2 ^ (n/12) where n is the number of semitones to tweak.
For example, if we want to do +1 ST: then Pitch = 2 ^ (1/12)
Another example, to do +1 ST: then Pitch = 2 ^ (-1/12)

TL;DR:
```bash
yt-dlp -x --audio-format mp3 -o "%(title)s.%(ext)s" <YOUTUBE_LINK>
ffmpeg -i <FILENAME> -filter:a "rubberband=pitch=<PITCH>" output.mp3
```
