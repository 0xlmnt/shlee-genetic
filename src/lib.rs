#![feature(min_type_alias_impl_trait)]

use rand::{RngCore, Rng};
use rand::seq::SliceRandom;
use std::ops::Index;
use std::iter::FromIterator;
use rand::distributions::Uniform;

pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut f32> {
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
    type IntoIter = impl Iterator<Item=f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub trait SelectionMethod {
    fn select<'a, T>(&self,
                     population: &'a [T],
                     rng: &mut dyn RngCore) -> &'a T
        where
            T: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, T>(&self, population: &'a [T], rng: &mut dyn RngCore) -> &'a T where
        T: Individual {
        population.choose_weighted(rng, |individual| {
            individual.fitness()
        })
            .expect("[Err] Found empty population")
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn from_chromosome(chromosome: Chromosome) -> Self;
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

pub trait MutationMethod {
    fn mutate(&self,
              rng: &mut dyn RngCore,
              child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance <= 1.0 && chance >= 0.0);

        Self {
            chance,
            coeff,
        }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gen in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) {
                1.0
            } else {
                -1.0
            };

            if rng.gen_bool(self.chance as _) {
                *gen += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
    where S: SelectionMethod
{
    pub fn new(selection_method: S,
               crossover_method: impl CrossoverMethod + 'static,
               mutation_method: impl MutationMethod + 'static) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<T>(&self,
                     population: &[T],
                     rng: &mut dyn RngCore) -> Vec<T>
        where
            T: Individual
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(population, rng).chromosome();
                let parent_b = self.selection_method.select(population, rng).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);
                T::from_chromosome(child)
            })
            .collect()
    }
}