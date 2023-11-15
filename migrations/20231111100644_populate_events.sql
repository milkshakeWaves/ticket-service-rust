-- Add migration script here
INSERT INTO event (code, description, place, available_seats, price, date)
VALUES ('TECH', 'Tech Conference', 'Convention Center', 200, 150.00, '2023-11-09 10:00:00');

INSERT INTO event (code, description, place, available_seats, price, date)
VALUES ('MUSIC', 'Music Festival', 'City Park', 500, 50.00, '2023-12-15 18:30:00');

INSERT INTO event (code, description, place, available_seats, price, date)
VALUES ('GALA', 'Gala Dinner', 'Luxury Hotel', 100, 250.00, '2024-02-28 19:00:00');

INSERT INTO event (code, description, place, available_seats, price, date)
VALUES ('SPTOUR', 'Sports Tournament', 'Stadium', 10000, 30.00, '2024-05-20 14:00:00');
