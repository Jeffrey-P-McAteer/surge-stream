# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "arcgis"
# ]
# ///

import os
import pathlib
import zipfile
import pickle

import arcgis

from arcgis.gis import GIS

data_pickle_path = os.path.join(os.path.dirname(__file__), 'data', 'raw-layer-data.pickle')
downloaded_data = dict()
if os.path.exists(data_pickle_path):
  with open(data_pickle_path, 'rb') as fd:
    downloaded_data = pickle.load(fd)

try:
  gis = GIS()

  layer_urls_to_download = [
    ('lines? idk', 'https://services.arcgis.com/jDGuO8tYggdCCnUJ/ArcGIS/rest/services/PLALLPLS_polyline/FeatureServer/0'),
  ]

  for layer_name, layer_url in layer_urls_to_download:
    fl = arcgis.features.FeatureLayer(layer_url, gis)
    print(f'[ {layer_name} ] fl = {fl}')

    num_items_in_layer = fl.query(return_count_only=True)

    if layer_name in downloaded_data and len(downloaded_data[layer_name]) >= num_items_in_layer:
      continue # We've already fetched this, keep going

    # Data is currently Layers => List of Features in Esri JSON format.
    downloaded_data[layer_name] = list()
    for item in fl.query(out_sr=4326):
     downloaded_data[layer_name].append( item )

except:
  traceback.print_exc()
finally:
  os.makedirs(os.path.dirname(data_pickle_path), exist_ok=True)
  with open(data_pickle_path, 'wb') as fd:
    pickle.dump(downloaded_data, fd, protocol=pickle.HIGHEST_PROTOCOL)
  print(f'Saved {len(downloaded_data)} items to {data_pickle_path}  ({",".join(downloaded_data.keys())})')
  print(f'{data_pickle_path} is {round(os.path.getsize(data_pickle_path)/1000000, 1)} mb large')



