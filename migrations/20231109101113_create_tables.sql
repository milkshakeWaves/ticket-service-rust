-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS event (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    description VARCHAR(1024) NOT NULL,
    place VARCHAR(255) NOT NULL,
    available_seats INTEGER NOT NULL,
    price INTEGER NOT NULL,
    date TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS ticket (
    user_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, event_id),

    CONSTRAINT user_id FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT event_id FOREIGN KEY(event_id) REFERENCES event(id) ON DELETE CASCADE
);

