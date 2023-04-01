EXPLAIN (ANALYZE ,BUFFERS )
SELECT * FROM mv_map_geo_data;

EXPLAIN (ANALYZE ,BUFFERS )
SELECT gd.id,
    gd.geo_data_capital,
    gd.geo_data_group,
    gd.numeric_code,
    l.id as id_language,
    get_translations(l.id, gd.id) as geo_data_label,
    get_translations(l.id, gd.geo_data_capital) as geo_data_capital_label,
    get_translations(l.id, gd.geo_data_group) as geo_data_group_label
FROM geo_data gd,
    lang l