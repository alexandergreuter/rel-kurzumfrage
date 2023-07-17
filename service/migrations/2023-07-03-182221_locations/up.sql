-- Your SQL goes here

CREATE TABLE
    locations (
        id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
        title TEXT NOT NULL,
        prompt TEXT NOT NULL
    );

CREATE TABLE
    votes (
        id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
        user_agent TEXT NOT NULL,
        agrees BOOLEAN NOT NULL,
        comment TEXT,
        location_id uuid REFERENCES locations(id) NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
    );