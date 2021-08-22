//! Small crate to generate simple CLI graphs
//!


// type ValueType;

/// Primary data point of a bar graph
#[derive(Clone)]
pub struct Column<T> where T: Copy {
    /// Name will be printed below the x-axis for the bar
    pub name: String,
    /// Value will determine the bar height
    pub value: T,
}

/// Used to provide additional configuration for graphs
pub struct GraphConfig<T> {
    max_width: usize,
    max_height: usize,
    title: Option<String>,
    y_range: YDataRange<T>
}

/// Used to set the range type of the y-axis
pub enum YDataRange<T> {
    /// Use the maximum and minimum values of the data given
    Min2Max,
    /// Between 0 and the maximum of the data given
    Zero2Max,
    /// Between the first and the second value supplied
    /// the first value should be the lower end of the range
    Custom(T,T)
}

impl<T> Default for GraphConfig<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GraphConfig<T> {
    /// Create a default config, specifying 80 column width, 5 column height and no title
    pub fn new() -> Self {
        GraphConfig {
            max_width: 80,
            max_height: 5,
            title: None,
            y_range: YDataRange::Min2Max,
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

    /// Set the graph title 
    pub fn title<S: ToString>(mut self, t: S) -> Self {
        self.title = Some(t.to_string());
        self
    }

    // Set the y-axis range style
    pub fn y_range(mut self, range: YDataRange<T>) -> Self {
        self.y_range = range;
        self
    }
}


const REASONABLE_MIN_MAX_WIDTH : usize = 40;
const REASONABLE_MIN_MAX_HEIGHT : usize = 3;

#[derive(Debug)]
pub enum GraphError {
    /// Indicates the Vec of Column structs given to the graph function was empty
    NoData,
    ColumnNameTooWideForGraphConfig,
    GraphConfigMaxWidthTooSmall,
    GraphConfigMaxHeightTooSmall,
}

type GraphData<T> = Vec<Column<T>>;

/// Render a bar graph to the CLI using Column data in f64 format
pub fn bar_graph<T> (data: T, config: Option<GraphConfig<T>>) -> Result<(), GraphError> where Vec<Column<f64>>: From<T> {
    // // Main Setup
    
    // check there is data
    let mut graph_data : GraphData<f64> = data.into();
    if graph_data.is_empty() { return Err(GraphError::NoData); }

    // check reasonable config
    let config = match config {
        Some(external_config) => external_config,
        None => GraphConfig::new(),
    };
    if config.max_width < REASONABLE_MIN_MAX_WIDTH { return Err(GraphError::GraphConfigMaxWidthTooSmall); } 
    if config.max_width < REASONABLE_MIN_MAX_HEIGHT { return Err(GraphError::GraphConfigMaxHeightTooSmall); } 

    
    let mut max_val : f64 = 0.0;
    let mut min_val : f64 = graph_data[0].value;
    graph_data.iter().map(|d| d.value).for_each(|v| {
        max_val = max_val.max(v);
        min_val = min_val.min(v);
    });
    
    let range = max_val-min_val;
    let scale = range/((config.max_height-3) as f64);
    
    
    // print title if appropriate
    if let Some(title) = config.title {
        println!("\t{}",title);
    }
    
    // Graphing Section!
    while !graph_data.is_empty() {
        // plan graph
        let mut current_pass_render_cols : Vec<Column<f64>> = Vec::new();
        let mut useable_column_width = config.max_width - 1; // to account for numbers and y axis
        while !graph_data.is_empty() {
            // // for each column to be rendered, if there is space left on the current graph figure
            if graph_data[0].name.len() < useable_column_width {
                let col = graph_data.pop().unwrap();
                // subtract the space required for the column from the remaining figure space
                useable_column_width -= col.name.len();
                // insert the column to be rendered
                current_pass_render_cols.push(col);
                // advance the column index
                // column_index+=1;
            } else {
                // if there is no more space left, move on to next section
                break;
            }
        }

        // // get max y val string length for this figure
        let mut max_y_val_character_width: usize = 0;
        // for each col in this figure
        (0..config.max_height).for_each(|line_index|{
            // use linear interpolation to get the values to be rendered
            let line_val = scale*(line_index as f64)+min_val;
            // format to a string and record the value
            max_y_val_character_width = max_y_val_character_width.max(format!("{}",line_val).len());
        });

        // determin remaining terminal cols after rendering y values and y axis
        let graph_width : usize = config.max_width+max_y_val_character_width - useable_column_width;

        // // generate graph rows
        let mut rows: Vec<String> = Vec::with_capacity(config.max_height);
        for index in 0..config.max_height {
            let mut row = String::new();
            match index {
                // names are printed on first row
                0 => { 
                    // add padding to account for y axis value width
                    (0..max_y_val_character_width+1).for_each(|_| row.push(' '));
                    // print column names with spacing
                    current_pass_render_cols.iter().for_each(|c| row.push_str( &format!("{} ",c.name) ) );  
                },
                // add x axis
                1 => {
                    (0..max_y_val_character_width).for_each(|_| row.push(' ')); // padding
                    (0..graph_width).for_each(|_| row.push('-') ); // x axis
                },
                // render graph area
                _ => {
                    let y_index_pos = index-2;
                    let y_val_at_line = (y_index_pos as f64)*scale + min_val;
                    // print y value corresponding with row (or space if row index is odd)
                    if index%2 == 0  || index==(config.max_height-1) {
                        let y_val_at_line_str = format!("{}",y_val_at_line);
                        row.push_str(&y_val_at_line_str);
                        (0..(max_y_val_character_width-y_val_at_line_str.len())).for_each(|_| row.push(' '));
                        
                    } else {
                        (0..max_y_val_character_width).for_each(|_| row.push(' '));
                    }
                    // print y axis 
                    row.push('|');
                    // for each column to be rendered in this figure
                    current_pass_render_cols.iter().for_each(|c| {
                        if c.value >= y_val_at_line {
                            // render hash if the column value is greater than the y axis value
                            row.push('#');
                        } else {
                            // space otherwise
                            row.push(' ');
                        }
                        // fill remaining space with empty space
                        (0..c.name.len()).for_each(|_| row.push(' '));
                    });
                }
            }
            rows.push(row);
        } 


        // print graph pass
        rows.iter().rev().for_each(|r| {
            println!("{}", r);
        });
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        names: Vec<String>,
        values: Vec<f64>
    }

    impl From<TestData> for Vec<Column<f64>> {
        fn from(td: TestData) -> GraphData<f64> {
            td.names.iter().zip(td.values.iter()).map(|(n,v)| {
                Column { name: n.clone(), value: *v } }).collect()
        }
    }

    #[test]
    fn bar_graph_single_figure_f64() {
        let names  = vec!["apples","oranges","bananas","grapes"].iter().map(|&s| s.to_owned() ).collect();
        let values = vec![5.0,3.0,8.0,2.0];
        let td = TestData { names, values };
        let values : GraphData<f64> = td.into();

        let gc = GraphConfig::new().max_height(11);

        bar_graph(values, Some(gc)).unwrap();
    }

    #[test]
    fn bar_graph_multi_figure_f64() {
        let names  = vec!["apples","oranges","bananas","grapes","apples","oranges","bananas","grapes","apples","oranges","bananas","grapes"].iter().map(|&s| s.to_owned() ).collect();
        let values = (0..12).map(|v| v as f64).collect();
        let td = TestData { names, values };
        let values : GraphData<f64> = td.into();

        let gc = GraphConfig::new().max_height(11).max_width(50).title("Lots of Fruit");

        bar_graph(values, Some(gc)).unwrap();
    }
}
