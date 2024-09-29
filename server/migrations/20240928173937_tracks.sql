CREATE TABLE tracks (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    youtube_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TYPE semitone_status AS ENUM (
  'PENDING',
  'PROCESSING',
  'DONE'
);

CREATE TABLE semitones (
    id UUID PRIMARY KEY,
    track_id UUID NOT NULL REFERENCES tracks(id),
    shift INT NOT NULL,
    status semitone_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
