CREATE OR REPLACE FUNCTION f_map_geo_data (
	param_id_lang CHAR(5),
    param_id_map VARCHAR(50)
) 
RETURNS TABLE (
	id VARCHAR(50),
	data_label TEXT,
	id_group VARCHAR(50),
    group_label TEXT,
	id_capital VARCHAR(50),
    capital_label TEXT,
	numeric_code VARCHAR(50)
) 
language plpgsql
as $$
declare 
    var_r record;
begin
	for var_r in(
            SELECT mv_gd.id,
                mv_gd.geo_data_label,
                mv_gd.geo_data_group,
                mv_gd.geo_data_group_label,
                mv_gd.geo_data_capital,
                mv_gd.geo_data_capital_label,
                mv_gd.numeric_code
            FROM map_geo_data mgd, mv_geo_data mv_gd
            WHERE mv_gd.id_lang = param_id_lang
            AND mgd.id_map = param_id_map
            AND mv_gd.id = mgd.id_geo_data
        ) loop
        id := var_r.id;
        data_label := var_r.geo_data_label;
        id_group := var_r.geo_data_group;
        group_label := var_r.geo_data_group_label;
        id_capital := var_r.geo_data_capital;
        capital_label := var_r.geo_data_capital_label;
        numeric_code := var_r.numeric_code;
        return next;
	end loop;
end; $$ 