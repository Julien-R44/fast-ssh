<div align="center">
    <img src="https://i.imgur.com/4Mb6msT.png" />
    <br/>
    <b>Connect quickly to your services 🚀</b>
    <br/>
    <br/>
    <a href="https://github.com/Julien-R44/fash-ssh/actions/workflows/release.yml">
        <img src="https://github.com/Julien-R44/fast-ssh/actions/workflows/release.yml/badge.svg" />
    </a>
    <a href="https://crates.io/crates/fast-ssh">
        <img src="https://img.shields.io/crates/v/fast-ssh.svg" />
    </a>
    <img src="https://img.shields.io/crates/l/fast-ssh.svg">
    <br/>
    <br/>
    <div>
        FastSSH is a TUI that allows you to quickly connect to your services by navigating through your SSH config.
    </div>
    <br/>
</div>

![](https://i.imgur.com/pVf2hES.png)

# Documentation
If you already have an SSH configuration file you don't have to add anything, Fast-SSH just parses this file and displays it. 
If you don't have an SSH configuration file or if you don't know what it does, read this: https://linuxize.com/post/using-the-ssh-config-file/

Fast-SSH has a group system. This allows you to sort your servers, for example, by project, mission or client.
To make some groups, it's simple, just define your `Host` as `Group/ServerName` ( see full configuration in above picture ) and your groups will be displayed in FastSSH. You can now select a group and display only the servers defined in that group.

Now all you have to do is launch Fast-SSH, select your service and press enter to connect.

## File Database
A file database is stored at `$XDG_CONFIG_HOME/FastSSH/db.ron` ( `%APPDATA%/FastSSH/db.ron` for Windows ). 

This file is automatically created when you launch Fast-SSH. 
This database is used to store the number of connections to a service and the date of last connection.

## Search Mode
Fast-SSH implements a search mode ( fuzzy ) that allows you to type to find one of your hosts. To use it, press `s`, start typing, finish your selection with the arrow keys then press enter once the host is selected to make the SSH connection. Press ESC if you wish to leave the search mode and return to the "groups" mode.

## Shortcuts
| Key           | Action                               |
| ------------- | -------------                        |
| h             | Display Shortcuts Panel              |
| Enter         | Validate selection : Execute SSH cmd |
| Tab/Left/Right| Navigate through your groups         |
| Up/Down       | Navigate through your hosts          |
| c             | Switch Config display mode           |
| PageUp/Down   | Scroll Configuration                 |
| s             | Enable Search Mode                   |
| Esc           | Exit Search Mode                     |
| q             | Exit Fast-SSH                        |


## Configuration
Fast-SSH has a few configuration options. You can find the configuration in :
- Unix : `$XDG_CONFIG_HOME/FastSSH/config.yaml`
- Windows : `%APPDATA%/FastSSH/config.yaml`

```
# This is the default configuration for FastSSH.

theme:
    text_primary: "#b967ff"
    text_secondary: "#ffffff"
    border_color: "#b967ff"
```

# Installation
Download the latest release for your platform [here](https://github.com/Julien-R44/fast-ssh/releases) and put in directory in your PATH. ( Packages managers coming soon )

If you use cargo you can run `cargo install fast-ssh`

Then you can launch Fast-SSH with `fast-ssh`. 

# Known issues
If you are a user of the new Windows terminal and you encounter raw AINSI escape sequences when you run fast-ssh, you must unset the TERM environment variable before running the application. Read more about this here : https://github.com/crossterm-rs/crossterm/issues/580

Best solution would be to create an alias like this : 
```
alias fast-ssh="TERM= fast-ssh"
```
