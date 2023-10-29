build:
	@go build -o bin/main
run: build
	./bin/dotman
