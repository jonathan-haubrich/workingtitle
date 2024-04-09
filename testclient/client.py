import dispatch_pb2
import os_module_pb2

import os
import json
from socket import *

MODULE_FILE_PATH = os.path.normpath(os.path.join(os.path.dirname(__file__), r"..\module.json"))

def main():
    with open(MODULE_FILE_PATH) as fp:
        module_info = json.load(fp)

    dirlist = os_module_pb2.DirectoryListing()
    dirlist.path = r"C:\Windows\System32\*"
    dirlist.recursive = True

    payload = dirlist.SerializeToString()

    dispatch = dispatch_pb2.DispatchMessage()
    dispatch.module_id = module_info['id']
    dispatch.function_id = module_info['functions'][0]['id']
    dispatch.payload = payload

    message = dispatch.SerializeToString()

    print(len(message), message)

    with open("dispatch.bin", "wb") as fp:
        fp.write(message)

    s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP)
    s.connect(('127.0.0.1',4444))
    print("Connected")
    s.send(message)
    s.shutdown(SHUT_WR)

    buf = b''
    chunk_size = 1024 * 16
    chunk = s.recv(chunk_size)
    while len(chunk) == chunk_size:
        buf += chunk
        chunk = s.recv(chunk_size)

    buf += chunk

    print(f"Received {len(buf)} bytes")
    print(buf.decode())
    #response = dispatch_pb2.DispatchMessage()
    #response.ParseFromString(buf)

    s.close()

if __name__ == '__main__':
    main()