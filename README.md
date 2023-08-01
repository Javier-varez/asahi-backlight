# Asahi linux brightness control

Tiny rust app to control the brightness of the display by directly interacting with the `sysfs` node.

```
# increase the brightness by 50
cargo run -- up 50

# decrease the brightness by 50
cargo run -- down 50

# set the brightness to 100
cargo run -- set 100

# get the current brightness 
cargo run -- get
```
