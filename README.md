# arduino-rust-registers

Example implements a basic UART workflow to

- send `hello world`
- receive then send back a message with the received data such as `received: $BYTE`

---

# usage

Note that in most cases, `root` permission is required to access `/dev/ttyXXX`.

```sh
# run with serial output with `ravedude`
cargo run

# build .elf in target/avr-none/arduino.elf
cargo build --release

# push to board
sudo avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-none/release/arduino.elf
```
