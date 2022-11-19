## Basic Server setup

<small> Note: Make sure you're in the python subdirectory while executing these commands. </small>

1. Create a virtual environment

```
python -m venv env
```

and activate using

```
# for macos

source env/bin/activate


# for windows

.\env\Scripts\activate
```

2. Install required packages

```
pip install -r requirements.txt
```

3. Open the terminal and run the sanic server

```
python app.py
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
