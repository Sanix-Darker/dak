# DAK

A controversial dumb alternative to `docker run` (dak).

## REQUIREMENTS

- rust (1.76).
- make (optional).

Create manually your cgroup before starting...

```bash
$ cd /sys/fs/cgroup/
$ sudo mkdir dak.cgroup
$ sudo chown -R dk ./dak.cgroup/
````
