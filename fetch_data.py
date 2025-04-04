# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "arcgis"
# ]
# ///

import arcgis

from arcgis.gis import GIS
from pathlib import Path
from zipfile import ZipFile

gis = GIS()

public_data_item_id = 'a04933c045714492bda6886f355416f2'

# `ContentManager.get` will return `None` if there is no Item with ID `a04933c045714492bda6886f355416f2`
data_item = gis.content.get(public_data_item_id)

print(f'data_item={data_item}')






