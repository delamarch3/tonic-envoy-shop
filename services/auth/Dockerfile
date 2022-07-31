FROM golang:1.16-alpine

WORKDIR /app

COPY server ./server
COPY go.mod ./
COPY go.sum ./
RUN go mod download

COPY *.go ./

RUN go build -o /auth

CMD ["/auth"]
