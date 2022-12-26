# Smartbots starter code for Go
A stupid starter bot that always bids 0, reveals and plays trump cards whenever possible, and throws the first valid card (card of the current hand's suite if available) it finds from the player's deck. Not really secure but suffices for submission.

## Requirements
Built in go 1.19, but should run on previous go versions (>=1.1)
No need to install third party libraries.

## Running
```sh
go run .
```
## Running on Docker
## Install Docker
You can install docker from [here](https://docs.docker.com/get-docker/). If you are using Windows, see below.

###  Docker on Windows

1. Install Windows Subsystem for Linux (WSL) from Microsoft Store. We recommend [Ubuntu](https://apps.microsoft.com/store/detail/ubuntu/9PDXGNCFSCZV). 

2. Download and install the Linux kernal update package from [Microsoft](https://learn.microsoft.com/en-us/windows/wsl/install-manual#step-4---download-the-linux-kernel-update-package).

3. Install Docker Desktop from Docker [website](https://www.docker.com/products/docker-desktop/).

## Using Docker

2. Build the image
    ```sh
    docker build -t <TAG> .
    # or on m1 mac machines:
    docker build --platform x86_64 -t <TAG> .
    ```
3. Run the container
    ```sh
    docker run -p 8001:8001 <TAG>
    ```

## Saving the docker image
Once you've built the image, run:
```
docker save -o <FILE_NAME> <TAG>
```

