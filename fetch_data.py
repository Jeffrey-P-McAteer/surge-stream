# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "arcgis"
# ]
# ///

import os
import sys
import pathlib
import zipfile
import pickle
import traceback
import urllib.request
import json

import arcgis

from arcgis.gis import GIS

def get_all_key_values_in(collection, search_key):
  values = []
  if isinstance(collection, dict):
    if search_key in collection:
      values.append( collection[search_key] )
    else:
      for k,v in collection.items():
        values += get_all_key_values_in(v, search_key)
  elif isinstance(collection, list):
    for v in collection:
        values += get_all_key_values_in(v, search_key)
  return values

DEBUG = 'debug' in sys.argv

data_pickle_path = os.path.join(os.path.dirname(__file__), 'data', 'raw-layer-data.pickle')
downloaded_data = dict()
if os.path.exists(data_pickle_path):
  with open(data_pickle_path, 'rb') as fd:
    downloaded_data = pickle.load(fd)

try:
  gis = GIS()

  layer_urls_to_download = [
    ('north-american-pipeline',  'https://services.arcgis.com/jDGuO8tYggdCCnUJ/ArcGIS/rest/services/PLALLPLS_polyline/FeatureServer/0'),
    ('mexican-oil-refinery',     'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/Mexican_Oil_Refinery_Capacity/FeatureServer/0'),
    ('canadian-oil-refinery',    'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/Canadian_Oil_Refinery_Capacity/FeatureServer/0'),
    ('us-oil-refinery',          'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/US_Oil_Refineries_Broken_Out_By_Capacity_and_Product_Type/FeatureServer/0'),
    ('us-oil-refinery',          'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/US_Oil_Refineries_Broken_Out_By_Capacity_and_Product_Type/FeatureServer/0'),
    ('us-lng-terminals',         'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/United_States_LNG_Terminals_and_Peak_Shavers/FeatureServer/0'),
  ]

  # "QUANTITY12" isn't useful esri! This is the result of Jeff mapping data names to layer label names -_-
  attribute_rich_names = {
    'QUANTITY1': 'Cracking "Fresh Feed", Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)',
    'QUANTITY10': 'Hydrocracking, Gas Oil, Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)',
  }

  # Even better; just ask the layer popupInfo.fieldInfos structure for the data
  with urllib.request.urlopen('https://www.arcgis.com/sharing/rest/content/items/67980e7ee1904cbcb3b53cdd2c3731c7/data?f=json') as fd:
    layer_meta_data = json.loads(fd.read().decode('utf-8'))
  for field_info_list in get_all_key_values_in(layer_meta_data, 'fieldInfos'):
    for field_info in field_info_list:
      field_name = field_info.get('fieldName', '')
      field_label = field_info.get('label', '')
      if field_name.casefold() != field_label.casefold():
        if DEBUG:
          print(f'{field_name} is actually a {field_label}')
        attribute_rich_names[field_name] = field_label


  for layer_name, layer_url in layer_urls_to_download:
    fl = arcgis.features.FeatureLayer(layer_url, gis)
    if DEBUG:
      print(f'[ {layer_name} ] fl = {fl}')

    num_items_in_layer = fl.query(return_count_only=True)

    if layer_name in downloaded_data and len(downloaded_data[layer_name]) >= num_items_in_layer:
      continue # We've already fetched this, keep going

    # Data is currently Layers => List of Features in Esri JSON format.
    downloaded_data[layer_name] = list()
    for item in fl.query(out_sr=4326):
      # print(f'typeof(item) = {type(item)}') # It's a arcgis.features.feature.Feature
      item_d = item.as_dict
      for key in list(item_d['attributes'].keys()):

        if key in attribute_rich_names:
          item_d['attributes'][attribute_rich_names[key]] = item_d['attributes'][key]
          item_d['attributes'].pop(key, None)

      # print(f'item_d = {item_d}')

      downloaded_data[layer_name].append( item_d )

except:
  traceback.print_exc()
finally:
  os.makedirs(os.path.dirname(data_pickle_path), exist_ok=True)
  with open(data_pickle_path, 'wb') as fd:
    pickle.dump(downloaded_data, fd, protocol=5) # because our Rust end has support for 5
  print(f'Saved {len(downloaded_data)} items to {data_pickle_path}  ({",".join(downloaded_data.keys())})')
  print(f'{data_pickle_path} is {round(os.path.getsize(data_pickle_path)/1000000, 1)} mb large')



