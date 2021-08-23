use std::fmt::Display;

// pub trait LabelTrait: Clone+Display {}

// this is used in graph function
// impl LabelTrait for String {}

/// Primary data point of a bar graph
#[derive(Clone)]
pub struct DataPoint<L,V> where L: Clone+Display {
    /// Label will be printed below the x-axis for the data point
    pub label: L,
    /// Value will determine the bar height
    pub value: V,
}

pub struct GraphData<L,V> where L: Clone+Display {
    data: Vec<DataPoint<L,V>>,
    title: Option<String>,
}

impl<V: Copy, L:Clone+Display> From<(Vec<L>,Vec<V>)> for GraphData<L,V> {
    fn from(d: (Vec<L>,Vec<V>)) -> GraphData<L,V> {
        let data = d.0.iter().zip(d.1.iter()).map(|(n,v)| {
            DataPoint { label: n.clone(), value: *v } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}
impl<V: Copy, L:Clone+Display> From<Vec<(L,V)>> for GraphData<L,V> {
    fn from(d: Vec<(L,V)>) -> GraphData<L,V> {
        let data = d.iter().map(|(n,v)| {
            DataPoint { label: n.clone(), value: *v } }).collect();

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