# Bash / Shell

# Basics

Error Handling

```bash
function die() {
    echo "ERROR $1"
    exit 1
}
```

## String Operations

Trim trailing '/' in `X` ->  `${X%/}`

# Background Jobs

Pipe stdout & stderr into a file and launch in the background:

```bash
some_command > the_log_file 2>&1 &
```

Filename with the current date:

```bash
some_command > log-$(date +%Y-%m-%d).log 2>&1 &
```

# See Also

[WSL - Windows Subsystem for Linux](WSL%20-%20Windows%20Subsystem%20for%20Linux%202a0b3dbec82842db90368fb1b6456503.md)