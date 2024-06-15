# Sceawian

Service for automatically mirroring git repositories to multiple forges.


## Configuration

Configuration for sceawian is done through a `config.toml`
file located next to the executable or in `pwd`,
or mounted in the docker image at `/usr/share/sceawian/config.toml`.

The format of `config.toml` is as follows:

```toml
# Interval between updating the mirrors as seconds
update_interval = 40
# Directory to search for repo configuration files
repos = "repos"
# Number of concurrent tasks to spawn for mirroring repositories
# This can be set higher but with increased risk of mirroring failing
# due to one of the forges having caps for concurrent connections
task_count = 4
```

If no `config.toml` is found a warning is logged and the default values will be used.

## Repository configuration

Sceawian needs you to create configuration files for each repository
you want to mirror inside of the directory specified in the `repos` key
in `config.toml`.

Each of these configuration files is a `toml` file with the following fields:

```toml
name = "<repo_name>"
source = "<source_repository_location>"
target = "<target_repository_location>"
```

## Using SSH in Docker

Using SSH for cloning and pushing the repositories around will require
you to mount your SSH keys along with a `known_hosts` file
containing the expected SSH keys of your Git forges.

These can be obtained with `ssh-keyscan <forge_address> >> <output_file>`

Additionally it can be the easiest option to set the environment variable
`GIT_SSH_COMMAND` to `ssh -i <ssh_private_key_path>`
when running the Docker container.
