# DOCKER-CLI
A cli that will execute docker compose command on server via [OpenSSH](https://man.openbsd.org/ssh.1).

### requirements
[OpenSSH](https://man.openbsd.org/ssh.1) with properly id_rsa setup. \
Linux user with root privilege (if you have to run sudo in order run docker)

<sub>Testing on Windows to Ubuntu.</sub>

## Command
### env
create .env that store username hostname docker_path inside of it.
```bash
compose env \
--username <USERNAME> \
--hostname <HOSTNAME> \
--path <PATH> # optional
```
<sup>$HOME/pcode-cli/docker/.env</sup>
### start
start docker-compose.yml at specific path.
```bash
compose run \
--username <USERNAME> \ # optional if set in env
--hostname <HOSTNAME> \ # optional if set in env
--path <PATH> # optional if set in env
```
### start
stop docker-compose.yml at specific path.
```bash
compose stop \
--username <USERNAME> \ # optional if set in env
--hostname <HOSTNAME> \ # optional if set in env
--path <PATH> # optional if set in env
```

## ROADMAP
<sup>if i am really going to maintain it.</sup>

* env profile support
* other compose command \
  <sup>i can actually do this one right now but...</sup>