CREATE TABLE private.lang (
    id CHAR(5),
    PRIMARY KEY(id)
);

CREATE TABLE private.translation (
    id SERIAL PRIMARY KEY,
    id_lang VARCHAR(50) REFERENCES private.lang(id),
    translated_value text,
    id_item VARCHAR(50)
);

CREATE TABLE private.map (
    id VARCHAR(50) PRIMARY KEY,
    id_description VARCHAR(50),
    url_wiki  VARCHAR(250)
);

CREATE TABLE private.tag_group (
    id VARCHAR(50) PRIMARY KEY
);

CREATE TABLE private.tag (
    id VARCHAR(50) PRIMARY KEY,
    id_group VARCHAR(50) NOT NULL,
    FOREIGN KEY(id_group) REFERENCES private.tag_group(id)
);

CREATE TABLE private.tag_map (
    id_map VARCHAR(50) REFERENCES private.map(id),
    id_tag VARCHAR(50) REFERENCES private.tag(id),
    PRIMARY KEY(id_map, id_tag)
);

CREATE TABLE private.gamemod (
    id VARCHAR(50) PRIMARY KEY
);

CREATE TABLE private.gamemod_map (
    id_gamemod VARCHAR(50) REFERENCES private.gamemod(id),
    id_map VARCHAR(50) REFERENCES private.map(id),
    PRIMARY KEY(id_gamemod, id_map)
);

CREATE TABLE private.geo_data (
    id VARCHAR(50) PRIMARY KEY,
    geo_data_group VARCHAR(50) REFERENCES private.geo_data(id),
    geo_data_capital VARCHAR(50) REFERENCES private.geo_data(id),
    numeric_code VARCHAR(50)
);

CREATE TABLE private.map_geo_data (
    id SERIAL PRIMARY KEY,
    id_geo_data VARCHAR(50) REFERENCES private.geo_data(id),
    id_map VARCHAR(50) REFERENCES private.map(id)
);

CREATE TABLE private.map_statistic (
    id_map VARCHAR(50) REFERENCES private.map(id),
    id_lang VARCHAR(50) REFERENCES private.lang(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_map, id_lang)
);

CREATE TABLE private.gamemod_statistic (
    id_gamemod VARCHAR(50) REFERENCES private.gamemod(id),
    id_lang VARCHAR(50) REFERENCES private.lang(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_lang)
);

CREATE TABLE private.success_or_give_up_statistic (
    id_map VARCHAR(50) REFERENCES private.map(id),
    id_gamemod VARCHAR(50) REFERENCES private.gamemod(id),
    id_lang VARCHAR(50) REFERENCES private.lang(id),
    play_count INT DEFAULT 0,
    success_count INT DEFAULT 0,
    give_up_count INT DEFAULT 0,
    unfinished_count INT GENERATED ALWAYS AS (play_count - (success_count+give_up_count)) STORED,
    PRIMARY KEY(id_map, id_gamemod, id_lang)
);

CREATE TABLE private.geo_data_statistic (
    id_gamemod VARCHAR(50) REFERENCES private.gamemod(id),
    id_geo_data INT REFERENCES private.map_geo_data(id),
    id_lang VARCHAR(50) REFERENCES private.lang(id),
    id_map VARCHAR(50) REFERENCES private.map(id),
    found_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_geo_data, id_lang, id_map)
);
