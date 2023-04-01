.DEFAULT_GOAL := help

.PHONY: help
help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

pgsql: ## Start PostgreSQL
	docker run -it --rm --name pgsql -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres:15.2

build: ## Build the project
	cargo build

stop: ## Stop PostgreSQL
	docker stop pgsql
