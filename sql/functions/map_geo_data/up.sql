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
begin
	RETURN QUERY
        SELECT mv_gd.id,
            mv_gd.geo_data_label AS data_label,
            mv_gd.geo_data_group as id_group,
            mv_gd.geo_data_group_label as group_label,
            mv_gd.geo_data_capital as id_capital,
            mv_gd.geo_data_capital_label as capital_label,
            mv_gd.numeric_code
        FROM map_geo_data mgd, mv_geo_data mv_gd
        WHERE mv_gd.id_lang = param_id_lang
        AND mgd.id_map = param_id_map
        AND mv_gd.id = mgd.id_geo_data;
end; $$ 