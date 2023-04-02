CREATE OR REPLACE FUNCTION f_average_awareness (
    param_id_map VARCHAR(50),
    param_id_gamemod VARCHAR(50),
	param_id_lang CHAR(5) DEFAULT NULL
) 
RETURNS TABLE (
	id VARCHAR(50),
	found_count INT
) 
language plpgsql
as $$
begin
	RETURN QUERY
        SELECT mgd.id_geo_data as id,
            SUM(COALESCE(gds.found_count::int, 0))::int as found_count
        FROM map_geo_data mgd 
        LEFT JOIN geo_data_statistic gds
        ON mgd.id = gds.id_geo_data
        WHERE mgd.id_map = param_id_map
        AND gds.id_lang like '%'||COALESCE(param_id_lang,'')||'%'
        AND gds.id_gamemod = param_id_gamemod
        GROUP BY mgd.id_geo_data
        ORDER BY mgd.id_geo_data;
end; $$