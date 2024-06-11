import socket

UDP_IP = "127.0.0.1"
UDP_PORT = 4444

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind((UDP_IP, UDP_PORT))

print(f"Listening on {UDP_IP}:{UDP_PORT}")

while True:
    data, addr = sock.recvfrom(1024)  # Taille du buffer
    print("Message reçu:", data.decode(), "de", addr)