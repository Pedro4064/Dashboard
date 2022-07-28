# Dashboard

## Project Overview
Dashboard is a TUI tool written in RUST to easily launch and maintain processes on *Nix based systems.
It allows you to launch Cron Jobs, add systemd services and attach the output of services running on the aforementioned systems plus docker.

## Development
We can use a docker container running Ubuntu to test the necessary system calls even when developing on different systems. To do so you just need to run the following command while on the root directory of the project:
```bash
docker run -it --name rust_dashboard --mount type=bind,source="$(pwd)",target=/rust_dashboard ubuntu /bin/bash
```

After starting the container we need to update and upgrade all the packages to then install the necessary dependencies:
```bash
apt update && apt upgrade
```

```bash
apt install systemd && apt install systemctl && apt install cron
```

Now we need enable the cron daemon:
```bash 
systemctl enable cron
```

And the last step is to download the necessary rust tooling
```bash
apt install curl && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And now we can, inside the docker container, build and test our project on a Linux distro (e.g if you are on MacOS or Windows )
## Installation
