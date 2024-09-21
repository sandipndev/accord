CREATE TYPE process_status AS ENUM ('PENDING', 'DOWNLOADING', 'CONVERTING', 'DONE');

CREATE TABLE processes (
    id UUID PRIMARY KEY,
    youtube_url TEXT NOT NULL,
    status process_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
