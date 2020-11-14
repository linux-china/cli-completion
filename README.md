CLI completion
==============

CLI completion for bash, zsh, fish and powershell.

# How to install?

```
$ cargo install cli-completion
```

# How to use?

Please refer https://github.com/clap-rs/clap#using-yaml to write YAML file for the cli.

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
$ cli-completion --bash xxx.yaml >  ~/.oh-my-zsh/custom/plugins/xxx/_xxx 
```

Don't forget to enable xxx plugin in ~/.zshrc file！

### powershell completion

```
$ cli-completion --powershll xxx.yaml > xxx-completion.ps
```

# References

* clap-rs: https://github.com/clap-rs/clap
