CREATE USER wanderways_api WITH PASSWORD 'pwd';

GRANT USAGE ON SCHEMA public TO wanderways_api;
GRANT USAGE ON SCHEMA private TO wanderways_api;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO wanderways_api;
GRANT SELECT ON ALL TABLES IN SCHEMA private TO wanderways_api;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO wanderways_api;