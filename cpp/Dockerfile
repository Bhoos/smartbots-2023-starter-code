FROM alpine:latest

WORKDIR client

RUN mkdir ./src
RUN mkdir ./include 
ADD ./src/* ./src
ADD ./include/* ./include

RUN apk update 
RUN apk add g++
RUN apk add libstdc++

EXPOSE 8001

RUN g++ -Ofast ./src/main.cpp ./src/bot.cpp -march=native -std=c++20 -o game29

CMD ["./game29"]
