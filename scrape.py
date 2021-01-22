# curl \
#   -H "Accept: application/vnd.github.v3+json" \
#   https://api.github.com/repos/octocat/hello-world/contents/PATH

# curl \
#   -H "Accept: application/vnd.github.v3+json" \
#   https://api.github.com/users/oofsauce/repos

import sys
import requests
import json
import base64
import markdown as md
import os
headers = {
  'Authorization': 'Basic b29mc2F1Y2U6M2M2MTNkMTZhOTQyZTMwZDUyZGNmNjYzYTU5ZWJjNDAzOTIyZjk5NA=='
}
r = requests.get('https://api.github.com/users/oofsauce/repos', headers=headers)
repos = r.json()
if len(sys.argv) < 2:
  print("No target directory specified!")
  sys.exit(1)
if "message" in repos:
  print("ERROR:", repos["message"])
  sys.exit(1)
for repo in repos:
  readme = requests.get('https://api.github.com/repos/' + repo["full_name"] +'/contents/README.md', headers=headers).json()
  if "message" in readme:
    print(repo["full_name"])
    print("ERROR:", readme)
    continue
  with open(os.path.join(sys.argv[1],repo["name"] + '.yml'), 'w') as file: 
    markdown = str(base64.b64decode(readme["content"].replace("\\n", "")), "utf-8")
    splits = markdown.split('\n')
    firstLine = splits[0].strip()
    title = repo["name"]
    print(firstLine[1:])
    if(firstLine[0] == "#"):
      title = firstLine[1:].strip()
      splits = splits[1:]
      
    desc = md.markdown("\n".join(splits)).replace("\n", "").replace("\"", "\\\"")
    file.write("id: " + repo["name"])
    file.write("\ntitle: " + title)
    file.write("\nurl: " + repo["html_url"])
    file.write("\ndescription: \"" + desc + "\"")
  #print()

sys.exit(0)