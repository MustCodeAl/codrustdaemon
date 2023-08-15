# codrustdaemon
cod rust daemon remake



Cod checks each command you run in the shell. When cod detects usage of --help flag it asks if you want it to learn this command. If you choose to allow cod to learn this command cod will run command itself parse the output and generate completions based on the --help output.

How cod detects help commands
Cod performs following checks to decide if command is help invocation:

checks if the --help flag is used
checks that command is simple i.e. doesn't contain any pipes, file descriptor redirections, and other shell magic
checks that command exit code is 0.
If cod cannot automatically detect that your command is help invocation you can use learn subcommand to learn this command anyway.

How cod runs help commands
Cod always uses absolute paths to run programs. (So it finds the binary in $PATH or resolves relative path if required). Arguments other than the binary path are left unchanged.

The current shell environment and current working directory will be used.

If the program is successfully executed, cod will store: - the absolute path to binary - any used arguments - the working directory - environment variables This info will be used to update command if required (check: cod help update).

How cod parses help output
cod has generic parser that works with most help pages and recognizes flags (starting with -), while not recognizing subcommands.

It also has a special parser tuned for the python argparse library that recognizes flags and subcommands.

Configuration
Cod will search for the default config file $XDG_CONFIG_HOME/cod/config.toml.

The config file allows you to specify rules to either ignore or trust specified binaries

cod example-config prints an example configuration to stdout.

cod example-config --create writes an example config to the default directory of said config file ($XDG_CONFIG_HOME/cod/config.toml)

Data directories
cod uses $XDG_DATA_HOME/cod (default: ~/.local/share/cod) to store all generated data files.