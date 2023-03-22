// @generated automatically by Diesel CLI.

diesel::table! {
    game_statistic (id_gamemod, id_map_geo_data, id_lang, id_map) {
        id_gamemod -> Varchar,
        id_map_geo_data -> Int4,
        id_lang -> Varchar,
        id_map -> Varchar,
        found_count -> Nullable<Int4>,
    }
}

diesel::table! {
    gamemod (id) {
        id -> Varchar,
    }
}

diesel::table! {
    gamemod_map (id_gamemod, id_map) {
        id_gamemod -> Varchar,
        id_map -> Varchar,
    }
}

diesel::table! {
    gamemod_statistic (id_gamemod, id_lang) {
        id_gamemod -> Varchar,
        id_lang -> Varchar,
        play_count -> Nullable<Int4>,
    }
}

diesel::table! {
    geo_data (id) {
        id -> Varchar,
        geo_data_group -> Nullable<Varchar>,
        geo_data_capital -> Nullable<Varchar>,
        numeric_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    lang (id) {
        id -> Bpchar,
    }
}

diesel::table! {
    map (id) {
        id -> Varchar,
        id_description -> Nullable<Varchar>,
        url_wiki -> Nullable<Varchar>,
    }
}

diesel::table! {
    map_geo_data (id) {
        id -> Int4,
        id_geo_data -> Nullable<Varchar>,
        id_map -> Nullable<Varchar>,
    }
}

diesel::table! {
    map_statistic (id_map, id_lang) {
        id_map -> Varchar,
        id_lang -> Varchar,
        play_count -> Nullable<Int4>,
    }
}

diesel::table! {
    success_or_give_up_statistic (id_map, id_gamemod, id_lang) {
        id_map -> Varchar,
        id_gamemod -> Varchar,
        id_lang -> Varchar,
        play_count -> Nullable<Int4>,
        success_count -> Nullable<Int4>,
        give_up_count -> Nullable<Int4>,
        unfinished_count -> Nullable<Int4>,
    }
}

diesel::table! {
    tag (id) {
        id -> Varchar,
        id_group -> Varchar,
    }
}

diesel::table! {
    tag_group (id) {
        id -> Varchar,
    }
}

diesel::table! {
    tag_map (id_map, id_tag) {
        id_map -> Varchar,
        id_tag -> Varchar,
    }
}

diesel::table! {
    translation (id) {
        id -> Int4,
        id_lang -> Nullable<Varchar>,
        translated_value -> Nullable<Text>,
        id_item -> Nullable<Varchar>,
    }
}

diesel::joinable!(game_statistic -> gamemod (id_gamemod));
diesel::joinable!(game_statistic -> lang (id_lang));
diesel::joinable!(game_statistic -> map (id_map));
diesel::joinable!(game_statistic -> map_geo_data (id_map_geo_data));
diesel::joinable!(gamemod_map -> gamemod (id_gamemod));
diesel::joinable!(gamemod_map -> map (id_map));
diesel::joinable!(gamemod_statistic -> gamemod (id_gamemod));
diesel::joinable!(gamemod_statistic -> lang (id_lang));
diesel::joinable!(map_geo_data -> geo_data (id_geo_data));
diesel::joinable!(map_geo_data -> map (id_map));
diesel::joinable!(map_statistic -> lang (id_lang));
diesel::joinable!(map_statistic -> map (id_map));
diesel::joinable!(success_or_give_up_statistic -> gamemod (id_gamemod));
diesel::joinable!(success_or_give_up_statistic -> lang (id_lang));
diesel::joinable!(success_or_give_up_statistic -> map (id_map));
diesel::joinable!(tag -> tag_group (id_group));
diesel::joinable!(tag_map -> map (id_map));
diesel::joinable!(tag_map -> tag (id_tag));
diesel::joinable!(translation -> lang (id_lang));

diesel::allow_tables_to_appear_in_same_query!(
    game_statistic,
    gamemod,
    gamemod_map,
    gamemod_statistic,
    geo_data,
    lang,
    map,
    map_geo_data,
    map_statistic,
    success_or_give_up_statistic,
    tag,
    tag_group,
    tag_map,
    translation,
);
