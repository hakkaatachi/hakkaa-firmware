## Toolchain
1. [Rustup.rs](https://rustup.rs/)
2. [The Rust on ESP Book
](https://docs.espressif.com/projects/rust/book/installation/riscv.html)
3. [hakkaa-firmware](https://github.com/sirhcel/hakkaa-firmware)
4. Espflash
    ```
    $ cargo install --locked espflash
    ```
    `--locked` installiert genau die im Projekt hinterlegten Versionen der Abhängigkeiten

## Firmware bauen

1. Im Firware Ordner ausführen
    ```
    $ cargo build
    ```
  
## Hardware-Test auf dem Target ausführen

```
$ cargo run --bin eol-test
```

Board beobachten 

## Eigene Firmware fürs Target

* Dokumentation für Hakkaa-Board-Support und benutzte Module erstellen und im Browser anzeigen lassen
    ```
    $ cargo doc --open
    ```

* Auf Basis einer Kopie des Blinky-Beispiels `examples/blinky.rs`
* Kopie in `src/bin` erstellen, zum Beispiel `src/bin/pov.rs`
* Anschließend analog zum Hardware-Test ausführen
   ```
   $ cargo run --bin pov
   ```
