#!/usr/bin/env python3
"""Benchmark every author's Stack of Plates crate and combine the results.

Runs are strictly sequential -- benchmarking in parallel would have the arms
competing for CPU and contaminate every measurement.

Usage:
    python bench_all.py                  # full run, real timings (slow)
    python bench_all.py --quick          # short timings, for a smoke test
    python bench_all.py --author Kevin   # one author (repeatable)
    python bench_all.py --fresh          # discard previous results first
    python bench_all.py --markdown --open
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
import webbrowser

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH_TARGET = "benchmark"  # the [[bench]] name every crate uses


# --------------------------------------------------------------------------- #
# discovery
# --------------------------------------------------------------------------- #
def find_authors(only: list[str] | None) -> list[tuple[str, str]]:
    """Return [(author, crate_dir)] for every author crate, alphabetically."""
    found = []
    for name in sorted(os.listdir(HERE)):
        crate = os.path.join(HERE, name)
        if not os.path.isdir(crate) or name.startswith((".", "_")):
            continue
        if not os.path.isfile(os.path.join(crate, "Cargo.toml")):
            continue
        found.append((name, crate))
    if only:
        wanted = {a.lower() for a in only}
        missing = wanted - {a.lower() for a, _ in found}
        for m in sorted(missing):
            print(f"warning: no crate found for author {m!r}", file=sys.stderr)
        found = [(a, c) for a, c in found if a.lower() in wanted]
    return found


def bench_dirs(criterion_home: str) -> set[str]:
    """Directories holding a completed benchmark, for partial-failure detection."""
    out = set()
    for dirpath, _dirnames, filenames in os.walk(criterion_home):
        if "estimates.json" in filenames and os.path.basename(dirpath) == "new":
            out.add(dirpath)
    return out


# --------------------------------------------------------------------------- #
# running
# --------------------------------------------------------------------------- #
def run_author(author: str, crate: str, criterion_home: str, quick: bool, timeout: int | None):
    """Run one author's benchmark. Returns (ok, detail)."""
    if not os.path.isfile(os.path.join(crate, "benches", f"{BENCH_TARGET}.rs")):
        return False, f"no benches/{BENCH_TARGET}.rs"

    cmd = [
        "cargo", "bench",
        "--manifest-path", os.path.join(crate, "Cargo.toml"),
        "--bench", BENCH_TARGET,
    ]
    if quick:
        cmd += ["--", "--measurement-time", "1", "--warm-up-time", "1", "--sample-size", "10"]

    env = dict(os.environ, CRITERION_HOME=criterion_home)
    started = time.time()
    try:
        proc = subprocess.run(
            cmd, cwd=crate, env=env, timeout=timeout,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        )
    except subprocess.TimeoutExpired:
        return False, f"timed out after {timeout}s"
    except FileNotFoundError:
        return False, "cargo not found on PATH"

    elapsed = time.time() - started
    text = proc.stdout.decode("utf-8", errors="replace")
    if proc.returncode == 0:
        return True, f"{elapsed:.1f}s"

    # Surface the most useful line rather than a wall of cargo output.
    detail = f"exit {proc.returncode}"
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("thread ") and "panicked" in s:
            detail = f"exit {proc.returncode}: panicked"
        elif s in ("not yet implemented",) or s.startswith("not yet implemented"):
            detail = f"exit {proc.returncode}: not yet implemented (todo!())"
            break
        elif s.startswith("error[") or (s.startswith("error:") and "bench failed" not in s):
            detail = f"exit {proc.returncode}: {s[:70]}"
            break
    return False, detail


# --------------------------------------------------------------------------- #
# reading results
# --------------------------------------------------------------------------- #
def collect(criterion_home: str, since: float):
    """Read every stored benchmark. Returns list of result dicts."""
    results = []
    for dirpath, _dirnames, filenames in os.walk(criterion_home):
        if os.path.basename(dirpath) != "new":
            continue
        if "estimates.json" not in filenames or "benchmark.json" not in filenames:
            continue
        try:
            with open(os.path.join(dirpath, "estimates.json"), encoding="utf-8") as fh:
                est = json.load(fh)
            with open(os.path.join(dirpath, "benchmark.json"), encoding="utf-8") as fh:
                meta = json.load(fh)
        except (OSError, json.JSONDecodeError):
            continue

        label = meta.get("function_id") or meta.get("full_id") or os.path.basename(dirpath)
        author, _, impl = label.partition(" - ")
        results.append({
            "author": author.strip(),
            "impl": impl.strip() or "-",
            "mean": est["mean"]["point_estimate"],
            "median": est["median"]["point_estimate"],
            "std_dev": est["std_dev"]["point_estimate"],
            "stale": os.path.getmtime(os.path.join(dirpath, "estimates.json")) < since,
        })
    results.sort(key=lambda r: r["mean"])
    return results


def fmt_time(ns: float) -> str:
    for unit, scale in (("ns", 1.0), ("us", 1e3), ("ms", 1e6), ("s", 1e9)):
        if ns < scale * 1000:
            return f"{ns / scale:.3g} {unit}"
    return f"{ns / 1e9:.3g} s"


def print_table(results, markdown: bool):
    if not results:
        print("\nNo results to report.")
        return

    fastest = results[0]["mean"]
    rows = [
        (
            r["author"],
            r["impl"],
            fmt_time(r["mean"]),
            fmt_time(r["median"]),
            f"+/-{fmt_time(r['std_dev'])}",
            "fastest" if r["mean"] == fastest else f"{r['mean'] / fastest:.2f}x",
            "*" if r["stale"] else "",
        )
        for r in results
    ]
    headers = ("Author", "Implementation", "Mean", "Median", "Std dev", "vs fastest", "")

    if markdown:
        print("\n| " + " | ".join(headers[:-1]) + " |")
        print("|" + "|".join("---" for _ in headers[:-1]) + "|")
        for row in rows:
            cells = list(row[:-1])
            if row[-1]:
                cells[0] += " *"
            print("| " + " | ".join(cells) + " |")
    else:
        widths = [max(len(str(r[i])) for r in (headers, *rows)) for i in range(len(headers))]
        print()
        print("  ".join(h.ljust(w) for h, w in zip(headers, widths)).rstrip())
        print("  ".join("-" * w for w in widths).rstrip())
        for row in rows:
            print("  ".join(str(c).ljust(w) for c, w in zip(row, widths)).rstrip())

    if any(r["stale"] for r in results):
        print("\n  * carried over from an earlier run, not measured just now"
              " (use --fresh to discard)")


# --------------------------------------------------------------------------- #
def main() -> int:
    ap = argparse.ArgumentParser(prog="bench_all", description=__doc__.splitlines()[0])
    ap.add_argument("--author", action="append", metavar="NAME",
                    help="only this author (repeatable)")
    ap.add_argument("--quick", action="store_true",
                    help="short timings: a smoke test, not a real measurement")
    ap.add_argument("--fresh", action="store_true",
                    help="discard previously stored results first")
    ap.add_argument("--markdown", action="store_true", help="emit the table as markdown")
    ap.add_argument("--open", dest="open_report", action="store_true",
                    help="open the combined HTML report when finished")
    ap.add_argument("--criterion-home", default=os.path.join(HERE, "target", "criterion"),
                    help="shared criterion directory (default: StackOfPlates/target/criterion)")
    ap.add_argument("--timeout", type=int, default=None,
                    help="per-author timeout in seconds (default: none)")
    args = ap.parse_args()

    authors = find_authors(args.author)
    if not authors:
        print("No author crates found.", file=sys.stderr)
        return 1

    home = os.path.abspath(args.criterion_home)
    if args.fresh and os.path.isdir(home):
        shutil.rmtree(home)
        print(f"discarded previous results in {home}")
    os.makedirs(home, exist_ok=True)

    if args.quick:
        print("QUICK MODE: timings are for smoke-testing only, not comparison.\n")
    print(f"criterion home : {home}")
    print(f"authors        : {', '.join(a for a, _ in authors)}")
    print("running sequentially so the arms do not contend for CPU\n")

    started = time.time()
    succeeded, failed, partial = [], [], []

    for author, crate in authors:
        before = bench_dirs(home)
        print(f"  {author:<10} ... ", end="", flush=True)
        ok, detail = run_author(author, crate, home, args.quick, args.timeout)
        added = bench_dirs(home) - before
        if ok:
            succeeded.append(author)
            print(f"ok    ({detail})")
        else:
            failed.append((author, detail))
            print(f"SKIP  ({detail})")
            if added:
                # Partially completed then died: its data is stored but the group
                # report was never redrawn, so the plot would omit it.
                partial.append(author)

    # A failed run never regenerates the group report. If one died *after*
    # storing results, redraw by re-running the last author known to work.
    if partial and succeeded:
        redraw = succeeded[-1]
        print(f"\n{', '.join(partial)} stored partial results without redrawing the report;"
              f" re-running {redraw} to refresh it")
        crate = dict(authors)[redraw]
        ok, detail = run_author(redraw, crate, home, args.quick, args.timeout)
        print(f"  redraw     ... {'ok' if ok else 'FAILED'}  ({detail})")

    print_table(collect(home, started), args.markdown)

    if failed:
        print("\nSkipped:")
        for author, detail in failed:
            print(f"  {author:<10} {detail}")

    report = os.path.join(home, "report", "index.html")
    violin = os.path.join(home, "stack of plates", "report", "violin.svg")
    print()
    if os.path.isfile(report):
        print(f"combined report : {report}")
    if os.path.isfile(violin):
        print(f"violin plot     : {violin}")
    if args.open_report and os.path.isfile(report):
        webbrowser.open(f"file://{os.path.abspath(report)}")

    return 0 if succeeded else 1


if __name__ == "__main__":
    sys.exit(main())
