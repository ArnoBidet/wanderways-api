CREATE OR REPLACE FUNCTION f_game_data (
	param_id_lang char(5)
) 
RETURNS TABLE (
	id_item VARCHAR(50),
	translated_value TEXT
) 
language plpgsql
as $$
BEGIN
	RETURN QUERY
        SELECT t.id_item, t.translated_value
        FROM translation t
        WHERE t.id_lang = param_id_lang
		AND t.id_item LIKE '%AW-ORA%'
		OR t.id_item LIKE '%AT-VIENNA%'
		;
END; $$;