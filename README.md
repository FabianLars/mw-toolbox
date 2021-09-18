# mw-toolbox

some tools to interact with [fandom](https://community.fandom.com/) wikis

uses limited editing rate of ~1-2 edits per second according to fandoms rules

-   GUI usage (development):
    -   cd into gui subdir
    -   run "pnpm|yarn|npm install"
    -   run "pnpm|yarn|npm start" to start the dev server
    -   run "pnpm|yarn|npm tauri dev" to start the tauri app
-   CLI usage (development):
    -   Input files need to be formatted with newline seperation (eg 1 wiki article per line)
    -   run via "cargo run \<command\> (\<args if needed\>)" inside cli folder
        -   every command needs Fandom login credentails created via Special:BotPasswords. There are two ways to provide them:
            -   the FANDOM_BOT_NAME & FANDOM_BOT_PASSWORD environment variables
            -   cli flags: "cargo run [--loginname \<name\>|-n \<name\>] and [--loginpassword \<pw\>|-p \<pw\>]"
        -   example: "cargo run delete ../todelete.txt"
            -   deletes every page listed in specified file (separation via newline)
