target: hello3 hello4 hello5

ncurses-lib-built: ncurses.rs
	rustc --lib $<
	touch $@

hello%: hello%.rs ncurses-lib-built
	rustc -L. $<
