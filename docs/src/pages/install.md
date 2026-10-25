{% extends "layouts/index.html" %}
{% block title %}Install{% endblock title %}

{% block header %}
# Install

[&ShortLeftArrow; Home](/)
{% endblock header %}

{% block content %}
See the [releases page](https://github.com/aleksrutins/ionian/releases") for all downloads.

| Method | Command |
| --- | --- |
| Mise (UBI) | `mise use -g github:aleksrutins/ionian` |
| Homebrew | `brew install aleksrutins/tap/ionian` |
| Shell (Linux/macOS) | `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aleksrutins/ionian/releases/download/latest/ionian-installer.sh \| sh` |
| PowerShell | `powershell -ExecutionPolicy Bypass -c "irm https://github.com/aleksrutins/ionian/releases/download/v0.2.1/ionian-installer.ps1 \| iex"` |
{% endblock content %}
