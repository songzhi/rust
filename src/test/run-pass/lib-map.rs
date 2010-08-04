// -*- rust -*-

use std;
import std.map;

fn test_simple() {
  fn eq(&uint x, &uint y) -> bool { ret x == y; }

  let map.hashfn[uint] hasher = std.util.id[uint];
  let map.eqfn[uint] eqer = eq;
  let map.hashmap[uint, uint] hm = map.mk_hashmap[uint, uint](hasher, eqer);
}

fn main() {
  test_simple();
}