# This file contains information on how you should structure your docker image for submission.

# Always use alpine version to minimize docker image size. If alpine version 
# is not available, use the smallest base image available.
FROM python:3.11-alpine

# This will be the base directory where our project will live. 
# The name can be anything but for now, let's name it client since 
# the bot is a single client in our game.
WORKDIR client

# ADD command adds the file or folder to the destination. 
# Since the working directory is `./client`, it copies the file inside `./client`.
# Copy requirements
COPY requirements.txt requirements.txt
COPY ./src .

# RUN commands are run when the docker image is built. 
# Prefer to use RUN commands to install packages, build packages 
# and stuff that needs to be done only once.
# RUN command runs the command in command line
# build-base adds gcc and other tools required to build sanic
RUN apk add build-base
RUN pip3 install -r requirements.txt

# EXPOSE opens up the port to communication outside the container.
# WE ASSUME THAT YOUR SERVER WILL RUN ON THIS PORT. 
# DO NOT CHANGE THIS.
EXPOSE 8001

# CMD runs the specified command on docker image startup.
# Note that we are inside the working directory `./client` so, 
# `python app.py` is run inside the `./client` directory.
CMD [ "python3", "app.py" ]