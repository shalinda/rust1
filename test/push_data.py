import requests
import json
import time

# REST API URL
API_URL = "http://localhost:8080/employee"

# File containing employee data
DATA_FILE = "employees.txt"

# Function to push a single employee to the REST API
def push_data(employee):
    try:
        response = requests.post(API_URL, json=employee)
        if response.status_code == 200:
            print(f"Successfully pushed: {employee}")
        else:
            print(f"Failed to push: {employee} (HTTP Status: {response.status_code})")
    except requests.exceptions.RequestException as e:
        print(f"Failed to push: {employee} due to error: {e}")

# Main function to read data and push one by one
def main():
    with open(DATA_FILE, "r") as file:
        for line in file:
            employee_data = json.loads(line.strip())
            push_data(employee_data)

            # Sleep for a while to prevent overloading the server
            # time.sleep(0.1)  # Adjust the sleep duration as needed

if __name__ == "__main__":
    main()

