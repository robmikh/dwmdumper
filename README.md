# dwmdumper
A utility to take a memory dump of DWM.exe using a hotkey.

## Instructions

1. Go to the [releases section](https://github.com/robmikh/dwmdumper/releases) and download `dwmdumper_x86_64-pc-windows-msvc_release.zip` from the latest release available.
2. Unpack that in some temp directory.
3. Open a console window **running as admin** and run dwmdumper.exe.
4. Repro the condition.
5. Hit SHIFT+CTRL+D, a dump of the DWM will be collected and in the same directory as the tool.
