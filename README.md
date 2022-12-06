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
### up
up docker-compose.yaml at specific path.
```bash
compose up \
--username <USERNAME> \ # optional if set in env
--hostname <HOSTNAME> \ # optional if set in env
--path <PATH> # optional if set in env
```
### down
down docker-compose.yaml at specific path.
```bash
compose down \
--username <USERNAME> \ # optional if set in env
--hostname <HOSTNAME> \ # optional if set in env
--path <PATH> # optional if set in env
--rmi <RMI> # optional [local, all]
```
### start
start docker-compose.yml at specific path.
```bash
compose start \
--username <USERNAME> \ # optional if set in env
--hostname <HOSTNAME> \ # optional if set in env
--path <PATH> # optional if set in env
```
### stop
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