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
        LEFT JOIN lang l
        ON l.id like '%'||COALESCE(param_id_lang, '')||'%'
        LEFT JOIN map m
        ON m.id = param_id_map
        LEFT JOIN gamemod g
        ON g.id = param_id_gamemod
        LEFT JOIN geo_data_statistic gds
        ON mgd.id = gds.id_geo_data AND m.id = gds.id_map AND g.id = gds.id_gamemod AND l.id = gds.id_lang
        WHERE mgd.id_map = m.id
        GROUP BY mgd.id_geo_data;
end; $$