

## Docker File Submission

1. Download the Docker GUI [here](https://docs.docker.com/get-docker/).

2. Open the terminal and run the following command to build a docker image

```
docker build -t <TAG> .

# for m1 mac
docker build --platform x86_64 -t <TAG> .
```

once executed, the docker image is visible with

```
docker images
```

3. To export your docker image for submission, run

<small> Note: Make sure gzip is intalled before proceeding. </small>

```
docker save <TAG> | gzip > <TAG>.tar.gz
```

4. To run the built container locally, run

```
docker run -p 8001:8001 -m=2048m --cpus=1 -it <TAG or IMAGE ID>
```

#### To delete your existing docker image, get the `IMAGE ID` from `docker images`, and run

```
docker rmi <IMAGE ID>
```
