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
declare 
    var_r record;
begin
	for var_r in(
            SELECT mtl.id_tag,
            mtl.label,
            mtl.id_group,
            mtl.group_label
            FROM mv_tag_list mtl
            WHERE mtl.id_lang = param_id_lang
        ) loop
        id_tag := var_r.id_tag;
        label := var_r.label;
        id_group := var_r.id_group;
        group_label := var_r.group_label;
        return next;
	end loop;
end; $$ 