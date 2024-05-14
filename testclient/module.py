import argparse
import json
import os

import importlib.util
import shlex
import sys

class Module:
    def __init__(self, base_dir):
        self.base_dir = base_dir

        self.functions = {}
        self.module = None

        self.load_module_json()

    def load_module_json(self):
        with open(os.path.join(self.base_dir, "module.json")) as fp:
            details = json.load(fp)

        file_path = os.path.join(self.base_dir, details["pyfile"])
        module_name = details["name"]
        spec = importlib.util.spec_from_file_location(module_name, file_path)
        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)
        self.module = module

        for function in details["functions"]:
            function["parser"] = self.init_parser(function)
            self.functions[function["name"]] = function

        print(self.functions)

    def init_parser(self, function_info):
        parser = argparse.ArgumentParser(function_info["name"])

        for arg in function_info["args"]:
            parser.add_argument(f'-{arg["short_name"]}',
                f'--{arg["name"]}',
                required=arg["required"],
                type=eval(arg["type"]))
            
        return parser

    def call_command(self, func, arg_str):
        args = shlex.split(arg_str)

        function = self.functions[func]

        parsed = function["parser"].parse_args(args)

        obj = getattr(self.module, function["args_type"])()

        for arg in function["args"]:
            setattr(obj, arg["name"], getattr(parsed, arg["name"]))

        print(obj)

        print(obj.SerializeToString())


