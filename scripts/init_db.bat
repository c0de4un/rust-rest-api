set DB_USER=postgres
set PGPASSWORD=password
set DB_NAME=newsletter
set DB_PORT=5432
set DATABASE_URL=postgres://%DB_USER%:%DB_PASSWORD%@localhost:%DB_PORT%/%DB_NAME%
sqlx database create
sqlx migrate run