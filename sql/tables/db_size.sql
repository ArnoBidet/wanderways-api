SELECT
    pg_size_pretty (
        pg_database_size ('wanderways_db')
    );