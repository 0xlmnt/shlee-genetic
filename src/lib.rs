use rand::{RngCore, Rng};
use rand::seq::SliceRandom;
use std::ops::Index;
use std::iter::FromIterator;
use rand::distributions::Uniform;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

pub struct Chromosome {
    genes: Vec<f32>,
}
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item=f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect()
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub struct RouletteWheelSelection;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, T>(&self,
                     population: &'a [T],
                     rng: &mut dyn RngCore) -> &'a T
        where
            T: Individual;
}
pub trait CrossoverMethod {
    fn crossover(&self,
    rng: &mut dyn RngCore,
    parent_a: &Chromosome,
    parent_b: &Chromosome) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;
impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a.zip(parent_b)
            .map(|(&a, &b)| {
                if rng.gen_bool(0.5) {
                    a
                } else {
                    b
                }
            })
            .collect()
    }
}

impl<S> GeneticAlgorithm<S>
    where S: SelectionMethod
{
    pub fn new(selection_method: S) -> Self {
        Self {
            selection_method
        }
    }

    pub fn evolve<S, T>(&self,
                        population: &[T],
                        rng: &mut dyn RngCore) -> Vec<T>
        where
            T: Individual
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|| {
                let parent_a = self.selection_method.select(population, rng).chromosome();
                let parent_b = self.selection_method.select(population, rng).chromosome();


                //TODO: crossover
                //TODO: mutation
            })
            .collect();

        todo!()
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, T>(&self, population: &'a [T], rng: &mut dyn RngCore) -> &'a T where
        T: Individual {
        population.choose_weighted(rng, |indiv| {
            indiv.fitness()
        })
            .expect("[Err] Found empty population")
    }
}