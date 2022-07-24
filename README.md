# README for stm32f303k8-rust-driver
## Requirements
- openocd
- stm32f303k8
- gcc
- arm-none-eabi-gdb(for debug)
## Setup
1. install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
2. install target
```
rustup target add thumbv7em-none-eabihf 
```

## Build
```
cargo build
```

## Steps for Debug
1. launch openocd
```
openocd -f stm32f3xx_stlink_v2.cfg 
```
2. connect gdb server with arm-none-eabi-gdb and load program
```
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/sample
(gdb) target remote :3333
(gdb) load
```
3. enable semihosting(when use arm semihosting)
```
(gdb) monitor arm semihosting enable
```
4. continue the program
```
continue
```

## Install binary to Nucleo
```
openocd -f stm32f3xx_stlink_v2.cfg -c "program target/thumbv7em-none-eabihf/debug/sample verify reset exit"
```
