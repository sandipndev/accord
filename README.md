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
mutation create_process(URL: YoutubeLink!)
-----------------------------------
(Sanitize Input)
Spawn: Create Download Job
Returns ProcessID

query get_process(id: ProcessID!)
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

#### Docker Compose to run this directly
```
version: "4"
server-pg:
  image: postgres:16.4
  ports:
  environment:
    - POSTGRES_USER=user
    - POSTGRES_PASSWORD=password
    - POSTGRES_DB=pg
  healthcheck:
    test: ["CMD-SHELL", "pg_isready"]
    interval: 5s
    timeout: 5s
    retries: 5
accorde:
  image: sandipndev/accorde
  ports:
    - "9099:3000"
  depends_on:
    - server-pg
  environment:
    - PG_CON=postgresql://user:password@server-pg:5432/pg
```
