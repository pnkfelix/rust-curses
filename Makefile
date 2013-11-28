RUSTC_FLAGS=-Z debug-info

target: ncurses-intro-test ncurses-intro
#target: ncurses-intro-test ncurses-intro hello3 hello4 hello5

clean:
	rm -f ncurses-lib*-built
	rm -f hello{3,4,5}
	rm ncurses-intro{,-test}
	rm libncurses_core-*.dylib
	rm -r *.dSYM
	rm libncurses-*.dylib

ncurses-lib-core-built: ncurses_core_lib.rs
	rustc $(RUSTC_FLAGS) --lib $<
	touch $@

ncurses-lib-built: ncurses.rs ncurses_core.rs ncurses-lib-core-built
	rustc --lib $<
	touch $@

hello%: hello%.rs ncurses-lib-built
	rustc $(RUSTC_FLAGS) -L. --link-args -lncurses $<

ncurses-intro: ncurses-intro.rs ncurses-lib-built locale.rs signal_h.rs
	rustc $(RUSTC_FLAGS) -L. $< -o $@

ncurses-intro-test: ncurses-intro.rs ncurses-lib-built locale.rs signal_h.rs
	rustc $(RUSTC_FLAGS) --test -L. $< -o $@
