base_keyspace = "DROP KEYSPACE uni_twitter; CREATE KEYSPACE IF NOT EXISTS uni_twitter WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}  AND durable_writes = true AND tablets = {'enabled': false}"


.PHONY: setup-multi-dc
setup:
	@echo "Setting up multi DC..."
	@docker compose --file ./docker/network.compose.yml --file ./docker/multi-docker-compose.yml up -d
	@echo "Done! Multi DC setup is ready!"

.PHONY: migrate
migrate:
	@echo "Migrating the base schema"
	@docker exec -it scylla-dc1-n1 cqlsh -e $(base_keyspace)
	migrate --keyspace=uni_twitter --host=localhost:9042
	@echo "Done! Data migration is complete!"
