# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "arcgis"
# ]
# ///

import os
import pathlib
import zipfile


import arcgis

from arcgis.gis import GIS

gis = GIS()

public_data_item_id = '5e7f84d84b4542f09b17c398a90ec5be'

# `ContentManager.get` will return `None` if there is no Item with ID `5e7f84d84b4542f09b17c398a90ec5be`
#data_item = gis.content.get(public_data_item_id)
#print(f'data_item = {data_item}')

#for layer in data_item.layers:
#  print(f'layer = {layer}')

flc = arcgis.features.FeatureLayerCollection('https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/PLALLPLS_polyline/FeatureServer', gis)
print(f'flc = {flc}')

for item in flc.layers[0].query():
  print(f'item = {item}')







