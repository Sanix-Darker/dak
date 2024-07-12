# DAK

A controversial dumb docker alternative (dak).

## REQUIREMENTS

- rust (1.76).
- make (optional).

Create manually your cgroup before starting...

```bash
$ cd /sys/fs/cgroup/
$ sudo mkdir dak.cgroup
$ sudo chown -R dk ./dak.cgroup/
````

## RESOURCES

- Overlayfs (https://jvns.ca/blog/2019/11/18/how-containers-work--overlayfs/)
```txt
>> code representation.
...lower...

FROM ubuntu:latest
RUN ...
WORKDIR app

...upper...

-------- schematic representation.
upper
.....
.........
...
....lower...
images(ubuntu/debian/node)
```
