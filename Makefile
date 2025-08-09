NAME=spider

all: build

build:
	cargo build --release
	cp target/release/$(NAME) ./$(NAME)

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

.PHONY: all build clean fclean