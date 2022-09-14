# dns-rs

An easy to setup dynamic DNS updater

## Installation

Use the following commands to install the CLI:

```bash
git clone https://github.com/typable/dns-rs.git
cd dns-rs
cargo install --path .
```

## Usage

Use the `dns-rs` command to update the configured domains.

Create the `~/.config/dns-rs/config.toml` configuration file.<br>
Use the following config template to configure your provider and domains.

```toml
[provider]
host = "https://domain.com"
path = "/update"
args = "host=%h&subdomain=%s&ip=%i&password=%s"

# %h = host
# %s = subdomain
# %i = ip address
# %p = password

# example.com
[[domains]]
host = "example.com"
password = "XXXXXXXX"

# test.example.com
[[domains]]
host = "example.com"
subdomain = "test"
password = "XXXXXXXX"
update = false
```
