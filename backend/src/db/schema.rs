table! {
    camera (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
    }
}

table! {
    encounter (id) {
        id -> Nullable<Integer>,
        camera_id -> Integer,
        time_start -> Double,
        time_end -> Double,
    }
}

table! {
    person (id) {
        id -> Nullable<Integer>,
        name_first -> Text,
        name_last -> Text,
        is_of_interest -> Integer,
        is_associate -> Integer,
    }
}

table! {
    person_encounter (id) {
        id -> Nullable<Integer>,
        encounter_id -> Integer,
        person_id -> Integer,
    }
}

table! {
    person_photo (id) {
        id -> Nullable<Integer>,
        photo_id -> Integer,
        person_id -> Integer,
    }
}

table! {
    photo (id) {
        id -> Nullable<Integer>,
        image -> Text,
        taken -> Double,
    }
}

table! {
    vehicle (id) {
        id -> Nullable<Integer>,
        license -> Text,
        make -> Text,
        model -> Text,
        color -> Text,
    }
}

table! {
    vehicle_encounter (id) {
        id -> Nullable<Integer>,
        encounter_id -> Integer,
        vehicle_id -> Integer,
    }
}

joinable!(encounter -> camera (camera_id));
joinable!(person_encounter -> encounter (encounter_id));
joinable!(person_encounter -> person (person_id));
joinable!(person_photo -> person (person_id));
joinable!(person_photo -> photo (photo_id));
joinable!(vehicle_encounter -> encounter (encounter_id));
joinable!(vehicle_encounter -> vehicle (vehicle_id));

allow_tables_to_appear_in_same_query!(
    camera,
    encounter,
    person,
    person_encounter,
    person_photo,
    photo,
    vehicle,
    vehicle_encounter,
);
