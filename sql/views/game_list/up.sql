CREATE VIEW public.v_game_list AS
    SELECT g.id, COALESCE(gs.play_count,0) as play_count
    FROM private.gamemod g 
    LEFT JOIN private.gamemod_statistic gs
    ON g.id = gs.id_gamemod;
