run: main
	./main < input.txt

test: main
	./main < test.txt

main: main.rs
	rustc main.rs

clean:
	rm -f main
