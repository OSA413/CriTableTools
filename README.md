# CriTableTools, a partial Rust rewrite of SonicAudioTools

A set of tools to modify CRIWARE file formats.

Currently it only supports CSB files (that can be bundled with a CPK file).

The latest version of the programs can be found on [GitHub Actions](https://github.com/OSA413/CriTableTools/actions).

More or less suitable for use versions are planned to be published on GitHub's release page.

------------

Please note that most of the CLI tools support the `--help` flag to show help.

------------

## What is left behind / will not be implemented in the Rust rewrite (as for now)

* Only CSB/CPK files are supported
* the compression of files, since in the original code the reading was only implemented, but I want to write the files as well.
* parallel processing of files since it would still hit the IO bottleneck (original project has a parallel extractor of the files).
* file alignment since it wasn't implemented in the original code.
* also the masking seems to be unused (always `false`)
* ahead-of-time values loading was implemented instead of lazy-loading (I mean the file is read fully in one iteration instead of jumping on the stream in the memory between the pointers and values)
* Only "latest" version of CSB and CPK and FileName mode are supported

## List of tools

### cri-table2json

A tool for a convenient examination of arbitrary CriTable files (CSB, CPK...)

### csb-editor-rs (wip)

A tool to extract audio files from a CSB file (supports related CPK files)

## Tested games

* Sonic and the Black Knight (Wii) (read, extraction with metadata, creation from the extracted CSB, *need more research*)
* Sonic 4 Episode 1 (PC) (read, extraction (**temporarily broken in latest release, use v0.1.0**))
* Sonic 4 Episode 2 (PC) (read, extraction (**temporarily broken in latest release, use v0.1.0**))
* Sonic Generations (PC) (read, extraction (**temporarily broken in latest release, use v0.1.0**))
* Sonic Lost World (PC) (CPK read)
* Sonic Forces (PC) (CPK read)

## Want to contribute?

Contributions welcome to the Rust rewrite.

You can also create [issues](https://github.com/OSA413/CriTableTools/issues) if you find that something doesn't work properly.

If you want to contribute to the original C# project, please follow the original project's repository.

## Special thanks

Special thanks to [Skyth](https://github.com/blueskythlikesclouds) for the [original C# tools](https://github.com/blueskythlikesclouds/SonicAudioTools) and for licensing them under the MIT license.

## AI disclosure and usage policy and contribution guide (for the rewritten project)

The word "slop" is not what I want to achieve with this project, so I prepared an [AI usage policy and contribution guide](./AI_USAGE_POLICY_AND_CONTRIBUTION_GUIDE.md).

Currently, AI assistance from SourceCraft Code Assistant helped in a few ways:

* Helped debugging the rewritten library and tools in the cases that I overlooked due to inattention.
