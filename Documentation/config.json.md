# ```config.json```
* The ```config.json``` is located in the directory in which your other config files or config directories are located
* It contains 1 array of objects
* Each object contains either  the ```config_file_path``` or ```config_dir_path``` and  the ```desired_path```
* **Example:**
```json
[
	{
	    "config_file_path": "configfile1.conf",
	    "desired_path": ".config/configdir1"
	},
	{
	    "config_file_path": "homework.md",
	    "desired_path": "Desktop/school"
	},
	{
	    "config_dir_path": "nvim",
	    "desired_path": ".config"
	}
]
```
## ```desired_path```
* The ```desired_path``` is the path, where either a single file or a directory gets copied to.
* It automatically adds the users home directory (~). So starting it with ```/home/username``` will not work
* **Example:**
```json
"desired_path":".config/htop"
```

## ```config_file_path```
* The ```config_file_path``` is the path of the file, which should be copied to ```desired_path```
* It is located relative to your directory which contains your configurations
* **Example:**
```json
"config_file_path":"config.jsonc"
```

## ```config_dir_path```
* The ```config_dir_path``` is the path of the directory, which should be copied to ```desired_path```
* It is located relative to your directory which contains your configurations
* **Example:**
```json
"config_dir_path":"nvim"
