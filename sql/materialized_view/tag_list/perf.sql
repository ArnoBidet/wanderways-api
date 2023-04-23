EXPLAIN (ANALYZE ,BUFFERS )
SELECT * FROM mv_tag_list;

EXPLAIN (ANALYZE ,BUFFERS )
SELECT l.id as id_lang,
    t.id as id_tag,
    t.id_group,
    f_get_translations(l.id, t.id) as label,
    f_get_translations(l.id, t.id_group) as group_label
FROM tag t,
    lang l
ORDER BY l.id,
    t.id_group,
    group_label,
    label;