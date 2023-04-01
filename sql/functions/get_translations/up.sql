CREATE OR REPLACE FUNCTION get_translations(in lang_id CHAR(5), in item_id VARCHAR(50), out result text)
    AS $$ SELECT string_agg(t.translated_value, '||')
        FROM translation t
        WHERE t.id_lang = lang_id
        AND t.id_item = item_id $$
    LANGUAGE SQL;