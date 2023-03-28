CREATE OR REPLACE FUNCTION map_list (
	id_lang char(5)
) 
RETURNS TABLE (
	id_map VARCHAR(50),
	map_label TEXT,
	map_description TEXT,
	tags TEXT,
	url_wiki VARCHAR(250),
	play_count INT
) 
language plpgsql
as $$
declare 
    var_r record;
begin
	for var_r in(
            SELECT m.id as id_map,
            get_translations(l.id, m.id) as map_label,
            get_translations(l.id, m.id_description) as map_description,
            (SELECT string_agg(tm.id_tag, '||') FROM tag_map tm WHERE  tm.id_map = m.id)  as tags,
            m.url_wiki,
            COALESCE(ms.play_count,0) as play_count
            FROM map m
            LEFT JOIN lang l
            ON l.id = id_lang
            LEFT JOIN map_statistic ms
            ON ms.id_map = m.id
        ) loop
        id_map := var_r.id_map;
        map_label := var_r.map_label;
        map_description := var_r.map_description;
        tags := var_r.tags;
        url_wiki := var_r.url_wiki;
        play_count := var_r.play_count;
           return next;
	end loop;
end; $$ 