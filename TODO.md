## NOTE

- docker cli (to send commands to the daemon).
- docker daemon (to manage the daemon).

## CONTAINER (x docker/ podman..)

- [ ] namespaces ( )
    group1(p1, p2, p3)
        ----
    group2(p5, p4)

- [ ] networking (... .sock)

- [x] cgroups (control groups)
    manage (memory max)(CPU, memory, network, I/O filesystem).
- [x] filesystem (... I/O) # as root (looks like it's required).


## TODO

- [ ] Test dak on a virtual machine.
- [ ] Test `dak run --image xxx` (by creating a new 'xxx' container with all related resources and specs).
- [ ] Mess around with `dak pull --image xxx`
