NANOVG_REPO = https://github.com/memononen/nanovg
NANOVG_DIR = $(OUT_DIR)/nanovg
LIBNANOVG = $(OUT_DIR)/libnanovg.a

ORIG_SHIM_DIR = $(PWD)/shim
SHIM_DIR = $(OUT_DIR)/shim
LIBNANOVG_SHIM = $(OUT_DIR)/libnanovg_shim.a

$(LIBNANOVG_SHIM): $(LIBNANOVG)
	mkdir -p "$(OUT_DIR)"
	cd "$(OUT_DIR)" && ([ -d "shim" ] || cp -r "$(ORIG_SHIM_DIR)" .)
	cd "$(SHIM_DIR)" && premake4 "--nanovg-out=$(OUT_DIR)" "--nanovg-root=$(NANOVG_DIR)" gmake
	cd "$(SHIM_DIR)/build" && CFLAGS='-fPIC' make config=release verbose=1
	cp "$(SHIM_DIR)/build/libnanovg_shim.a" "$(OUT_DIR)"

$(LIBNANOVG):
	mkdir -p "$(OUT_DIR)"
	#cd "$(OUT_DIR)" && ([[ -d "nanovg" ]] && (cd nanovg; git pull) || git clone https://github.com/KevinKelley/nanovg)
	cd "$(OUT_DIR)" && ([ -d "nanovg" ] && (cd nanovg; git pull) || git clone $(NANOVG_REPO))
	cd "$(NANOVG_DIR)" && premake4 gmake
	cd "$(NANOVG_DIR)/build" && CFLAGS='-fPIC' make config=release verbose=1 nanovg
	cp "$(NANOVG_DIR)/build/libnanovg.a" "$(OUT_DIR)"
