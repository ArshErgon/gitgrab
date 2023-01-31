<p align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/Made%20With-Rust-000000?style=for-the-badge" alt="made with rust" /></a></a>
</p>
<div align="center">

# Gitfetch

</div>

**Like neofetch but for GitHub**
> The approach of seeing your OS details, in `Neofetch` on terminal is cool, I thought why not for github as I'm using github on daily purpose and I need to check about my followers and stars etc, going back and forth on google and then github I found that a lot of work, I'm lazy.

# Running it locally Or Installing for your OS
## Linux Or MacOS
For running it on Linux or MacOS(not tested) download the binary [here](https://github.com/ArshErgon/gitfetch/blob/main/download/linux_&_macOS)
```git
$ sudo install -c gitfetch /usr/local/bin
check
$ gitfetch -h
    _____ _ _   ______   _       _ 
    / ____(_) | |  ____| | |     | |    
   | |  __ _| |_| |__ ___| |_ ___| |__  
   | | |_ | | __|  __/ _ \ __/ __| '_ \ 
   | |__| | | |_| | |  __/ || (__| | | |
    \_____|_|\__|_|  \___|\__\___|_| |_|
                                        
        
https://github.com/ArshErgon/gitfetch/
Just like `Neofetch` but for GitHub!

USAGE:
    gitfetch [FLAGS] [OPTIONS]

FLAGS:
    -a, --author     Show the information about the creator of gitfetch ex: gitfetch -a
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --temporally <temp>    Show an user info temporally, ex: ex: gitfetch -t USERNAME
    -u, --user <username>      saves the username, so that you don't have to put your username over again.
```
## Windows
```git

```


# Development

```bash
$ git clone https://github.com/USERNAME/gitfetch.git
$ cd gitfetch 
$ cargo run -- -t USERNAME
OR
$ cargo run -- -u USERNAME (this command will save your username)
```
to run your own or your friends github stats

```bash
$ cargo run -- -t USERNAME
```

# Plans
> I have a plan of making it more useful, by showing more information and using the graphQL api. There's still ton of work left.
 