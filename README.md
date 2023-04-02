<p align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/Made%20With-Rust-000000?style=for-the-badge" alt="made with rust" /></a></a>
</p>
<div align="center">
<img src="https://github.com/ArshErgon/gitfetch/blob/fc2a521c3b3b3536fadaa18497b4fd29b25b1270/gitfetch.gif">
</div>
<div align="center">

# **GitFetch**

An Open-Source tool for Open-Source Enthusiast that shows your GitHub information, languages bars, and contribution graph on terminal.
</div>

# Why its created?
I was quite active on Open-Source contributions and I needed to track my progress across multiple repositories. I was constantly going back and forth between different tools to check my issues, followers, and pull requests. In the mean time I got to know about neofetch an awesome CLI program which tells the basic information of OS, so I thought why not create something similar to it? The starting phase of the gitfetch was quite similar to neofetch later I have added language bars and contribution graph to it.

# Features
* Can see a user information without deleting the permantent user.
* All viewers information
* Languages Bars
* Contribution Graph
# Commands

### Basic

```rust
$ gitfetch -o
  1. Create a User
  2. Enter/Update the Github API key
  3. Exit
```

### For a temporary User

```rust
$ gitfetch -t <USERNAME>
```

### More commands

```rust
$ gitfetch -a

$ gitfetch -h
```

# Installation
Remember you need to add github token also: [your safety](https://github.com/ArshErgon/gitfetch#api-key-security), What are [token?](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) and where it will be [created](https://github.com/settings/tokens). Give every permission except **creating and deleting**
## Linux
Download the binary from [here](https://github.com/ArshErgon/gitfetch/releases/download/v1.0.0/gitfetch), after downloading go to the place where its download (mostly on Desktop) and enter the command below.

```rust
$ sudo install -c gitfetch /usr/local/bin
```
## Windows
Download the binary from [here](https://github.com/ArshErgon/gitfetch/releases/download/v1.0.0/gitfetch.exe),
There are two ways to run in windows.
1. Add to the path
```rust
C:> PATH=%PATH%;C:\path\to\gitfetch.exe
```
2. Directly running the binary
```rust
C/Downloads>./gitfetch #or add gitfetch.exe if gets an error.
```
## MacOS
```
its in alpha stage.
```
# Development

```git
$ git clone https://github.com/USERNAME/gitfetch.git
$ cd gitfetch
$ cargo run -- -t USERNAME
OR
$ cargo run -- -o
```

to run your own or your friends github stats

```bash
cargo run -- -t USERNAME
```

# API Key Security

> As the key is save on your computer and I have no power to get it from your computer, your key is safe, but still when you are giving it the permission( as the contribution graph keeps all the ticks selected) please dont select the delete and creating; or anything you find which can harm you in data breach

https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token and give it all permission (expect: deleting or creating)

