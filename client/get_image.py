import requests
import base64
import json
import sys

if len(sys.argv) != 3:
    print('Usage: python %s <url> <img_path>' % sys.argv[0])
    quit()

url = sys.argv[1]
img_path = sys.argv[2]

res = requests.get(url)

encoded_img = res.json()["mosaic_art"]
img = base64.standard_b64decode(encoded_img)

print('Wrinting image into %s' % img_path)
open(img_path, "wb").write(img)
