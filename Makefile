define timeline_liked_mv
create materialized view uni_twitter.timeline_liked as \
	select tweet_id, username, author, author, text, liked, bookmarked, retweeted, created_at \
	from timeline \
	where tweet_id is not null \
		and username is not null \
		and created_at is not null \
		and liked is not null \
	primary key ((username, liked), created_at, tweet_id) \
	WITH CLUSTERING ORDER BY (created_at DESC);
endef

define first_tweets_mv
CREATE MATERIALIZED VIEW uni_twitter.first_timeline_tweets AS
    SELECT username, created_at, tweet_id, author, bookmarked, liked, retweeted, text
    FROM uni_twitter.timeline
    WHERE username IS NOT null AND created_at IS NOT null AND tweet_id IS NOT null AND liked IS NOT null
    PRIMARY KEY (username, created_at, tweet_id)
    WITH CLUSTERING ORDER BY (created_at ASC);
endef


base_keyspace = "CREATE KEYSPACE IF NOT EXISTS uni_twitter WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': '3'}  AND durable_writes = true AND tablets = {'enabled': false}"
drop_base_keyspace = "DROP KEYSPACE IF EXISTS uni_twitter"
single_dc_node_name = "ws-scylla-1"
multi_dc_node_name = "scylla-dc1-node1"

multi_dc_alter_system_auth = "ALTER KEYSPACE system_auth WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3}"
multi_dc_alter_system_distributed = "ALTER KEYSPACE system_distributed WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3}"
multi_dc_alter_system_traces = "ALTER KEYSPACE system_traces WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3}"

.PHONY: single-dc
single-dc:
	@echo "Setting up single DC using files inside docker folder..."
	@docker compose --file ./docker/network.compose.yml --file ./docker/single-docker-compose.yml up -d
	@echo "Done! Single DC setup is ready!"

.PHONY: multi-dc
multi-dc:
	@echo "Setting up multi DC using files inside docker folder..."
	@docker compose --file ./docker/network.compose.yml --file ./docker/single-docker-compose.yml down --volumes
	@docker compose --file ./docker/network.compose.yml --file ./docker/multi-docker-compose.yml up -d
	@(MAKE) setup-multi-dc
	@docker exec -it $(multi_dc_node_name) cqlsh -e $(base_keyspace)
	@echo "Done! Multi DC setup is ready!"

.PHONY: new-keyspace
new-keyspace:
	@echo "Creating base Keyspace: uni_twitter..."
	@docker exec -it $(single_dc_node_name) cqlsh -e $(drop_base_keyspace)
	@docker exec -it $(single_dc_node_name) cqlsh -e $(base_keyspace)
	@echo "Done! The lab keyspace 'uni_twitter' is ready and set!"

.PHONY: setup-multi-dc
setup-multi-dc:
	@echo "Setting up multi DC..."
	@docker exec -it $(multi_dc_node_name) cqlsh -e $(multi_dc_alter_system_auth)
	@docker exec -it $(multi_dc_node_name) cqlsh -e $(multi_dc_alter_system_distributed)
	@docker exec -it $(multi_dc_node_name) cqlsh -e $(multi_dc_alter_system_traces)
	@echo "Done! Multi DC setup is ready!"

.PHONY: migrate
migrate:
	@echo "Migrating the base schema"
	cd lessons/step1 && migrate --keyspace=uni_twitter --host=localhost:9042
	@echo "Done! Data migration is complete!"

.PHONY: timeline-liked-mv
timeline-liked-mv:
	@echo "Creating materialized view timeline_liked..."
	@docker exec -it $(single_dc_node_name) cqlsh -e "$$timeline_liked_mv"
	@echo "Done! Materialized view timeline_liked is ready!"

.PHONY: first-tweets-mv
first-tweets-mv:
	@echo "Creating materialized view first_timeline_tweets..."
	@docker exec -it $(multi_dc_node_name) cqlsh -e "$$first_tweets_mv"
	@echo "Done! Materialized view first_timeline_tweets is ready!"