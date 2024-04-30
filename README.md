# foo-foil

Clone the repo and:

```sh
cargo run
```

Other terminal:

```sh
./get-hf.sh
cd hyperfoil-0.23
./bin/cli.sh
```

* start-local
* upload ../hf.yml
* run
* stats

```sh
[hyperfoil]$ start-local
Starting controller in default directory (/tmp/hyperfoil)
Controller started, listening on 127.0.0.1:43093
Connecting to the controller...
Connected to 127.0.0.1:43093!
[hyperfoil@in-vm]$ upload ../hf.yml
Loaded benchmark foo-foil, uploading...
... done.
[hyperfoil@in-vm]$ run
Started run 0000
Run 0000, benchmark foo-foil
Agents: in-vm[STARTING]
Agents: in-vm[REGISTERED]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[READY]
Agents: in-vm[STOPPED]
Started: 2024/04/30 08:30:49.848    Terminated: 2024/04/30 08:30:59.854
NAME  STATUS        STARTED                      REMAINING
      COMPLETED     TOTAL DURATION               DESCRIPTION
------------------------------------------------------------------------
main  TERMINATED    08:30:49.849
      08:30:59.854  10005 ms (exceeded by 5 ms)  50.00 users per second
------------------------------------------------------------------------
[hyperfoil@in-vm]$
[hyperfoil@in-vm]$ stats
Total stats from run 0000
PHASE  METRIC  THROUGHPUT   REQUESTS  MEAN     p50      p90      p99      p99.9    p99.99   TIMEOUTS  ERRORS
               BLOCKED      2xx       3xx      4xx      5xx      CACHE
--------------------------------------------------------------------------------------------------------------------
main   hello   49.68 req/s       497  1.28 ms  1.30 ms  1.72 ms  2.13 ms  3.78 ms  3.78 ms         0       0
                      0 ns       497        0        0        0        0
--------------------------------------------------------------------------------------------------------------------
```
