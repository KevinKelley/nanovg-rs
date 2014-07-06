lib_path=lib
bin_path=bin
doc_path=doc

nanovg_url	= https://github.com/KevinKelley/nanovg
glfw_url	= https://github.com/bjz/glfw-rs.git
gl_url  	= https://github.com/bjz/gl-rs.git

nanovg_path		= lib/nanovg
nanovg_lib_path	= lib/nanovg/build
glfw_path 		= lib/glfw-rs
glfw_lib_path 	= lib/glfw-rs/lib
gl_path 		= lib/gl-rs
gl_lib_path 	= lib/gl-rs/lib
libnanovg.a		= $(nanovg_lib_path)/libnanovg.a

NANOVG_FLAGS = -DNANOVG_GL3_IMPLEMENTATION

libs = -L$(nanovg_lib_path) -L$(glfw_lib_path) -L$(gl_lib_path)

# to build examples:
build_cmd = rustc -Llib  $(libs) --opt-level 3 --out-dir $(bin_path)

EXAMPLE_FILES = examples/*.rs
SOURCE_FILES = $(shell test -e src/ && find src -type f)
NANOVG_FILES = $(shell test -e lib/nanovg/src && find lib/nanovg/src -type f)
NANOVG_FILES+= $(shell test -e lib/nanovg/example && find lib/nanovg/example -type f)

# bindgen -builtins -o ../examples/demo.rs demo.c
# bindgen -builtins -o ../examples/perf.rs perf.c

all: lib examples

run: lib examples
	cd bin; ./example_gl3

lib: $(libnanovg.a)
	mkdir -p $(lib_path)
	rustc src/nanovg.rs --opt-level 3 --out-dir $(lib_path) $(libs)

examples: lib  $(EXAMPLE_FILES)
	mkdir -p $(bin_path)
	$(build_cmd) ./examples/example_gl3.rs

doc:
	mkdir -p $(doc_path)
	rustdoc $(libs) src/lib.rs

get-deps:
	mkdir -p $(lib_path)
	git clone $(nanovg_url) $(nanovg_path)
	git clone $(glfw_url)   $(glfw_path)
	git clone $(gl_url)     $(gl_path)

$(libnanovg.a): $(NANOVG_FILES)
	rm -rf $(nanovg_lib_path)
	cd $(nanovg_path); premake4 gmake; cd build; make CFLAGS=$(NANOVG_FLAGS) config=release verbose=1 nanovg

deps:
	make lib -C lib/gl-rs
	make lib -C $(glfw_path)

clean:
	rm $(libnanovg.a)
	rm $(lib_path)/*.rlib

cleaner:
	rm -rf $(lib_path)

.PHONY:      \
	run      \
	doc      \
	get-deps \
	deps     \
	clean    \
	cleaner  \
	doc
