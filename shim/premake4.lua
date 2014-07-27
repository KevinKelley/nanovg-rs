local action = _ACTION or ""


solution "nanovg_shim"
	location ( "build" )
	configurations { "Debug", "Release" }
	platforms {"native", "x64", "x32"}

    newoption {
        trigger = "nanovg-root",
        value = "path",
        description = "path to nanovg source root"
    }

    newoption {
        trigger = "nanovg-out",
        value = "path",
        description = "path to nanovg compile output"
    }

    if not _OPTIONS["nanovg-root"] then
        _OPTIONS["nanovg-root"] = "nanovg"
    end

    if not _OPTIONS["nanovg-out"] then
        _OPTIONS["nanovg-out"] = "out"
    end

   	project "nanovg_shim"
		language "C"
		kind "StaticLib"
		includedirs { "src", _OPTIONS["nanovg-root"].."/src", _OPTIONS["nanovg-root"].."/example" }
        libdirs { _OPTIONS["nanovg-out"] }
		files { "src/*.c" }
        links { "nanovg" }
		targetdir "build"

		configuration "Debug"
			defines { "DEBUG" }
			flags { "Symbols", "ExtraWarnings"}

		configuration "Release"
			defines { "NDEBUG" }
			flags { "Optimize", "ExtraWarnings"}

