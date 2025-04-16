# Ronfig 🗂️

Easily manage and deploy your configuration files across devices. 🚀

Ronfig helps you store and organize configuration files in one directory and copy them to their respective locations effortlessly. Instead of manually managing dotfiles or config backups, use Ronfig to automate the process.

## Requirements 📌
Before installing, make sure you have the following installed:

* [Rust & Cargo 🦀](https://www.rust-lang.org/tools/install)
* [Git 🔧](https://git-scm.com/downloads)

## Installation 📥

### Quick Install 🔧

#### Clone Ronfig:

```bash
git clone https://github.com/DerIch69420/Ronfig.git
cd Ronfig
```

#### Run the Install Script:
```bash
chmod u+x build/build.sh 
./build/build.sh
```

#### Add it to your shell onfig

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.${SHELL##*/}rc"
```

#### Reload your Shell Configuration:
```bash
source ~/.zshrc # or source ~/.bashrc
```

### Manual Install

#### Clone and build Ronfig:

```bash
git clone https://github.com/DerIch69420/Ronfig.git
cd Ronfig
cargo build --release
```

#### Add it to your Path:
Move the compiled binary to `~/.local/bin` (or any directory in your `PATH`):

```bash
mkdir -p ~/.local/bin
cp target/release/ronfig ~/.local/bin/
```

Then add the following line to your shell config (`~/.zshrc`, `~/.bashrc`, etc.) if not already present:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

#### Reload your Shell Configuration:
```bash
source ~/.zshrc # or source ~/.bashrc
```

## Qiuck Usage ⚙️

### 1. Create a Config Directory

```bash
ronfig new config_dir
```

### 2. Define Your Configurations

Edit the config.json file:

```bash
vi config_dir/config.json
```

### 3. Add Your Configuration Files

Create the actual configuration files referenced in config.json:

```bash
vi config_dir/my_config_file.conf
vi config_dir/my_dir_to_copy/file1.conf
vi config_dir/my_dir_to_copy/file2.conf
```

### 4. Copy Files to Their Destination

Use Ronfig to deploy your configuration files:

```bash
ronfig copy config_files
```

## Future Plans 🏗️

* Add colored output
* Add automatic directory creation
* Add symlink support

## Contributing 🤝

Contributions are welcome! If you find a bug or have a feature request, open an issue or submit a pull request.

## [Documentation](Documentation/_main.md)
## [MIT License](LICENSE)

