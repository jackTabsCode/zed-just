[default]
[arg('check', long, value='true')]
all check='false': (rust check) (ts check)

[arg('check', long, value='true')]
rust check='false':
    cargo fmt --all {{if check == 'true' { '--check' } else { '' }}}

[arg('check', long, value='true')]
ts check='false':
    pnpm exec oxfmt {{if check == 'true' { '--check' } else { '' }}}
