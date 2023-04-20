<p align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/Made%20With-Rust-000000?style=for-the-badge" alt="made with rust" /></a></a>
</p>
<div align="center">
  <img src="https://github.com/ArshErgon/gitfetch/blob/main/main.gif" />
</div>
<div align="center">

# **gitGrab**

An Open-Source tool for Open-Source Enthusiast that shows your GitHub Contribution on the terminal

</div>

# Why its created?

I was quite active on open source contributions and needed to track my progress across multiple repositories. I was constantly going back and forth between different tools to check my issues, followers, and pull requests. In the meantime, I discovered neofetch, an awesome command-line interface program that displays basic information about the operating system. I thought, why not create something similar to it? The starting phase of GitGrab was quite similar to neofetch, but later I added language bars and a contribution graph etc to it. For those unfamiliar with these terms, language bars display the percentage of code contributed in each programming language, and a contribution graph shows the frequency of contributions over time.

# Features

- Can see a user information without deleting the permantent user.
- All viewers information
- Languages Bars
- Contribution Graph
- Lines Of Code
- Compare with 2 users

## LOC (lines of code)
<div align="center">
<img src="https://github.com/ArshErgon/gitfetch/blob/2821d4719bd7aaf6d28adb08eb85b2f200f51ed1/loc.gif">
</div>

## Compare two users
<div align="center">
<img src="https://github.com/ArshErgon/gitfetch/blob/2821d4719bd7aaf6d28adb08eb85b2f200f51ed1/compare.gif">
</div>

# Commands

### Basic

```rust
$ gitgrab -o
  1. Create a User
  2. Enter/Update the Github API key
  3. Exit
```

### For a temporary User

```rust
$ gitgrab -t <USERNAME>
```

### For LOC (lines of code)
```rust
$ gitgrab --loc <URL>
```

### For comparing users
```rust
$ gitgrab --com "<UserOne> <UserTwo>"
```

### More commands

```rust
$ gitgrab -a

$ gitgrab -h
```

# Installation

Remember you need to add github token also: [your safety](https://github.com/ArshErgon/gitgrab#api-key-security), What are [token?](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) and where it will be [created](https://github.com/settings/tokens). Give every permission except **creating and deleting**

# Downloads

## Cargo 

If you have rust installed on your system run these commands to install

```
$ rustup default nightly

```
Then

```
$ cargo install gitgrab --git https://github.com/carghai/gitgrab
```

if you need to update you now just need to run

```
$ cargo install  gitgrab --git https://github.com/ArshErgon/gitgrab --force
```

And if you want to remove it you do

```
$ cargo uninstall  gitgrab   
```


## Linux

Download the binary from [here](https://github.com/ArshErgon/gitgrab/releases/download/v0.2.3/gitgrab), after downloading go to the place where its download (mostly on Desktop) and enter the command below.

```rust
$ sudo install -c gitgrab /usr/local/bin
```

## Windows

Download the binary from [here](https://github.com/ArshErgon/gitfetch/releases/download/v0.2.3/gitgrab.exe),
There are two ways to run in windows.

1. On the CMD, add to the path

```rust
C:> PATH=%PATH%;C:\path\to\gitgrab.exe
```

2. Directly running the binary

```rust
C/Downloads>./gitgrab #or add gitgrab.exe if gets an error.
```

## MacOS

Some builds gets failed as a result the installer wouldn't be made but you can still install it if you have `cargo` installed

```rust
$ cargo install path/to/project/gitfetch
```

# Development

```git
$ git clone https://github.com/USERNAME/gitgrab.git
$ cd gitgrab
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
