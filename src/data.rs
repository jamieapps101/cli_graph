use std::fmt::Display;
use crate::colour::*;

/// Primary data point of a bar graph
#[derive(Clone)]
pub struct DataPoint<L,V> where L: Clone+Display {
    /// Label will be printed below the x-axis for the data point
    pub label: L,
    /// Value will determine the bar height
    pub value: V,
    pub colour: Option<Colour>,
}

pub struct GraphData<L,V> where L: Clone+Display {
    pub data: Vec<DataPoint<L,V>>,
    pub title: Option<String>,
}

impl<V: Copy, L:Clone+Display> From<(Vec<L>,Vec<V>)> for GraphData<L,V> {
    fn from(d: (Vec<L>,Vec<V>)) -> GraphData<L,V> {
        let data = d.0.iter().zip(d.1.iter()).map(|(n,v)| {
            DataPoint { label: n.clone(), value: *v, colour: None } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}

impl<V: Copy, L:Clone+Display> From<(Vec<L>,Vec<V>,Vec<Colour>)> for GraphData<L,V> {
    fn from(d: (Vec<L>,Vec<V>,Vec<Colour>)) -> GraphData<L,V> {
        let data = d.0.iter().zip(d.1.iter()).zip(d.2.iter()).map(|((n,v),c)| {
            DataPoint { label: n.clone(), value: *v, colour: Some(*c) } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}
impl<V: Copy, L:Clone+Display> From<Vec<(L,V)>> for GraphData<L,V> {
    fn from(d: Vec<(L,V)>) -> GraphData<L,V> {
        let data = d.iter().map(|(n,v)| {
            DataPoint { label: n.clone(), value: *v, colour: None } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}

impl<V: Copy, L:Clone+Display> From<Vec<(L,V, Colour)>> for GraphData<L,V> {
    fn from(d: Vec<(L,V, Colour)>) -> GraphData<L,V> {
        let data = d.iter().map(|(n,v,c)| {
            DataPoint { label: n.clone(), value: *v, colour: Some(*c) } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}

impl<V, L:Clone+Display> GraphData<L,V> {
    pub fn title<S: Clone+Display>(mut self, t: S) -> Self {
        self.title = Some(t.to_string());
        self
    }

    pub fn split(self) -> (Vec<DataPoint<L,V>>,Option<String>) {
        (self.data,self.title)
    }
}