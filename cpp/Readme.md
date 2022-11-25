# Starter Code for Bhoos's Smart Bots 2023 

Still wondering why the language that runs the world is missing from this grand competition? <br>
Now's probably the right time to stop wondering, (>.<) <br>

## Installation

Requires a compatible c++ compiler. One that supports c++20 is a plus. <br><br>
This code uses single header libraries HTTPServer and JSON parser (Credit to their writers for these high quality libraries). <br>
They need to be downloaded from their respective github repository as explained below. They were removed from direction inclusion due to their quite large size.<br>

If `curl` is installed, simply run <br>

### Windows
`setup.bat` <br> 

### Linux
```
chmod +x ./setup.sh
setup.sh 
```

In case, `curl`ing isn't the way you want to do, simply download both headers : <br><br>
https://github.com/yhirose/cpp-httplib/blob/master/httplib.h <br>
https://github.com/nlohmann/json/blob/develop/single_include/nlohmann/json.hpp <br> <br>
and place both header files inside folder 'include' inside this directory

The structure of `cpp` folder will then be like this  : 

```
include\
src\
setup.bat
setup.sh
Readme.md 
```

## Build Instructions

From inside this directory,<br>
compile both files together and simply run the executable.<br> 

### Windows 

```
g++/clang++ ./src/main.cpp ./src/bot.cpp -ofast -std=c++17 -march=native -lws2_32 -o game
game.exe
```
### Linux 
```
g++/clang++ ./src/main.cpp ./src/bot.cpp -ofast -std=c++17 -march=native -pthread -o game
./game
```

<br>Info : This isn't thorougly tested. Feel free to report issues, bugs or unintended behaviors. If you have trouble with docker submission or face performance regression, remove `-march-native` from the compiler flag inside dockerfile. 