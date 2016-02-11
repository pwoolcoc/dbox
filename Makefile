SOURCE := $(shell ls ./src/*.rs)

tags: $(SOURCE)
	ctags -f tags --options=$(HOME)/code/rust/src/etc/ctags.rust --recurse ./src/

