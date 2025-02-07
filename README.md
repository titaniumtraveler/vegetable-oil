# Vegetable Oil

Implementation of a [`oil.nvim`]-like system to transform one arbitrary
filesystem-state into another arbitrary filesystem-state.

With the goals of being editor independent and as unlimited of what
filesystem-states and transformations between those it can represent.

**Heavily** work in progress!

[`oil.nvim`]: https://github.com/stevearc/oil.nvim

## Basic concepts

The main concepts of `Vegetable Oil` are the `current`-state, the `next`-state and
the diff that is generated from them.

### Current State

The `current`-state is a collection of entries that contain an ID, a path and an
entry type (which might also be associated with some additional data).

One example for one such a `current`-state is this:

```json
{
  "current": [
    {
      "id": 1,
      "path": "src",
      "type": "dir"
    },
    {
      "id": 2,
      "path": "src/cli.rs",
      "type": "file"
    },
    {
      "id": 3,
      "path": "src/lib.rs",
      "type": "file"
    },
    {
      "id": 4,
      "path": "src/main.rs",
      "type": "file"
    }
  ]
}
```

Generated using this command from this directory.
```bash
ls -1 --directory src src/main.rs src/cli.rs src/lib.rs | vegetable-oil read --cwd '.'
```

### Next State

The `next`-state is similarly made up of a list of file entries and uses the
entry-IDs from the `current`-state to relate the files that are represented by
these entries with some new state.

To **create** new entries theoretically *any* unused ID could be used, but to
simplify the process and to prevent accidental ID collisions, the special value
`0` is used.

The combination of `current`-state and `next`-state already contains all context
needed to execute a filesystem transformation.

For example combined with the [`current`-state](#current-state) of the last
chapter the following `next`-state:

```json
{
  "next": [
    {
      "id": 1,
      "path": "src",
      "type": "dir"
    },
    {
      "id": 2,
      "path": "src/main.rs",
      "type": "file"
    },
    {
      "id": 3,
      "path": "src/lib.rs",
      "type": "file"
    },
    {
      "id": 0,
      "path": "example-dir",
      "type": "dir"
    }
  ]
}
```

would based on there being no entry with the ID `4` and there being an entry of
ID `0` delete `src/main.rs` and create the new directory `example-dir`.

Roughly equivalent with running
```bash
rm src/main.rs
mkdir example-dir
```

### Diff

But what happens if the user messes up the `next`-state in some way?

For that purposes there is another step: The generation of a diff between the
`current`-state and the `next`-state, which can be used to generate a summary of
the planned changes between `current`-state and `next`-state:

```json
{
  "diff": [
    {
      "id": 1,
      "path": "src",
      "type": "dir"
      "action": "keep",
    },
    {
      "id": 2,
      "path": "src/cli.rs",
      "type": "file"
      "action": "keep",
    },
    {
      "id": 3,
      "path": "src/lib.rs",
      "type": "file"
      "action": "keep",
    },
    {
      "id": 4,
      "path": "src/main.rs",
      "type": "file"
      "action": "delete",
    },
    {
      "id": 5,
      "path": "example-dir",
      "type": "dir",
      "action": "create"
    }
  ]
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your
option. Unless you explicitly state otherwise, any contribution intentionally
submitted for the project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
