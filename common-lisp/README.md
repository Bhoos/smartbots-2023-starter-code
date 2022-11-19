# 29Points Bot in Common Lisp

## Installation

Install a Common Lisp implementation. E.g. [SBCL](https://www.sbcl.org/)

```shell
apt install sbcl
```

Setup [quicklisp](https://www.quicklisp.org/beta/).

```shell
curl -O https://beta.quicklisp.org/quicklisp.lisp
sbcl --load quicklisp.lisp --eval "(quicklisp-quickstart:install)" --eval "(ql:add-to-init-file)"
```

Clone this repo to **~/common-lisp/** folder or your quicklisp projects directory.

## Run

Start sbcl, load this project and start the server.

```common-lisp
	(ql:quicklod :29points)
	(29points:start 8001)
```

Preferably you would use an IDE with interactive REPL during development. See [Emacs](https://www.gnu.org/software/emacs/), [VS Code](https://code.visualstudio.com/) or [Portacle](https://portacle.github.io/).

<br>

Now head to sandbox and try the api checkpoints. Happy coding ( •̀ .̫ •́ )✧
