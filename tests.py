#!/bin/sh

# Instruqt Track Script in POSIX sh
# Goal: Check Prometheus for specific metrics/gauges. If any gauge value is greater than zero,
# it will execute a fail message and stop the script. If all values are zero, the script will finish successfully.

PROMETHEUS_URL="http://localhost:9090/api/v1/query"

# List of metrics to check (key, query, fail_message)
payloads="
non-prepared-queries|sum(cql:non_system_prepared1m)|Are you sure that all queries are running under Prepared Statement? (Tip: check the 'get_timeline_by_username' function.)
consistency-level|sum(rate(scylla_query_processor_queries{consistency_level='QUORUM'}[30s]))|You're running Multi-DC but are still using CL=Quorum. Wouldn't it be better to only be consistent under the selected datacenter?
unpaged-queries|sum(rate(scylla_cql_unpaged_select_queries[30s])) - sum(rate(scylla_cql_unpaged_select_queries_per_ks{ks='system',cluster='cluster1', dc=~'.*', shard=~'.*'}[30s]))|You have huge timelines being fetched. Please make sure to set a page size for every SELECT statement.
reverse-cql|sum(rate(scylla_cql_reverse_queries[30s]))|One of your queries is using reverse clustering order. Please remove or denormalize it.
allow-filtering|sum(rate(scylla_cql_filtered_read_requests[30s]))|One of your queries is using ALLOW FILTERING and slowing the entire cluster. Please remove or denormalize it.
"

# Function to check each gauge value from Prometheus
check_gauge() {
    key="$1"
    query="$2"
    message="$3"

    # Fetch the data from Prometheus using curl
    response=$(curl -s "${PROMETHEUS_URL}?query=${query}")
    
    # Check for valid response and parse the gauge value using jq
    gauge_value=$(echo "$response" | jq -r '.data.result[0].value[1]' 2>/dev/null)

    # Ensure the gauge value is parsed and valid
    if [ -z "$gauge_value" ]; then
        echo "No result for query '${key}'. Moving on..."
        return 0  # No result, continue to next payload
    fi

    # If gauge value is greater than zero, fail the task with the message
    if [ "$(echo "$gauge_value > 0" | bc)" -eq 1 ]; then
        echo "Gauge '${key}' is not zero: ${gauge_value}. Failing with message."
        fail-message "$message" || exit 1
        exit 1  # Exit the script after failure
    else
        echo "Gauge '${key}' is zero: ${gauge_value}. Moving on..."
    fi
}

# Loop through all payloads and check each gauge
echo "$payloads" | while IFS="|" read -r key query message; do
    check_gauge "$key" "$query" "$message"
done

# If all gauges are clear, finish successfully
echo "All gauges clear. Finishing the lab..."
exit 0
