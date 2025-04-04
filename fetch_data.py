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
data_item = gis.content.get(public_data_item_id)

print(f'data_item={data_item}')

data_path = pathlib.Path(os.path.join(os.path.dirname(__file__), 'data'))
if not data_path.exists():
    data_path.mkdir()

data_item.download(save_path=data_path)

zip_path = data_path.joinpath('LA_Hub_Datasets.zip')
extract_path = data_path.joinpath('LA_Hub_datasets')
zip_file = zipfile.ZipFile(zip_path)
zip_file.extractall(path=data_path)








