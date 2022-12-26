## Basic Server setup

<small> Note: Make sure you're in the python subdirectory while executing these commands. </small>

<br>

1. Create a virtual environment

```
python -m venv env
```

and activate using

```
# for macos
source env/bin/activate
```

```
# for windows

.\env\Scripts\activate
```
On Windows, you may get `Running scripts is disabled on this system` error. You will need to follow these steps to solve this.

1. Right click on the start menu.
2. Click on Windows PowerShell(admin)
3. Type `set-executionpolicy remotesigned` on powershell and press enter
4. Press 'A' to select 'Yes to All'.
---

2. Install required packages

```
pip install -r requirements.txt
```

3. Open the terminal and run the sanic server

```
python src/app.py
```

You should see the following message in the terminal

```
Goin' Fast @ http://0.0.0.0:8001
```

4. Open your browser and enter `http://0.0.0.0:8001/hi` in the website section.
   You should get `{"value":"hello"}` as the response.

   In case, the response isn't visible, try `http://localhost:8001/hi`. You should see the above response here.

<br>

Now head to sandbox and try the api checkpoints. Happy coding ( •̀ .̫ •́ )✧

<br>
<br>


## Docker File Submission

### Docker Installation

Download the Docker Desktop from [here](https://docs.docker.com/get-docker/). If you are using Windows, see below.

####  Installing Docker on Windows
1. Install Windows Subsystem for Linux (WSL) from Microsoft Store. We recommend [Ubuntu](https://apps.microsoft.com/store/detail/ubuntu/9PDXGNCFSCZV). 

2.  Download and install the Linux kernal update package from [Microsoft](https://learn.microsoft.com/en-us/windows/wsl/install-manual#step-4---download-the-linux-kernel-update-package).

3.  Install Docker Desktop from Docker [website](https://www.docker.com/products/docker-desktop/).

---
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

