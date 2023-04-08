CREATE MATERIALIZED VIEW mv_tag_list AS
SELECT l.id as id_lang,
    t.id as id_tag,
    t.id_group,
    get_translations(l.id, t.id) as label,
    get_translations(l.id, t.id_group) as group_label
FROM tag t,
    lang l
ORDER BY l.id,
    t.id_group,
    group_label,
    label;