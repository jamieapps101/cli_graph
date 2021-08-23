
/// Used to provide additional configuration for graphs
#[derive(Debug)]
pub struct GraphConfig<T> where T: Copy+Clone {
    max_width: usize,
    max_height: usize,
    y_range: YDataRange<T>,
    graph_symbol: char,
}

/// Used to set the range type of the y-axis
#[derive(Clone,Copy,Debug)]
pub enum YDataRange<T> where T: Copy+Clone {
    /// Use the maximum and minimum values of the data given
    Min2Max,
    /// Between 0 and the maximum of the data given
    Zero2Max,

    // Between the first and the second value supplied
    // the first value should be the lower end of the range
    Custom(T,T)
}

impl<T: Copy+Clone> Default for GraphConfig<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy+Clone> GraphConfig<T> {
    /// Create a default config, specifying 80 column width, 5 column height and no title
    pub fn new() -> Self {
        GraphConfig {
            max_width: 80,
            max_height: 5,
            y_range: YDataRange::Min2Max,
            graph_symbol: '#',
        }
    }

    /// Set the max height of the graph
    pub fn max_height(mut self, h: usize) -> Self {
        self.max_height = h;
        self
    }

    /// Set the max width of the x-axis across the terminal
    pub fn max_width(mut self, w: usize) -> Self {
        self.max_width = w;
        self
    }

    /// Set the y-axis range style
    pub fn y_range(mut self, range: YDataRange<T>) -> Self {
        self.y_range = range;
        self
    }

    /// Set the plotting symbol, default '#'
    pub fn plotting_symbol(mut self, s: char) -> Self {
        self.graph_symbol = s;
        self
    }
    
    /// Gets current value of the max width
    pub fn get_max_width(&self) -> usize { self.max_width }
    /// Gets current value of the max height
    pub fn get_max_height(&self) -> usize { self.max_height }
    /// Gets current value of the y ranging mode
    pub fn get_y_range(&self) -> YDataRange<T> { self.y_range }
    /// Gets the plotting symbol
    pub fn get_plotting_symbol(&self) -> char { self.graph_symbol }
}
