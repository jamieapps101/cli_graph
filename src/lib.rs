//! Small crate to generate simple CLI graphs
//!

mod config;
pub use config::*;

mod data;
pub use data::*;

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



/// Enum used to specify graph type to render in `graph` function
#[derive(PartialEq)]
pub enum GraphType {
    Bar,
    Scatter,
    ScatterInterpolated,
}

/// Umbrella method giving easier access to each graph type
pub fn graph<T: Into<GraphData<f64>>>(data: T, config: GraphConfig, graph_type: GraphType) -> Result<(), GraphError> {
    match graph_type {
        GraphType::Bar                 => bar_graph(data,config,graph_type),
        GraphType::Scatter             => bar_graph(data,config,graph_type),
        GraphType::ScatterInterpolated => unimplemented!(),
    }
}


/// Render a bar graph to the CLI using Column data in f64 format
pub fn bar_graph<T: Into<GraphData<f64>>> (data: T, config: GraphConfig, graph_type: GraphType) -> Result<(), GraphError> {
    // // Main Setup
    
    // check there is data
    let source_data: GraphData<f64> = data.into();
    let (mut graph_data,title_option) = source_data.split();
    if graph_data.is_empty() { return Err(GraphError::NoData); }

    // check reasonable config
    if config.get_max_width() < REASONABLE_MIN_MAX_WIDTH { return Err(GraphError::GraphConfigMaxWidthTooSmall); } 
    if config.get_max_width() < REASONABLE_MIN_MAX_HEIGHT { return Err(GraphError::GraphConfigMaxHeightTooSmall); } 

    
    let mut max_val : f64 = 0.0;
    let mut min_val : f64 = graph_data[0].value;
    graph_data.iter().map(|d| d.value).for_each(|v| {
        max_val = max_val.max(v);
        min_val = min_val.min(v);
    });
    
    let range = max_val-min_val;
    let scale = range/((config.get_max_height()-3) as f64);
    
    
    // print title if appropriate
    if let Some(title) = title_option {
        println!("\t{}",title);
    }
    
    // Graphing Section!
    while !graph_data.is_empty() {
        // plan graph
        let mut current_pass_render_cols : Vec<Column<f64>> = Vec::new();
        let mut useable_column_width = config.get_max_width() - 1; // to account for numbers and y axis
        while !graph_data.is_empty() {
            // // for each column to be rendered, if there is space left on the current graph figure
            if graph_data[0].name.len() < useable_column_width {
                let col = graph_data.pop().unwrap();
                // subtract the space required for the column from the remaining figure space
                useable_column_width -= col.name.len();
                // insert the column to be rendered
                current_pass_render_cols.push(col);
                // advance the column index
            } else {
                // if there is no more space left, move on to next section
                break;
            }
        }

        // // get max y val string length for this figure
        let mut max_y_val_character_width: usize = 0;
        // for each col in this figure
        (0..config.get_max_height()).for_each(|line_index|{
            // use linear interpolation to get the values to be rendered
            let line_val = scale*(line_index as f64)+min_val;
            // format to a string and record the value
            max_y_val_character_width = max_y_val_character_width.max(format!("{}",line_val).len());
        });

        // determin remaining terminal cols after rendering y values and y axis
        let graph_width : usize = config.get_max_height()+max_y_val_character_width - useable_column_width;

        // // generate graph rows
        let mut rows: Vec<String> = Vec::with_capacity(config.get_max_height());
        for index in 0..config.get_max_height() {
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
                    let y_val_at_next_line = ((y_index_pos+1) as f64)*scale + min_val;
                    // print y value corresponding with row (or space if row index is odd)
                    if index%2 == 0  || index==(config.get_max_height()-1) {
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
                        match graph_type {
                            GraphType::Bar => {
                                if c.value >= y_val_at_line {
                                    row.push('#');
                                } else {
                                    row.push(' ');
                                }
                            }

                            GraphType::Scatter => {
                                if c.value >= y_val_at_line &&  c.value < y_val_at_next_line {
                                    row.push('#');
                                } else {
                                    row.push(' ');
                                }
                            }

                            _ => unreachable!(),
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

    fn gen_small_data() -> GraphData<f64> {
        let names  : Vec<String> = vec!["apples","oranges","bananas","grapes"].iter().map(|&s| s.to_owned() ).collect();
        let values : Vec<f64>    = vec![5.0,3.0,8.0,2.0];
        let gd = GraphData::from((names, values));
        return gd;
    }

    #[test]
    fn bar_graph_single_figure_f64() {
        println!("\n\n");
        let gd = gen_small_data();
        let gc = GraphConfig::new().max_height(11);
        graph(gd, gc, GraphType::Bar).unwrap();
    }

    #[test]
    fn bar_graph_multi_figure_f64() {
        println!("\n\n");
        let names  : Vec<String> = vec!["apples","oranges","bananas","grapes","apples","oranges","bananas","grapes","apples","oranges","bananas","grapes"].iter().map(|&s| s.to_owned() ).collect();
        let values : Vec<f64>    = (0..12).map(|v| v as f64).collect();
        let gd : GraphData<f64> = (names, values).into();
        let gd = gd.title("Lots of Fruit");
        let gc = GraphConfig::new().max_height(11).max_width(50);
        graph(gd, gc, GraphType::Bar).unwrap();
    }

    #[test]
    fn scatter_graph_single_figure_f64() {
        println!("\n\n");
        let gd = gen_small_data();
        let gc = GraphConfig::new().max_height(11);
        graph(gd, gc, GraphType::Scatter).unwrap();
    }
}
