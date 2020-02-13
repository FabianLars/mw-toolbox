# wikibot
some tools and standalone scripts for automated tasks on http://leagueoflegends.fandom.com/de/wiki/

## scripts
* api_rotation.php
  * Called weekly via CRON-Job to update the wiki's champion rotation template with the new rotation fetched from Riot's API.
* updateChampsById.php
  * Updates champsById.json (see "helper files")

## wapi
server-side usage  
api stuff for 3rd-party server for things the wiki can't handle
* server_url/wapi/skinlink/CHAMPION/SKIN
  * redirects to the 3D-Skinviewer on [teemo.gg](https://www.teemo.gg/model-viewer) via [communitydragon](https://communitydragon.org) files.
* server_url/wapi/update/champs
  * updates champion data (local json file)

## tools
client-side usage  
run via "cargo run [toolname] [filename]"  
these commands need FANDOM_BOT_NAME and FANDOM_BOT_PASSWORD environment variables
* eg. "cargo run delete ../todelete.txt"
  * deletes every page listed in specified file (separation via newline)


## helper files:
* No. 1: champsById.json
  * json object to return a champion's name by its id.
  
  
  
## Upcoming:
* More scripts converted to rust
* Handling of Response Codes from Riot's API (imo not really needed at the moment when called once per week)



# Important!
These scripts aren’t endorsed by Riot Games and don’t reflect the views or opinions of Riot Games
or anyone officially involved in producing or managing League of Legends. League of Legends and Riot Games are
trademarks or registered trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc.
