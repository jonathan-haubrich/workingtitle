import dispatch_pb2
import os_module_pb2

import os
import json

MODULE_FILE_PATH = os.path.normpath(os.path.join(os.path.dirname(__file__), r"..\module.json"))

def main():
    with open(MODULE_FILE_PATH) as fp:
        module_info = json.load(fp)

    dirlist = os_module_pb2.DirectoryListing()
    dirlist.path = r"C:\Users\dweller\testpath"
    dirlist.recursive = True

    payload = dirlist.SerializeToString()

    dispatch = dispatch_pb2.DispatchMessage()
    dispatch.module_id = module_info['id']
    dispatch.function_id = module_info['functions'][0]['id']
    dispatch.payload = payload

    message = dispatch.SerializeToString()

    print(len(message), message)


if __name__ == '__main__':
    main()