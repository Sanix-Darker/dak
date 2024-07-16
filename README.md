# DAK

<img width="100%" src="https://github.com/user-attachments/assets/54d62f84-7e30-4944-8b38-4e4f8a41216f" />

A controversial dumb docker alternative (dak) for learning purposes.

## FEATURES

- [ ] Create a container and "start it" (in progress).
    ```bash
    $ dak run --image xyz
    ```

- [ ] Pull a container, (comming soon).
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
- linux errors (https://gist.github.com/greggyNapalm/2413028)

## AUTHOR

- [dk](https://github.com/sanix-darker/dak.git)
