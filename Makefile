ccr: src/main.rs
	rustc -o sacme src/main.rs

test: sacme
	./test.sh

clean:
	rm -f sacme tmp*

.PHONY: clean test