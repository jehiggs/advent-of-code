# /// script
# dependencies = [
#   "toml",
# ]
# ///

import argparse
import pathlib
import subprocess
import shutil
import toml


def new(args):
    pathlib.Path(f"./{args.year}").mkdir(exist_ok=True)
    run_cargo(
        "new",
        "--bin",
        "--name",
        f"day-{args.day}-{args.year}",
        f"{args.year}/day-{args.day}",
    )
    shutil.copyfile("./scripts/template.rs", f"./{args.year}/day-{args.day}/src/main.rs")
    toml_file = f"./{args.year}/day-{args.day}/Cargo.toml"
    cargo = toml.load(toml_file)
    cargo["dependencies"]["aoc-lib"] = {'workspace': True}
    with open(toml_file, 'w') as f:
        toml.dump(cargo, f)
    run_cargo("run", "--release", "--bin", "aoc", "--", f"{args.year}", f"{args.day}")


def run(args):
    output = run_cargo("run", "--release", "--bin", f"day-{args.day}-{args.year}")
    print(output.stdout.decode("utf-8"))


def run_cargo(*args):
    cmd = ["cargo"]
    cmd.extend(args)
    output = subprocess.run(cmd, capture_output=True)
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()

    new_command = subparsers.add_parser("new")
    new_command.add_argument("year", type=int)
    new_command.add_argument("day", type=int)
    new_command.set_defaults(func=new)

    run_command = subparsers.add_parser("run")
    run_command.add_argument("year", type=int)
    run_command.add_argument("day", type=int)
    run_command.set_defaults(func=run)

    args = parser.parse_args()
    args.func(args)
