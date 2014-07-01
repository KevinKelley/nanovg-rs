lib_path=lib
bin_path=bin
doc_path=doc

nanovg_url	= https://github.com/memononen/nanovg.git
glfw_url	= https://github.com/bjz/glfw-rs.git
gl_url  	= https://github.com/bjz/gl-rs.git

nanovg_path		= lib/nanovg
nanovg_lib_path	= lib/nanovg/build
glfw_path 		= lib/glfw-rs
glfw_lib_path 	= lib/glfw-rs/lib
gl_path 		= lib/gl-rs
gl_lib_path 	= lib/gl-rs/lib

NANOVG_FLAGS = NANOVG_GL3_IMPLEMENTATION

libs = -L$(nanovg_lib_path) -L$(glfw_lib_path) -L$(gl_lib_path)

# to build examples:
build_cmd = rustc -Llib  $(libs) --opt-level 3 --out-dir $(bin_path)

EXAMPLE_FILES = examples/*.rs
SOURCE_FILES = $(shell test -e src/ && find src -type f)

# bindgen -builtins -o ../examples/demo.rs demo.c
# bindgen -builtins -o ../examples/perf.rs perf.c

all: lib examples

lib: $(SOURCE_FILES)
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
	git clone $(glfw_url) $(glfw_path)
	git clone $(gl_url) $(gl_path)

nanovg: 
	rm -rf $(nanovg_lib_path)
	cd $(nanovg_path); premake4 gmake; cd build; make CFLAGS=-D$(NANOVG_FLAGS) config=release nanovg
	echo "MUST ReWrap!"

deps: nanovg
	make lib -C $(glfw_path)
	make -C lib/gl-rs
	#make -C lib/nalgebra
	#make deps -C lib/ncollide
	#make 3df32 -C lib/ncollide
	#cd lib/rust-stb-image; ./configure
	#make clean -C lib/rust-stb-image
	#make -C lib/rust-stb-image
	#cd lib/rust-freetype; ./configure
	#make clean -C lib/rust-freetype
	#make -C lib/rust-freetype

## manually compile ncollide and rust-fmpeg as they cannot support cargo yet.
#deps_for_cargo:
#	make deps  -C lib/ncollide
#	make 3df32 -C lib/ncollide
#	cd lib/rust-stb-image; ./configure
#	make clean -C lib/rust-stb-image
#	make -C lib/rust-stb-image
#	cd lib/rust-ffmpeg; ./build.sh
#	cp lib/ncollide/lib/* target/deps/.
#	cp lib/rust-ffmpeg/lib/* target/deps/.
#	cp lib/rust-stb-image/libstb* target/deps/.

#cargo:
#	cargo build

#distcheck:
#	rm -rf $(tmp)
#	git clone --recursive . $(tmp)
#	make -C $(tmp) cargo
#	rm -rf $(tmp)
#	git clone --recursive . $(tmp)
#	make -C $(tmp) deps
#	make -C $(tmp)
#	make -C $(tmp) examples
#	rm -rf $(tmp)

.PHONY:nanovg
.PHONY:deps
.PHONY:doc
.PHONY:examples




#MT = -f rust-empty.mk
#
#EXAMPLE_FILES 
#
#deps: glfw-rs
#
#glfw-rs: 
#	cd deps/glfw-rs && make link && make -f rust-empty.mk
#
#examples: $(EXAMPLE_FILES)
#	make $(MT) examples
#
#$(EXAMPLE_FILES): lib examples-dir
#	$(Q)$(COMPILER) --target "$(TARGET)" $(COMPILER_FLAGS) $@ -L "$(TARGET_LIB_DIR)" -L "target" --out-dir examples/ \
#	&& echo "--- Built '$@' (make $@)"
#
#
#lib:
#	make $(MT) lib
#
#exe: 
#	make $(MT) exe

