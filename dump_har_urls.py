# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "haralyzer"
# ]
# ///

import os
import sys
import pathlib
import traceback
import urllib.request
import json

import haralyzer

if len(sys.argv) < 3:
  print(f'Usage: uv run {sys.argv[1]} ./path/to/file.har filter_text')
  sys.exit(1)

har_file = sys.argv[1]
filter_text = sys.argv[2]


with open(har_file, 'rb') as fd:
  har_parser = HarParser(json.loads(fd.read()))




