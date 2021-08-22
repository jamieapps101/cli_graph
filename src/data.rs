/// Primary data point of a bar graph
#[derive(Clone)]
pub struct Column<T> {
    /// Name will be printed below the x-axis for the bar
    pub name: String,
    /// Value will determine the bar height
    pub value: T,
}

pub struct GraphData<T> {
    data: Vec<Column<T>>,
    title: Option<String>,
}

impl<T: Copy> From<(Vec<String>,Vec<T>)> for GraphData<T> {
    fn from(d: (Vec<String>,Vec<T>)) -> GraphData<T> {
        let data = d.0.iter().zip(d.1.iter()).map(|(n,v)| {
            Column { name: n.clone(), value: *v } }).collect();

        GraphData {
            data,
            title: None,
        }
    }
}

impl<T> GraphData<T> {
    pub fn title<S: ToString>(mut self, t: S) -> Self {
        self.title = Some(t.to_string());
        self
    }

    pub fn split(self) -> (Vec<Column<T>>,Option<String>) {
        (self.data,self.title)
    }
}