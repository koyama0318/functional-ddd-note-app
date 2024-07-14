DB_NAME := database.sqlite3
INIT_SCRIPT := migration/init.sql

.PHONY: all init_db clean

all: init_db

init_db:
	@echo "Initializing database..."
	@sqlite3 $(DB_NAME) < $(INIT_SCRIPT)
	@echo "Database initialized."

clean:
	@echo "Cleaning up..."
	@rm -f $(DB_NAME)
	@echo "Clean up complete."