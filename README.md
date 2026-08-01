# SafeExec 🛡️

> A lightweight, low-level Linux container runtime and sandbox engine written in Rust.

**SafeExec** executes untrusted binaries safely by leveraging core Linux kernel primitives—specifically **Namespaces** for visibility isolation and **cgroups v2** for physical resource constraints. It acts as an observable, lightweight execution engine designed to study how production container runtimes isolate code at the system level.

An integrated **Execution Theater** provides a real-time, human-readable narrative of every system event, state transition, and resource decision as it unfolds, making kernel-level sandboxing tangible for evaluators and learners.

---

## Key Features

* **Visibility Isolation (Namespaces):** Isolates PID, Mount, UTS, and Network spaces using low-level system calls.
* **Resource Controls (cgroups v2):** Enforces hard limits on RAM usage, CPU quotas, and thread counts (fork-bomb protection).
* **Air-Gapped Networking:** Spawns an isolated Network Namespace (`NetNS`) with loopback-only access to prevent unauthorized network calls or data exfiltration.
* **VFS Workspace Isolation:** Allocates temporary directory workspaces with bind-mounted input/output paths that clean up automatically upon termination.
* **Execution Theater:** A live, human-centric observability layer that renders every kernel decision, namespace transition, and resource event into a coherent, timestamped narrative stream.
* **Real-Time Telemetry Pipeline:** Streams live operational events, state transitions, resource utilization metrics, and stdout/stderr in a timestamped terminal interface.

---

## Architecture Overview

```text
                                  USER / CLI INPUT
                                         │
                                         ▼
                         ┌───────────────────────────────┐
                         │          SafeExec CLI         │
                         └───────────────┬───────────────┘
                                         │
                                         ▼
                         ┌───────────────────────────────┐
                         │     RUNTIME ORCHESTRATOR      │
                         └───────────────┬───────────────┘
                                         │
    ┌───────────────────┬────────────────┼───────────────────┬───────────────────┐
    │                   │                │                   │                   │
    ▼                   ▼                ▼                   ▼                   ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│  NAMESPACE    │ │    CGROUP     │ │     MOUNT     │ │     NETNS     │ │   TELEMETRY   │
│  CONTROLLER   │ │    MANAGER    │ │    MANAGER    │ │   ISOLATOR    │ │   PIPELINE    │
├───────────────┤ ├───────────────┤ ├───────────────┤ ├───────────────┤ ├───────────────┤
│• PID NS       │ │• cgroups v2   │ │• Mount NS     ││• Network NS   │ │• Real-time    │
│• UTS NS       │ │• memory.max   │ │• Temp Workspace││• Loopback-only│ │  broadcaster  │
│• USER NS      │ │• pids.max     │ │• Bind Mount   │ │  (Air-gapped) │ │• Event bus    │
│               │ │• CPU quota    │ │  (In/Out)     │ │               │ │• Metrics pump │
└───────┬───────┘ └───────┬───────┘ └───────┬───────┘ └───────┬───────┘ └───────┬───────┘
        │                 │                 │                 │                 │
        └─────────────────┴────────┬────────┴─────────────────┴─────────────────┘
                                   │
                                   ▼
                         ┌───────────────────────────────┐
                         │        TASK LAUNCHER          │
                         │    (clone / unshare / exec)   │
                         └───────────────┬───────────────┘
                                         │
                                         ▼
                         ┌───────────────────────────────┐
                         │      OBSERVABILITY UI         │
                         │   (Terminal Telemetry Stream) │
                         └───────────────┬───────────────┘
                                         │
                                         ▼
                         ┌───────────────────────────────┐
                         │      EXECUTION THEATER        │
                         │  (Human-Centric Narrative &   │
                         │   Live System Visualization)  │
                         └───────────────────────────────┘
```

---

## Component Breakdown

| Component | Linux Primitive / Subsystem | Responsibility |
| --- | --- | --- |
| **Namespace Controller** | `CLONE_NEWPID`, `CLONE_NEWUTS`, `CLONE_NEWUSER` | Hides host processes, isolates system hostnames, and maps local container root to an unprivileged user on the host. |
| **cgroup Manager** | `cgroups v2` (`/sys/fs/cgroup`) | Configures `memory.max`, `cpu.max`, and `pids.max` limits to prevent resource exhaustion or fork-bomb attacks. |
| **Mount Manager** | `CLONE_NEWNS`, VFS Bind Mounts | Sets up an isolated workspace under `/tmp/safeexec_*`, mounting input paths as read-only and output paths as read-write. |
| **NetNS Isolator** | `CLONE_NEWNET` | Completely air-gaps the container environment by initializing an unconfigured, isolated network interface stack. |
| **Telemetry Pipeline** | Rust Async Event Bus | Streams real-time operational logs, process state transitions, and live resource utilization metrics directly to the terminal. |
| **Execution Theater** | Structured Event Narrative + Visual Stage | Transforms raw kernel events into a coherent, human-readable story of system execution for evaluators. |

---

## Execution Theater

The **Execution Theater** is SafeExec's human-centric observability layer. While the Telemetry Pipeline handles raw metrics and structured events, the Theater interprets those events into a **live narrative** that answers the evaluator's implicit questions:

- *What did the kernel just do?*
- *Why was that resource limit enforced?*
- *Where is the process in its lifecycle?*
- *What would have happened without isolation?*

### Theater Stages

The Execution Theater operates across four conceptual stages, each mapping to a phase of container lifecycle:

| Stage | Kernel Activity | Theater Narrative |
|-------|----------------|-------------------|
| **Prologue** | Namespace creation, cgroup setup, VFS allocation | "The stage is being prepared: PID namespace born, memory cage set to 64MB, workspace mounted." |
| **Act I: Init** | `clone(2)` returns child PID, UID/GID maps written, `pivot_root(2)` executed | "The actor enters the stage. Root filesystem pivoted. The host is no longer visible." |
| **Act II: Execution** | `execve()` dispatched, process runs, cgroups enforce limits | "The binary speaks. CPU quota throttles burst. Memory pressure rises. The cage holds." |
| **Epilogue** | `waitpid()` returns, OOM or timeout detected, cleanup | "Curtain falls. Exit code 0. Workspace unmounted. No trace remains." |

### Theater Rendering Modes

```text
# Mode: NARRATIVE (default)
[18:21:02.102] 🎭 [PROLOGUE]  The SafeExec stage opens for session sf-89a1...
[18:21:02.105] 📁 [VFS]       A temporary world is carved at /tmp/safeexec_sf-89a1
[18:21:02.108] 🛡️ [CGROUP]    The memory cage is locked: 64MB maximum. Fork-bomb fuse: 20 threads.
[18:21:02.112] 🔒 [NS_INIT]   Four veils descend — PID, Mount, UTS, User — the host disappears.
[18:21:02.115] 🌐 [NETNS]     The network stage is stripped bare. Only loopback echoes remain.
[18:21:02.118] ⚡ [ACT I]     The actor steps through the clone gate. PID 1 in a universe of one.
─────────────────────────────────────────────────────────────────────────────────────────────
[18:21:02.120] 🎬 [ACT II]    The script begins: ./target_program
[STDOUT]       Calculating prime numbers...
[STDOUT]       Memory allocation test running...
[18:21:02.350] 📊 [METRICS]   RAM: 14.2 MB / 64 MB | CPU: 12% | Tasks: 1
[18:21:02.400] 🧠 [THEATER]   Memory pressure is nominal. The cage has 49.8 MB of headroom.
[STDOUT]       Calculation completed successfully.
─────────────────────────────────────────────────────────────────────────────────────────────
[18:21:02.501] ✅ [EPILOGUE]  The actor exits with grace. Status Code 0.
[18:21:02.505] 🧹 [CLEANUP]   The temporary world dissolves. Cgroup slice erased.
[18:21:02.508] ⏱️ [THEATER]   Performance duration: 386ms | Peak RAM: 14.2 MB | Isolation: intact.
```

```text
# Mode: TECHNICAL (evaluator debug view)
[18:21:02.112] 🔒 [NS_INIT]   clone(2) flags=0x7c020000 (NEWUSER|NEWPID|NEWNS|NEWUTS)
[18:21:02.115] 🌐 [NETNS]     unshare(CLONE_NEWNET) → lo: flags set to IFF_UP
[18:21:02.118] ⚡ [TASK]      pivot_root(/tmp/safeexec_sf-89a1, /tmp/safeexec_sf-89a1/.old_root)
[18:21:02.350] 📊 [METRICS]   memory.current=14811136 cpu.stat=usage_usec=45000
```

### Why Execution Theater Matters

In production container runtimes, observability is typically machine-first: JSON logs, Prometheus metrics, tracing spans. SafeExec inverts this for its educational mission. The Theater is **human-first**: it narrates kernel behavior in terms of intent and consequence, not just state and value.

This serves two audiences:
- **The Evaluator** (PBL context): Can trace every design decision to its kernel-level effect without reading source code.
- **The Learner**: Sees *why* a namespace was created, not just *that* it was created.

---

## Real-Time Telemetry Stream

When executing a task through SafeExec, the runtime streams lifecycle events live:

```text
[18:21:02.102] 🚀 [ORCH]     Initializing SafeExec Engine (ID: sf-89a1)...
[18:21:02.105] 📁 [VFS]      Allocated workspace: /tmp/safeexec_sf-89a1
[18:21:02.108] 🛡️ [CGROUP]   Configured cgroups v2 -> memory.max: 64MB | pids.max: 20
[18:21:02.112] 🔒 [NS_INIT]  Created PID, MNT, UTS namespaces via clone(2)
[18:21:02.115] 🌐 [NETNS]    Air-gapped network namespace initialized (lo interface)
[18:21:02.118] ⚡ [TASK]     Executing binary task: ./target_program
─────────────────────────────────────────────────────────────────────────────────────────────
[STDOUT]       Calculating prime numbers...
[STDOUT]       Memory allocation test running...
[18:21:02.350] 📊 [METRICS]  RAM: 14.2 MB / 64 MB | CPU: 12% | Tasks: 1
[STDOUT]       Calculation completed successfully.
─────────────────────────────────────────────────────────────────────────────────────────────
[18:21:02.501] ✅ [TASK_EXIT] Process exited with Status Code 0
[18:21:02.505] 🧹 [CLEANUP]   Unmounted VFS layers and deleted cgroup slice
[18:21:02.508] ⏱️ [TELEM]     Duration: 386ms | Peak RAM: 14.2 MB
```

---

## Security Boundary & Trade-offs

SafeExec is designed as an educational exploration of operating-system-level virtualization. It intentionally prioritizes structural visibility and simplicity:

* **Shared Kernel:** SafeExec shares the host Linux kernel. While namespaces and cgroups prevent standard resource abuse and visibility leaks, kernel-level vulnerabilities are outside the isolation scope.
* **Air-Gapping vs. NAT:** To eliminate complex host bridge configurations and `iptables` rules, network isolation is enforced by omitting external network interfaces entirely.
* **User-Space vs. MicroVMs:** SafeExec uses native Linux kernel features directly. Workloads requiring hardware-level virtualization boundaries require microVM runtimes like Firecracker.
* **Execution Theater vs. Formal Verification:** The Theater narrates observed behavior. It does not prove isolation correctness—only makes it comprehensible.

---

## Getting Started

### Prerequisites

* Operating System: Linux (Kernel 5.8+ recommended with cgroups v2 enabled)
* Compiler: Rust (1.70+)
* Privileges: Root or `CAP_SYS_ADMIN` capabilities (required for `unshare`/`clone` namespace operations)

### Building & Running

```bash
# Clone the repository
git clone https://github.com/your-username/SafeExec.git
cd SafeExec

# Build the release binary
cargo build --release

# Run an untrusted binary with 64MB RAM limit and 2s timeout
sudo ./target/release/safeexec \
  --exec ./samples/test_binary \
  --max-memory 64MB \
  --max-pids 20 \
  --timeout 2s

# Run with Execution Theater narrative mode (default)
sudo ./target/release/safeexec \
  --exec ./samples/test_binary \
  --max-memory 64MB \
  --theater-mode narrative

# Run with technical evaluator view
sudo ./target/release/safeexec \
  --exec ./samples/test_binary \
  --max-memory 64MB \
  --theater-mode technical
```

---

## Technology Stack

* **Language:** Rust 🦀
* **System Interface:** `nix` / `libc` crates for native Linux syscalls
* **Target OS:** Linux 🐧
* **Kernel Primitives:** Linux Namespaces (`CLONE_NEW*`), Control Groups (`cgroups v2`)
* **Observability:** Dual-layer — structured Telemetry Pipeline + human-centric Execution Theater
