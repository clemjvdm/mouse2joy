# Mouse2Joy
An ultra simple CLI tool for linux which enables you to convert mouse input to a virtual joystick. Only x-axis input is converted since this is intended for racing games. I recommend pairing this tool with a joystick/gamepad visualization tool to have a clear view of how far you are steering ;).

## Usage
You can grab a binary archive [here](https://github.com/clemjvdm/mouse2joy/releases), extract the binary and then run it ⚠️ **with root priviledges** ⚠️. Root priviledges are needed to read input devices.

After this the mouse2joy will ask you which mouse you would like to use, and you should be ready to go!

## Lore
I actually started working on a much more feature full keyboard and mouse to virtual gamepad tool (kbm-gamepad). But I didn't want to keep working on it as I wasn't sure it was going to be useful to anyone (it wasn't going to be for me).

But I can imagine it being useful in some scenarios, and since the tool is close to being done if someone thinks it would be useful let me know and I can probably finish it. Until then, this tool fills just my use case and nothing more :).

## To-do
To do's before release 1.0:
- [ ] Better CLI
- [X] Allow different levels of logging
- [ ] (maybe) A small GUI to indicate the current joystick position
- [X] (maybe) Configure joystick configuration through a .toml file
- [ ] testing differnt configurations
- [ ] user manual
