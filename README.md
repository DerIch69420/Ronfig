# Ronfig 🗂️

Manage your configuration files easily in one directory and copy them to other devices 🚀

## Installation 📥

Clone the repository:
```bash
git clone https://github.com/DerIch69420/Ronfig.git
```
Move into the repository:
```bash
cd Ronfig
```
Build it:
```bash
cargo build
```
Create a new directory to manage your config files:
```bash
mkdir config_files
```
Create a config.json:
```bash
vi config.json
```
Add the following to config.json:
```json
{
    "filename": "my_config_file.conf",
    "desired_path": "my_config_location"
}
```
Create your configuration file:
```bash
vi my_config_file.conf
```
Copy your files to the desired location:
```bash
path/to/ronfig copy config_files
```
## Documentation 📚
### Modes ⚙️
* help ➡️ Displays help
* copy ➡️ Copies your configuration files to the specified location
