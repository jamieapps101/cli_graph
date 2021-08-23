//! Small crate to generate simple CLI graphs
//!

use std::fmt::Display;

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
    CustomRangeLowerValueLargerThanUpperValue,
}



/// Enum used to specify graph type to render in `graph` function
#[derive(PartialEq)]
pub enum GraphType {
    Bar,
    Scatter,
    ScatterInterpolated,
}


#[derive(Debug)]
struct YScaleInformation {
    // range: f64,
    scale: f64,
    min:   f64,
}

fn handle_y_scaling(graph_data: &[DataPoint<String,f64>], graph_config: &GraphConfig<f64>) -> Result<YScaleInformation,GraphError > {
    let mut max_val : f64 = f64::MIN;
    let mut min_val : f64 = f64::MAX;
    println!("in handle_y_scaling");
    match graph_config.get_y_range() {
        YDataRange::Min2Max => {
            graph_data.iter().map(|d| d.value).for_each(|v| {
                max_val = max_val.max(v);
                min_val = min_val.min(v);
            });
        },
        YDataRange::Zero2Max => {
            min_val = 0.0;
            graph_data.iter().map(|d| d.value).for_each(|v| {
                max_val = max_val.max(v);
            });
        },
        YDataRange::Custom(min,max) => {
            println!("hit custom: ({},{})",min,max);
            if min > max {
                return Err(GraphError::CustomRangeLowerValueLargerThanUpperValue);
            }
            max_val = max;
            min_val = min;
        },
    }
    let range = max_val-min_val;
    let scale = range/((graph_config.get_max_height()-3) as f64);

    Ok(YScaleInformation {
        // range,
        scale,
        min:   min_val,
    })
}

/// Render a graph to the CLI using Column data in f64 format
pub fn graph<L: Clone+Display, T: Into<GraphData<L,f64>>> (data: T, config: GraphConfig<f64>, graph_type: GraphType) -> Result<(), GraphError> {
    // // Main Setup
    
    // check there is data
    let source_data: GraphData<L,f64> = data.into();
    let (graph_data,title_option) = source_data.split();
    if graph_data.is_empty() { return Err(GraphError::NoData); }

    let mut graph_data: Vec<DataPoint<String,f64>> = graph_data.iter().map(|dp| {
        DataPoint {
            label: format!("{}",dp.label),
            value: dp.value,
        }
    }).collect();

    // check reasonable config
    if config.get_max_width() < REASONABLE_MIN_MAX_WIDTH { return Err(GraphError::GraphConfigMaxWidthTooSmall); } 
    if config.get_max_width() < REASONABLE_MIN_MAX_HEIGHT { return Err(GraphError::GraphConfigMaxHeightTooSmall); } 

    // handle y scale
    let y_scale_info: YScaleInformation;
    match handle_y_scaling(&graph_data, &config) {
        Ok(ysi) => y_scale_info = ysi,
        Err(reason) => return Err(reason),
    }
    
    // print title if appropriate
    if let Some(title) = title_option {
        println!("\t{}",title);
    }
    
    // Graphing Section!
    while !graph_data.is_empty() {
        // plan graph
        let mut current_pass_render_cols : Vec<DataPoint<String,f64>> = Vec::new();
        let mut useable_column_width = config.get_max_width() - 1; // to account for numbers and y axis
        while !graph_data.is_empty() {
            // // for each column to be rendered, if there is space left on the current graph figure
            if graph_data[0].label.len() < useable_column_width {
                let col = graph_data.pop().unwrap();
                // subtract the space required for the column from the remaining figure space
                useable_column_width -= col.label.len()+1;
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
            let line_val = y_scale_info.scale*(line_index as f64)+y_scale_info.min;
            // format to a string and record the value
            max_y_val_character_width = max_y_val_character_width.max(format!("{}",line_val).len());
        });

        // determin remaining terminal cols after rendering y values and y axis
        let graph_width : usize = useable_column_width;

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
                    current_pass_render_cols.iter().for_each(|c| row.push_str( &format!("{} ",c.label) ) );  
                },
                // add x axis
                1 => {
                    (0..max_y_val_character_width).for_each(|_| row.push(' ')); // padding
                    (0..graph_width).for_each(|_| row.push('-') ); // x axis
                },
                // render graph area
                _ => {
                    let y_index_pos = index-2;
                    let y_val_at_line = (y_index_pos as f64)*y_scale_info.scale + y_scale_info.min;
                    let y_val_at_next_line = ((y_index_pos+1) as f64)*y_scale_info.scale + y_scale_info.min;
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
                                    row.push(config.get_plotting_symbol());
                                } else {
                                    row.push(' ');
                                }
                            }

                            GraphType::Scatter => {
                                if c.value >= y_val_at_line &&  c.value < y_val_at_next_line {
                                    row.push(config.get_plotting_symbol());
                                } else {
                                    row.push(' ');
                                }
                            }

                            _ => unreachable!(),
                        }
                        // fill remaining space with empty space
                        (0..c.label.len()).for_each(|_| row.push(' '));
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

    fn gen_small_data() -> GraphData<String,f64> {
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
        let gd : GraphData<String,f64> = (names, values).into();
        let gd = gd.title("Lots of Fruit");
        let gc = GraphConfig::new().max_height(11).max_width(50);
        graph(gd, gc, GraphType::Bar).unwrap();
    }

    #[test]
    fn scatter_graph_single_figure_f64_default_scale() {
        println!("\n\n");
        let gd = gen_small_data();
        let gc = GraphConfig::new().max_height(11);
        graph(gd, gc, GraphType::Scatter).unwrap();
    }


    #[test]
    fn scatter_graph_single_figure_f64_zero2max() {
        println!("\n\n");
        let gd = gen_small_data();
        let gc = GraphConfig::new().max_height(11).y_range(YDataRange::Zero2Max);
        graph(gd, gc, GraphType::Scatter).unwrap();
    }

    #[test]
    fn scatter_graph_single_figure_f64_custom() {
        println!("\n\n");
        let gd = gen_small_data();
        let gc = GraphConfig::new()
            .max_height(11)
            .y_range(YDataRange::Custom(1.0,15.0));
        graph(gd, gc, GraphType::Scatter).unwrap();
    }
}
