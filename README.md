# dns-rs

An easy to setup dynamic DNS updater
<br>
<br>

## Installation

Use the following commands to install the CLI:

```bash
git clone https://github.com/typable/dns-rs.git
cd dns-rs
cargo install --path .
```

## Usage

Use the `dns-rs` command to update the configured domains.

**Note**: You need to configure your provider and domains first in order to use this command. 

### Scheduled updates

Create a new folder for the log files.

```bash
mkdir -p $HOME/log/dns-rs
```

Use `crontab -e` to edit your cronjobs and add the following line. Replace `<user>` with your username.

```cron
*/30 * * * * /home/<user>/.cargo/bin/dns-rs >> "/home/<user>/log/dns-rs/$(date +\%Y-\%m-\%d).log"
```

This will execute the "dns-rs" command every 30 minutes and writes the output into a log file.<br>
<br>
You can find more details about scheduling your cronjob here: [crontab.guru](https://crontab.guru/#*/30_*_*_*_*)
<br>
<br>

## Configuration

Create the `$HOME/.config/dns-rs/config.toml` configuration file.<br>
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
update = false # check but don't update
```
