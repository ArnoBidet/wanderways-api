CREATE MATERIALIZED VIEW public.mv_geo_data AS
SELECT gd.id,
    l.id as id_lang,
    gd.geo_data_capital,
    gd.geo_data_group,
    gd.numeric_code,
    l.id as id_language,
    f_get_translations(l.id, gd.id) as geo_data_label,
    f_get_translations(l.id, gd.geo_data_capital) as geo_data_capital_label,
    f_get_translations(l.id, gd.geo_data_group) as geo_data_group_label
FROM private.geo_data gd,
    private.lang l;