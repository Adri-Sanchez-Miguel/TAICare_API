import json
import csv

# Read the JSON data
with open('data.json', 'r') as json_file:
    data = json.load(json_file)

# Prepare the CSV file
with open('output.csv', 'w', newline='') as csv_file:
    fieldnames = ['device_id', 'nickname', 'current_power', 'local_time']
    writer = csv.DictWriter(csv_file, fieldnames=fieldnames)

    writer.writeheader()
    
    # Loop through each instance by its ID
    for instance_id, instance_data in data.items():
        device_info = instance_data['device_info']
        energy_usage = instance_data['energy_usage']
        
        important_data = {
            'device_id': device_info['device_id'],
            'nickname': device_info['nickname'],
            'current_power': energy_usage['current_power'],
            'local_time': energy_usage['local_time']
        }
        
        writer.writerow(important_data)
