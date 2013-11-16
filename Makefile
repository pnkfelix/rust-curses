target: hello3 hello4 hello5

ncurses-lib-core-built: ncurses_core.rs
	rustc --lib $<
	touch $@

ncurses-lib-built: ncurses.rs ncurses_core.rs ncurses-lib-core-built
	rustc --lib $<
	touch $@

hello%: hello%.rs ncurses-lib-built
	rustc -L. $<
