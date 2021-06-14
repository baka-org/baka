# baka
**b**asic multiple p**a**c**k**age m**a**nager


### Docs
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
		"install": { // subcommand name
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
