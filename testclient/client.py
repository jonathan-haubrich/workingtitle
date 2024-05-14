import dispatch_pb2
import os_module_pb2
import module

import os
import json
import shlex
from socket import *

import struct

MODULE_FILE_PATH = os.path.normpath(os.path.join(os.path.dirname(__file__), r"..\module.json"))
MODULES_DIR = os.path.normpath(os.path.join(os.path.dirname(__file__), "modules"))

def recv_all(sock, recv_len):
    buf = sock.recv(recv_len)

    while len(buf) < recv_len:
        buf += sock.recv(recv_len - len(buf))

    return buf

def main():
    # with open(MODULE_FILE_PATH) as fp:
    #     module_info = json.load(fp)

    # dirlist = os_module_pb2.DirectoryListingRequest()
    # dirlist.path = r"C:\Users\dweller\.cargo"
    # dirlist.recursive = True

    # payload = dirlist.SerializeToString()

    # dispatch = dispatch_pb2.DispatchMessage()
    # dispatch.module_id = module_info['id']
    # dispatch.function_id = module_info['functions'][0]['id']
    # dispatch.payload = payload

    # message = dispatch.SerializeToString()

    # print(len(message), message)

    # with open("dispatch.bin", "wb") as fp:
    #     fp.write(message)

    # s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP)
    # s.connect(('127.0.0.1',4444))
    # print("Connected")

    for m in os.listdir("modules"):
        base_dir = os.path.abspath(os.path.join("modules",m))
        print(base_dir)
        loaded = module.Module(base_dir)

    for function in loaded.functions:
        loaded.functions[function]["parser"].print_help()
        loaded.functions[function]["parser"].parse_args(["-p", "C:\\ProgramData", "-r", "False"])
    
    loaded.call_command("dirlist", "-p C:\\\\ProgramData -r False")

    # wire_len_fmt = "!Q"

    # while True:

    #     command = input("<dir> <recurse>: ")
    #     directory, recurse = shlex.split(command)

    #     dir_list_req = os_module_pb2.DirectoryListingRequest()
    #     dir_list_req.path = directory
    #     dir_list_req.recursive = eval(recurse)

    #     message = dispatch_pb2.DispatchMessage()
    #     message.module_id = "758d227f-27e0-4406-b27e-cf9976948109"
    #     message.function_id = "758d227f-27e0-4406-b27e-cf9976948109"
    #     message.payload = dir_list_req.SerializeToString()

    #     serialized = message.SerializeToString()
    #     sent = s.send(struct.pack("!Q", len(serialized)))
    #     print(f"Sending {len(serialized)} ({struct.pack('!Q', len(serialized))})")
    #     sent = s.send(serialized)
    #     print(f"Sent {sent} bytes")

    #     wire_len = struct.calcsize(wire_len_fmt)
    #     print(f"wire_len: {wire_len}")
    #     response_len_bytes = recv_all(s, wire_len)

    #     response_len = struct.unpack(wire_len_fmt, response_len_bytes)[0]
    #     print(f"Receiving {response_len} bytes")

    #     response = recv_all(s, response_len)
    #     #print(response)

    #     dispatch = dispatch_pb2.DispatchResponse()
    #     dispatch.ParseFromString(response)

    #     error, payload = dispatch.error, dispatch.payload

    #     listing = os_module_pb2.DirectoryListingResponse()
    #     listing.ParseFromString(payload)

    #     for directory in listing.listing:
    #         for entry in directory.entries:
    #             print(entry.path)

    # s.close()

if __name__ == '__main__':
    main()