#!/bin/bash

# REST API URL
API_URL="http://localhost:8080/employee"

# File containing employee data
DATA_FILE="employees.txt"

# Function to push data to the REST API
push_data() {
    local employee_data=$1

    # Send POST request with employee data
    response=$(curl -s -o /dev/null -w "%{http_code}" -X POST -H "Content-Type: application/json" -d "$employee_data" "$API_URL")

    if [ "$response" -eq 200 ]; then
        echo "Successfully pushed: $employee_data"
    else
        echo "Failed to push: $employee_data (HTTP Status: $response)"
    fi
}

# Push data in batches (Optional: process in batches if needed)
batch_size=1000
counter=0
batch=""

while IFS= read -r line
do
    batch+="$line"$'\n'
    counter=$((counter+1))

    # If batch size is reached, push data
    if [ "$counter" -ge "$batch_size" ]; then
        push_data "$batch"
        batch=""
        counter=0
    fi
done < "$DATA_FILE"

# Push remaining data if there's any left
if [ "$counter" -gt 0 ]; then
    push_data "$batch"
fi

