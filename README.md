# nu_plugin_unreal_engine

This is a [Nushell](https://nushell.sh/) plugin called "unreal_engine".

## Installing

```nushell
> cargo install --path .
```

## Usage

FIXME: This reflects the demo functionality generated with the template. Update this documentation
once you have implemented the actual plugin functionality.

```nushell
> plugin add ~/.cargo/bin/nu_plugin_unreal_engine
> plugin use unreal_engine
> ue Ellie
Hello, Ellie. How are you today?
> ue --shout Ellie
HELLO, ELLIE. HOW ARE YOU TODAY?
```

## Features

- [ ] Wrapper for UnrealEditor-Cmd (UE5.5)
    - [ ] Start the editor
    - [ ] Start a game
    - [ ] Start a server 
- [ ] Export uasset header with [uasset-rs](https://github.com/jorgenpt/uasset-rs) ([fork for UE5.5](https://github.com/thibaultleouay/uasset-rs))
    - [ ] List assets redirectors
    - [ ] List level assets and their actors (in case of Level Streaming)
- [ ] Integration with git-lfs
    - [ ] Checkout by ignoring large files that are locked by Unreal Engine
    - [ ] List changed actors of Level Streaming
- [ ] Watch build status and emit events
    - [ ] AppData/Local/UnrealBuildTool/Log.txt
