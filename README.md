CLI completion
==============
[![cli-completion Crate](https://img.shields.io/crates/v/cli-completion)](https://crates.io/crates/cli-completion)

CLI completion for bash, zsh, fish and powershell.

![CLI Completion](https://github.com/linux-china/cli-completion/blob/master/structure.png?raw=true)

# How to install?

```
$ cargo install cli-completion
```

# How to use?

Please refer https://github.com/clap-rs/clap#using-yaml to write YAML file for the cli.
Or you can refer [multipass.yaml](https://github.com/linux-china/cli-completion/blob/master/commands/multipass.yaml) as an example.

```
$ cli-completion --bash commands/multipass.yaml
```

### bash completion

```
$ cli-completion --bash xxx.yaml > xxx-completion.sh
```

### fish completion

```
$ cli-completion --bash xxx.yaml > xxx-completion.fish
```

### zsh completion

```
$ cli-completion --bash xxx.yaml > /usr/local/share/zsh/site-functions/_xxx
$ autoload -U compinit && compinit
```

### oh-my-zsh completion

```
$ mkdir ~/.oh-my-zsh/custom/plugins/xxx 
$ cli-completion --zsh xxx.yaml >  ~/.oh-my-zsh/custom/plugins/xxx/_xxx 
```

Don't forget to enable xxx plugin in ~/.zshrc file！

### powershell completion

```
$ cli-completion --powershell xxx.yaml > xxx-completion.ps
```

# References

* clap-rs: https://github.com/clap-rs/clap
* cli-completion crate: https://crates.io/crates/cli-completion
* Command Line Interface Guidelines: https://clig.dev/
