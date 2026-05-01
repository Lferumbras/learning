FROM golang:1.22-alpine AS builder

WORKDIR /app

COPY go.mod ./
RUN go mod download

COPY . .
RUN go build -o /bin/app .

FROM alpine:3.20

WORKDIR /app
COPY --from=builder /bin/app /app/app

EXPOSE 8080
CMD ["/app/app"]
