CREATE MATERIALIZED VIEW public.mv_tag_list AS
SELECT l.id as id_lang,
    t.id as id_tag,
    t.id_group,
    f_get_translations(l.id, t.id) as label,
    f_get_translations(l.id, t.id_group) as group_label
FROM private.tag t,
    private.lang l
ORDER BY l.id,
    t.id_group,
    group_label,
    label;