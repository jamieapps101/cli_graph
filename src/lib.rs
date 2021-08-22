use std::cmp::{PartialOrd,PartialEq};



#[derive(Clone)]
pub struct Column<T> where T: Copy {
    name: String,
    value: T,
}

pub struct GraphConfig {
    max_width: usize,
    max_height: usize,
}

impl GraphConfig {
    pub fn new() -> Self {
        return DEFAULTGRAPHCONFIG;
    }

    pub fn max_height(mut self, h: usize) -> Self {
        self.max_height = h;
        return self;
    }

    pub fn max_width(mut self, w: usize) -> Self {
        self.max_width = w;
        return self;
    }
}

const DEFAULTGRAPHCONFIG : GraphConfig = GraphConfig {
    max_width: 80,
    max_height: 5,
};
const REASONABLE_MIN_MAX_WIDTH : usize = 40;
const REASONABLE_MIN_MAX_HEIGHT : usize = 3;

#[derive(Debug)]
pub enum GraphError {
    NoData,
    ColumnNameTooWideForGraphConfig,
    GraphConfigMaxWidthTooSmall,
    GraphConfigMaxHeightTooSmall,
}

type GraphData<T> = Vec<Column<T>>;
pub fn graph<T> (data: T, config: Option<GraphConfig>) -> Result<(), GraphError> where Vec<Column<f32>>: From<T> {
    // // Main Setup
    
    // check there is data
    let graph_data : GraphData<f32> = data.into();
    if graph_data.len() == 0 { return Err(GraphError::NoData); }

    // check reasonable config
    let config = match config {
        Some(external_config) => external_config,
        None => DEFAULTGRAPHCONFIG,
    };
    if config.max_width < REASONABLE_MIN_MAX_WIDTH { return Err(GraphError::GraphConfigMaxWidthTooSmall); } 
    if config.max_width < REASONABLE_MIN_MAX_HEIGHT { return Err(GraphError::GraphConfigMaxHeightTooSmall); } 

    
    let mut max_val : f32 = 0.0;
    let mut min_val : f32 = graph_data[0].value;
    graph_data.iter().map(|d| d.value).for_each(|v| {
        max_val = max_val.max(v);
        min_val = min_val.min(v);
    });
    
    let range = max_val-min_val;
    let scale = range/((config.max_height-3) as f32);
    
    
    
    
    // Graphing Section!
    let mut column_index = 0;
    loop {
        // plan graph pass
        let mut current_render_cols : Vec<Column<f32>> = Vec::new();
        let mut useable_column_width = config.max_width - 1; // to account for numbers and y axis
        loop {
            if graph_data.len() > column_index && graph_data[column_index].name.len() < useable_column_width {
                useable_column_width -= graph_data[column_index].name.len();
                current_render_cols.push(graph_data[column_index].clone());
                column_index+=1;
            } else {
                break;
            }
        }

        // get max y val string length for this graph
        let mut max_y_val_character_width: usize = 0;
        (0..config.max_height).for_each(|line_index|{
            let line_val = scale*(line_index as f32)+min_val;
            max_y_val_character_width = max_y_val_character_width.max(format!("{}",line_val).len());
        });
        println!("max_y_val_character_width: {}",max_y_val_character_width);

        let graph_width : usize = config.max_width - (max_y_val_character_width + 1);

        // generate rows
        let mut rows: Vec<String> = Vec::with_capacity(config.max_height);
        for index in 0..config.max_height {
            let mut row = String::new();
            match index {
                0 => { // names are printed
                    (0..max_y_val_character_width+1).for_each(|_| row.push(' '));
                    current_render_cols.iter().for_each(|c| row.push_str( &format!("{} ",c.name) ) );  
                },
                1 => { // first value printed
                    // row.push_str( &format!("{} ",min_val));
                    (0..max_y_val_character_width).for_each(|_| row.push(' '));
                    (0..graph_width).for_each(|_| row.push('-') );
                },
                _ => {
                    let line_pos = index-2;
                    let y_val_at_line = (line_pos as f32)*scale + min_val;
                    // push values
                    if index%2 == 0  || index==(config.max_height-1) {
                        let y_val_at_line_str = format!("{}",y_val_at_line);
                        println!("y_val_at_line_str: {}",y_val_at_line_str);
                        println!("y_val_at_line_str.len(): {}", y_val_at_line_str.len());
                        row.push_str(&y_val_at_line_str);
                        (0..(max_y_val_character_width-y_val_at_line_str.len())).for_each(|_| row.push(' '));
                        
                    } else {
                        (0..max_y_val_character_width).for_each(|_| row.push(' '));
                    }
                    // push y axis bar 
                    row.push('|');
                    current_render_cols.iter().for_each(|c| {
                        if c.value >= y_val_at_line {
                            row.push('#');
                        } else {
                            row.push(' ');
                        }
                        (0..c.name.len()).for_each(|_| row.push(' '));
                    });
                }
            }
            rows.push(row);
        } 


        // print graph pass
        println!("\n\n");
        rows.iter().rev().for_each(|r| {
            println!("{}", r);
        });


        break;
    }


    return Ok(());
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        names: Vec<String>,
        values: Vec<f32>
    }

    impl From<TestData> for Vec<Column<f32>> {
        fn from(td: TestData) -> GraphData<f32> {
            td.names.iter().zip(td.values.iter()).map(|(n,v)| {
                Column { name: n.clone(), value: *v } }).collect()
        }
    }

    #[test]
    fn test_overall() {
        let names  = vec!["apples","oranges","bananas","grapes"].iter().map(|&s| s.to_owned() ).collect();
        let values = vec![5.0,3.0,8.0,2.0];
        let td = TestData { names, values };
        let values : GraphData<f32> = td.into();

        let gc = GraphConfig::new().max_height(11);

        graph(values, Some(gc)).unwrap();
    }
}
