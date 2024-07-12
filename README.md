# DAK

A controversial dumb docker alternative (dak).

## FEATURES

- [ ] Create a container and "start it".
    ```bash
    $ dak run --image xyz
    ```

- [ ] Pull a container.
    ```bash
    $ dak pull --image xyz
    ```

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

- about Containers in general (https://www.docker.com/resources/what-container/).
- about Overlayfs (https://jvns.ca/blog/2019/11/18/how-containers-work--overlayfs/).

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

## AUTHOR

- [dk](https://github.com/sanix-darker/dak.git)
