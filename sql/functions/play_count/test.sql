INSERT INTO success_or_give_up_statistic
(id_map, id_gamemod, id_lang, play_count, success_count, give_up_count)
VALUES ('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'fr-FR', 15, 8, 6),
('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'en-US', 10, 5, 1),
('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'de-DE', 0, 0, 0);

SELECT * FROM f_play_count('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK');
SELECT * FROM f_play_count('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'fr-FR');
SELECT * FROM f_play_count('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'de-DE');
SELECT * FROM f_play_count('FRANCE_DEPARTMENTS', 'AGAINST_CLOCK', 'es-ES');

DELETE FROM success_or_give_up_statistic;