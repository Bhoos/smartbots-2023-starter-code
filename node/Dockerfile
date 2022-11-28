# This file contains information on how you should structure your docker image for submission.

# Always use alpine version to minimize docker image size. If alpine version 
# is not available, use the smallest size base image available.
FROM node:14-alpine

# RUN commands are run when the docker image is built. 
# Prefer to use RUN commands to install packages, build packages 
# and stuff that needs to be done only once.
# e.g> 
# RUN apk add gcc

# This will be the base directory where our project will live. 
# The name can be anything but for now, let's name it client since 
# the bot is a single client in our game.
WORKDIR ./client

# ADD command adds the file or folder to the destination. 
# Since the working directory is `./client`, it copies the file inside `./client`.
ADD ./* ./

# EXPOSE opens up the port to communication outside the container.
# WE ASSUME THAT YOUR SERVER WILL RUN ON THIS PORT. 
# DO NOT CHANGE THIS.
EXPOSE 8001

# CMD runs the specified command on docker image startup.
# Note that we are inside the working directory `./client` so, 
# `node node.js` is run inside the `./client` directory.
CMD ["node", "node.js"]