# Rust Terminal File Explorer
Small terminal file explorer written in rust

## How to use
- build project 
- copy `rs_file_explorer.exe` and `sd.bat` (rename sd to a command of your choice)
(depending on your operating system) into a folder that is on `PATH`<br>
unix:<br>
`sudo cp sd /usr/local/bin/sd`<br>
`sudo cp target/release/rs_file_explorer /usr/local/bin/rs_file_explorer`<br>
`sudo chmod +x /usr/local/bin/sd`<br>
`sudo chmod +x /usr/local/bin/rs_file_explorer`
- run command `sd` in your terminal (use sudo first time on unix)

## Navigation
- `Up`/`Down` Select file
- `Enter` open file (or directory). opens defalt file manager when enter on `.`
- `Esc`/`Del` go up a folder
- `Ctrl-C` exit

# Tested on
- Windows cmd
- ~~SSH remote shell to ubuntu from windows~~  
some ansi codes don't work, some ascii codes seem to be different