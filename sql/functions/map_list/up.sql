CREATE OR REPLACE FUNCTION f_map_list (
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
language plpgsql
as $$
declare 
    var_r record;
begin
	for var_r in(
            SELECT m.id,
            get_translations(l.id, m.id) as label,
            get_translations(l.id, m.id_description) as description,
            (SELECT string_agg(tm.id_tag, '||') FROM tag_map tm WHERE  tm.id_map = m.id)  as tags,
            m.url_wiki,
            COALESCE(ms.play_count,0) as play_count
            FROM map m
            LEFT JOIN lang l
            ON l.id = param_id_lang
            LEFT JOIN map_statistic ms
            ON ms.id_map = m.id
        ) loop
        id := var_r.id;
        label := var_r.label;
        description := var_r.description;
        tags := var_r.tags;
        url_wiki := var_r.url_wiki;
        play_count := var_r.play_count;
           return next;
	end loop;
end; $$ 