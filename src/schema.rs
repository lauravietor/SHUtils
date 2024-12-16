// @generated automatically by Diesel CLI.

diesel::table! {
    hunts (id) {
        id -> Integer,
        target -> Integer,
        previous_encounters -> Integer,
        phase_encounters -> Integer,
        phase_count -> Integer,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        completed -> Bool,
        version -> Nullable<Text>,
        method -> Nullable<Text>,
        place -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    shinies (id) {
        id -> Integer,
        species -> Integer,
        gender -> Nullable<Integer>,
        name -> Nullable<Text>,
        total_encounters -> Nullable<Integer>,
        phase_encounters -> Nullable<Integer>,
        phase_number -> Nullable<Integer>,
        found_time -> Nullable<Timestamp>,
        version -> Nullable<Text>,
        method -> Nullable<Text>,
        place -> Nullable<Text>,
        notes -> Nullable<Text>,
        hunt_id -> Nullable<Integer>,
    }
}

diesel::joinable!(shinies -> hunts (hunt_id));

diesel::allow_tables_to_appear_in_same_query!(
    hunts,
    shinies,
);
