# SafeExec 🛡️

> A lightweight, low-level Linux container runtime and sandbox engine written in Rust.

**SafeExec** executes untrusted binaries safely by leveraging core Linux kernel primitives—specifically **Namespaces** for visibility isolation and **cgroups v2** for physical resource constraints. It acts as an observable, lightweight execution engine designed to study how production container runtimes isolate code at the system level.

---

## Key Features

* **Visibility Isolation (Namespaces):** Isolates PID, Mount, UTS, and Network spaces using low-level system calls.
* **Resource Controls (cgroups v2):** Enforces hard limits on RAM usage, CPU quotas, and thread counts (fork-bomb protection).
* **Air-Gapped Networking:** Spawns an isolated Network Namespace (`NetNS`) with loopback-only access to prevent unauthorized network calls or data exfiltration.
* **VFS Workspace Isolation:** Allocates temporary directory workspaces with bind-mounted input/output paths that clean up automatically upon termination.
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
│• PID NS       │ │• cgroups v2   │ │• Mount NS     │ │• Network NS   │ │• Real-time    │
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

```

---

## Technology Stack

* **Language:** Rust 🦀
* **System Interface:** `nix` / `libc` crates for native Linux syscalls
* **Target OS:** Linux 🐧
* **Kernel Primitives:** Linux Namespaces (`CLONE_NEW*`), Control Groups (`cgroups v2`)
