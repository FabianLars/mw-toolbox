# wtools
some tools to interact with http://leagueoflegends.fandom.com/de/wiki/  
more or less connected to [wapi](https://github.com/FabianLars/wapi)  
uses limited editing rate of ~1 edit per second according to fandoms rules  
Interaction with Riot's API only via CLI (just "update rotation" for now)  
* Needs feature flag "riot-api"
Input files need to be formatted with newline seperation (eg 1 wiki article per line)  
Options (arguments specified with "-" or "--") work on compiled program only. (using "wtools" instead of "cargo run")
* start GUI:
  * double-click exe after building or type "cargo run -p gui" terminal
* CLI usage:
  * run via "[cargo run|wtools] \<command\> (\<args if needed\>)" 
    * these commands need FANDOM_BOT_NAME, FANDOM_BOT_PASSWORD and/or RIOT_API_KEY (depends on command) environment variables as of now  
      * Fandom Login data can be provided via:  
        "wtools [--loginname \<name\>|-n \<name\>] and [--loginpassword \<pw\>|-p \<pw\>]"
    * example: "cargo run delete ../todelete.txt"
      * deletes every page listed in specified file (separation via newline)
  
  
  
# Important!
This project isn’t endorsed by Riot Games and doesn’t reflect the views or opinions of Riot Games
or anyone officially involved in producing or managing League of Legends. League of Legends and Riot Games are
trademarks or registered trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc.  
  
Same for Fandom btw...
