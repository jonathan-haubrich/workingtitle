import glob
import os
from subprocess import Popen, PIPE


PROTO_DIR = os.path.normpath(os.path.join(os.path.dirname(__file__), "..\\proto"))
PROTOC_PATH = os.path.join(PROTO_DIR, r"protoc\bin\protoc.exe")

def compile_proto(proto_file):
    args = [PROTOC_PATH, "--python_out", ".", "-I", PROTO_DIR, proto_file]
    proc = Popen(args, stdout=PIPE, stderr=PIPE, text=True)
    stdout, stderr = proc.communicate()

    if proc.returncode != 0:
        raise Exception(f'stdout: {stdout}\nstderr: {stderr}\n')

def main():
    for proto_file in glob.glob(os.path.join(PROTO_DIR, "*.proto")):
        proto_file_path = os.path.join(PROTO_DIR, proto_file)
        compile_proto(proto_file_path)

if __name__ == '__main__':
    main()