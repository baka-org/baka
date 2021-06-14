# baka
**b**asic multiple p**a**c**k**age m**a**nager


## Docs

### Env

- `baka_root_setting`
    - Windows: %USERPROFILE%/.baka/config
    - Linux, Mac: $HOME/.baka/config
- `baka_plugins` (Just use plugin command)
    - Windows: %USERPROFILE%/.baka/plugins/*
    - Linux, Mac: $HOME/.baka/plugins/*

### Commands Structure

- found .baka

baka [command] [flag] `or` baka [baka-flags] [package-manager-command] [package-manager-args]

- not

baka [baka-flags] [package-manager-command] [package-manager-args]

- Commands
    - baka [baka-flags] [package-manager-command] [package-manager-args]
    - baka plugin
        - add [git-url]
        - remove [package-manager-name]
        - list
    - baka help
    - baka version
    - baka upgrade
- baka-flags
    - -p [package-manager-name]
    - -l [language-name] (for multiple language config)


### Files
* Available data formats are json, yaml, and toml.


* .baka.json
 ```json
 {
	"manager": "cargo" // The package manager name registered with the plugin
}
 ```
 
 * config.json
 ```json
 {
	"language": "en-us", // language
	"plugins": ["%name%-%version%"], // plugin list
	"programming_languages": { // plugin alias
		"python": "pip3"
	}
}
 ```


* plugin.json
```json
{
	"name": "[package-manager-name]", // package manager name
	"version": "1.0", // version
	"cmd": {
		"install": { // subCommand name
			"exec": "%path% install",
			"description": "Sth Sth", // subCommand description
			"help": "Sth Sth" // subCommand help 
		},
		"uninstall": {
			"exec": "%path% uninstall"
		},
		"search": {
			"exec": "%path% search"
		},
		"[Custom-Command]": {
			"exec": "%path% [Custom-Command]"
		}
	},
	"path": { 
  	// If all is null, should have at least one of darwin, win, Linux, other
		"all": "",
    		"darwin": "",
    		"win": "",
    		"linux": "",
    		"other": "",
	}
}
```
