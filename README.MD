Watches a specific folder for .torrent files and moves them to a specific folder.

When a torrent file is found it looks up the hostname in environment variables and if found moves the file to given folder.

If the hostname is not found in environment variables it will move it to the dead letter directory.

Example .env file:

```
WATCH_DIR=./testingDirectory/watch
DEAD_LETTER_DIR=./testingDirectory/dead
TORRENT_UBUNTU_COM=./testingDirectory/ubuntu
```

Prebuilt docker image available:
https://hub.docker.com/r/jdon278/torrent-router
