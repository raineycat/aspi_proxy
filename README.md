A DLL proxy for Casio's fxASPI.dll that logs all function calls, parameters and results.
This should implement everything needed to debug communication stuff.

Make sure to build this for the `i686-pc-windows-xxx` targets, since all Casio software I've seen is 32-bit only. GNU and MSVC are both fine.
