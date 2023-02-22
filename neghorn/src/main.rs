#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct Atom(char);

#[derive(Default, Eq, PartialEq)]
struct AtomSet {
    sorted_vec: Vec<Atom>,
}

#[derive(Debug, Copy, Clone)]
struct Literal {
    atom: Atom,
    pos: bool,
}

#[derive(Debug)]
struct Rule {
    head: Atom,
    body: Vec<Literal>,
}

#[derive(Debug)]
struct Kb {
    pos: AtomSet,                   // inverse is interpretation #N
    prev_pos: Option<AtomSet>,      // inverse is interpretation #N-1
    prev_prev_pos: Option<AtomSet>, // // inverse is interpretation #N-2
}

struct Valuation<'a>(&'a Kb);
////////////////////////////

impl std::fmt::Debug for Valuation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let t = self.0.pos.iter().map(|x| (x, 'T'));
        if let Some(prev_pos) = self.0.prev_pos.as_ref() {
            let u = prev_pos.set_minus_iter(&self.0.pos).map(|atom| (atom, '?'));
            f.debug_map().entries(t.chain(u)).finish()
        } else {
            let u = None;
            f.debug_map().entries(t.chain(u)).finish()
        }
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Debug for AtomSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_set().entries(self.sorted_vec.iter()).finish()
    }
}

impl Literal {
    fn pos(atom: Atom) -> Self {
        Self { pos: true, atom }
    }
    fn neg(atom: Atom) -> Self {
        Self { pos: false, atom }
    }
}

impl AtomSet {
    fn iter(&self) -> impl Iterator<Item = Atom> + '_ {
        self.sorted_vec.iter().copied()
    }
    fn insert(&mut self, atom: Atom) -> bool {
        match self.sorted_vec.binary_search(&atom) {
            Ok(_) => false,
            Err(i) => {
                self.sorted_vec.insert(i, atom);
                true
            }
        }
    }
    fn len(&self) -> usize {
        self.sorted_vec.len()
    }
    fn contains(&self, atom: Atom) -> bool {
        self.sorted_vec.binary_search(&atom).is_ok()
    }
    fn set_minus_iter<'a, 'b: 'a>(&'a self, other: &'b Self) -> impl Iterator<Item = Atom> + 'a {
        self.sorted_vec
            .iter()
            .copied()
            .filter(|&atom| !other.contains(atom))
    }
    fn in_not_in(&self, other: &Self) -> Option<Atom> {
        for (&x, &y) in self.sorted_vec.iter().zip(other.sorted_vec.iter()) {
            if x < y {
                return Some(x);
            }
        }
        None
    }
    fn is_subset_of(&self, other: &Self) -> bool {
        other.in_not_in(self).is_none()
    }
    fn is_superset_of(&self, other: &Self) -> bool {
        self.in_not_in(other).is_none()
    }
}

impl Default for Kb {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            prev_pos: None,
            prev_prev_pos: None,
        }
    }
}
impl Kb {
    fn contains(&self, lit: Literal) -> bool {
        if lit.pos {
            self.pos.contains(lit.atom)
        } else {
            if let Some(prev_pos) = &self.prev_pos {
                !prev_pos.contains(lit.atom)
            } else {
                false
            }
        }
    }
}
impl Rule {
    fn body_sat(&self, kb: &Kb) -> bool {
        self.body.iter().all(|&subgoal| kb.contains(subgoal))
    }
}

impl Kb {
    fn advance(&mut self) {
        self.prev_prev_pos = self.prev_pos.replace(std::mem::take(&mut self.pos));
    }
    fn saturate(&mut self, rules: &[Rule]) {
        'c: loop {
            println!("pos:{:?}", &self.pos);
            for rule in rules {
                if !self.contains(Literal::pos(rule.head)) && rule.body_sat(self) {
                    println!("applicable {:?}", rule);
                    self.pos.insert(rule.head);
                    continue 'c;
                }
            }
            break 'c;
        }
    }
}

fn main() {
    let rules = vec![
        Rule {
            head: Atom('a'),
            body: vec![Literal::pos(Atom('c')), Literal::neg(Atom('b'))],
        }, // ok
        Rule {
            head: Atom('b'),
            body: vec![Literal::neg(Atom('a'))],
        }, // ok
        Rule {
            head: Atom('c'),
            body: vec![],
        }, // ok
    ];
    let rules = vec![
        Rule {
            head: Atom('p'),
            body: vec![Literal::neg(Atom('p'))],
        }, // whee
        Rule {
            head: Atom('s'),
            body: vec![Literal::pos(Atom('p'))],
        }, // whee
        Rule {
            head: Atom('s'),
            body: vec![],
        }, // whee
    ];

    let mut kb = Kb {
        pos: Default::default(),
        prev_pos: None,
        prev_prev_pos: None,
    };
    for i in 0.. {
        println!("ADVANCED {:#?}", &kb);
        kb.saturate(&rules);
        println!("SATURATED {:#?}", &kb);
        if i % 2 == 0 && Some(&kb.pos) == kb.prev_prev_pos.as_ref() {
            println!("took {:?} iterations", i);
            break;
        }
        kb.advance();
    }
    println!("{:#?}", Valuation(&kb));
}
