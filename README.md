# Mouse2Joy
An ultra simple CLI tool for linux which enables you to convert mouse input to a virtual joystick. Only x-axis input is converted since this is intended for racing games. I recommend pairing this tool with a joystick/gamepad visualization tool to have a clear view of how far you are steering 😃.

## Install
First go to the directory where you wish to download mouse2joy, then run the following commands:
```
curl -L -O https://github.com/clemjvdm/mouse2joy/releases/download/v0.1.0/mouse2joy-v0.1.0-x86_64-unknown-linux-musl.tar.gz
tar -xzf mouse2joy-v0.1.0-x86_64-unknown-linux-musl.tar.gz
cp ./mouse2joy-v0.1.0-x86_64-unknown-linux-musl/mouse2joy .
```
Now you should be able to run it from the same directory with:
```
./mouse2joy
```

## Configuration
A few settings can be tweaked when using mouse2joy. To do so create a new directory `.config/mouse2joy`, in this directory create a new file called `configuration.toml`. In this file you can paste the following:
```
sensitivity = 100     # Adjust sensitivity of mouse movement
dead_zone = 10        # Dead zone for mouse input
flat = 5              # Flat response region
```
Now you can play around with the different values in this file, and mouse2joy should pick up on the changes. Just make sure to restart mouse2joy everytime.

## Building From Source
To build from source clone make sure you have rust and cargo installed. Then clone the repo, navigate into it and build it with the following commands:
```
git clone https://github.com/clemjvdm/mouse2joy
cd mouse2joy
cargo build
```

## To-do
To do's before release 1.0:
- [ ] better CLI
- [X] allow different levels of logging
- [ ] (maybe) A small GUI to indicate the current joystick position
- [X] (maybe) Configure joystick configuration through a .toml file
- [ ] testing different configurations
- [ ] better docs
