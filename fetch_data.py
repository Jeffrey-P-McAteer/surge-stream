# /// script
# requires-python = "==3.11.5"
# dependencies = [
#   "arcgis",
#   "earth-osm"
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

import earth_osm

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
  try:
    with open(data_pickle_path, 'rb') as fd:
      downloaded_data = pickle.load(fd)
  except:
    traceback.print_exc()
    yn = input('Error occurred; continue y/n? ')
    if not 'y'.casefold() in yn.casefold():
      sys.exit(1)

### Experiment Space
if DEBUG:
  print('TODO REMOVE THIS CODE!')
  print('DEBUG set, performing experiment...')

  import earth_osm.args

  print(f'earth_osm={earth_osm}')

  orig_sys_argv = list(sys.argv)
  # choose from 'aerialway', 'aeroway', 'amenity', 'barrier', 'boundary', 'building', 'craft', 'emergency', 'geological',
  #             'highway', 'historic', 'leisure', 'man_made', 'military', 'office', 'place', 'power', 'public_transport',
  #             'railway', 'shop', 'sport', 'tourism', 'waterway'
  sys.argv[:] = ['earth_osm.py', 'extract', 'man_made', '--regions', 'north-america', '--out_dir', os.path.join(os.path.dirname(__file__), 'data'), ]
  earth_osm.args.main()


  # Restore flags
  sys.argv[:] = orig_sys_argv

  sys.exit(1)
###

try:
  gis = GIS()

  # TODO better data identification; the goal is to funnel
  # named lists of item with GIS and attributes which will be
  # used by the Rust code to fill the model with data.
  layer_urls_to_download = [
    # ('north-american-pipeline',  'https://services.arcgis.com/jDGuO8tYggdCCnUJ/ArcGIS/rest/services/PLALLPLS_polyline/FeatureServer/0'),
    # ('mexican-oil-refinery',     'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/Mexican_Oil_Refinery_Capacity/FeatureServer/0'),
    # ('canadian-oil-refinery',    'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/Canadian_Oil_Refinery_Capacity/FeatureServer/0'),
    # ('us-oil-refinery',          'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/US_Oil_Refineries_Broken_Out_By_Capacity_and_Product_Type/FeatureServer/0'),
    # ('us-oil-refinery',          'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/US_Oil_Refineries_Broken_Out_By_Capacity_and_Product_Type/FeatureServer/0'),
    # ('us-lng-terminals',         'https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/United_States_LNG_Terminals_and_Peak_Shavers/FeatureServer/0'),

    # These were extracted from a running app w/ dump_har_urls.py
    ('Battery_Storage_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Battery_Storage_Plants/FeatureServer/0'),
    ('Biomass_Plants_Testing_view', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Biomass_Plants_Testing_view/FeatureServer/0'),
    ('BorderCrossing_Electric_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/BorderCrossing_Electric_EIA/FeatureServer/0'),
    ('BorderCrossing_Liquids_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/BorderCrossing_Liquids_EIA/FeatureServer/0'),
    ('Coal_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Coal_Power_Plants/FeatureServer/0'),
    ('CrudeOil_Pipelines_US_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/CrudeOil_Pipelines_US_EIA/FeatureServer/0'),
    ('CrudeOil_RailTerminals_US_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/CrudeOil_RailTerminals_US_EIA/FeatureServer/0'),
    ('Geothermal_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Geothermal_Power_Plants/FeatureServer/0'),
    ('HGL_Pipelines_US_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/HGL_Pipelines_US_EIA/FeatureServer/0'),
    ('Hydroelectric_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Hydroelectric_Power_Plants/FeatureServer/0'),
    ('Hydro_Pumped_Storage_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Hydro_Pumped_Storage_Power_Plants/FeatureServer/0'),
    ('Natural_Gas_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Natural_Gas_Power_Plants/FeatureServer/0'),
    ('Nuclear_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Nuclear_Power_Plants/FeatureServer/0'),
    ('Other_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Other_Power_Plants/FeatureServer/0'),
    ('Petroleum_Ports', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Petroleum_Ports/FeatureServer/0'),
    ('Petroleum_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Petroleum_Power_Plants/FeatureServer/0'),
    ('PetroleumProduct_Pipelines_US_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/PetroleumProduct_Pipelines_US_EIA/FeatureServer/0'),
    ('Petroleum_Waterways', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Petroleum_Waterways/FeatureServer/0'),
    ('Solar_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Solar_Power_Plants/FeatureServer/0'),
    ('TightOil_ShaleGas_Plays_Lower48_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/TightOil_ShaleGas_Plays_Lower48_EIA/FeatureServer/0'),
    ('Wind_Power_Plants', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/arcgis/rest/services/Wind_Power_Plants/FeatureServer/0'),
    ('USA_Railroads_1', 'https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Railroads_1/FeatureServer/0'),
    ('US_Wind_Turbine_Database', 'https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/US_Wind_Turbine_Database/FeatureServer/0'),
    ('NaturalGas_ProcessingPlants_US_EIA', 'https://services2.arcgis.com/FiaPA4ga0iQKduv3/arcgis/rest/services/Natural_Gas_Processing_Plants1/FeatureServer/0'),
    ('PetroleumProduct_Terminals_US_EIA', 'https://services7.arcgis.com/FGr1D95XCGALKXqM/ArcGIS/rest/services/PetroleumProduct_Terminals_US_EIA/FeatureServer/36'),
  ]

  # "QUANTITY12" isn't useful esri! This is the result of Jeff mapping data names to layer label names -_-
  # Data Attributes on the left get re-named to the values on the right.
  attribute_rich_names = {
    'QUANTITY1': 'Cracking "Fresh Feed", Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)',
    'QUANTITY10': 'Hydrocracking, Gas Oil, Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)',
  }

  # Even better; just ask the layer popupInfo.fieldInfos structure for the data
  # with urllib.request.urlopen('https://www.arcgis.com/sharing/rest/content/items/67980e7ee1904cbcb3b53cdd2c3731c7/data?f=json') as fd:
  #   layer_meta_data = json.loads(fd.read().decode('utf-8'))
  # for field_info_list in get_all_key_values_in(layer_meta_data, 'fieldInfos'):
  #   for field_info in field_info_list:
  #     field_name = field_info.get('fieldName', '')
  #     field_label = field_info.get('label', '')
  #     if field_name.casefold() != field_label.casefold():
  #       if DEBUG:
  #         print(f'{field_name} is actually a {field_label}')
  #       attribute_rich_names[field_name] = field_label

  # Begin downloading Esri Features from Esri Servers
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

      item_d['attributes']['from_layer_name'] = layer_name

      downloaded_data[layer_name].append( item_d )

    print(f'Saved data for {layer_name}')

  # Begin downloading Arbitrary Data from JSON data sources
  # (in this particular case, all oil wells + supporting infrastructure in Texas)
  if not 'texas-drilling.com' in downloaded_data or len(downloaded_data['texas-drilling.com']) < 10:
    county_ids_hmtl_url = 'https://www.texas-drilling.com/map-search'
    county_id_to_name_dict = dict()
    with urllib.request.urlopen(county_ids_hmtl_url) as fd:
      county_ids_html = fd.read().decode('utf-8')
      have_seen_Select_County_line = False
      for line in county_ids_html.splitlines():
        if 'Select County'.casefold() in line.casefold():
          have_seen_Select_County_line = True
          continue # Don't process _this_ line b/c it's not a number

        if have_seen_Select_County_line:
          if 'option value=' in line and '</option>' in line and len(line) < 180:
            number = line.split('"')[1]
            county_name = line.split('>')[1].split('<')[0].strip()
            county_id_to_name_dict[county_name] = int(number)

    if DEBUG:
      print(f'county_id_to_name_dict = {county_id_to_name_dict}')

    texas_oil_field_esri_features = []
    for county_name, county_id in county_id_to_name_dict.items():
      data_url = f'https://www.texas-drilling.com/map-search?api_no=&county={county_id}&lease_key=&well_name=&operator_name=&field_formation=&json='
      with urllib.request.urlopen(data_url) as fd:
        data_json = json.loads(fd.read().decode('utf-8'))
        for point in data_json.get('point', []):
          texas_oil_field_esri_features.append({
            'geometry': {
              'x': point.get('point', [0.0, 0.0])[0],
              'y': point.get('point', [0.0, 0.0])[1],
            },
            'attributes': {
              'name': point.get('name', ''),
              'from_layer_name': 'texas-drilling.com',
            },
          })
      print(f'Saved data from texas-drilling.com for {county_name}')

    downloaded_data['texas-drilling.com'] = texas_oil_field_esri_features

except:
  traceback.print_exc()
finally:
  os.makedirs(os.path.dirname(data_pickle_path), exist_ok=True)
  with open(data_pickle_path, 'wb') as fd:
    pickle.dump(downloaded_data, fd, protocol=5) # because our Rust end has support for 5
  print(f'Saved {len(downloaded_data)} items to {data_pickle_path}  ({",".join(downloaded_data.keys())})')
  print(f'{data_pickle_path} is {round(os.path.getsize(data_pickle_path)/1000000, 1)} mb large')



