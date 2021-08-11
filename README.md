# dns-rs

## Setup Deployment
<br>

Install rust toolchain

```bash
rustup target add armv7-unknown-linux-gnueabihf
```

Update/Upgrade system (if needed)
```bash
sudo apt update
sudo apt upgrade
```

Install linker and compiler

```bash
sudo apt install gcc-arm-linux-gnueabihf
```

Install cmake

```bash
sudo apt install cmake
```

Insert following code into `~/.cargo/config.toml`:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Start deployment

```bash
bash deploy.sh
```