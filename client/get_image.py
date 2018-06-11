import requests
import base64
import json
import sys

if len(sys.argv) != 4:
    print('Usage: python %s <host> <img_id>' % sys.argv[0])
    quit()

img_id = sys.argv[2]
url = sys.argv[1] + "/" + str(img_id)

print('Reading image file %s' % img_file_path)
img = open(img_file_path, 'rb').read()
encoded_img = base64.standard_b64encode(img)
print('Encoded image into base64')
# print(encoded_img.decode('utf-8'))
# quit()

res = requests.post(url)

encoded_img = res.json()["mosaic_art"]
img = base64.standard_b64decode(encoded_img)

print('Wrinting image file')
open(img_id + ".png", "wb").write(img)
