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
1. Install [docker](https://docs.docker.com/get-docker/)
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

