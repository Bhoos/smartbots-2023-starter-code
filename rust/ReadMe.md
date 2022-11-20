# Smartbots 2023 Starter Code in Rust

This is a basic bot made with  [js-common](https://github.com/Bhoos/smartbots-2023-starter-code/tree/master/js-common) taken as a reference. This should be taken as a boilerplate on top of which you should implement your logic. 



### Preresequites:

1. Rust And Docker Installed In The System
2. Good Code Editor Like VSCode With Rust-Analyzer And A Debugger Installed
3. Plenty Of Time And Patience Because Rust Is Hard 


### Running The Bot

    cargo run 

  

### Docker

To Build Docker Image

    docker build . -t smart_bot_docker 

To Run The Image

    docker run --rm -p 8001:8001  smart_bot_docker


Note: First build/run might take long because of dependency installation and compilation. Subsequent builds/runs will be faster.