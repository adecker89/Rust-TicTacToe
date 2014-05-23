all: bar
bar: bar.o libmnk-game.a
	gcc bar.o libmnk-game.a -o  bar

bar.o: bar.c
	gcc -g -O2 -Wall -c bar.c

libmnk-game.a: lib_main.rs main.rs board.rs ai.rs minimax.rs
	rustc lib_main.rs --crate-type="staticlib" -o libmnk-game.a

rust: main.rs board.rs ai.rs minimax.rs
	rustc main.rs -o main

test: main.rs board.rs ai.rs minimax.rs
	rustc main.rs --test -o test

clean:
	rm -rf *.a *.o bar test main
