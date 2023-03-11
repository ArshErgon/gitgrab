<p align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/Made%20With-Rust-000000?style=for-the-badge" alt="made with rust" /></a></a>
</p>
<div align="center">

# **GitFetch**

</div>

***Get for GitHub Information on your terminal***
> We all know `neofetch` an awesome tool to see your OS details on your terminal, so why not a website details which we developers use alot? Thats why I made `GitFetch` to showcase one's github profile in the terminal.

[img](https://user-images.githubusercontent.com/40994679/224485236-7f3c4207-3ac0-4bf7-8711-7f10718c1f36.webm)


# Commands
### Basic
```git
$ gitfetch -o
## this command will give you options for 
# 1. Creating a permanent user (so that next time just typing gitfetch will show you your information)
# 2. Enter your API key/ Update (the github API key)
# 3. Exit
# here what it will look like
---------------
$ gitfetch -o
  1. Create a User
  2. Enter/Update the Github API key
  3. Exit

```
### For a temporary User

```git
## There will be times when your friend need to see his/her github profile, but you dont want them to be a permanent user, for that you can use
$ gitfetch -t <USERNAME>
# where in <USERNAME> will be their github's username
```
### More commands
```git
$ gitfetch -a
## Shows the information about the creater(@ArshErgon)
$ gitfetch -h
## Shows all commands and how to use them.
```


For running it on Linux

```git
$ sudo install -c gitfetch /usr/local/bin
check


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
# Plans
>
> I have a plan of making it more useful, by showing more information and using the graphQL api. There's still ton of work left.
