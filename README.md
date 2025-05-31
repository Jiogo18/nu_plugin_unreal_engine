# nu_plugin_unreal_engine

This is a [Nushell](https://nushell.sh/) plugin called "unreal_engine".
It is meant to be used with Unreal Engine 5.

## Installing

```nushell
> cargo install --path . --locked
```

## Usage

```nushell
> plugin add $"~/.cargo/bin/nu_plugin_unreal_engine(if $nu.os-info.family == windows { '.exe' })"
> plugin use unreal_engine
> # cd to an Unreal Engine project (directory containing a .uproject file)
> ue start --log
> open Content/ThirdPerson/Blueprints/BP_ThirdPersonCharacter.uasset
```

## Features

- [x] `ue start ...` wrapper for UnrealEditor-Cmd (UE5.5)
    - [x] Start the editor
    - [x] Start a game
    - [x] Start a server 
- [x] `from uasset` export uasset header with [uasset-rs](https://github.com/jorgenpt/uasset-rs) ([fork for UE5.5](https://github.com/thibaultleouay/uasset-rs))
    - [ ] List assets redirectors
    - [ ] List level assets and their actors (in case of Level Streaming)
- [ ] Integration with git-lfs
    - [ ] Checkout by ignoring large files that are locked by Unreal Engine
    - [ ] List changed actors of Level Streaming
- [ ] Watch build status and emit events
    - [ ] AppData/Local/UnrealBuildTool/Log.txt
- [x] `ue build <command>` wrapper for UnrealBuildTool execution (auto-detect UE path)
- [x] `ue RunUAT <command>` wrapper for RunUAT execution (auto-detect UE path)
