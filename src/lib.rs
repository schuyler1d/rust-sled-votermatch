pub mod sledsearch {

    pub struct SearchComponent {
        // Load(path)
        // getScale() (or scale as a field value)
    }

    pub struct SearchComponentResult {
        //field: Vec<u32>
        // From/Into for &[u8]
        // BitAnd for intersection
    }

    pub struct MatchStrategy {
        //field: components: Vec<dyn SearchComponent>
        //field: groups=Vec/Array of indexes in components for matching
        // derivePairs() -- generates all pairs needed based on Groups/components
    }

}
