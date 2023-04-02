CREATE OR REPLACE FUNCTION f_tag_list (
	param_id_lang char(5)
) 
RETURNS TABLE (
	id_tag VARCHAR(50),
	label TEXT,
	id_group VARCHAR(50),
	group_label TEXT
) 
language plpgsql
as $$
BEGIN
	RETURN QUERY
        SELECT mtl.id_tag,
        mtl.label,
        mtl.id_group,
        mtl.group_label
        FROM mv_tag_list mtl
        WHERE mtl.id_lang = param_id_lang;
END; $$ 