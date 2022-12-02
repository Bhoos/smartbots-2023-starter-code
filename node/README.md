## How to submit docker image
<br />

### Install docker

You can install docker by following the instructions provided in the [Docker website](https://docs.docker.com/get-docker/).

You can also install it by running

```
brew install --cask docker
```

if you are using brew package manager.

You may need to provide previliged access to docker.

<br />

### Build container image

Open terminal in this directory, ie. `node` and run the following command

```
docker build --platform x86_64 -t <TAG> .
```

`<TAG>` is the name of your image. This builds the container image. The `platform` flag specifies that the architecture of the  docker image will be `x86_64`, which is the recommended architecture.

<br />

### Save the image

```
docker save <TAG> | gzip > <TAG>.tar.gz
```

`docker save <TAG>` creates a tar archive of your docker image. By piping its output to `gzip` command, we create a `gzip` compressed archive. This is the file you should upload using the smartbots website.


### Remove the image

To delete your existing docker image, get the `IMAGE ID` from `docker images`, and run

```
docker rmi <IMAGE ID>
```


## Example

Make sure docker is installed. Open Docker Desktop on your system.

Open terminal in this directory, i.e. `smartbots-2023-starter-code/node`, and run the following command

```
docker build --platform x86_64 -t joker .
```

In your docker desktop dashboard, you will see a new entry in the Images tab, named `joker`. This means your docker build was successful.

Now run the following command

```
docker save joker | gzip > joker.tar.gz
```

A new file will be created inside of your current directory named `joker.tar.gz`. Now you can upload this file.

Good luck!