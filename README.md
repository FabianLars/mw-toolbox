# wtools
some tools to interact with http://leagueoflegends.fandom.com/de/wiki/  
more or less connected to [wapi](https://github.com/FabianLars/wapi)  
client-side usage  
run via "cargo run [toolname] ([arg])" (or build it and run as wtools)  
these commands need FANDOM_BOT_NAME and FANDOM_BOT_PASSWORD environment variables as of now  
uses limited editing rate of ~1 edit per second according to fandoms rules
* eg. "cargo run delete ../todelete.txt"
  * deletes every page listed in specified file (separation via newline)
  
  
  
# Important!
This project isn’t endorsed by Riot Games and doesn’t reflect the views or opinions of Riot Games
or anyone officially involved in producing or managing League of Legends. League of Legends and Riot Games are
trademarks or registered trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc.  
  
Same for Fandom btw...
