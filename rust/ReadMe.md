# Smartbots 2023 Starter Code in Rust

This is an improved starter-code in Rust, everything is made again from scratch taking reference of old version.
This should be taken as a boilerplate on top of which you should implement your logic. 


## Setting Up
### Preresequites:

1. Rust And Docker Installed In The System
2. Good Code Editor Like VSCode With Rust-Analyzer And A Debugger Installed
3. Plenty Of Time And Patience Because Rust Is Hard 


### Running The Bot
```
    cargo run 
```
  

### Docker

To Build Docker Image
```
    docker build . -t smart_bot_docker 
```
To Run the Docker Image
```
    docker run --rm -p 8001:8001  smart_bot_docker
```
To Save the Docker Image
```
    docker save smart_bot_docker | gzip > smart_bot_docker.tar.gz
```
## Getting on with bot creation:

The code is divided into following parts:

`main.rs` : 
It has basic implementation to handle get and post request. You don't need to modify this file.

`payload_types.rs` : It has all the struct definition used to deserialize the json file sent into this file. 
You don't need to modify this file either. You may need to take reference to this file to see what data you are getting.

`action_types.rs` : It has all the types of actions that can be taken in game. You don't need to modify this file.
You can use this file to see how you can create any action to perform in the game.

`cards.rs` : It has type definitions for Card and Suit, it's formatting and some basic implementations.
You can use this file to see how you can manipulate cards to perform calculations in game.

`bid_and_trump.rs`: It has two functions to choose bid value and trump card. It's basic. You can improve upon it.

`play.rs`: It has single function to choose appropriate move. It's basic. You can improve upon it.


Note: First build/run might take long because of dependency installation and compilation. Subsequent builds/runs will be faster.