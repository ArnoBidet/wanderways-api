CREATE TABLE languages (
    id VARCHAR(50),
    PRIMARY KEY(id)
);

CREATE TABLE translations (
    id SERIAL PRIMARY KEY,
    id_lang VARCHAR(50) REFERENCES languages(id),
    translation text,
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

CREATE TABLE data (
    id VARCHAR(50) PRIMARY KEY,
    data_group VARCHAR(50) REFERENCES data(id),
    data_capital VARCHAR(50) REFERENCES data(id),
    numeric_code VARCHAR(50)
);

CREATE TABLE map_data (
    id SERIAL PRIMARY KEY,
    id_data VARCHAR(50) REFERENCES data(id),
    id_map VARCHAR(50) REFERENCES map(id)
);

CREATE TABLE map_statistics (
    id_map VARCHAR(50) REFERENCES map(id),
    id_lang VARCHAR(50) REFERENCES languages(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_map, id_lang)
);

CREATE TABLE gamemod_statistics (
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_lang VARCHAR(50) REFERENCES languages(id),
    play_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_lang)
);

CREATE TABLE success_or_give_up_statistics (
    id_map VARCHAR(50) REFERENCES map(id),
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_lang VARCHAR(50) REFERENCES languages(id),
    play_count INT DEFAULT 0,
    success_count INT DEFAULT 0,
    give_up_count INT DEFAULT 0,
    unfinished_count INT GENERATED ALWAYS AS (play_count - (success_count+give_up_count)) STORED,
    PRIMARY KEY(id_map, id_gamemod, id_lang)
);

CREATE TABLE game_statistics (
    id_gamemod VARCHAR(50) REFERENCES gamemod(id),
    id_map_data INT REFERENCES map_data(id),
    id_lang VARCHAR(50) REFERENCES languages(id),
    id_map VARCHAR(50) REFERENCES map(id),
    found_count INT DEFAULT 0,
    PRIMARY KEY(id_gamemod, id_map_data, id_lang, id_map)
);
