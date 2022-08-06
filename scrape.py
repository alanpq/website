# curl \
#   -H "Accept: application/vnd.github.v3+json" \
#   https://api.github.com/repos/octocat/hello-world/contents/PATH

# curl \
#   -H "Accept: application/vnd.github.v3+json" \
#   https://api.github.com/users/alanpq/repos
#!/usr/bin/env python3
import sys
import requests
import json
import base64
import markdown as md
import os
import re

# HEAD request can be iffy, but good enough for our purposes
def is_url_image(image_url):
   image_formats = ("image/png", "image/jpeg", "image/jpg", "image/gif")
   r = requests.head(image_url)
   if r.headers["content-type"] in image_formats:
      return True
   return False


imageSearch = re.compile(r"!\[.+\]\((https?:\/\/([a-z\.]+)\/?.*)\)", flags=re.IGNORECASE | re.MULTILINE)

denylist = {
  "img.shields.io"
}

quitTime = False
if not "USERNAME" in os.environ:
  print("'USERNAME' env var not set!")
  quitTime = True

if not "TOKEN" in os.environ:
  print("'TOKEN' env var not set!")
  quitTime = True

if quitTime:
  sys.exit(1)

data = os.environ["USERNAME"] + ":" + os.environ["TOKEN"]
headers = {
  'Authorization': 'Basic ' + str(base64.b64encode(data.encode("utf-8")), "utf-8")
}
r = requests.get('https://api.github.com/users/' + os.environ["USERNAME"] + '/repos', headers=headers)
repos = r.json()
if len(sys.argv) < 2:
  print("No target directory specified!")
  sys.exit(1)
if "message" in repos:
  print("ERROR:", repos["message"])
  sys.exit(1)
for repo in repos:
  print('=======[' + repo["full_name"] + ']=======')
  readme = requests.get('https://api.github.com/repos/' + repo["full_name"] +'/contents/README.md', headers=headers).json()
  if "message" in readme:
    print("ERROR:", readme)
    print()
    continue

  # TODO: implement this
  # readme = requests.get('https://api.github.com/repos/' + repo["full_name"] +'/contents/thumbnail.png', headers=headers).json()
  # if "message" in readme:
  #   print(repo["full_name"])
  #   print("ERROR:", readme)
  #   continue
  path = os.path.join(sys.argv[1],repo["name"] + '.yml')
  with open(path, 'w') as file: 
    markdown = str(base64.b64decode(readme["content"].replace("\\n", "")), "utf-8")
    
    thumbnail = ""
    print("[THUMBNAIL SCAN]")
    imgRes = re.findall(imageSearch, markdown)
    for res in imgRes:
      print('"'+res[0]+'"', "is ", end="")
      if res[1] in denylist:
        print("in deny list.")
        continue
      if not is_url_image(res[0]):
        print('not an image.')
        continue
      print("a valid thumbnail")
      thumbnail = res[0]
    
    print("[END OF THUMBNAIL SCAN]")
    # print(imgRes)
    splits = markdown.split('\n')
    firstLine = splits[0].strip()
    title = repo["name"]
    print('firstLine[1:] =', firstLine[1:])
    if(firstLine[0] == "#"):
      title = firstLine[1:].strip()
      splits = splits[1:]
      
    desc = md.markdown("\n".join(splits)).replace("\n", "").replace("\"", "\\\"")
    file.write("id: " + repo["name"])
    file.write("\ntitle: " + title)
    file.write("\nurl: " + repo["html_url"])
    file.write("\ndescription: \"" + desc + "\"")
    file.write("\nthumbnail: \"" + thumbnail + "\"")
    print('\nwritten to "' + path + '".')
  print()

sys.exit(0)