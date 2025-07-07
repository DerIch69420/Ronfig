# Modes
* The mode in which ronfig runs is the first one to be passed after the binary
```bash
ronfig $mode
```

## Copy
* Copy files from your config directory to your desired location
```bash
ronfig copy $config_dir
```
* If used without any directory as argument it uses your current directory
```bash
ronfig copy
```

## New
* Create a new directory and the corresponding config.json
```bash
ronfig new $config_dir
```
## Help
* See help about ronfig
```bash
ronfig help
```
