CREATE VIEW v_game_list AS
    SELECT g.id, COALESCE(gs.play_count,0) as play_count
    FROM gamemod g 
    LEFT JOIN gamemod_statistic gs
    ON g.id = gs. id_gamemod;