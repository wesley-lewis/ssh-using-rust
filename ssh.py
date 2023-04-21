import paramiko

hostname = "localhost"
username = "wesleylewis"
password = "W1e2s3l4e5y6"

client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

client.connect(hostname, username=username, password=password)

stdin, stdout, stderr = client.exec_command("ls -l")

print(stdout.read().decode())

client.close()
