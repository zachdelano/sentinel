PRAGMA foreign_keys = ON;

CREATE TABLE camera (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    address TEXT
);

CREATE TABLE person (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_first TEXT,
    name_last TEXT,
    is_of_interest INTEGER,
    is_associate INTEGER
);

CREATE TABLE photo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image TEXT,
    taken NUMERIC -- datetime
);

CREATE TABLE person_photo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    photo_id INTEGER,
    person_id INTEGER,
    FOREIGN KEY(photo_id) REFERENCES photo(id),
    FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE vehicle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    license TEXT,
    make TEXT,
    model TEXT,
    color TEXT
);

CREATE TABLE encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    camera_id INTEGER,
    time_start NUMERIC,
    time_end NUMERIC,
    FOREIGN KEY(camera_id) REFERENCES camera(id)
);

CREATE TABLE person_encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    encounter_id INTEGER,
    person_id INTEGER,
    FOREIGN KEY(encounter_id) REFERENCES encounter(id),
    FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE vehicle_encounter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    encounter_id INTEGER,
    vehicle_id INTEGER,
    FOREIGN KEY(encounter_id) REFERENCES encounter(id),
    FOREIGN KEY(vehicle_id) REFERENCES vehicle(id)
);
