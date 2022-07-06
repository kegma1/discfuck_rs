# discfuck_rs
discord bot for running brainfuck code


# How to run
## From source
### Prerequisite
- [git](https://git-scm.com/)
- [rust](https://www.rust-lang.org/)
- Basic rust knowlage and basic terminal use in your preferred OS

### Steps
1. Make a discord bot and invite it on your server. Follow [this](https://discordpy.readthedocs.io/en/stable/discord.html) tutorial if you dont know how. 
2. Clone the repository to your local machine. Open the terminal and enter the following command.\
`git clone https://github.com/kegma1/discfuck_rs.git`
3. Cd into the new directory.
4. Rename **env_sample** to **.env** and paste your bots token in the file.
5. Compile and run the bot. Enter the following commands in the terminal.\
`cargo build --release` \
`cargo run`
## From executable
WIP
## From Docker
WIP

# TODO
- [x] Make it work
- [ ] Add multi-therding
- [ ] Make UI look better
- [ ] Add debug fetures
- [ ] Add config file
- [ ] Add docker support
- [ ] Add comments to the code