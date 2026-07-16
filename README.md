# tactus-quadratic-extension

Formally verified (Lean + Rust, via tactus) real algebraic numbers for 2D CAD
constraint satisfaction.

Despite the crate name, the design covers general-degree towers of simple real
extensions (quadratic levels are the common special case). See **DESIGN.md**
for the full plan: verified certificate checker over extension towers, D5
dynamic evaluation for zero-tests, Sturm–Tarski queries for ordering, and a
port of the `verus-2d-constraint-satisfaction` constraint layer.
