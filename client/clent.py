from email.message import Message
import socket

HOST = "127.0.0.1"
PORT = 4485

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    buffer = bytearray()

    data = ("{'name': 'John', 'age': 25}").encode('utf-8')

    # Message Len
    # append 32 bits
    buffer.append(0)
    buffer.append(0)
    buffer.append(0)
    buffer.append(20 + len(data))

    # RequestId
    buffer.append(0)
    buffer.append(0)
    buffer.append(0)
    buffer.append(1)

    # ResponseTo
    buffer.append(0)
    buffer.append(0)
    buffer.append(0)
    buffer.append(1)

    # OpCode
    buffer.append(0)
    buffer.append(0)
    buffer.append(0x07)
    buffer.append(0xd1)

    # Flags
    buffer.append(0)
    buffer.append(0)
    buffer.append(0)
    buffer.append(0)

    # # Bson Value
    # buffer.append(0xaa)
    # buffer.append(0xaa)
    # buffer.append(0xaa)
    # buffer.append(0xaa)
    # buffer.append(0xaa)
    
    print(buffer)
    # size of encoded string

    s.sendall(buffer + data)
    data = s.recv(1024)

print(f"Received {data!r}")