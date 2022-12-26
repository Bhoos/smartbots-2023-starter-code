## How to submit docker image

<br />

### Install docker

You can install docker by following the instructions provided on the [Docker website](https://docs.docker.com/get-docker/). If you are using Windows, follow the instructions for installing Docker on Windows provided below.

You can also install it by running it

```
brew install --cask docker
```

if you are using the brew package manager.

You may need to provide privileged access to docker.

---

### Installing Docker on Windows

1. Install Windows Subsystem for Linux (WSL) from Microsoft Store. We recommend [Ubuntu](https://apps.microsoft.com/store/detail/ubuntu/9PDXGNCFSCZV). 

2. Download and install the Linux kernal update package from [Microsoft](https://learn.microsoft.com/en-us/windows/wsl/install-manual#step-4---download-the-linux-kernel-update-package).

3. Install Docker Desktop from Docker [website](https://www.docker.com/products/docker-desktop/).

<br />

### Build container image

Open the terminal in this directory, ie. `node` and run the following command

```
docker build --platform x86_64 -t <TAG> .
```

`<TAG>` is the name of your image. This builds the container image. The `platform` flag specifies that the architecture of the docker image will be `x86_64`, which is the recommended architecture.

<br />

### Save the image

```
docker save <TAG> | gzip > <TAG>.tar.gz
```

`docker save <TAG>` creates a tar archive of your docker image. By piping its output to `gzip` command, we create a `gzip` compressed archive. This is the file you should upload using the smartbots website.

<br />

### Run the container locally

```
docker run -p 8001:8001 -m=2048m --cpus=1 -it <TAG or IMAGE ID>
```

<br />

### Remove the image

To delete your existing docker image, get the `IMAGE ID` from `docker images`, and run

```
docker rmi <IMAGE ID>
```

## Example

Make sure docker is installed. Open Docker Desktop on your system.

Open the terminal in this directory, i.e. `smartbots-2023-starter-code/node`, and run the following command

```
docker build --platform x86_64 -t joker .
```

In your docker desktop dashboard, you will see a new entry in the Images tab, named `joker`. This means your docker build was successful.

Now run the following command

```
docker save joker | gzip > joker.tar.gz
```

A new file will be created inside your current directory named `joker.tar.gz`. Now you can upload this file.

Good luck!
