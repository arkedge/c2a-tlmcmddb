# Formal Methods Trial

This directory records a small formal-methods trial for `arkedge/c2a-tlmcmddb`.

The repository was selected from the accessible GitHub repositories because it is
a Rust data-model and CSV parser for C2A TlmCmd DB.  That shape is a good fit for
small predicate models: the implementation accepts rows, while downstream
codegen and operations depend on stronger database invariants.

## Commands

```bash
sh formal/check.sh
```

The check currently requires `z3` on `PATH`.

## Models

- `z3/tlm-field-group-contract.smt2`
  - Source: `tlmcmddb/src/tlm.rs`, `tlmcmddb-csv/src/tlm/body.rs`
  - Shape: pure predicate over a telemetry field group
  - Checks: field ranges must be non-empty, fit within the group variable
    width, not overlap, and multi-field groups must use an unsigned integer
    variable type.

## Trial Result

The model found three current-parser witnesses that are accepted at the CSV
shape level but should be domain questions for a TlmCmd DB contract:

1. A multi-field group can use a non-unsigned-integer variable type even though
   `tlmcmddb/src/tlm.rs` documents that multi-field groups must use
   `uint8_t`, `uint16_t`, or `uint32_t`.
2. A field range can exceed the group variable width.
3. Two field ranges in the same group can overlap.

These are not yet asserted as implementation bugs.  They identify the next
useful regression guards if the project wants the parser to reject invalid TLM
DB shapes before codegen or operations consume them.
