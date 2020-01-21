import sys

"""Create eddystone btle beacon command for provided arg
arg[1] should be ip (and port if needed) for http
returns the command that must be run with superuser priv.
hardcoded for http:// 
"""

ip = sys.argv[1]
command = f"hcitool -i hci0 cmd 0x08 0x0008 {hex(len(ip)+14)} 02 01 06 03 03 aa fe {hex(len(ip)+6)} 16 aa fe 10 00 02"
for char in ip:
    h = hex(ord(char))
    command = f"{command} {h[2:]}"

zeros = 17-len(ip)
for i in range(0, zeros):
    command = f"{command} 00"
print(command)
