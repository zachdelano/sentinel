DROP TABLE IF EXISTS camera;
DROP TABLE IF EXISTS person;
DROP TABLE IF EXISTS photo;
DROP TABLE IF EXISTS person_photo;
DROP TABLE IF EXISTS vehicle;
DROP TABLE IF EXISTS encounter;
DROP TABLE IF EXISTS person_encounter;
DROP TABLE IF EXISTS vehicle_encounter;

CREATE TABLE camera (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT NOT NULL
);

CREATE TABLE person (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_first TEXT NOT NULL,
    name_last TEXT NOT NULL,
    is_of_interest INTEGER NOT NULL,
    is_associate INTEGER NOT NULL
);

CREATE TABLE photo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image TEXT NOT NULL,
    taken NUMERIC NOT NULL-- datetime
);

CREATE TABLE person_photo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    photo_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    FOREIGN KEY(photo_id) REFERENCES photo(id),
    FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE vehicle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    license TEXT NOT NULL,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    color TEXT NOT NULL
);

CREATE TABLE encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    camera_id INTEGER NOT NULL,
    time_start NUMERIC NOT NULL,
    time_end NUMERIC NOT NULL,
    FOREIGN KEY(camera_id) REFERENCES camera(id)
);

CREATE TABLE person_encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    encounter_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    FOREIGN KEY(encounter_id) REFERENCES encounter(id),
    FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE vehicle_encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    encounter_id INTEGER NOT NULL,
    vehicle_id INTEGER NOT NULL,
    FOREIGN KEY(encounter_id) REFERENCES encounter(id),
    FOREIGN KEY(vehicle_id) REFERENCES vehicle(id)
);
