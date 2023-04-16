#!/bin/bash

DB_HOST=
DB_PORT=
DB_NAME=
DB_USER=
DB_PASS=

# SQL query to reset limit_user
SQL="UPDATE api_controller SET limit_user = 0;"

# Execute SQL query
PGPASSWORD="$DB_PASS" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "$SQL"

# 0 0 * * * /path/to/reset_limits.sh