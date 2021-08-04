# Maximum Effort!

Focus is a simple tool to twiddle your /etc/host file in order to remap distracting domains.

## Installation

```sh
git clone git@github.com:stuarth/focus.git 
cd focus
cargo install --path . --force
```

When run, focus will look for a `~/.focus` file containing a list of domains, e.g.

```sh
foo.com
bar.biz
```