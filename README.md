<div align="center">
    <img src="https://i.imgur.com/4Mb6msT.png" />
    <br/>
    <b>Connect quickly to your services 🚀</b>
    <br/>
    <br/>
    <a href="https://github.com/Julien-R44/cli-candlestick-chart/actions/workflows/release.yml">
        <img src="https://github.com/Julien-R44/cli-candlestick-chart/actions/workflows/release.yml/badge.svg" />
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

Fast-SSH has a group system. This allows you to sort your servers, for example, by project, mission or client.
To make some groups, it's simple, just define your `Host` as `Group/ServerName` ( see full configuration in above picture ) and your groups will be displayed in FastSSH. You can now select a group and display only the servers defined in that group.

Now all you have to do is launch Fast-SSH, select your service and press enter to connect.

## File Database
A file database is stored at ~/.fastssh/db.ron. This file is automatically created when you launch Fast-SSH. 
This database is used to store the number of connections to a service and the date of last connection.

# Installation
Download the latest release for your platform [here](https://github.com/Julien-R44/cli-candlestick-chart/releases) and put in directory in your PATH. ( Packages managers coming soon )

If you use cargo you can run `cargo install fast-ssh`

Then you can launch Fast-SSH with `fast-ssh`. 
