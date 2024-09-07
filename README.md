# Nb-cli

This is a cli application for the Notebrook API.

Notebrook is a notetaking app which sets it apart from others due to its ease of us, the fact that it's self hosted, and uses a very simple API. Notebrook is made by [Ghorthalon](https://github.com/Ghorthalon?tab=repositories) and is still unreleased.

This is still very much a Work in Progress, so expect changes.

## Commands

- login [url] [token]: Logs you in and saves the URL and token into settings.json
- lsc: Get a list of your channels
- send: takes an optional channel name `-c`or `--channel` or goes to last channel by default and adds it.
- read(wip): reads a channel and outputs to stdout.
