# Ronfig 🗂️

Easily manage and deploy your configuration files across devices. 🚀

Ronfig helps you store and organize configuration files in one directory and copy them to their respective locations effortlessly. Instead of manually managing dotfiles or config backups, use Ronfig to automate the process.

## Installation 📥

### Clone and build Ronfig:

```bash
git clone https://github.com/DerIch69420/Ronfig.git
cd Ronfig
cargo build --release
```

## Usage ⚙️

### 1. Create a Config Directory

```bash
mkdir config_files
cd config_files
```

### 2. Define Your Configurations

Create a config.json file:

```bash
vi config.json
```

Add the following content:

```json
[
  {
    "filename": "my_config_file.conf",
    "desired_path": "my/config/location"
  },
  {
    "filename": "my_other_config_file",
    "desired_path": "other/config/location"
  }
]
```

> filename: A relative path to the file from your configuration directory. This can include subdirectories.
> desired_path: The target location where the file should be placed, relative to your home directory (~).

### 3. Add Your Configuration Files

Create the actual configuration files referenced in config.json:

```bash
vi my_config_file.conf
vi my_other_config_file
```

### 4. Copy Files to Their Destination

Run Ronfig to deploy your configuration files:

```bash
path/to/ronfig copy config_files
```

## How It Works 🛠️

Ronfig reads config.json and copies the specified files from the source directory to their respective desired_path locations. If a file already exists in the target location, it will be overwritten.

### Available Modes 🎛️

* help → Displays usage information.

* copy → Copies configuration files to their specified locations.


## Features ✨

* Simple JSON-based configuration management

* Automates copying configuration files to desired locations

* Easily backup and restore configuration files

* Lightweight and fast

##
Future Plans 🏗️

* Add symlinks


## Contributing 🤝

Contributions are welcome! If you find a bug or have a feature request, open an issue or submit a pull request.

