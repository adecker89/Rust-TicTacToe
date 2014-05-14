all: bar
bar: bar.o libmnk-game.a
	gcc bar.o libmnk-game.a -o  bar

bar.o: bar.c
	gcc -g -O2 -Wall -c bar.c

libmnk-game.a: mnk-game.rs main.rs board.rs ai.rs
	rustc mnk-game.rs --crate-type="staticlib" -o libmnk-game.a

clean:
	rm -rf *.a *.o bar
