-- Add migration script here
CREATE TABLE buyer (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at timestamp NOT NULL
);

CREATE TABLE event (
    id SERIAL PRIMARY KEY,
    description VARCHAR(1024) NOT NULL,
    place VARCHAR(255) NOT NULL,
    available_seats INTEGER NOT NULL,
    price MONEY NOT NULL,
    date TIMESTAMP NOT NULL
);

CREATE TABLE ticket (
    buyer_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (buyer_id, event_id),

    CONSTRAINT buyer_id FOREIGN KEY(buyer_id) REFERENCES buyer(id) ON DELETE CASCADE,
    CONSTRAINT event_id FOREIGN KEY(event_id) REFERENCES event(id) ON DELETE CASCADE
);

