INSERT INTO geo_data_statistic (id_gamemod,id_geo_data,id_lang,id_map,found_count)
SELECT 'AGAINST_CLOCK', id, 'fr-FR', id_map, floor(random() * 100 + 1)::int
FROM map_geo_data
WHERE id_map = 'FRANCE_DEPARTMENTS';

INSERT INTO geo_data_statistic (id_gamemod,id_geo_data,id_lang,id_map,found_count)
SELECT 'AGAINST_CLOCK', id, 'en-US', id_map, floor(random() * 50 + 1)::int
FROM map_geo_data
WHERE id_map = 'FRANCE_DEPARTMENTS';

SELECT * FROM f_average_awareness('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK','de-DE');
SELECT * FROM f_average_awareness('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'fr-FR');
SELECT * FROM f_average_awareness('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK');

DELETE FROM geo_data_statistic;