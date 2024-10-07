import json
import random

# Function to generate sample employee data
def generate_employee_data(num_employees):
    employees = []
    for i in range(1, num_employees + 1):
        employee = {
            "id": i,
            "name": f"Employee_{i}",
            "salary": round(random.uniform(30000, 100000), 2)
        }
        employees.append(employee)
    return employees

# Generate 100000 employees and write them to a file
num_employees = 100000
employees = generate_employee_data(num_employees)

with open("employees.txt", "w") as f:
    for employee in employees:
        f.write(json.dumps(employee) + "\n")

print(f"{num_employees} employees written to employees.txt")

