# CriTableTools, a partial Rust rewrite of SonicAudioTools

Tools and libs to modify CRIWARE file formats, rewritten in Rust.

These tools and libs are supposed to do its job at extracting/creating/patching the CriTable containers the best it can. If you are looking for the sound (ADX/DSP) convertion tools, you should find it somewhere else.

The Midday version of the programs can be found on [GitHub Actions](https://github.com/OSA413/CriTableTools/actions).

More or less suitable for use versions are planned to be published on GitHub's release page.

------------

Please note that most of the CLI tools support the `--help` flag to show help.

------------

## Differences from the original SonicAudioTools

* Only CSB/CPK files are supported, if you are looking for ACB/AWB file support, try using this: https://github.com/seiunx-dev/cridecoder (I haven't tested it but it looks promising).
* It contains only CLI tools, not GUI tools.
* Decompression of files is not supported.
* Ahead-of-time values loading is implemented instead of lazy-loading (I mean the file is read fully in one iteration instead of jumping on the stream in the memory between the pointers and values).
* For the simplification purposes, only the "latest" version of CPK files is supported (so far I haven't seen games with other versions of CPK).
* Added support for DSP/ADPCM_WII audio files.
* Pure Rust implementation.

## List of tools

### cri-table2json

A tool for a convenient examination of arbitrary CriTable files (CSB, CPK). You can use [the hexpat for ImHex](./imhex-patterns/cri-table.hexpat) alongside.

### csb-rs (wip)

A tool to fully extract, re-create, and patch CSB files (supports related CPK files)

## Tested games

| Game | Platform(s) | Table Read | Structure Read | Extraction | Re-creation | Patching | Notes |
| ---- | ----------- | ---------- | -------------- | ---------- | ----------- | -------- | ----- |
| Sonic and the Black Knight | Wii | ✅ | ✅ | ✅ | ✅¹ | ✅ | ¹ |
| Sonic Free Riders | XBOX 360 | ✅ | ✅ (CPK WIP) | ✅ | ❓ | ❓ | Not tested |
| Sonic 4 Episode 1 | PC | ✅ | ⏳ | ⏳² | ❌ | ❌ | ², WIP |
| Sonic 4 Episode 2 | PC | ✅ | ⏳ | ⏳² | ❌ | ❌ | ², WIP |
| Sonic Generations | PC | ✅ | ⏳ | ⏳² | ❌ | ❌ | ², WIP |
| Sonic Lost World | PC | ✅(CPK table read) | ⏳ | ❌ | ❌ | ❌ | Low-tier WIP |
| Sonic Forces | PC | ✅(CPK table read) | ⏳ | ❌ | ❌ | ❌ | Low-tier WIP |

* "Table Read" means that the files can be read with `cri-table2json` and its underlying libraries such as `cri-table`.
* "Structure Read" means that the files can be read/extracted with a higher-level abstraction tools and libs such as `csb-rs` and `cri-rs-lib`.

¹ - Please note that this game **heavily** expects the file to be almost exactly byte-to-byte correct, so that's why you should try patching the CSB file first instead of trying to recreate it from the extracted directory.

² - Last time it properly worked in [v0.1.0](https://github.com/OSA413/CriTableTools/releases/tag/v0.1.0).

*Your game is not in the list? Try creating an issue with game's name and platform and I'll think about it.*

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
