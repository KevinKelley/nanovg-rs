NANOVG_FLAGS = -DNANOVG_GL3_IMPLEMENTATION
LIBNANOVG = "$(DEPS_DIR)/libnanovg.a"

nanovg: $(LIBNANOVG)

$(LIBNANOVG):
	mkdir -p "$(DEPS_DIR)"
	cd "$(DEPS_DIR)" && ([[ -d "nanovg" ]] && (cd nanovg; git pull) || git clone https://github.com/KevinKelley/nanovg)
	cd "$(DEPS_DIR)/nanovg" && premake4 gmake
	cd "$(DEPS_DIR)/nanovg/build" && make CFLAGS=$(NANOVG_FLAGS) config=release verbose=1 nanovg
	cp "$(DEPS_DIR)/nanovg/build/libnanovg.a" "$(DEPS_DIR)"


