# rtodo

A CLI todo app that helps me learn rust.

## What

rtodo is a basic CLI-based todo app that lets you keep track of your todos. Run the subcommand `rtodo help` to get all available options.

## How

It stores all todos in a file system structure. This means a folder is created in the home folder (`~/.rtodo`). That folder has two sub folders, `/todo` and `/done`. These folders have their respective todos listed as markdown files. This approach allows to extend the app on basic file system functionality.

## v0.1.0 Roadmap

The first version will contain:

- [ ] base app structure
- [ ] rtodo initialization
      This sets up the folder structure.
- [ ] list todos
      With arguments for `all` and `done` (default is undone todos)
- [ ] add todo
- [ ] mark todo as done
