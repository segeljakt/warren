use crate::program::ProgramBuilder;
use crate::{Cell, Machine, Program, TermBuilder};

/// Reference to query part for building complex (structure)
/// queries, and later for extracting unification result
#[derive(Clone, Copy)]
pub struct QueryRef(pub(crate) usize);

/// Result of running query
pub struct QueryResult<'a> {
    pub(crate) machine: &'a Machine,
    pub(crate) regs: Vec<Cell>,
}

/// Query to be executed
pub struct Query<'a> {
    pub(crate) program: Program<'a>,
    // Register with top-level struct assigned
    pub(crate) top_level: usize,
}

impl<'a> Query<'a> {
    pub fn assembly(&self) -> String {
        self.program.assembly()
    }
}

/// Builder for structured query
pub struct QueryBuilder {
    program: ProgramBuilder,
    next_register: usize,
}

impl QueryRef {
    pub fn id(self) -> usize {
        self.0
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self {
            program: Default::default(),
            // 0 register is reserved for top level term
            next_register: 1,
        }
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    fn next_register(&mut self) -> usize {
        self.next_register += 1;
        self.next_register - 1
    }

    pub fn variable(&mut self) -> QueryRef {
        let register = self.next_register();
        self.program.set_variable(register);
        QueryRef(register)
    }

    pub fn structure(
        &mut self,
        ident: usize,
        subterms: impl IntoIterator<
            Item = QueryRef,
            IntoIter = impl ExactSizeIterator<Item = QueryRef>,
        >,
    ) -> QueryRef {
        let subterms = subterms.into_iter();
        let register = self.next_register();
        self.program.put_structure(ident, subterms.len(), register);
        for subterm in subterms {
            let QueryRef(reg) = subterm;
            self.program.set_value(reg);
        }
        QueryRef(register)
    }

    pub fn constant(&mut self, ident: usize) -> QueryRef {
        self.structure(ident, std::iter::empty())
    }

    pub fn build(self, QueryRef(r): QueryRef) -> Query<'static> {
        Query {
            program: self.program.build(),
            top_level: r,
        }
    }
}

impl<'a> QueryResult<'a> {
    pub fn build_term<Builder: TermBuilder>(
        &self,
        QueryRef(qref): QueryRef,
        builder: &mut Builder,
    ) -> Option<Builder::Term> {
        self.machine.build_term(*self.regs.get(qref)?, builder)
    }
}
