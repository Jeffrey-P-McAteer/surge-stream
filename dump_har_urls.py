# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "haralyzer"
# ]
# ///

# Example use:
#   uv run dump_har_urls.py research-journey/atlas.eia.gov_Archive\ \[25-04-05\ 13-54-44\].har 'FeatureServer/0/query' | sed 's/\/query.*//g' | sort | uniq

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
_ed = os.environ.get('DEBUG', '')
DEBUG = 'y' in _ed or '1' in _ed

with open(har_file, 'rb') as fd:
  har_parser = haralyzer.HarParser(json.loads(fd.read().decode('utf-8')))

for page in har_parser.pages:
  if DEBUG:
    print(f'page = {page}')
  for entry in page:
      if DEBUG:
        print(f'  entry = {entry}')
      if filter_text in entry.url:
        print(f'{entry.url}')










