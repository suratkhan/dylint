# redundant_reference

### What it does

Checks for fields that are references used only to read one copyable subfield, and whose
lifetimes are not used elsewhere.

### Why is this bad?

Storing the reference instead of a copy of the subfield adds an unnecessary lifetime
parameter to the struct. It also creates an unnecessary pointer dereference at runtime.

### Example

```rust
struct V<'cx, 'tcx> {
    cx: &'cx LateContext<'tcx>,
}

impl<'cx, 'tcx> Visitor<'tcx> for V<'cx, 'tcx> {
    type MaybeTyCtxt = rustc_middle::ty::TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.cx.tcx
    }
}
```

Use instead:

```rust
struct V<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> Visitor<'tcx> for V<'tcx> {
    type MaybeTyCtxt = rustc_middle::ty::TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }
}
```

### Configuration

- `lifetime_check: bool` (default `true`): Setting this to `false` disables the check that
  the lifetime use is unique. That is, the lint becomes a check for: fields that are
  references used only to read one copyable subfield.
