CREATE OR REPLACE FUNCTION public.f_get_translations(in param_lang_id CHAR(5), in param_item_id VARCHAR(50), out result text)
    AS $$ SELECT string_agg(t.translated_value, '||')
        FROM private.translation t
        WHERE t.id_lang = param_lang_id
        AND t.id_item = param_item_id $$
    LANGUAGE SQL;