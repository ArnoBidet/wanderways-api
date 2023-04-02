CREATE TABLE lang (
    id CHAR(5),
    PRIMARY KEY(id)
);

CREATE TABLE translation (
    id SERIAL PRIMARY KEY,
    id_lang VARCHAR(50) REFERENCES lang(id),
    translated_value text,
    id_item VARCHAR(50)
);

CREATE TABLE map (
    id VARCHAR(50) PRIMARY KEY,
    id_description VARCHAR(50),
    url_wiki  VARCHAR(250)
);

CREATE TABLE tag_group (
    id VARCHAR(50) PRIMARY KEY
);

CREATE TABLE tag (
    id VARCHAR(50) PRIMARY KEY,
    id_group VARCHAR(50) NOT NULL,
    FOREIGN KEY(id_group) REFERENCES tag_group(id)
);

CREATE TABLE tag_map (
    id_map VARCHAR(50) REFERENCES map(id),
    id_tag VARCHAR(50) REFERENCES tag(id),
    PRIMARY KEY(id_map, id_tag)
);

CREATE TABLE gamemod (
    id VARCHAR(50) PRIMARY KEY
);

CREATE TABLE gamemod_map (
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_map VARCHAR(50) REFERENCES map(id),
    PRIMARY KEY(id_gamemod, id_map)
);

CREATE TABLE geo_data (
    id VARCHAR(50) PRIMARY KEY,
    geo_data_group VARCHAR(50) REFERENCES geo_data(id),
    geo_data_capital VARCHAR(50) REFERENCES geo_data(id),
    numeric_code VARCHAR(50)
);

CREATE TABLE map_geo_data (
    id SERIAL PRIMARY KEY,
    id_geo_data VARCHAR(50) REFERENCES geo_data(id),
    id_map VARCHAR(50) REFERENCES map(id)
);

CREATE TABLE map_statistic (
    id_map VARCHAR(50) REFERENCES map(id),
    id_lang VARCHAR(50) REFERENCES lang(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_map, id_lang)
);

CREATE TABLE gamemod_statistic (
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_lang VARCHAR(50) REFERENCES lang(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_lang)
);

CREATE TABLE success_or_give_up_statistic (
    id_map VARCHAR(50) REFERENCES map(id),
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_lang VARCHAR(50) REFERENCES lang(id),
    play_count INT DEFAULT 0,
    success_count INT DEFAULT 0,
    give_up_count INT DEFAULT 0,
    unfinished_count INT GENERATED ALWAYS AS (play_count - (success_count+give_up_count)) STORED,
    PRIMARY KEY(id_map, id_gamemod, id_lang)
);

CREATE TABLE geo_data_statistic (
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_geo_data INT REFERENCES map_geo_data(id),
    id_lang VARCHAR(50) REFERENCES lang(id),
    id_map VARCHAR(50) REFERENCES map(id),
    found_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_geo_data, id_lang, id_map)
);
