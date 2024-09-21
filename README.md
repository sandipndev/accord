## Accorde
A simple web app that splits a youtube karoke video into multiple semitones.

#### Process

How do we calculate the Pitch?
Pitch = 2 ^ (n/12) where n is the number of semitones to tweak.
For example, if we want to do +1 ST: then Pitch = 2 ^ (1/12)
Another example, to do +1 ST: then Pitch = 2 ^ (-1/12)

TL;DR:
```bash
yt-dlp -x --audio-format mp3 -o "%(title)s.%(ext)s" <YOUTUBE_LINK>
ffmpeg -i <FILENAME> -filter:a "rubberband=pitch=<PITCH>" output.mp3
```

---

Plan:

```
mutation accorde(URL: YoutubeLink!)
-----------------------------------
(Sanitize Input)
Spawn: Create Download Job
Returns ProcessID

query status(id: ProcessID!)
----------------------------
Returns Status of Job

===========================================

Download Job:
yt-dlp -x --audio-format mp3 -o "%(title)s.%(ext)s" <YOUTUBE_LINK>
Spawn: Create STs Job
Update State

STs Job:
ffmpeg -i <FILENAME> -filter:a "rubberband=pitch=<PITCH>" output.mp3
Update State
```
