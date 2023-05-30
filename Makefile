NAME = rustoban
SRC = src/main.rs \
	  src/parser.rs \
	  src/game.rs \

all: $(NAME)

$(NAME): $(SRC)
	cargo build --release
	mv ./target/release/rustoban ./

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all