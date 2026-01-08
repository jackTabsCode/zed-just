set unstable := true

# Simple attribute
[private]
hidden-recipe:
    echo "This recipe won't show in --list"

# Attribute with parentheses (string argument)
[group('dev')]
test-parens:
    echo "Testing parentheses syntax"

# Attribute with colon syntax
[working-directory('/tmp')]
test-colon:
    echo "Testing colon syntax - working in /tmp"

[doc('Build the project')]
[group('build')]
build:
    echo "Building..."

# Confirmation attributes
[confirm]
dangerous-task:
    echo "This requires confirmation"

[confirm('Are you absolutely sure?')]
very-dangerous:
    echo "Custom confirmation prompt"

# Platform-specific attributes
[linux]
linux-only:
    echo "Only runs on Linux"

[macos]
macos-only:
    echo "Only runs on macOS"

[windows]
windows-only:
    echo "Only runs on Windows"

[unix]
unix-only:
    echo "Runs on Unix-like systems (Linux, macOS, BSD)"

[openbsd]
openbsd-only:
    echo "Only runs on OpenBSD"

# Working directory control
[no-cd]
stay-here:
    echo "Don't change directory for this recipe"

# Error message control
[no-exit-message]
quiet-failure:
    false

# Script execution (shebang recipes run as scripts automatically)
script-recipe:
    #!/usr/bin/env bash
    echo "Running as a script"

[script('python3')]
python-recipe:
    print("Hello from Python!")

# Other attributes
[no-quiet]
always-verbose:
    echo "This ignores global quiet setting"

[positional-arguments]
with-args *args:
    echo "Args: $@"

[parallel]
parallel-deps: dep1 dep2
    echo "Dependencies run in parallel"

dep1:
    echo "Dependency 1"

dep2:
    echo "Dependency 2"

[extension('py')]
python-shebang:
    #!/usr/bin/env python3
    print("File will have .py extension")

[metadata('author: test')]
with-metadata:
    echo "Has custom metadata"

[default]
default-recipe:
    @just --list

# Combined attributes on one line
[no-cd]
[private]
combined:
    echo "Multiple attributes on one line"

[arg('n', help='HELP')]
[arg('n', pattern='\d+')]
double n:
    echo $(({{n}} * 2))
