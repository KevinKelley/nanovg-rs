CC ?= gcc

NANOVG_REPO = https://github.com/memononen/nanovg
NANOVG_DIR = $(DEPS_DIR)/nanovg
LIBNANOVG = $(DEPS_DIR)/libnanovg.a

ORIG_SHIM_DIR = $(CURDIR)/shim
SHIM_DIR = $(DEPS_DIR)/shim
LIBNANOVG_SHIM = $(DEPS_DIR)/libnanovg_shim.a

$(LIBNANOVG_SHIM): $(LIBNANOVG) 
	mkdir -p "$(DEPS_DIR)"
	cd "$(DEPS_DIR)" && ([ -d "shim" ] || cp -r "$(ORIG_SHIM_DIR)" .)
	cd "$(SHIM_DIR)" && premake4 "--nanovg-out=$(DEPS_DIR)" "--nanovg-root=$(NANOVG_DIR)" gmake
	cd "$(SHIM_DIR)/build" && make config=release verbose=1
	cp "$(SHIM_DIR)/build/libnanovg_shim.a" "$(DEPS_DIR)"

$(LIBNANOVG):
	mkdir -p "$(DEPS_DIR)"
	#cd "$(DEPS_DIR)" && ([[ -d "nanovg" ]] && (cd nanovg; git pull) || git clone https://github.com/KevinKelley/nanovg)
	cd "$(DEPS_DIR)" && ([ -d "nanovg" ] && (cd nanovg; git pull) || git clone $(NANOVG_REPO))
	cd "$(NANOVG_DIR)" && premake4 gmake
	cd "$(NANOVG_DIR)/build" && make config=release verbose=1 nanovg
	cp "$(NANOVG_DIR)/build/libnanovg.a" "$(DEPS_DIR)"


