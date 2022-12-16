FROM golang:1.19-alpine

WORKDIR client

COPY . ./

# download dependency libraries
RUN go mod download

# compile, expose the port in use, and run
RUN go build -o /docker-client

EXPOSE 8001

CMD [ "/docker-client" ]