target: hello4

ncurses-lib-built: ncurses.rs
	rustc --lib $<
	touch $@

hello%: hello%.rs ncurses-lib-built
	rustc -L. $<
