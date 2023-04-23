CREATE OR REPLACE FUNCTION public.f_map_list (
	param_id_lang char(5)
) 
RETURNS TABLE (
	id VARCHAR(50),
	label TEXT,
	description TEXT,
	tags TEXT,
	url_wiki VARCHAR(250),
	play_count INT
) 
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY 
    SELECT m.id AS id_map,
        f_get_translations(l.id, m.id) AS map_label,
        f_get_translations(l.id, m.id_description) AS map_description,
        (SELECT string_agg(tm.id_tag, '||') FROM private.tag_map tm WHERE  tm.id_map = m.id)  AS tags,
        m.url_wiki,
        COALESCE(ms.play_count,0) AS play_count
        FROM private.map m
        LEFT JOIN private.lang l
        ON l.id = param_id_lang
        LEFT JOIN private.map_statistic ms
        ON ms.id_map = m.id;
END; $$;