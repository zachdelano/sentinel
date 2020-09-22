table! {
    camera (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

table! {
    encounter (id) {
        id -> Nullable<Integer>,
        camera_id -> Nullable<Integer>,
        time_start -> Nullable<Double>,
        time_end -> Nullable<Double>,
    }
}

table! {
    person (id) {
        id -> Nullable<Integer>,
        name_first -> Nullable<Text>,
        name_last -> Nullable<Text>,
        is_of_interest -> Nullable<Integer>,
        is_associate -> Nullable<Integer>,
    }
}

table! {
    person_encounter (id) {
        id -> Nullable<Integer>,
        encounter_id -> Nullable<Integer>,
        person_id -> Nullable<Integer>,
    }
}

table! {
    person_photo (id) {
        id -> Nullable<Integer>,
        photo_id -> Nullable<Integer>,
        person_id -> Nullable<Integer>,
    }
}

table! {
    photo (id) {
        id -> Nullable<Integer>,
        image -> Nullable<Text>,
        taken -> Nullable<Double>,
    }
}

table! {
    vehicle (id) {
        id -> Nullable<Integer>,
        license -> Nullable<Text>,
        make -> Nullable<Text>,
        model -> Nullable<Text>,
        color -> Nullable<Text>,
    }
}

table! {
    vehicle_encounter (id) {
        id -> Nullable<Integer>,
        encounter_id -> Nullable<Integer>,
        vehicle_id -> Nullable<Integer>,
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
