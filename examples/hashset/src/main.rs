/*
## Data structure: HashSet

---

### Data structure: HashSet

* A hash set uses a hash function to find the slot for an item efficiently
* We use `integer % 42`. We use a fixed size of 42 slots
* Slots are vectors containing all elements.
* Ever heard of python's `-R` or `PYTHONHASHSEED`? CVE-2012-1150. Same in [ruby](https://www.ruby-lang.org/en/news/2011/12/28/denial-of-service-attack-was-found-for-rubys-hash-algorithm-cve-2011-4815/). CVS-2011-4815.

---

### Data structure: HashSet

[HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)

> `with_hasher` method: Warning: hasher is normally randomly generated, and is designed to allow HashSets to be resistant to attacks that cause many collisions and very poor performance. Setting it manually using this function can expose a DoS attack vector.

---

### Data structure: HashSet

```rust
#[derive(Debug)]
struct Slot { slot: Vec<u64> }
struct HashSet { set: [Slot; 42] }

impl Slot {
  fn new() -> Slot { Slot { slot: Vec::new() } }
}
impl HashSet {
  fn new() -> HashSet {
    HashSet { set: [Slot::new(); 42] }
  }
}

fn main() { HashSet::new(); }
```

Does it compile?

---

### Data structure: HashSet

```
error[E0277]: the trait bound `Slot: std::marker::Copy` is not satisfied
  --> src/main.rs:11:20
   |
11 |     HashSet { set: [Slot::new(); 42] }
   |                    ^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy`
   |                                      is not implemented for `Slot`
   |
   = note: the `Copy` trait is required
           because the repeated element will be copied
```

---

### Data structure: HashSet

```rust
#[derive(Debug,Copy,Clone)]
struct Slot { slot: Vec<u64> }

#[derive(Copy,Clone)]
struct HashSet { set: [Slot; 42] }
```

Does it compile? <span class="fragment">No.</span>

---

### Data structure: HashSet

```
error[E0204]: the trait `Copy` may not be implemented for this type
 --> src/main.rs:1:16
  |
1 | #[derive(Debug,Copy,Clone)]
  |                ^^^^
2 | struct Slot { slot: Vec<u64> }
  |               -------------- this field does not implement `Copy`
```

---

*/

struct Slot {
    slot: Vec<u64>
}

impl Slot {
    fn new() -> Slot {
        Slot { slot: Vec::new() }
    }
}

struct HashSet {
    set: [Slot; 42]
}

impl HashSet {
    fn new() -> HashSet {
        HashSet { set: [Slot; 42] }
    }

    fn insert(&mut self, item: u64) -> &Self {
        let slot = self.set[item % 42].slot;
        for i in 0..slot.len() {
            if let Some(slot_item) = slot.get(i) && slot_item == item {
                return self
            }
        }
        slot.push(item);
        self
    }

    fn remove(&mut self, item: u64) -> &Self {
        let slot_id = item % 42;
        self.set[slot_id].remove(item);
        self
    }
}
/*
impl Iterator for HashSet {
    type Item = u64;

    fn next(&mut self) -> Option<HashSet> {
        if !self.init {
            return None
        }
        if self.value + self.step > self.end {
            return None
        }
        self.value += self.step;
        Some(self.clone())
    }
}*/

fn main() {
    let hs = HashSet::new();
    hs.insert(2).insert(99).insert(5);
    println!("{}", hs.contains(42));
    hs.remove(2);
    println!("{}", hs.contains(2));

    for item in hs {
        print!("[{}] ", item);
    }
    println!("");
}
