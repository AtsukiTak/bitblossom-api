import requests
import base64
import json
import sys

if len(sys.argv) != 4:
    print('Usage: python %s <url> <img_file> <hashtag>' % sys.argv[0])
    quit()

url = sys.argv[1]
img_file_path = sys.argv[2]
hashtag = sys.argv[3]

print('Reading image file %s ...' % img_file_path)
img = open(img_file_path, 'rb').read()
encoded_img = base64.b64encode(img)
print('Encoded image into base64')

payload = {'origin_img': str(encoded_img), 'hashtags': [hashtag]}
res = requests.post(url, data=json.dumps(payload))

print(res)
