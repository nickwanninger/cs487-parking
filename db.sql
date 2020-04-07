/* I know, deleting all the fields is bad but I don't care */
DROP TABLE users, lots, spots, vehicles;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email text,
    pass_hash text,
    acct_type integer /* 0 = Driver, 1 = Owner */
);

CREATE TABLE lots (
    id SERIAL PRIMARY KEY,
    owner_id integer NOT NULL REFERENCES users(id),
    name text NOT NULL,
    address text NOT NULL,
    price integer NOT NULL
);

CREATE TABLE spots (
    id SERIAL PRIMARY KEY,
    lot_id integer NOT NULL REFERENCES lots(id),
    name text NOT NULL
);

CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY,
    driver_id integer REFERENCES users(id),
    license_plate text NOT NULL,
    name text NOT NULL
);

