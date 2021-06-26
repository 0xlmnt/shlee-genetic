pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, T>(&self, population: &'a [T]) -> &'a T
        where
            T: Individual;
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<T>(&self, population: &[T]) -> Vec<T>
        where
            T: Individual
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|| {
                //TODO: selection
                //TODO: crossover
                //TODO: mutation
            })
            .collect();

        todo!()
    }
}