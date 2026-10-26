# ABOUTME: Task runner for the documentation site. `just site` builds it into site/;
# ABOUTME: `just serve` serves it locally with live reload.

site:
    cp CHANGELOG.md docs/changelog.md
    uvx --with mkdocs-material mkdocs build

serve:
    uvx --with mkdocs-material mkdocs serve
