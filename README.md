# build / flash

assuming microbit:v2

`cargo embed --target thumbv7em-none-eabihf`

release 

`cargo embed --target thumbv7em-none-eabihf --relase`

inspect size

`cargo size -- -A`
`cargo size --release -- -A`


# debug

```
gdb target/thumbv7em-none-eabihf/debug/<file>
target remote :1337 (port running gdb stub)
```

some useful gdb commands

```
layout src/asm
tui disable

c / continue
next
break <name / linenum>
info break
delete <break>
info locals

monitor reset
```

# start minicom for serial communication

create `~/.minirc.dfl`
```
pu baudrate 115200
pu bits 8
pu parity N
pu stopbits 1
pu rtscts No
pu xonxoff No
```

sudo seems necessary, not sure why udev rules do not apply
`sudo minicom -D /dev/ttyACM0 -b 115200` 

**before anything usb**
create `/etc/udev/rules.d/69-microbit.rules`
```
# CMSIS-DAP for microbit
ACTION!="add|change", GOTO="microbit_rules_end"
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess"
LABEL="microbit_rules_end"
```

`sudo udevadm control --reload`

