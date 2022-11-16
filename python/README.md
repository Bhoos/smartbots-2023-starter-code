## Basic Server setup


1. Create a virtual environment 

```
python3 -m venv env
``` 

and activate using 

```
source env/bin/activate
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
