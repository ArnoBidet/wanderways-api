CREATE OR REPLACE FUNCTION public.f_play_count (
    param_id_map VARCHAR(50),
    param_id_gamemod VARCHAR(50),
	param_id_lang CHAR(5) DEFAULT NULL
) 
RETURNS TABLE (
	play_count INT
) 
language plpgsql
as $$
begin
	RETURN QUERY
        SELECT sum(COALESCE(sogus.play_count,0)::INT)::INT as play_count
        FROM private.gamemod g
        LEFT JOIN private.map m
        ON m.id = param_id_map
        LEFT JOIN private.lang l
        ON l.id like '%'||COALESCE(param_id_lang, '')||'%'
        LEFT JOIN private.success_or_give_up_statistic sogus
        ON sogus.id_gamemod = g.id AND sogus.id_map = m.id AND sogus.id_lang = l.id
        WHERE g.id = param_id_gamemod
        GROUP BY g.id, m.id;
end; $$;