# This file contains information on how you should structure your docker image for submission.
FROM python:3.11

# This will be the base directory where our project will live. 
# The name can be anything but for now, let's name it client since 
# the bot is a single client in our game.
WORKDIR ./client

# COPY copies files from a source location to a destination in the Docker container.
# Since the working directory is `./client`, it copies the file inside `./client`.
COPY . /client/

# RUN commands are run when the docker image is built. 
# Prefer to use RUN commands to install packages, build packages 
# and stuff that needs to be done only once.
# e.g> 
# RUN apk add gcc
# Install requirements using frozen requirements file (pip).
RUN pip install --no-cache-dir -r requirements.txt

# EXPOSE opens up the port to communication outside the container.
# WE ASSUME THAT YOUR SERVER WILL RUN ON THIS PORT. 
# DO NOT CHANGE THIS.
EXPOSE 8001

# CMD runs the specified command on docker image startup.
# Note that we are inside the working directory `./client` so, 
# `python app.py` is run inside the `./client` directory.
CMD [ "python", "app.py" ]
