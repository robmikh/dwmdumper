# dwmdumper
A utility to take a memory dump of DWM.exe using a hotkey.

## Instructions

1. Install the Microsoft Visual C++ Redistributable: https://aka.ms/vs/17/release/vc_redist.x64.exe
2. Go here https://github.com/robmikh/dwmdumper/releases and download `dwmdumper_x86_64-pc-windows-msvc_release.zip` from the latest release available.
3. Unpack that in some temp directory.
4. Open a console window **running as admin** and run dwmdumper.exe.
5. Repro the condition.
6. Hit SHIFT+CTRL+D, a dump of the DWM will be collected and in the same directory as the tool.
