import re
ip_addresses_list = []
def extract_ip_addresses(file_path):    
    with open(file_path, "r") as file:
        for line in file:
            ip_addresses = re.findall(r'ip-address: Some\(([^)]+)\)', line)
            ip_addresses_list.extend(ip_addresses)
    
    return ip_addresses_list

file_path = "[path to your logs.txt file]"
result = extract_ip_addresses(file_path)
sub_result = [ip[:-6] for ip in result]
counter = len(sub_result)
print("IP Addresses:")
print(result)
print("The same ip adress of:")
print(sub_result)
print("Has tried to log in ")
print(counter, "times")