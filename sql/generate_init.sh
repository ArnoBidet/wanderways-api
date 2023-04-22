#!/bin/bash

cat "" > sql/init_script.sql

# Define the order in which files should be merged
order=(database tables functions/** materialized_view/** views/** inserts)

# Merge files in the given order
for item in "${order[@]}"
do
  # Merge files in subdirectories
  for file_path in sql/$item/up.sql
  do
    echo "Merging $file_path"
    echo "-- $file_path" >> sql/init_script.sql
    cat "$file_path" >> sql/init_script.sql
    echo "" >> sql/init_script.sql
  done
done

echo "All files merged into sql/init_script.sql"