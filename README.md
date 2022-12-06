# DOCKER-CLI
A cli that will execute docker compose command on server via [OpenSSH](https://man.openbsd.org/ssh.1).

### Requirements
[OpenSSH](https://man.openbsd.org/ssh.1) \
Linux user with root privilege (if you have to run sudo in order run docker)

<sub>Testing on Windows to Ubuntu.</sub>

## Installation
### Remote
```bash
cargo install --git https://github.com/momozahara/docker-cli.git
```
### Local
```bash
cargo install --path .
```

## Command
### Profile
Create user profile that contained username, hostname, path inside of it.
```bash
compose env \
--username <USERNAME> \ # require
--hostname <HOSTNAME> \ # require
--path <PATH> \ # require
--profile <PROFILE> # require
```
<sup>$HOME/pcode-cli/docker/{profile}.env</sup>
### Up
Up docker-compose.yaml at specific path.
```bash
compose up \
# conflicts with profile
--username <USERNAME> \ # require
--hostname <HOSTNAME> \ # require
--path <PATH> \ # require

--profile <PROFILE> # optional
```
### Down
Down docker-compose.yaml at specific path.
```bash
compose down \
# conflicts with profile
--username <USERNAME> \ # require
--hostname <HOSTNAME> \ # require
--path <PATH> \ # require

--profile <PROFILE> # optional
--rmi <RMI> # optional [local, all]
```
### Start
Start docker-compose.yml at specific path.
```bash
compose start \
# conflicts with profile
--username <USERNAME> \ # require
--hostname <HOSTNAME> \ # require
--path <PATH> \ # require

--profile <PROFILE> # optional
```
### Stop
Stop docker-compose.yml at specific path.
```bash
compose stop \
# conflicts with profile
--username <USERNAME> \ # require
--hostname <HOSTNAME> \ # require
--path <PATH> \ # require

--profile <PROFILE> # optional
```

## ROADMAP
<sup>if i am really going to maintain it.</sup>
* other compose command \
  <sup>i can actually do this one right now but...</sup>