# rtodo

A CLI todo app that helps me learn rust. You can read about it [here](https://brdv.mataroa.blog/)!

## What

rtodo is a basic CLI-based todo app that lets you keep track of your todos. Run the subcommand `rtodo help` to get all available options.

## How

It stores all todos in a file system structure. This means a directory is created in the home directory (`~/.rtodo`). That directory has two sub directories, `/todo` and `/done`. These directories have their respective todos listed as markdown files. This approach allows to extend the app on basic file system functionality.

## v0.1.0 Roadmap

The first version will contain:

- [x] base app structure
- [x] rtodo initialization
      this sets up the directory structure.
- [x] list todos
      with arguments for `all` and `done` (default is undone todos)
- [x] add todo
- [ ] mark todo as done
