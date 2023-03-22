use crate::utils::*;

pub fn gen_gamemod_map() -> String {
    let gamemod_map = yaml_reader::yaml_reader::<Vec<schemas::GamemodMap>>("gamemod_map");
    let mut gamemod_map_value_lines: Vec<String> = Vec::new();
    gamemod_map.iter().for_each(|ele| {
        sql::gen_value_line(
            vec![&ele.id_map, &ele.id_gamemod],
            &mut gamemod_map_value_lines,
        );
    });
    let gamemod_map_inserts = sql::gen_insert(
        "gamemod_map",
        "(id_map, id_gamemod)",
        &gamemod_map_value_lines,
    );
    return gamemod_map_inserts;
}
